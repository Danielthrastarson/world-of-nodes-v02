#!/usr/bin/env python3
"""Information absent: replay 169 historical cases plus 48 F11C seed-zero arms.

Successful old output is compared, digested, and discarded from temporary storage;
the original research remains the single raw reference. Failed runs retain their
entire scratch directory as an archive. No historical research file is rewritten.
"""
from concurrent.futures import ThreadPoolExecutor
import csv
import gzip
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tarfile
import tempfile


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    with path.open("x") as f:
        json.dump(value, f, indent=2, sort_keys=True)
        f.write("\n")


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def run_command(command, log, timeout):
    """Bound the whole subprocess tree, including nested historical verifiers."""
    with log.open("x") as f:
        process = subprocess.Popen(
            command, stdout=f, stderr=f, start_new_session=True
        )
        try:
            result = process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            raise
    require(result == 0, f"command failed ({result}); see {log}")


def rows(path):
    with path.open(newline="") as f:
        reader = csv.DictReader(f)
        return reader.fieldnames, list(reader)


def content_digest(path):
    """Hash uncompressed content so gzip container metadata is irrelevant."""
    data = gzip.decompress(path.read_bytes()) if path.suffix == ".gz" else path.read_bytes()
    return {"content_sha256": sha(data), "uncompressed_bytes": len(data)}


def verify_combat(item, temporary):
    case, arm = item
    name = f"{case}_{arm}"
    reference = Path("research/combat/matrix") / name
    output = temporary / "combat" / name
    run_command(
        ["target/release/combat_probe", case, arm, str(output), "1"],
        temporary / f"{name}.log",
        300,
    )
    files = {}
    for filename in ("runs.csv", "people.csv", "activity.csv", "timeline.csv"):
        current = output / filename
        old = reference / filename
        fields, actual = rows(current)
        old_fields, all_expected = rows(old)
        expected = [row for row in all_expected if row["seed"] == "0"]
        require(fields == old_fields and actual == expected, f"{name}/{filename} differs")
        files[filename] = {
            **content_digest(current),
            "reference": str(old),
            "reference_sha256": sha(old.read_bytes()),
            "rows": len(actual),
            "comparison": "all CSV fields and ordered seed-zero rows exactly equal",
        }
    for filename in ("events.tsv", "goods.tsv"):
        current = output / filename
        old = reference / (filename + ".gz")
        lines = gzip.decompress(old.read_bytes()).splitlines(keepends=True)
        expected = lines[0] + b"".join(line for line in lines[1:] if line.startswith(b"0\t"))
        require(current.read_bytes() == expected, f"{name}/{filename} differs")
        files[filename] = {
            **content_digest(current),
            "reference": str(old),
            "reference_sha256": sha(old.read_bytes()),
            "comparison": "uncompressed seed-zero trace bytes exactly equal",
        }
    _, summary = rows(output / "runs.csv")
    require(len(summary) == 1, f"{name}: expected one seed-zero result")
    require(summary[0]["accounting_error"] == "0", f"{name}: accounting error")
    require(summary[0]["overtakes"] == "0", f"{name}: FIFO overtake")
    print(name, "exact outcomes/hashes/traces", flush=True)
    return {
        "case": case,
        "arm": arm,
        "seed": 0,
        "state_hash": summary[0]["state_hash"],
        "event_hash": summary[0]["event_hash"],
        "files": files,
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("verify_information.py NEW_DIRECTORY")
    root = Path(sys.argv[1])
    root.mkdir(parents=True, exist_ok=False)
    manifest_path = Path("research/information/prior-artifacts.json")
    prior = json.loads(manifest_path.read_text())
    require(isinstance(prior, dict) and prior, "missing prior artifact manifest")
    for path, expected in prior.items():
        require(sha(Path(path).read_bytes()) == expected, f"prior artifact changed: {path}")
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    dirty = subprocess.check_output(["git", "status", "--porcelain"], text=True)
    binary_names = (
        "provisioning_probe", "settlement_probe", "infrastructure_probe", "paths_probe",
        "household_probe", "affect_probe", "lifecycle_probe", "mobilization_probe",
        "formation_probe", "combat_probe",
    )
    provenance = {
        "source_revision": revision,
        "working_tree_status_at_start": dirty.splitlines(),
        "binary_sha256": {
            name: sha((Path("target/release") / name).read_bytes())
            for name in binary_names
        },
        "verifier_sha256": {
            str(path): sha(path.read_bytes())
            for path in sorted(Path("scripts").glob("verify_*.py"))
        },
        "prior_manifest": str(manifest_path),
        "prior_manifest_sha256": sha(manifest_path.read_bytes()),
        "prior_files_unchanged": len(prior),
        "interpretation": "regression execution, not new independent social evidence",
    }
    write_json(root / "provenance.json", provenance)
    with tempfile.TemporaryDirectory(prefix="won-information-regression-") as scratch:
        temporary = Path(scratch)
        try:
            run_command(
                [sys.executable, "scripts/verify_combat.py", str(temporary / "older")],
                root / "older.log",
                1800,
            )
            old_validation = json.loads((temporary / "older/validation.json").read_text())
            require(old_validation["historical_executions"] == 169, "old replay count changed")
            write_json(root / "older-validation.json", old_validation)
            matrix = json.loads(Path("research/combat/matrix/provenance.json").read_text())
            tasks = [(case, arm) for case in matrix["cases"] for arm in matrix["arms"]]
            require(len(tasks) == 48, "F11C registered panel differs from 48 arms")
            with ThreadPoolExecutor(max_workers=2) as pool:
                checks = list(pool.map(lambda item: verify_combat(item, temporary), tasks))
            # Preserve fingerprints of successful reproduction, not copies of old raw data.
            digests = {
                str(path.relative_to(temporary)): content_digest(path)
                for path in sorted(temporary.rglob("*")) if path.is_file()
            }
            write_json(root / "replay-digests.json", digests)
            write_json(root / "combat-checks.json", checks)
            for path, expected in prior.items():
                require(sha(Path(path).read_bytes()) == expected, f"prior artifact changed: {path}")
            write_json(root / "validation.json", {
                "source_revision": revision,
                "historical_executions": 169 + len(checks),
                "older_executions": 169,
                "f11c_seed_zero_executions": len(checks),
                "prior_files_unchanged": len(prior),
                "binary_sha256": provenance["binary_sha256"],
                "status": "PASS",
                "exceptions": (
                    "Historical wall runtime and old B2 debug-derived physical_signature "
                    "retain the existing verifier exceptions. Optional disabled information "
                    "may change Config debug parameters, which are fingerprinted but not "
                    "compared. Physical outcomes, authoritative state/event hashes, ordered "
                    "CSV fields and raw seed-zero traces are unchanged."
                ),
                "retention": (
                    "Exact reproduced raw output discarded after comparison; originals remain "
                    "in historical research. replay-digests.json fingerprints generated output; "
                    "combat-checks.json links each F11C reference. No historical file rewritten."
                ),
            })
        except BaseException as error:
            # Retain all evidence, including partial output, when a foundation fails.
            with tarfile.open(root / "failed-replay.tar.gz", "w:gz") as archive:
                archive.add(temporary, arcname="replay")
            write_json(root / "failure.json", {
                "source_revision": revision,
                "error": repr(error),
                "evidence": "failed-replay.tar.gz",
            })
            raise
    print("217 historical executions exactly reproduced; compact evidence retained", flush=True)


if __name__ == "__main__":
    main()
