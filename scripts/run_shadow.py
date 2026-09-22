#!/usr/bin/env python3
"""Run the observational audit without overwriting existing evidence."""
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys

root = Path(sys.argv[1])
if root.exists():
    raise SystemExit('Use a new output directory')
root.mkdir(parents=True)
arms = sys.argv[3:] or ['reference', 'current', 'known_source', 'reactive', 'committed', 'forecast_corrected']
seeds = sys.argv[2] if len(sys.argv) > 2 else '30'
original = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and root not in p.parents}
(root / 'prior-artifacts.json').write_text(json.dumps(original, indent=2) + '\n')
for arm in arms:
    with (root / f'{arm}-execution.log').open('w') as log:
        subprocess.run(['target/release/shadow_probe', arm, str(root), seeds], check=True, stderr=log)
    for key in ['captures.tsv', 'memory.csv', 'routes.csv', 'plans.csv', 'needs.csv', 'primitives.csv', 'self_count.csv', 'route_feasibility.csv']:
        p = root / f'{arm}-{key}'
        with p.open('rb') as src, p.with_suffix(p.suffix + '.gz').open('wb') as out:
            with gzip.GzipFile(fileobj=out, mode='wb', filename='', mtime=0) as zipped:
                for chunk in iter(lambda: src.read(1024 * 1024), b''):
                    zipped.write(chunk)
        p.unlink()
    print(f'{arm}: completed {seeds} observational pairs', flush=True)
for name, digest in original.items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest, name
subprocess.run([sys.executable, 'scripts/analyze_shadow.py', str(root)], check=True)
