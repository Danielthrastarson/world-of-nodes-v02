#!/usr/bin/env python3
"""Review one exact F13A clock-fix discrepancy; never relax the strict verifier.

This is an allowlist of independently inspected artifact contents, not a generic
hash exception. Any other byte, outcome, timestamp or hash requires a new review.
"""
import csv
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys

from verify_command_regressions import compare_panel


PINS = {
    "institutions.tsv.gz": (
        "318c614c6c6fe7acd0528f24002f79e597dc5553beb79525f6c14150cd03c2cf",
        "3fe33b28ad251b45f0894f66496d2127b2401790efea4cbb941a5d883c9a8e87"),
    "events.tsv.gz": (
        "38e16e714838226f6174220c19db200c9670ade8afe1ffd532982a10d9c218da",
        "1267f632522ca9b623f350d9496f788cc7804bd87d060502c603b2bc80d17f56"),
    "runs.csv": (
        "57f0fa31bc85f0ea0db26a1d9973ee83cc4c1586549c90d91c593835466d030f",
        "a8ac91fe291ec8990b1e44e1c6a6d4e3afcb44b15b9a0ea13e1a4a4366490c35"),
    "timeline.csv": (
        "974248a56b57a4e91289661d9600305537e6a5559e9332522028aa2b2e13efba",
        "0844a8c14a51b051cf8d79ff905ebbb07e9bc5dff006bb2a66a23c76bf9d8391"),
}


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def table(path):
    opener = gzip.open if path.suffix == ".gz" else open
    with opener(path, "rt", newline="") as stream:
        reader = csv.DictReader(stream, delimiter="\t" if ".tsv" in path.name else ",")
        return reader.fieldnames, list(reader)


def raw(path):
    data = path.read_bytes()
    return gzip.decompress(data) if path.suffix == ".gz" else data


def pinned_content(path):
    if path.suffix != ".csv":
        return raw(path)
    fields, rows = table(path)
    # Wall runtime remains the preexisting replay exception. State/event hashes
    # are retained in this exact canonical content fingerprint.
    fields = [field for field in fields if field != "elapsed_ms"]
    return json.dumps({"fields": fields, "rows": [[row[field] for field in fields] for row in rows]},
                      separators=(",", ":")).encode()


def verify_events(old, new):
    old_fields, before = table(old)
    new_fields, after = table(new)
    require(old_fields == new_fields and len(before) == 100566 and len(after) == 100564,
            "unreviewed event schema/count")
    require(all(row["seed"] == "0" and int(row["id"]) == i for i, row in enumerate(before)),
            "old event identity differs")
    require(all(row["seed"] == "0" and int(row["id"]) == i for i, row in enumerate(after)),
            "new event identity differs")
    # Construct the complete corrected trace from the original. No other event
    # contents are ignored. Move the three assembly events to the ordinary next
    # policy decision, replacing the now-redundant duty selection. Shift the same
    # twelve-hour term's end and remove its redundant early Wait decision.
    early = [dict(row) for row in before if row["person"] == "Some(1)"
             and row["time_ms"] == "108232800"]
    require([row["kind"] for row in early] == ["Decision", "UnitAssembled", "MusterArrived"],
            "unexpected early assembly events")
    expected = []
    removed_decisions = []
    moved_end = 0
    for source in before:
        row = dict(source)
        if row["person"] == "Some(1)" and row["time_ms"] == "108232800":
            continue
        if row["person"] == "Some(1)" and row["time_ms"] == "108240000" and row["kind"] == "Decision":
            require(row["detail"] == "intent=MilitaryDuty nutrition_ms=86160000 starvation_ms=0 reason=voluntary announced service term",
                    "unexpected replacement decision")
            removed_decisions.append(int(row["id"]))
            for original in early:
                moved = dict(original)
                moved["time_ms"] = "108240000"
                moved["detail"] = moved["detail"].replace("nutrition_ms=86167200", "nutrition_ms=86160000")
                moved["detail"] = moved["detail"].replace("travel_ms=52800", "travel_ms=60000").replace("until=151432800", "until=151440000")
                expected.append(moved)
            continue
        if row["person"] == "Some(1)" and row["time_ms"] == "151432800":
            if row["kind"] == "Decision":
                require(row["detail"] == "intent=Wait nutrition_ms=64567200 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed",
                        "unexpected removed post-service decision")
                removed_decisions.append(int(row["id"]))
                continue
            require(row["kind"] in ("Interrupted", "UnitServiceEnded", "ServiceEnded"),
                    "unexpected service-end event")
            row["time_ms"] = "151440000"
            moved_end += 1
        expected.append(row)
    require(removed_decisions == [40339, 56727] and moved_end == 3, "unreviewed event changes")
    mapping = {int(row["id"]): i for i, row in enumerate(expected)}
    changed_references = []
    for i, row in enumerate(expected):
        row["id"] = str(i)

        def remap(match):
            previous = int(match.group(1))
            require(previous in mapping, "causal pointer references removed event")
            replacement = mapping[previous]
            if previous != replacement:
                changed_references.append({"event": i, "old_source": previous, "new_source": replacement})
            return f"source_event: {replacement}"

        row["detail"] = re.sub(r"source_event: (\d+)", remap, row["detail"])
    require(len(changed_references) == 5, "unexpected causal-reference changes")
    require(expected == after, "events differ beyond the exact reviewed clock transformation")
    return {"old_events": len(before), "new_events": len(after),
            "removed_redundant_decision_ids": removed_decisions, "causal_reference_remapping": changed_references,
            "result": "Every event field matches the explicit timestamp/selection/identity transformation"}


def main():
    if len(sys.argv) != 4:
        raise SystemExit("audit_command_clock_regression.py OLD_F13A_PANEL NEW_F13A_PANEL NEW_REVIEW_JSON")
    old_root, new_root, output = map(Path, sys.argv[1:])
    require(not output.exists(), "review output already exists")
    result = {"status": "FAIL", "old_panel": str(old_root), "new_panel": str(new_root),
              "reviewer_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              "strict_replay_status": "FAIL", "scope": "Exact disclosed volunteer-clock correction; no general regression exception"}
    try:
        strict = compare_panel(old_root, new_root, "F13A", 180, "elapsed_ms")
        expected_scopes = {"long_commissioned/" + filename for filename in PINS}
        require(strict["status"] == "FAIL" and len(strict["differences"]) == 4
                and {d["scope"] for d in strict["differences"]} == expected_scopes,
                "regression footprint differs from reviewed four artifacts")
        result["strict_differences"] = strict["differences"]
        old, new = old_root / "long_commissioned", new_root / "long_commissioned"
        result["pinned_content"] = {}
        for filename, expected in PINS.items():
            actual = tuple(hashlib.sha256(pinned_content(folder / filename)).hexdigest() for folder in (old, new))
            require(actual == expected, f"exact reviewed contents changed: {filename}")
            result["pinned_content"][filename] = {"old": actual[0], "new": actual[1]}
        _, old_runs = table(old / "runs.csv")
        _, new_runs = table(new / "runs.csv")
        for a, b in zip(old_runs, new_runs):
            changed = {field for field in a if a[field] != b[field]} - {"elapsed_ms"}
            require(changed == {"state_hash", "event_hash"}, "final measured physical outcomes changed")
        _, a_timeline = table(old / "timeline.csv")
        _, b_timeline = table(new / "timeline.csv")
        changed_physical = 0
        for a, b in zip(a_timeline, b_timeline):
            t = float(a["hour"])
            changed = {field for field in a if a[field] != b[field]}
            wanted = {"state_hash", "event_hash"} if t >= 30.25 else set()
            if 30.25 <= t <= 42:
                wanted |= {"other_initial_people_idle_h", "other_initial_people_military_duty_h"}
                for field, delta in (("other_initial_people_idle_h", .002),
                                     ("other_initial_people_military_duty_h", -.002)):
                    require(abs(float(b[field]) - float(a[field]) - delta) < 1e-12,
                            "activity shift differs from 7,200ms")
                changed_physical += 1
            require(changed == wanted, "unreviewed timeline field/time changed")
        require(changed_physical == 48 * 30, "unexpected transient activity window")
        roster_before, roster_after = raw(old / "institutions.tsv.gz"), raw(new / "institutions.tsv.gz")
        require(roster_before.count(b"108232800") == roster_before.count(b"151432800") == 30,
                "unexpected original roster times")
        require(roster_before.replace(b"108232800", b"108240000").replace(b"151432800", b"151440000") == roster_after,
                "other institutional state changed")
        result["trace_transformation"] = verify_events(old / "events.tsv.gz", new / "events.tsv.gz")
        result.update({
            "status": "REVIEWED_TECHNICAL_CORRECTION",
            "strict_replay_status": "FAIL_RETAINED",
            "exact_unaffected_runs": 150,
            "affected_runs": 30,
            "final_nonruntime_nonhash_csv_values_exact": True,
            "people_rows_exact": 540, "final_activity_rows_exact": 2070,
            "shift_ms": 7200, "affected_person": 1,
            "old_assembly_ms": 108232800, "new_assembly_ms": 108240000,
            "old_term_end_ms": 151432800, "new_term_end_ms": 151440000,
            "transient_activity_checkpoints": changed_physical,
            "conclusion": "Accept the independently bounded mechanical clock correction with disclosure; retain original raw evidence and strict failure. Do not claim all historical hashes replay exactly.",
        })
    except BaseException as error:
        result["error"] = repr(error)
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(result["status"], flush=True)
    if result["status"] == "FAIL":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
