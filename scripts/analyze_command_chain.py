#!/usr/bin/env python3
"""Describe F13B paired evidence without turning execution checks into gate claims."""
import argparse
from collections import Counter
import csv
import hashlib
import json
import math
from pathlib import Path
from statistics import fmean


CASES = ("local", "distant", "late", "stale", "revocation", "death",
         "scope", "resource", "refusal", "integrated")
ARMS = ("control", "order")
IDENTITY = {"case", "arm", "seed", "state_hash", "event_hash"}
TIMING = {"elapsed_ms", "runtime_ms"}
PHYSICAL = ("deaths", "consumed", "spoiled", "delivered", "starvation_h", "queue_h",
            "constructed", "wood_committed", "construction_labor_h", "improved_trips",
            "farm_output", "food_stolen", "care_food")
FUNNEL = ("orders_issued", "orders_transmitted", "orders_received", "orders_valid",
          "orders_invalid", "obligations_formed", "obligation_Received",
          "obligation_Active", "obligation_Paused", "obligation_Completed",
          "obligation_Failed", "obligation_Expired", "obligation_Refused",
          "obligation_Revoked", "revocations_sent", "revocations_received",
          "commissioned", "constructed")


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def number(value):
    if value in (None, ""):
        return None
    try:
        result = float(value)
    except ValueError:
        return None
    return result if math.isfinite(result) else None


def stats(values):
    if not values:
        return {"n": 0, "mean": None, "min": None, "max": None, "sum": None}
    return {"n": len(values), "mean": fmean(values), "min": min(values),
            "max": max(values), "sum": sum(values)}


def write_json(path, value):
    with path.open("x") as stream:
        json.dump(value, stream, indent=2, sort_keys=True, allow_nan=False)
        stream.write("\n")


def show(value):
    if value is None:
        return "—"
    return f"{value:.6f}".rstrip("0").rstrip(".")


def cell(summary, column):
    metric = summary["numeric"].get(column)
    if metric is None or metric["mean"] is None:
        return "—"
    bounds = "" if metric["min"] == metric["max"] else f" [{show(metric['min'])}, {show(metric['max'])}]"
    missing = "" if metric["n"] == summary["runs"] else f" (n={metric['n']})"
    return show(metric["mean"]) + bounds + missing


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("matrix", type=Path, help="immutable runner output directory")
    parser.add_argument("output", type=Path, help="new analysis directory")
    args = parser.parse_args()
    source = args.matrix / "runs.csv"
    with source.open(newline="") as stream:
        reader = csv.DictReader(stream)
        fields = reader.fieldnames
        require(fields and len(set(fields)) == len(fields), "missing or duplicate columns")
        rows = list(reader)
    require(rows, "no completed runs")
    require(all(None not in row and None not in row.values() for row in rows), "malformed CSV")
    require(IDENTITY <= set(fields), "missing identity/hash columns")
    indexed = {}
    for row in rows:
        require(row["case"] in CASES and row["arm"] in ARMS, "unregistered case or arm")
        key = (row["case"], row["arm"], int(row["seed"]))
        require(key not in indexed, f"duplicate run {key}")
        indexed[key] = row
    cases = [case for case in CASES if any(row["case"] == case for row in rows)]
    numerical = [field for field in fields if field not in IDENTITY
                 and any(number(row[field]) is not None for row in rows)
                 and all(row[field] == "" or number(row[field]) is not None for row in rows)]
    groups = {}
    for case in cases:
        groups[case] = {}
        for arm in ARMS:
            subset = [row for row in rows if row["case"] == case and row["arm"] == arm]
            groups[case][arm] = {
                "runs": len(subset), "seeds": sorted(int(row["seed"]) for row in subset),
                "numeric": {field: stats([number(row[field]) for row in subset
                                           if number(row[field]) is not None])
                            for field in numerical},
                "categorical": {field: dict(sorted(Counter(row[field] for row in subset).items()))
                                for field in fields if field not in numerical and field not in IDENTITY},
            }
    paired_rows = []
    paired = {}
    missing_pairs = []
    for case in cases:
        seeds = sorted({seed for c, _, seed in indexed if c == case})
        per_case = []
        for seed in seeds:
            control = indexed.get((case, "control", seed))
            order = indexed.get((case, "order", seed))
            if control is None or order is None:
                missing_pairs.append({"case": case, "seed": seed,
                                      "missing": "control" if control is None else "order"})
                continue
            pair = {"case": case, "seed": seed}
            changed = [field for field in fields if field not in IDENTITY | TIMING
                       and control[field] != order[field]]
            physical = [field for field in PHYSICAL if field in fields
                        and control[field] != order[field]]
            pair["changed_outcome_fields"] = "|".join(changed)
            pair["changed_physical_fields"] = "|".join(physical)
            pair["physical_outcome_changed"] = int(bool(physical))
            pair["state_hash_changed"] = int(control["state_hash"] != order["state_hash"])
            pair["event_hash_changed"] = int(control["event_hash"] != order["event_hash"])
            for field in numerical:
                a, b = number(control[field]), number(order[field])
                pair[f"control_{field}"] = control[field]
                pair[f"order_{field}"] = order[field]
                pair[f"delta_{field}"] = b - a if a is not None and b is not None else ""
            for field in ("project_outcome",):
                if field in fields:
                    pair[f"control_{field}"] = control[field]
                    pair[f"order_{field}"] = order[field]
            paired_rows.append(pair)
            per_case.append(pair)
        paired[case] = {
            "pairs": len(per_case),
            "physical_outcome_changed_seeds": sum(p["physical_outcome_changed"] for p in per_case),
            "state_hash_changed_seeds": sum(p["state_hash_changed"] for p in per_case),
            "event_hash_changed_seeds": sum(p["event_hash_changed"] for p in per_case),
            "deltas_order_minus_control": {
                field: {**stats([p[f"delta_{field}"] for p in per_case
                                if p[f"delta_{field}"] != ""]),
                        "nonzero_pairs": sum(p[f"delta_{field}"] != "" and p[f"delta_{field}"] != 0
                                             for p in per_case),
                        "unpaired_value_count": sum(p[f"delta_{field}"] == "" for p in per_case)}
                for field in numerical},
        }
    failures = [{"case": r["case"], "arm": r["arm"], "seed": r["seed"],
                 "audit_ok": r.get("audit_ok"), "overtakes": r.get("overtakes")}
                for r in rows if r.get("audit_ok") != "1" or r.get("overtakes") != "0"]
    full_panel = (set(cases) == set(CASES) and not missing_pairs
                  and all(groups[c][a]["seeds"] == list(range(30)) for c in cases for a in ARMS))
    observations = {}
    for case in cases:
        order_rows = [r for r in rows if r["case"] == case and r["arm"] == "order"]
        observations[case] = {
            "ordered_runs": len(order_rows),
            "runs_with": {field: sum((number(r.get(field)) or 0) > 0 for r in order_rows)
                          for field in FUNNEL if field in fields},
            "project_outcomes": dict(sorted(Counter(r.get("project_outcome", "") for r in order_rows).items())),
        }
    summary = {
        "source": str(source), "runs_csv_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "analyzer_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "run_count": len(rows), "paired_count": len(paired_rows),
        "complete_registered_600_run_panel": full_panel,
        "validation_failures": failures, "missing_pairs": missing_pairs,
        "groups": groups, "paired": paired, "raw_gate_observations": observations,
        "scope": "Descriptive evidence only. Successful subprocesses, message counts and physical differences do not automatically pass F13.",
        "interpretation_limits": [
            "Institutional command completion means local commission, not completed construction.",
            "Controlled refusal tests an explicit choice path, not emergent political psychology.",
            "Message transmission includes relay contacts; it is not identical to addressed-office receipt.",
            "A changed state/event hash includes message/institutional records and is not proof of changed physical behavior.",
            "Means show available observations; missing receipt times are not zero latency.",
            "Stale-case physical change does not by itself establish an issuer's remembered food belief or motive.",
            "Resource failure, expiry and nonreceipt are not voluntary refusal.",
            "Controlled initial intent, local acceptance and courier purposes are not spontaneous government initiative.",
            "No aggregate efficiency/benefit claim follows from construction or throughput alone.",
        ],
    }
    args.output.mkdir(parents=True, exist_ok=False)
    write_json(args.output / "summary.json", summary)
    with (args.output / "paired.csv").open("x", newline="") as stream:
        pair_fields = ["case", "seed"] + sorted({k for row in paired_rows for k in row} - {"case", "seed"})
        writer = csv.DictWriter(stream, fieldnames=pair_fields)
        writer.writeheader()
        writer.writerows(paired_rows)
    lines = ["# F13B command-chain evidence", "",
             f"Runs: {len(rows)}. Matched pairs: {len(paired_rows)}. Full registered 600-run panel: {full_panel}.",
             f"Kernel/FIFO failures: {len(failures)}. Missing paired runs: {len(missing_pairs)}.", "",
             "Values are means; nonconstant ranges appear in brackets. Available-observation denominators are shown where receipt times are missing. Paired differences are order minus control.", "",
             "These are descriptive findings. The causal trace review and milestone decision remain separate.", "",
             "## Institutional funnel", "",
             "| Case / arm | Received | Valid | Invalid | Completed duties | Refused | Expired | Revoked | Commissions | Crossings | Receipt h |",
             "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"]
    funnel_columns = ("orders_received", "orders_valid", "orders_invalid", "obligation_Completed",
                      "obligation_Refused", "obligation_Expired", "obligation_Revoked",
                      "commissioned", "constructed", "first_receipt_h")
    for case in cases:
        for arm in ARMS:
            lines.append("| " + f"{case} / {arm}" + " | " + " | ".join(cell(groups[case][arm], c) for c in funnel_columns) + " |")
    lines += ["", "A completed duty authorizes a local project; construction still requires voluntary resources and labor. Relay transmissions are counted separately from addressed-office receipt in the raw results.", "",
              "## Physical outcomes", "",
              "| Case / arm | Food produced | Consumed | Deaths | Starvation h | Queue h | Care food | Construction labor h | Food stolen |",
              "|---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for case in cases:
        for arm in ARMS:
            lines.append("| " + f"{case} / {arm}" + " | " + " | ".join(cell(groups[case][arm], c) for c in
                         ("farm_output", "consumed", "deaths", "starvation_h", "queue_h", "care_food", "construction_labor_h", "food_stolen")) + " |")
    lines += ["", "## Paired changes", "",
              "| Case | Pairs | Seeds with changed physical outcomes | Δ produced | Δ consumed | Δ deaths | Δ starvation h | Δ queue h |",
              "|---|---:|---:|---:|---:|---:|---:|---:|"]
    for case in cases:
        p = paired[case]
        deltas = p["deltas_order_minus_control"]
        lines.append(f"| {case} | {p['pairs']} | {p['physical_outcome_changed_seeds']} | " +
                     " | ".join(show(deltas.get(c, {}).get("mean")) for c in
                                ("farm_output", "consumed", "deaths", "starvation_h", "queue_h")) + " |")
    lines += ["", "Changed physical outcomes refers only to the explicitly listed production, consumption, death, starvation, queue, care, goods and construction measures. It is not inferred from hashes or command bookkeeping.", "",
              "## Raw gate observations", ""]
    for case in cases:
        o = observations[case]
        count = o["runs_with"]
        lines.append(f"* `{case}`: {o['ordered_runs']} ordered runs; receipt in {count.get('orders_received', 0)}, completed commission duty in {count.get('obligation_Completed', 0)}, constructed crossing in {count.get('constructed', 0)}. Project outcomes: {o['project_outcomes']}.")
    lines += ["", "## Interpretation limits", ""]
    lines.extend("* " + limitation for limitation in summary["interpretation_limits"])
    lines += ["", "All numeric means/ranges, categorical counts, paired deltas and input hashes are preserved in `summary.json`; every seed comparison is retained in `paired.csv`. No previous research is rewritten.", ""]
    with (args.output / "SUMMARY.md").open("x") as stream:
        stream.write("\n".join(lines))
    print(f"Analyzed {len(rows)} runs and {len(paired_rows)} pairs; full panel={full_panel}; invariant failures={len(failures)}.")


if __name__ == "__main__":
    main()
