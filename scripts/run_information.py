#!/usr/bin/env python3
"""F12 physical courier and matched local receipt experiments; immutable output."""
from concurrent.futures import ThreadPoolExecutor
import csv
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys


def write_json(path, value):
    with path.open("x") as f:
        json.dump(value, f, indent=2, sort_keys=True)
        f.write("\n")


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def main():
    if len(sys.argv) not in (2, 3):
        raise SystemExit("run_information.py NEW_DIRECTORY [SEEDS=30]")
    seeds = int(sys.argv[2]) if len(sys.argv) == 3 else 30
    require(seeds > 0, "positive seed count required")
    root = Path(sys.argv[1])
    root.mkdir(parents=True, exist_ok=False)
    source = Path("src/provisioning/information/research.rs").read_text()
    declared = re.search(r"pub const CASES:.*?=\s*&\[(.*?)\];", source, re.S)
    require(declared is not None, "cannot read registered courier cases")
    live = re.findall(r'"([a-z_]+)"', declared.group(1))
    matched = ["matched_fresh", "matched_depleted", "matched_spoiled"]
    cases = live + matched
    require(len(live) == 11 and len(set(cases)) == 14, "registered F12 case list changed")
    arms = ["off", "on"]
    binary = Path("target/release/information_probe")
    provenance = {
        "source_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "working_tree_status_at_start": subprocess.check_output(
            ["git", "status", "--porcelain"], text=True
        ).splitlines(),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "runner_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "fixture_source_sha256": hashlib.sha256(source.encode()).hexdigest(),
        "cases": cases,
        "live_cases": live,
        "matched_cases": matched,
        "arms": arms,
        "seeds": list(range(seeds)),
        "runs": len(cases) * len(arms) * seeds,
        "duration_h": 24,
        "subprocess_timeout_s": 300,
        "live_intervention": (
            "on: controlled seeded courier undertaking; off: no undertaking. "
            "Recipient deliberation is held by research actor filter until6h in both arms, "
            "while ordinary physiology continues. Difference includes courier labor/movement, "
            "not solely receipt of information."
        ),
        "matched_intervention": (
            "Identical real physical courier journey followed by legal local report handoff "
            "versus no handoff before free autonomy; compare the matched recipient choice."
        ),
        "knowledge": "timestamped source observation remains stale; no future/remote truth supplied",
        "validation": "probe runs kernel audit before emitting each completed result; zero FIFO overtakes required",
    }
    write_json(root / "provenance.json", provenance)

    def run(item):
        case, arm = item
        name = f"{case}_{arm}"
        output = root / name
        with (root / (name + ".log")).open("x") as log:
            subprocess.run(
                [str(binary), case, arm, str(output), str(seeds)],
                stdout=log, stderr=log, check=True, timeout=300,
            )
        with (output / "runs.csv").open(newline="") as f:
            rows = list(csv.DictReader(f))
        require(len(rows) == seeds, f"{name}: incomplete run count")
        require([int(row["seed"]) for row in rows] == list(range(seeds)), f"{name}: seeds differ")
        require(all(None not in row and None not in row.values() for row in rows), f"{name}: malformed CSV")
        require(all(row["case"] == case and row["enabled"] == str(arm == "on").lower() for row in rows), f"{name}: mislabeled arm")
        require(all(row["overtakes"] == "0" for row in rows), f"{name}: FIFO overtake")
        with (output / "truth.tsv").open(newline="") as f:
            truth_rows = list(csv.DictReader(f, delimiter="\t"))
        require(all(None not in row and None not in row.values() for row in truth_rows), f"{name}: malformed truth trace")
        require(len(truth_rows) == sum(int(row["received"]) for row in rows), f"{name}: missing receipt truth checkpoints")
        for filename in ("events.tsv", "reports.tsv", "decisions.tsv", "truth.tsv"):
            path = output / filename
            with path.with_suffix(".tsv.gz").open("xb") as f:
                f.write(gzip.compress(path.read_bytes(), mtime=0))
            path.unlink()
        print(
            name, "runs", len(rows), "receipts", sum(int(r["received"]) for r in rows),
            "matched_choices_changed", sum(int(r["matched_choice_changed"]) for r in rows),
            flush=True,
        )
        return name

    try:
        with ThreadPoolExecutor(max_workers=2) as pool:
            completed = list(pool.map(run, [(case, arm) for case in cases for arm in arms]))
        write_json(root / "completion.json", {"status": "PASS", "completed_arms": completed, "runs": provenance["runs"]})
    except BaseException as error:
        write_json(root / "failure.json", {"status": "INCOMPLETE", "error": repr(error), "retention": "all completed and partial outputs preserved"})
        raise


if __name__ == "__main__":
    main()
