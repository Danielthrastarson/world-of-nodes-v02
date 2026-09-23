#!/usr/bin/env python3
"""Compare complete preserved/replayed F12 panels without changing either one.

Usage: verify_information_replay.py OLD_PANEL NEW_PANEL NEW_VALIDATION_JSON
Only the wall-clock runtime_ms CSV values are exempt from exact comparison.
"""
import csv
import gzip
import hashlib
import json
from pathlib import Path
import sys


def digest(data):
    return hashlib.sha256(data).hexdigest()


def load_json(path):
    return json.loads(path.read_text())


def main():
    if len(sys.argv) != 4:
        raise SystemExit("verify_information_replay.py OLD_PANEL NEW_PANEL NEW_VALIDATION_JSON")
    old_root, new_root, output = map(Path, sys.argv[1:])
    if output.exists():
        raise FileExistsError(output)
    differences, checks = [], []
    result = {
        "old_panel": str(old_root), "new_panel": str(new_root),
        "verifier_sha256": digest(Path(__file__).read_bytes()),
        "exception": "runtime_ms values only; CSV schema and all other fields remain exact",
        "differences": differences, "checks": checks,
    }

    def fail(scope, reason, **details):
        differences.append({"scope": scope, "reason": reason, **details})

    try:
        manifests = {}
        for label, root in (("old", old_root), ("new", new_root)):
            provenance = load_json(root / "provenance.json")
            manifests[label] = provenance
            result[f"{label}_source_revision"] = provenance["source_revision"]
            result[f"{label}_binary_sha256"] = provenance["binary_sha256"]
            result[f"{label}_provenance_sha256"] = digest((root / "provenance.json").read_bytes())
            if load_json(root / "completion.json").get("status") != "PASS":
                fail(label, "panel completion is not PASS")
            if not provenance.get("source_revision") or not provenance.get("binary_sha256"):
                fail(label, "missing implementation/binary provenance")
            if (len(provenance["cases"]), len(provenance["arms"]), len(provenance["seeds"]), provenance["runs"]) != (14, 2, 30, 840):
                fail(label, "not the registered full 840-run panel")
        old, new = manifests["old"], manifests["new"]
        for key in ("cases", "arms", "seeds", "runs", "duration_h", "live_cases", "matched_cases"):
            if old[key] != new[key]:
                fail("provenance", f"registered {key} differs", old=old[key], new=new[key])
        expected_dirs = {f"{case}_{arm}" for case in old["cases"] for arm in old["arms"]}
        if len(expected_dirs) != 28 or len(set(old["seeds"])) != 30:
            fail("old provenance", "duplicate registered cases/arms/seeds")
        totals = {"old": 0, "new": 0}
        for label, root in (("old", old_root), ("new", new_root)):
            present = {p.name for p in root.iterdir() if p.is_dir()}
            if present != expected_dirs:
                fail(label, "arm directories differ", missing=sorted(expected_dirs - present), extra=sorted(present - expected_dirs))
        for name in sorted(expected_dirs):
            paths = {label: root / name for label, root in (("old", old_root), ("new", new_root))}
            if not all(path.is_dir() for path in paths.values()):
                continue
            tables = {}
            for label, folder in paths.items():
                path = folder / "runs.csv"
                with path.open(newline="") as f:
                    reader = csv.DictReader(f)
                    fields, rows = reader.fieldnames, list(reader)
                tables[label] = (fields, rows)
                totals[label] += len(rows)
                if [int(row["seed"]) for row in rows] != old["seeds"]:
                    fail(f"{name}/{label}", "ordered seeds differ", seeds=[row.get("seed") for row in rows])
                if len(rows) != 30:
                    fail(f"{name}/{label}", "run count differs", rows=len(rows))
                if any(None in row or None in row.values() for row in rows):
                    fail(f"{name}/{label}", "malformed CSV")
            old_fields, old_rows = tables["old"]
            new_fields, new_rows = tables["new"]
            if old_fields != new_fields:
                fail(f"{name}/runs.csv", "CSV schema differs", old=old_fields, new=new_fields)
            else:
                for index, (before, after) in enumerate(zip(old_rows, new_rows)):
                    for field in old_fields:
                        if field != "runtime_ms" and before[field] != after[field]:
                            fail(f"{name}/runs.csv", "field differs", row=index, seed=before.get("seed"), field=field, old=before[field], new=after[field])
            checks.append({
                "scope": f"{name}/runs.csv", "comparison": "exact ordered CSV fields excluding runtime_ms values",
                "old_sha256": digest((paths["old"] / "runs.csv").read_bytes()),
                "new_sha256": digest((paths["new"] / "runs.csv").read_bytes()),
                "old_rows": len(old_rows), "new_rows": len(new_rows),
            })
            for filename in ("events.tsv.gz", "reports.tsv.gz", "decisions.tsv.gz", "truth.tsv.gz", "parameters.txt"):
                contents = {}
                for label, folder in paths.items():
                    raw = (folder / filename).read_bytes()
                    contents[label] = gzip.decompress(raw) if filename.endswith(".gz") else raw
                before, after = contents["old"], contents["new"]
                if before != after:
                    offset = next((i for i, pair in enumerate(zip(before, after)) if pair[0] != pair[1]), min(len(before), len(after)))
                    fail(
                        f"{name}/{filename}", "uncompressed bytes differ",
                        first_byte_offset=offset,
                        old_line=before[:offset].count(b"\n") + 1,
                        new_line=after[:offset].count(b"\n") + 1,
                        old_context=before[max(0, offset - 120):offset + 240].decode(errors="replace"),
                        new_context=after[max(0, offset - 120):offset + 240].decode(errors="replace"),
                    )
                checks.append({
                    "scope": f"{name}/{filename}", "comparison": "exact uncompressed bytes",
                    "old_content_sha256": digest(before), "new_content_sha256": digest(after),
                    "old_bytes": len(before), "new_bytes": len(after),
                })
        result["rows"] = totals
        for label, count in totals.items():
            if count != 840:
                fail(label, "total row count differs from 840", rows=count)
    except (OSError, ValueError, KeyError, TypeError, csv.Error) as error:
        fail("verification", "cannot fully validate panels", error=repr(error))
    result["status"] = "FAIL" if differences else "PASS"
    result["interpretation"] = (
        "Every prescribed output matched apart from recorded runtime_ms values."
        if not differences else
        "Differences are preserved for diagnosis. This result does not invalidate or rewrite the original historical experiment."
    )
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("x") as f:
        json.dump(result, f, indent=2, sort_keys=True)
        f.write("\n")
    print(result["status"], "F12 full-panel replay; differences", len(differences), "checked files", len(checks), flush=True)
    if differences:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
