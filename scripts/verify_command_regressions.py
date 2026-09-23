#!/usr/bin/env python3
"""Replay 217 historical controls, all 840 F12 runs, and all 180 F13A runs.

Creates new evidence only. The sole new parameter normalization is the explicit
disabled `commands: false, ` Config field. Wall runtimes are the only CSV value
exceptions. Every other field and uncompressed trace byte must remain exact.
"""
import csv
import gzip
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys


PARAMETER_ADDITIONS = (b"commands: false, ", b"    commands: false,\n", b"    commands: false,\\n")

def normalize_parameters(data):
    for addition in PARAMETER_ADDITIONS:
        data = data.replace(addition, b"")
    return data


def require(condition, detail):
    if not condition:
        raise RuntimeError(detail)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    with path.open("x") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")


def run(command, log, timeout=3600):
    """Bound the entire nested runner process group; retain all output on failure."""
    print("Running", " ".join(map(str, command)), flush=True)
    with log.open("x") as stream:
        process = subprocess.Popen(command, stdout=stream, stderr=stream, start_new_session=True)
        try:
            status = process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            raise
    require(status == 0, f"subprocess returned {status}; see {log}")


def content(path):
    raw = path.read_bytes()
    return gzip.decompress(raw) if path.suffix == ".gz" else raw


def csv_rows(path):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        fields, rows = reader.fieldnames, list(reader)
    require(fields and len(set(fields)) == len(fields), f"missing/duplicate CSV columns: {path}")
    require(all(None not in row and None not in row.values() for row in rows), f"malformed CSV: {path}")
    return fields, rows


def compare_panel(old_root, new_root, name, expected, runtime_field):
    """Compare every artifact within each prescribed case/arm directory."""
    result = {"panel": name, "old_root": str(old_root), "new_root": str(new_root),
              "differences": [], "checks": [], "runs": 0,
              "csv_exception": f"Only {runtime_field} values; column identity/order remains exact",
              "parameter_exception": "Remove only disabled commands:false Config field in compact, pretty or escaped-pretty layout; all remaining bytes exact"}
    differences = result["differences"]

    def fail(scope, reason, **detail):
        differences.append({"scope": scope, "reason": reason, **detail})

    old_provenance = json.loads((old_root / "provenance.json").read_text())
    new_provenance = json.loads((new_root / "provenance.json").read_text())
    for label, root, provenance in (("old", old_root, old_provenance),
                                     ("new", new_root, new_provenance)):
        require(json.loads((root / "completion.json").read_text()).get("status") == "PASS",
                f"{name}/{label}: incomplete panel")
        require(provenance.get("source_revision") and provenance.get("binary_sha256"),
                f"{name}/{label}: missing source/binary provenance")
        require(provenance["runs"] == expected, f"{name}/{label}: wrong registered run count")
        require(provenance["seeds"] == list(range(30)), f"{name}/{label}: expected seeds 0–29")
        result[f"{label}_source_revision"] = provenance["source_revision"]
        result[f"{label}_provenance_sha256"] = sha((root / "provenance.json").read_bytes())
        result[f"{label}_binary_sha256"] = provenance["binary_sha256"]
    keys = ["cases", "arms", "seeds", "runs", "duration_h"]
    if name == "F12":
        keys += ["live_cases", "matched_cases"]
    for key in keys:
        if old_provenance.get(key) != new_provenance.get(key):
            fail("registration", f"{key} changed", old=old_provenance.get(key), new=new_provenance.get(key))
    folders = {f"{case}_{arm}" for case in old_provenance["cases"] for arm in old_provenance["arms"]}
    require(len(folders) * 30 == expected, f"{name}: duplicate registered cases/arms")
    for label, root in (("old", old_root), ("new", new_root)):
        actual = {path.name for path in root.iterdir() if path.is_dir()}
        if actual != folders:
            fail(label, "arm directory set changed", missing=sorted(folders - actual), extra=sorted(actual - folders))
    for folder in sorted(folders):
        old_dir, new_dir = old_root / folder, new_root / folder
        if not old_dir.is_dir() or not new_dir.is_dir():
            continue
        old_files = {str(path.relative_to(old_dir)) for path in old_dir.rglob("*") if path.is_file()}
        new_files = {str(path.relative_to(new_dir)) for path in new_dir.rglob("*") if path.is_file()}
        if old_files != new_files:
            fail(folder, "artifact file set changed", missing=sorted(old_files - new_files), extra=sorted(new_files - old_files))
        for filename in sorted(old_files & new_files):
            before_path, after_path = old_dir / filename, new_dir / filename
            scope = f"{folder}/{filename}"
            before, after = content(before_path), content(after_path)
            check = {"scope": scope, "old_content_sha256": sha(before), "new_content_sha256": sha(after),
                     "old_bytes": len(before), "new_bytes": len(after)}
            if filename.endswith(".csv"):
                old_fields, old_rows = csv_rows(before_path)
                new_fields, new_rows = csv_rows(after_path)
                check.update(comparison=f"Ordered CSV exact except {runtime_field} values",
                             old_rows=len(old_rows), new_rows=len(new_rows))
                if old_fields != new_fields:
                    fail(scope, "CSV column identity/order changed", old=old_fields, new=new_fields)
                if len(old_rows) != len(new_rows):
                    fail(scope, "CSV row count changed", old=len(old_rows), new=len(new_rows))
                samples, changed_values = [], 0
                for index, (old_row, new_row) in enumerate(zip(old_rows, new_rows)):
                    for field in old_fields:
                        if field == runtime_field or field not in new_row:
                            continue
                        if old_row[field] != new_row[field]:
                            changed_values += 1
                            if len(samples) < 50:
                                samples.append({"row": index, "seed": old_row.get("seed"), "field": field,
                                                "old": old_row[field], "new": new_row[field]})
                if changed_values:
                    fail(scope, "semantic CSV values changed", changed_values=changed_values,
                         first_differences=samples, all_raw_rows_preserved=True)
                if filename == "runs.csv":
                    result["runs"] += len(new_rows)
                    for label, table in (("old", old_rows), ("new", new_rows)):
                        if [int(row["seed"]) for row in table] != list(range(30)):
                            fail(scope, f"{label} ordered seeds differ from 0–29")
                result["checks"].append(check)
                continue
            if filename == "parameters.txt":
                check["comparison"] = "Exact bytes after removing only explicit disabled commands Config field"
                check["old_disabled_command_fields"] = sum(before.count(p) for p in PARAMETER_ADDITIONS)
                check["new_disabled_command_fields"] = sum(after.count(p) for p in PARAMETER_ADDITIONS)
                before = normalize_parameters(before)
                after = normalize_parameters(after)
                check["old_normalized_sha256"] = sha(before)
                check["new_normalized_sha256"] = sha(after)
            else:
                check["comparison"] = "Exact uncompressed bytes; no semantic exception"
            if before != after:
                first = next((i for i, pair in enumerate(zip(before, after)) if pair[0] != pair[1]),
                             min(len(before), len(after)))
                fail(scope, "uncompressed bytes changed", first_byte_offset=first,
                     old_line=before[:first].count(b"\n") + 1, new_line=after[:first].count(b"\n") + 1,
                     old_context=before[max(0, first - 100):first + 240].decode(errors="replace"),
                     new_context=after[max(0, first - 100):first + 240].decode(errors="replace"))
            result["checks"].append(check)
    if result["runs"] != expected:
        fail(name, "replayed run total differs", expected=expected, actual=result["runs"])
    result["status"] = "FAIL" if differences else "PASS"
    return result


def main():
    if len(sys.argv) != 2:
        raise SystemExit("verify_command_regressions.py NEW_DIRECTORY")
    root = Path(sys.argv[1])
    binaries = ("provisioning_probe", "settlement_probe", "infrastructure_probe", "paths_probe",
                "household_probe", "affect_probe", "lifecycle_probe", "mobilization_probe",
                "formation_probe", "combat_probe", "information_probe", "kingdom_readiness_probe")
    for name in binaries:
        require((Path("target/release") / name).is_file(), f"build release binary {name} first")
    root.mkdir(parents=True, exist_ok=False)
    source_revision = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    scripts = [Path(__file__), Path("scripts/verify_information.py"),
               Path("scripts/run_information.py"), Path("scripts/run_kingdom_readiness.py")]
    specs = [Path("docs/COMMAND_CHAIN_EXPERIMENT.md"), Path("docs/INFORMATION_EXPERIMENT.md"),
             Path("docs/KINGDOM_READINESS_EXPERIMENT.md")]
    provenance = {
        "source_revision": source_revision,
        "working_tree_status_at_start": subprocess.check_output(
            ["git", "status", "--porcelain"], text=True).splitlines(),
        "binary_sha256": {name: sha((Path("target/release") / name).read_bytes()) for name in binaries},
        "script_sha256": {str(path): sha(path.read_bytes()) for path in scripts},
        "specification_sha256": {str(path): sha(path.read_bytes()) for path in specs},
        "expected_executions": {"historical": 217, "F12": 840, "F13A": 180, "total": 1237},
        "csv_exceptions": {"F12": ["runtime_ms"], "F13A": ["elapsed_ms"]},
        "parameter_normalization": "parameters.txt only: remove disabled commands:false field in compact/pretty/escaped-pretty Config formatting",
        "historical_verifier": "scripts/verify_information.py retains its documented preexisting historical comparison rules",
        "retention": "All new F12/F13A raw outputs retained; nested historical verifier retains fingerprints or a failure archive. No old evidence rewritten.",
    }
    prior_path = Path("research/command-chain/prior-artifacts.json")
    prior = json.loads(prior_path.read_text()) if prior_path.exists() else {}
    if prior:
        require(isinstance(prior, dict), "prior-artifact manifest must be a hash mapping")
        provenance["prior_artifact_manifest"] = str(prior_path)
        provenance["prior_artifact_manifest_sha256"] = sha(prior_path.read_bytes())
        provenance["prior_artifact_count"] = len(prior)
    write_json(root / "provenance.json", provenance)
    completed = {}
    current_phase = "prior artifact validation"
    try:
        for path, expected in prior.items():
            require(sha(Path(path).read_bytes()) == expected, f"prior artifact changed: {path}")
        current_phase = "217 historical controls"
        run([sys.executable, "scripts/verify_information.py", str(root / "historical")], root / "historical.log")
        older = json.loads((root / "historical/validation.json").read_text())
        require(older.get("status") == "PASS" and older.get("historical_executions") == 217,
                "historical verifier did not pass its exact 217-run scope")
        completed["historical"] = {"status": "PASS", "runs": 217, "validation": "historical/validation.json"}
        for name, runner, reference, output, count, excluded in (
            ("F12", "scripts/run_information.py", "research/information/matrix", "information", 840, "runtime_ms"),
            ("F13A", "scripts/run_kingdom_readiness.py", "research/kingdom-readiness/matrix", "kingdom-readiness", 180, "elapsed_ms"),
        ):
            current_phase = name
            run([sys.executable, runner, str(root / output), "30"], root / f"{output}.log")
            comparison = compare_panel(Path(reference), root / output, name, count, excluded)
            validation_name = f"{output}-validation.json"
            write_json(root / validation_name, comparison)
            completed[name] = {"status": comparison["status"], "runs": comparison["runs"],
                               "validation": validation_name, "differences": len(comparison["differences"])}
            require(comparison["status"] == "PASS", f"{name} semantic regression; see {validation_name}")
            print(name, count, "exact replayed runs; checked files", len(comparison["checks"]), flush=True)
        current_phase = "final prior artifact validation"
        for path, expected in prior.items():
            require(sha(Path(path).read_bytes()) == expected, f"prior artifact changed: {path}")
        write_json(root / "validation.json", {
            "status": "PASS", "source_revision": source_revision, "historical_executions": 1237,
            "panels": completed, "prior_files_unchanged": len(prior),
            "exceptions": provenance["csv_exceptions"],
            "parameter_normalization": provenance["parameter_normalization"],
            "interpretation": "Exact historical regressions; not new independent empirical runs and not an automatic F13 milestone pass.",
        })
    except BaseException as error:
        write_json(root / "failure.json", {
            "status": "FAIL", "phase": current_phase, "error": repr(error),
            "source_revision": source_revision, "completed_panels": completed,
            "retention": "All new completed/partial output preserved. Inspect phase logs and comparison differences; old research untouched.",
        })
        raise
    print("PASS: 1,237 historical executions revalidated; prior evidence unchanged", flush=True)


if __name__ == "__main__":
    main()
