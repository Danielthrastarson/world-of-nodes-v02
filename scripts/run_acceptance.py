#!/usr/bin/env python3
"""Run every seed; losslessly deduplicate identical per-seed decision journals.

Usage: python3 scripts/run_acceptance.py OUTPUT CASE [SEEDS]
Build acceptance_probe first. Output directory must be new for this case.
"""
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys

root = Path(sys.argv[1])
case = sys.argv[2]
seeds = int(sys.argv[3]) if len(sys.argv) > 3 else 30
root.mkdir(parents=True, exist_ok=True)
archive = root / 'journals'
archive.mkdir(exist_ok=True)
assert not (root / f'{case}-runs.csv').exists(), 'Do not overwrite previous experiments'
manifest = []
active = None
stream = None
raw = None
digest = None
rows = 0


def close_seed():
    global stream, raw
    if stream is None:
        return
    stream.close()
    raw.close()
    sha = digest.hexdigest()
    destination = archive / f'{sha}.tsv.gz'
    if destination.exists():
        temporary.unlink()
    else:
        temporary.rename(destination)
    manifest.append(dict(scenario=case, seed=int(active), rows=rows,
                         canonical_sha256=sha, path=str(destination.relative_to(root))))
    stream = None


with (root / f'{case}-execution.log').open('w') as log:
    process = subprocess.Popen(['target/release/acceptance_probe', case, str(root), str(seeds)],
                               stdout=subprocess.PIPE, stderr=log)
    header = process.stdout.readline().split(b'\t', 2)[2]
    assert header.startswith(b'time_ms\tagent\t')
    for line in process.stdout:
        label, seed, canonical = line.split(b'\t', 2)
        assert label.decode() == case
        if seed != active:
            close_seed()
            active = seed
            temporary = archive / f'.{case}-{seed.decode()}.tmp'
            raw = temporary.open('wb')
            stream = gzip.GzipFile(fileobj=raw, mode='wb', filename='', mtime=0)
            digest = hashlib.sha256()
            stream.write(header)
            digest.update(header)
            rows = 0
        stream.write(canonical)
        digest.update(canonical)
        rows += 1
    close_seed()
    assert process.wait() == 0, log.name
assert [r['seed'] for r in manifest] == list(range(seeds))
(root / f'{case}-journal-index.json').write_text(json.dumps(manifest, indent=2) + '\n')
trace = root / f'{case}.trace'
with trace.open('rb') as source, (root / f'{case}.trace.gz').open('wb') as output:
    with gzip.GzipFile(fileobj=output, mode='wb', filename='', mtime=0) as compressed:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            compressed.write(chunk)
trace.unlink()
print(f'{case}: ran {seeds} seeds, {len(set(r["canonical_sha256"] for r in manifest))} distinct full journals')
