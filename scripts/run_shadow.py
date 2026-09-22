#!/usr/bin/env python3
"""Run the observational audit without overwriting existing evidence."""
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import shutil
import tempfile
import sys

root = Path(sys.argv[1])
if root.exists():
    raise SystemExit('Use a new output directory')
root.mkdir(parents=True)
arms = sys.argv[3:] or ['reference', 'current', 'known_source', 'reactive', 'committed', 'forecast_corrected', 'contested_requests', 'stale_depletion']
seeds = sys.argv[2] if len(sys.argv) > 2 else '30'
original = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and root not in p.parents}
(root / 'prior-artifacts.json').write_text(json.dumps(original, indent=2) + '\n')
for arm in arms:
    with (root / f'{arm}-execution.log').open('w') as log:
        subprocess.run(['target/release/shadow_probe', arm, str(root), seeds] + (['kernel-validation'] if arm in ['contested_requests', 'stale_depletion'] else []), check=True, stderr=log)
    for key in ['captures.tsv', 'memory.csv', 'routes.csv', 'plans.csv', 'needs.csv', 'primitives.csv', 'self_count.csv', 'route_feasibility.csv']:
        p = root / f'{arm}-{key}'
        with p.open('rb') as src, p.with_suffix(p.suffix + '.gz').open('wb') as out:
            with gzip.GzipFile(fileobj=out, mode='wb', filename='', mtime=0) as zipped:
                for chunk in iter(lambda: src.read(1024 * 1024), b''):
                    zipped.write(chunk)
        p.unlink()
    print(f'{arm}: completed {seeds} observational pairs', flush=True)
# Broaden only kernel witness coverage; primary historical decisions are identical.
for arm in ['reference', 'current', 'known_source', 'contested_requests', 'reactive', 'committed', 'forecast_corrected']:
    if arm not in arms:
        continue
    with tempfile.TemporaryDirectory(prefix='won-shadow-kernel-') as temp:
        subprocess.run(['target/release/shadow_probe', arm, temp, '1', 'kernel-validation'], check=True)
        for kind in ['kernel_plans', 'runs']:
            shutil.copyfile(Path(temp)/f'{arm}-{kind}.csv', root/f'{arm}-validation-{kind}.csv')
        if arm in ['reactive', 'committed', 'forecast_corrected']:
            shutil.copyfile(Path(temp)/f'{arm}-self_count_witness.csv', root/f'{arm}-self_count_witness.csv')
for name, digest in original.items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest, name
subprocess.run([sys.executable, 'scripts/analyze_shadow.py', str(root)], check=True)
