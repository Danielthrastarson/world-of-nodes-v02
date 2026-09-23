#!/usr/bin/env python3
"""F8 registered distribution/transport/capital controls; immutable outputs."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess, sys, json, hashlib, csv, gzip, shutil, statistics

root = Path(sys.argv[1])
root.mkdir(parents=True, exist_ok=False)
revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
arms = ['complementary', 'symmetric', 'high_cost', 'low_cost', 'no_carts', 'paths', 'paths_no_carts']


def run(arm):
    with (root / (arm + '.log')).open('w') as log:
        subprocess.run(['target/release/settlement_probe', arm, str(root / arm), '30'],
                       stdout=log, stderr=log, check=True)
    for p in (root / arm).glob('*.tsv'):
        with p.open('rb') as a, p.with_suffix('.tsv.gz').open('wb') as b:
            with gzip.GzipFile(fileobj=b, mode='wb', filename='', mtime=0) as z:
                shutil.copyfileobj(a, z)
        p.unlink()
    print(arm + ' complete', flush=True)


with ThreadPoolExecutor(max_workers=3) as pool:
    list(pool.map(run, arms))


def describe(values):
    return {'mean': statistics.mean(values), 'min': min(values), 'max': max(values)}


keys = ['alive', 'deaths', 'produced', 'consumed', 'spoiled', 'delivered', 'jobs',
        'starvation_h', 'queue_h', 'food_flows_ab', 'food_flows_ba',
        'wood_flows_ab', 'wood_flows_ba', 'visits']
summary, ends = {}, {}
for arm in arms:
    rows = list(csv.DictReader((root / arm / 'runs.csv').open()))
    assert len(rows) == 210
    assert all(r['accounting_error'] == '0' and r['overtakes'] == '0' for r in rows)
    ends[arm] = {r['seed']: r for r in rows if r['hour'] == '168'}
    assert len(ends[arm]) == 30
    summary[arm] = {hour: {k: describe([float(r[k]) for r in rows if int(r['hour']) == hour])
                           for k in keys} for hour in [12, 36, 48, 72, 120, 168]}
pairs = [(arm, 'complementary') for arm in arms if arm != 'complementary']
pairs += [('paths', 'paths_no_carts'), ('paths_no_carts', 'no_carts')]
paired = {a + '_minus_' + b: {k: describe([float(ends[a][s][k]) - float(ends[b][s][k])
                                           for s in ends[a]]) for k in keys} for a, b in pairs}
prior = json.loads(Path('research/settlements/prior-artifacts.json').read_text())
for p, h in prior.items():
    assert hashlib.sha256(Path(p).read_bytes()).hexdigest() == h, p
(root / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
(root / 'paired.json').write_text(json.dumps(paired, indent=2) + '\n')
(root / 'provenance.json').write_text(json.dumps({
    'revision': revision, 'runs': 210, 'seeds': list(range(30)), 'prior_unchanged': len(prior),
    'policy_changed': False, 'institution': 'existing Commons(0), personal crops/carts; barter absent',
}, indent=2) + '\n')
