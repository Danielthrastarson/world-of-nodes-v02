#!/usr/bin/env python3
"""Registered F7 paired traffic/surface experiment; never overwrite evidence."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess, sys, json, hashlib, csv, gzip, shutil, statistics

root = Path(sys.argv[1])
root.mkdir(parents=True, exist_ok=False)
revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
cases = ['line', 'diamond', 'detour', 'dead_end', 'local_food', 'crossing', 'carts']
arms = ['frozen', 'enabled']


def run(pair):
    name, arm = pair
    key = name + '_' + arm
    with (root / (key + '.log')).open('w') as log:
        subprocess.run(['target/release/paths_probe', name, arm, str(root / key), '30'],
                       stdout=log, stderr=log, check=True)
    for p in (root / key).glob('*.tsv'):
        with p.open('rb') as a, p.with_suffix('.tsv.gz').open('wb') as b:
            with gzip.GzipFile(fileobj=b, mode='wb', filename='', mtime=0) as z:
                shutil.copyfileobj(a, z)
        p.unlink()
    print(key + ' complete', flush=True)


with ThreadPoolExecutor(max_workers=3) as pool:
    list(pool.map(run, [(n, a) for n in cases for a in arms]))

metrics = ['alive', 'consumed', 'spoiled', 'delivered', 'starvation_h', 'queue_h',
           'sleep_h', 'visited_food', 'at_c', 'at_food', 'completed_trips',
           'matched_work_reduction_h']


def describe(values):
    return {'mean': statistics.mean(values), 'min': min(values), 'max': max(values)}


summary, paired = {}, {}
for name in cases:
    ends = {}
    for arm in arms:
        key = name + '_' + arm
        rows = list(csv.DictReader((root / key / 'runs.csv').open()))
        assert len(rows) == 180
        assert all(r['accounting_error'] == '0' and r['overtakes'] == '0' for r in rows)
        last = [r for r in rows if r['hour'] == '72']
        assert len(last) == 30
        ends[arm] = {r['seed']: r for r in last}
        summary[key] = {k: describe([float(r[k]) for r in last]) for k in metrics}
    paired[name] = {k: describe([float(ends['enabled'][s][k]) - float(ends['frozen'][s][k])
                                for s in ends['enabled']]) for k in metrics}

prior = json.loads(Path('research/paths/prior-artifacts.json').read_text())
for p, h in prior.items():
    assert hashlib.sha256(Path(p).read_bytes()).hexdigest() == h, p
(root / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
(root / 'paired.json').write_text(json.dumps(paired, indent=2) + '\n')
(root / 'provenance.json').write_text(json.dumps({
    'revision': revision, 'runs': len(cases) * 60, 'seeds': list(range(30)),
    'prior_unchanged': len(prior), 'intervention': 'surface law only; policy unchanged',
}, indent=2) + '\n')
