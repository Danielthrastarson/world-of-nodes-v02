#!/usr/bin/env python3
"""Paired F12 outcomes. Receipt, transported information and welfare are distinct."""
import csv
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import statistics
import sys


IDENTITY = {"seed", "case", "enabled", "state_hash", "event_hash"}
PHYSICAL = {
    "consumed", "starvation_h", "deaths", "overtakes",
    *[f"person{i}_{suffix}" for i in (0, 1) for suffix in (
        "consumed", "dead", "nutrition_h", "starvation_h", "travel_h", "queue_h", "sleep_h", "gather_h", "idle_h"
    )],
}


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def write_json(path, value):
    with path.open("x") as f:
        json.dump(value, f, indent=2, sort_keys=True)
        f.write("\n")


def number(value):
    if value == "":
        return None
    parsed = float(value)
    require(math.isfinite(parsed), "nonfinite experiment metric")
    return parsed


def describe(values):
    known = [value for value in values if value is not None]
    if not known:
        return {"n": 0, "missing": len(values), "mean": None, "min": None, "max": None, "median": None, "sum": None}
    return {
        "n": len(known), "missing": len(values), "mean": statistics.mean(known),
        "min": min(known), "max": max(known), "median": statistics.median(known), "sum": sum(known),
    }


def fmt(value):
    return "—" if value is None else f"{value:.3f}".rstrip("0").rstrip(".")


def truth_comparisons(root, case, arm, seeds, runs):
    path = root / f"{case}_{arm}" / "truth.tsv.gz"
    with gzip.open(path, "rt", newline="") as f:
        raw = list(csv.DictReader(f, delimiter="\t"))
    seen, records, per_seed = set(), [], {seed: 0 for seed in seeds}
    for row in raw:
        require(None not in row and None not in row.values(), f"{case}/{arm}: malformed truth checkpoint")
        seed, recipient, report = (int(row[key]) for key in ("seed", "recipient", "report"))
        require(seed in per_seed and recipient == 1, f"{case}/{arm}: unexpected truth recipient/seed")
        key = (seed, recipient, report)
        require(key not in seen, f"{case}/{arm}: duplicate report checkpoint")
        seen.add(key)
        per_seed[seed] += 1
        observed, received, checkpoint = (int(row[key]) for key in ("observed_ms", "received_ms", "checkpoint_ms"))
        require(observed <= received <= checkpoint, f"{case}/{arm}: invalid report timestamp order")
        believed, actual = (int(row[key]) for key in ("believed_legal_available", "true_legal_available"))
        require(believed >= 0 and actual >= 0, f"{case}/{arm}: negative availability")
        records.append({
            "case": case, "arm": arm, "seed": seed, "recipient": recipient, "report": report,
            "source_node": int(row["source_node"]),
            "observed_ms": observed, "received_ms": received, "checkpoint_ms": checkpoint,
            "age_at_receipt_h": (received - observed) / 3_600_000,
            "checkpoint_lag_h": (checkpoint - received) / 3_600_000,
            "age_at_checkpoint_h": (checkpoint - observed) / 3_600_000,
            "believed_legal_available": believed, "true_legal_available": actual,
            "quantity_error": believed - actual,
            "quantity_differs": int(believed != actual),
            "positive_belief_zero_true_availability": int(believed > 0 and actual == 0),
        })
    require(all(per_seed[seed] == int(runs[seed]["received"]) for seed in seeds), f"{case}/{arm}: missing received-report checkpoints")
    result = {
        "reports_compared": len(records),
        "runs_with_checkpoint": sum(value > 0 for value in per_seed.values()),
        "quantity_differs_count": sum(row["quantity_differs"] for row in records),
        "positive_belief_zero_true_availability_count": sum(row["positive_belief_zero_true_availability"] for row in records),
        "quantities_equal_count": sum(not row["quantity_differs"] for row in records),
        "distributions": {key: describe([row[key] for row in records]) for key in (
            "age_at_receipt_h", "checkpoint_lag_h", "age_at_checkpoint_h",
            "believed_legal_available", "true_legal_available", "quantity_error",
        )},
        "true_state_scope": "research checkpoint at or just after receipt, never supplied to the recipient",
        "input": str(path), "input_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    }
    return result, records


def matched_destinations(root, case, arm, seeds, runs):
    """This registered matched fixture emits typed Travel decisions in Rust Debug form.

    Intent change includes the food-purpose flag; destination change is narrower.
    Refuse an unexpected schema rather than silently treating it as no change.
    """
    path = root / f"{case}_{arm}" / "decisions.tsv.gz"
    with gzip.open(path, "rt", newline="") as f:
        rows = list(csv.DictReader(f, delimiter="\t"))
    require([int(row["seed"]) for row in rows] == seeds, f"{case}/{arm}: unmatched decision seeds")
    pattern = re.compile(r"intent: Travel \{ edge: (\d+), target: (\d+), food: (true|false) \}")
    details = []
    for row in rows:
        before, after = (pattern.search(row[key]) for key in ("before", "after"))
        require(before is not None and after is not None, f"{case}/{arm}: unexpected matched decision schema")
        seed = int(row["seed"])
        intent_changed = before.groups() != after.groups()
        require(int(intent_changed) == int(runs[seed]["matched_choice_changed"]), f"{case}/{arm}: decision counter differs from trace")
        details.append({
            "seed": seed, "before_edge": int(before[1]), "after_edge": int(after[1]),
            "before_target": int(before[2]), "after_target": int(after[2]),
            "before_food_purpose": before[3] == "true", "after_food_purpose": after[3] == "true",
            "intent_changed": intent_changed, "destination_changed": before[2] != after[2],
        })
    return {
        "destination_changed_runs": sum(row["destination_changed"] for row in details),
        "intent_changed_runs": sum(row["intent_changed"] for row in details),
        "per_seed": details, "input": str(path),
        "input_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    }


def main():
    if len(sys.argv) != 3:
        raise SystemExit("analyze_information.py PANEL_DIRECTORY NEW_ANALYSIS_DIRECTORY")
    root, output = map(Path, sys.argv[1:])
    output.mkdir(parents=True, exist_ok=False)
    provenance = json.loads((root / "provenance.json").read_text())
    require(json.loads((root / "completion.json").read_text())["status"] == "PASS", "panel did not complete")
    seeds = provenance["seeds"]
    require(len(seeds) == len(set(seeds)), "duplicate registered seeds")
    summaries, pairs, rows_out, input_hashes = {}, {}, [], {}
    truth_summary, truth_rows = {}, []
    for case in provenance["cases"]:
        arms = {}
        for arm in ("off", "on"):
            path = root / f"{case}_{arm}" / "runs.csv"
            input_hashes[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
            with path.open(newline="") as f:
                rows = list(csv.DictReader(f))
            require(len(rows) == len(seeds), f"{case}/{arm}: wrong row count")
            require([int(row["seed"]) for row in rows] == seeds, f"{case}/{arm}: paired seed mismatch")
            require(all(row["case"] == case and row["enabled"] == str(arm == "on").lower() for row in rows), f"{case}/{arm}: arm mismatch")
            require(all(None not in row and None not in row.values() for row in rows), f"{case}/{arm}: malformed CSV")
            require(all(row["overtakes"] == "0" for row in rows), f"{case}/{arm}: FIFO violation")
            arms[arm] = {int(row["seed"]): row for row in rows}
        require(arms["on"][seeds[0]].keys() == arms["off"][seeds[0]].keys(), f"{case}: schema mismatch")
        truth_summary[case] = {}
        for arm in ("off", "on"):
            result, records = truth_comparisons(root, case, arm, seeds, arms[arm])
            truth_summary[case][arm] = result
            truth_rows.extend(records)
            input_hashes[result["input"]] = result["input_sha256"]
        metrics = sorted(set(arms["on"][seeds[0]]) - IDENTITY)
        require(PHYSICAL <= set(metrics), f"{case}: missing physical metrics")
        summaries[case] = {
            arm: {metric: describe([number(arms[arm][seed][metric]) for seed in seeds]) for metric in metrics}
            for arm in ("off", "on")
        }
        paired = {}
        for metric in metrics:
            if metric == "runtime_ms":
                continue
            differences, changed = [], 0
            for seed in seeds:
                off, on = (number(arms[arm][seed][metric]) for arm in ("off", "on"))
                delta = None if off is None or on is None else on - off
                changed += int(off != on)
                differences.append(delta)
                rows_out.append({"case": case, "seed": seed, "metric": metric, "off": off, "on": on, "on_minus_off": delta})
            paired[metric] = {**describe(differences), "changed_seeds": changed}
        # Equality covers only these aggregate columns. Route order, endpoint,
        # decision meaning and full physical execution can still differ.
        physical_changed = sum(any(
            number(arms["on"][seed][metric]) != number(arms["off"][seed][metric])
            for metric in PHYSICAL
        ) for seed in seeds)
        destination_choices = {}
        if case in provenance["matched_cases"]:
            for arm in ("off", "on"):
                detail = matched_destinations(root, case, arm, seeds, arms[arm])
                destination_choices[arm] = detail
                input_hashes[detail["input"]] = detail["input_sha256"]
        pairs[case] = {
            "comparison_scope": "matched physical state, handoff only" if case in provenance["matched_cases"] else "seeded courier undertaking plus downstream information effects",
            "runs_with_receipt": {arm: sum(int(arms[arm][seed]["received"]) > 0 for seed in seeds) for arm in ("off", "on")},
            "matched_choice_changed_runs": {arm: sum(int(arms[arm][seed]["matched_choice_changed"]) for seed in seeds) for arm in ("off", "on")},
            "matched_destination_choices": destination_choices,
            "selected_physical_aggregate_changed_pairs": physical_changed,
            "aggregate_scope": sorted(PHYSICAL),
            "paired_metrics": paired,
        }
    summary = {
        "panel": str(root), "source_revision": provenance["source_revision"],
        "binary_sha256": provenance["binary_sha256"], "runs": provenance["runs"],
        "seeds": seeds, "case_statistics": summaries, "paired": pairs,
        "belief_vs_truth_at_receipt_checkpoint": truth_summary,
        "input_sha256": input_hashes,
        "analyzer_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "interpretation": [
            "Live arm comparisons include controlled courier assignment and its physical costs; they do not isolate information alone.",
            "Recipients are experimentally held until6h in live fixtures; this is not spontaneous waiting for reports.",
            "Matched arms share the same physical journey; immediate recipient-choice differences isolate local handoff under that controlled state.",
            "Selected physical aggregate equality does not imply identical paths, endpoint locations or execution. A changed Travel food-purpose flag counts as intent change even when immediate destination is unchanged.",
            "Receipt age is observed_at to received_at, not freshness guaranteed by delivery. Missing receipt times are undefined, never zero-delay deliveries.",
            "Truth is sampled immediately for matched handoffs or at the first live minute checkpoint at/after receipt. Checkpoint lag is reported; mismatch is not automatically evidence of arithmetic error or proof of exact hidden state at receipt.",
            "Positive believed legal availability with zero true legal availability does not imply all physical food disappeared: ownership/access, depletion or spoilage can make remembered food unavailable.",
            "Starvation exposure, food consumption, nutrition, death and labor are separate outcomes. No net-welfare or efficiency score is inferred.",
            "runtime_ms is diagnostic only and is excluded from paired physical-change counts and deltas.",
            "Repeated seeds in a fixed fixture are reproducibility checks, not independent environments or proof of statistical generality.",
        ],
    }
    write_json(output / "summary.json", summary)
    with (output / "paired.csv").open("x", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["case", "seed", "metric", "off", "on", "on_minus_off"])
        writer.writeheader()
        writer.writerows(rows_out)
    with (output / "truth-comparisons.csv").open("x", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=[
            "case", "arm", "seed", "recipient", "report", "source_node",
            "observed_ms", "received_ms", "checkpoint_ms", "age_at_receipt_h",
            "checkpoint_lag_h", "age_at_checkpoint_h", "believed_legal_available",
            "true_legal_available", "quantity_error", "quantity_differs",
            "positive_belief_zero_true_availability",
        ])
        writer.writeheader()
        writer.writerows(truth_rows)
    lines = [
        "# F12 paired information results", "",
        f"{provenance['runs']} runs; {len(seeds)} paired seeds per arm and fixture. Deltas are on minus off; hours are person-hours where accumulated.", "",
        "Live comparisons include seeded courier work and movement. Both arms hold recipient deliberation until6h while physiology continues. Matched comparisons differ only in local receipt after an identical real journey. These are controlled experiments, not autonomous messenger recruitment.", "",
        "| Case | On runs receiving report | Mean receipt age h, received only | Matched intents changed on/off | Matched destinations changed on/off | Selected physical aggregates differ | Δ total starvation h | Δ consumed | Δ deaths |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for case in provenance["cases"]:
        p, s = pairs[case], summaries[case]
        d = p["paired_metrics"]
        changed = p["matched_choice_changed_runs"]
        immediate = f"{changed['on']}/{changed['off']}" if case in provenance["matched_cases"] else "not a matched test"
        destinations = p["matched_destination_choices"]
        target_changes = f"{destinations['on']['destination_changed_runs']}/{destinations['off']['destination_changed_runs']}" if destinations else "not a matched test"
        lines.append(f"| {case} | {p['runs_with_receipt']['on']}/{len(seeds)} | {fmt(s['on']['report_age_h']['mean'])} | {immediate} | {target_changes} | {p['selected_physical_aggregate_changed_pairs']}/{len(seeds)} | {fmt(d['starvation_h']['mean'])} | {fmt(d['consumed']['mean'])} | {fmt(d['deaths']['mean'])} |")
    lines += ["", "Aggregate equality is not execution equality: the measured columns omit route order and final node. In matched fresh/depleted fixtures, intent can switch from exploration to food seeking while retaining the same immediate destination, or reorder equally long trips. `matched_destination_choices` separately records each typed Travel target and purpose flag."]
    lines += [
        "", "Research-only truth comparison at receipt checkpoint (on arm; both arms retained in JSON). No report means no comparison, not an accurate prediction:", "",
        "| Case | Reports compared | Unequal available quantities | Positive belief / zero true availability | Mean observation age at receipt h | Maximum checkpoint lag h |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    for case in provenance["cases"]:
        t = truth_summary[case]["on"]
        dist = t["distributions"]
        lines.append(f"| {case} | {t['reports_compared']} | {t['quantity_differs_count']} | {t['positive_belief_zero_true_availability_count']} | {fmt(dist['age_at_receipt_h']['mean'])} | {fmt(dist['checkpoint_lag_h']['max'])} |")
    lines += ["", "The current-truth sample may lag live receipt by up to the next checkpoint. Quantity mismatch is informational evidence, not automatically a forecasting defect. Legal availability can fall to zero through access/ownership, depletion or spoilage while physical goods remain. Original observation time is never refreshed by delivery."]
    lines += ["", "Courier/recipient deltas are fixed Person0/Person1 cohorts, not role-time accounting:", "", "| Case | Person | Δ travel h | Δ queue h | Δ sleep h | Δ consumed | Δ starvation h | Δ deaths |", "|---|---:|---:|---:|---:|---:|---:|---:|"]
    for case in provenance["cases"]:
        d = pairs[case]["paired_metrics"]
        for person in (0, 1):
            values = [fmt(d[f"person{person}_{metric}"]["mean"]) for metric in ("travel_h", "queue_h", "sleep_h", "consumed", "starvation_h", "dead")]
            lines.append(f"| {case} | {person} | " + " | ".join(values) + " |")
    lines += ["", "No welfare conclusion follows from survival, consumption or receipt alone. Reduced starvation may coexist with displaced work; changed consumption is not proof of nourishment. Missing report age means no received report. `summary.json` retains all per-arm statistics, paired metrics, missing values and reproducibility hashes; `paired.csv` retains each seed's deltas. Runtime is excluded from causal differences.", ""]
    with (output / "SUMMARY.md").open("x") as f:
        f.write("\n".join(lines))
    print(provenance["runs"], "runs analyzed with exact paired seeds; no aggregate welfare claim", flush=True)


if __name__ == "__main__":
    main()
