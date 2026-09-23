#!/usr/bin/env python3
"""Immutable F13B paired command-chain runs; never overwrite prior evidence."""
import argparse
from concurrent.futures import FIRST_COMPLETED, ThreadPoolExecutor, wait
import csv
import gzip
import hashlib
import json
from pathlib import Path
import subprocess


CASES = ("local", "distant", "late", "stale", "revocation", "death",
         "scope", "resource", "refusal", "integrated")
ARMS = ("control", "order")


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    with path.open("x") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path, help="new output directory")
    parser.add_argument("seeds", type=int, nargs="?", default=30)
    parser.add_argument("--cases", nargs="+", choices=CASES, default=list(CASES))
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--timeout", type=int, default=300)
    parser.add_argument("--compress-traces", action="store_true")
    args = parser.parse_args()
    require(args.seeds > 0, "positive seed count required")
    require(1 <= args.workers <= 4, "use one to four bounded workers")
    require(args.timeout > 0, "positive timeout required")
    require(len(set(args.cases)) == len(args.cases), "duplicate case")
    binary = Path("target/release/command_chain_probe")
    require(binary.is_file(), "build cargo build --release --bin command_chain_probe first")
    registration = Path("docs/COMMAND_CHAIN_EXPERIMENT.md")
    require(registration.is_file(), "missing experiment registration")
    source_files = sorted(Path("src").rglob("*.rs"))
    source_files += [p for p in (Path("Cargo.toml"), Path("Cargo.lock")) if p.exists()]
    source_manifest = {str(path): digest(path) for path in source_files}
    tasks = [(case, arm, seed) for case in args.cases for arm in ARMS
             for seed in range(args.seeds)]
    root = args.directory
    root.mkdir(parents=True, exist_ok=False)
    provenance = {
        "source_revision": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], text=True).strip(),
        "working_tree_status": subprocess.check_output(
            ["git", "status", "--porcelain"], text=True).splitlines(),
        "binary_sha256": digest(binary), "runner_sha256": digest(Path(__file__)),
        "registration_sha256": digest(registration),
        "source_manifest": source_manifest,
        "cases": args.cases, "arms": list(ARMS),
        "seeds": list(range(args.seeds)), "runs": len(tasks),
        "full_registered_panel": set(args.cases) == set(CASES) and args.seeds >= 30,
        "duration_h": {case: 72 if case == "integrated" else 24 for case in args.cases},
        "subprocess_timeout_s": args.timeout, "workers": args.workers,
        "probe_interface": "command_chain_probe CASE ARM SEED NEW_OUTDIR",
        "intervention": "Physically delivered scoped sovereign order versus no order; paired physical initial conditions and explicit office acceptance.",
        "scope": "Controlled ruler intent and courier purpose; office duty competes within the common Person policy. No autonomous recruitment or guaranteed obedience.",
        "outcome_separation": "Command commission completion is distinct from downstream construction success; refusal is distinct from resource/physical failure.",
        "compressed_traces": args.compress_traces,
    }
    write_json(root / "provenance.json", provenance)
    completed = {}

    def run(task):
        case, arm, seed = task
        name = f"{case}_{arm}_{seed:03}"
        output = root / name
        with (root / f"{name}.log").open("x") as log:
            subprocess.run([str(binary), case, arm, str(seed), str(output)],
                           stdout=log, stderr=log, check=True, timeout=args.timeout)
        for filename in ("results.csv", "events.tsv", "parameters.txt"):
            require((output / filename).is_file(), f"{name}: missing {filename}")
        with (output / "results.csv").open(newline="") as stream:
            reader = csv.DictReader(stream)
            require(reader.fieldnames and len(set(reader.fieldnames)) == len(reader.fieldnames),
                    f"{name}: missing/duplicate result header")
            rows = list(reader)
        require(len(rows) == 1, f"{name}: expected exactly one run")
        row = rows[0]
        require(None not in row and None not in row.values(), f"{name}: malformed CSV")
        require(row.get("case") == case and row.get("arm") == arm
                and row.get("seed") == str(seed), f"{name}: mismatched case/arm/seed")
        require(row.get("audit_ok") == "1", f"{name}: kernel audit did not pass")
        require(row.get("overtakes") == "0", f"{name}: FIFO overtake")
        require(row.get("state_hash") and row.get("event_hash"), f"{name}: missing hashes")
        with (output / "events.tsv").open(newline="") as stream:
            reader = csv.DictReader(stream, delimiter="\t")
            require(reader.fieldnames, f"{name}: missing event header")
            for event in reader:
                require(None not in event and None not in event.values(),
                        f"{name}: malformed event trace")
        if args.compress_traces:
            for path in output.glob("*.tsv"):
                with path.with_suffix(".tsv.gz").open("xb") as stream:
                    stream.write(gzip.compress(path.read_bytes(), mtime=0))
                path.unlink()
        print(f"{name}: audited", flush=True)
        return row

    try:
        # Keep the queue bounded so a failed invariant stops expansion. At most
        # workers-1 already-running cases can finish after the first failure.
        remaining = iter(tasks)
        with ThreadPoolExecutor(max_workers=args.workers) as pool:
            pending = {}
            for _ in range(args.workers):
                task = next(remaining, None)
                if task is not None:
                    pending[pool.submit(run, task)] = task
            while pending:
                ready, _ = wait(pending, return_when=FIRST_COMPLETED)
                for future in ready:
                    task = pending.pop(future)
                    completed[task] = future.result()
                for _ in ready:
                    task = next(remaining, None)
                    if task is not None:
                        pending[pool.submit(run, task)] = task
        # Column union permits integration-only diagnostics without silently
        # discarding them. Each run's native one-row file remains authoritative.
        fields = sorted({key for row in completed.values() for key in row})
        with (root / "runs.csv").open("x", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=fields)
            writer.writeheader()
            writer.writerows(completed[task] for task in tasks)
        write_json(root / "completion.json", {
            "status": "PASS", "runs": len(completed),
            "meaning": "All registered runs finished and passed kernel/FIFO validation, not an automatic milestone gate decision.",
            "completed": [f"{c}_{a}_{s:03}" for c, a, s in tasks],
        })
    except BaseException as error:
        write_json(root / "failure.json", {
            "status": "INCOMPLETE", "error": repr(error),
            "completed_runs_validated": len(completed),
            "retention": "All completed and partial outputs retained; no new cases scheduled after detected failure.",
        })
        raise


if __name__ == "__main__":
    main()
