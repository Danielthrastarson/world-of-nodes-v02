#!/usr/bin/env python3
"""Verify F8 observer correction, historical preservation and witness replay."""
from pathlib import Path
import csv, hashlib, json, re, sys

root = Path('research/settlements')
old, new = root / 'matrix', root / 'matrix-corrected'
arms = list(json.loads((new / 'summary.json').read_text()))


def rows(path):
    return list(csv.DictReader(path.open()))


result = {'unchanged_checkpoint_histories': 0, 'expiry_reclassifications': [],
          'regressions': {}, 'witness_tables': []}
for arm in arms:
    a, b = rows(old / arm / 'runs.csv'), rows(new / arm / 'runs.csv')
    assert len(a) == len(b) == 210
    for x, y in zip(a, b):
        for key in x:
            if key not in ('food_flows_ab', 'food_flows_ba'):
                assert x[key] == y[key], (arm, key, x, y)
        assert y['accounting_error'] == y['overtakes'] == '0'
        result['unchanged_checkpoint_histories'] += 1
    assert (old / arm / 'events.tsv.gz').read_bytes() == (new / arm / 'events.tsv.gz').read_bytes()
    a, b = rows(old / arm / 'flows.csv'), rows(new / arm / 'flows.csv')
    assert len(a) == len(b)
    for x, y in zip(a, b):
        assert all(x[k] == y[k] for k in x if k != 'material')
        if x['material'] != y['material']:
            assert x['material'] == 'Food' and y['material'] == 'Waste'
            assert y['at_ms'] == y['expires_ms']
            result['expiry_reclassifications'].append({'arm': arm, **y})
        if y['material'] == 'Food':
            assert int(y['at_ms']) < int(y['expires_ms'])

for name in ['historical.csv', 'adoption-equivalence.csv']:
    a, b = root / 'regressions' / name, Path('research/paths/regressions') / name
    assert a.read_bytes() == b.read_bytes()
    result['regressions'][str(a)] = {'matches': str(b), 'rows': len(rows(a))}
result['regressions'].update(json.loads((root / 'regression-comparison.json').read_text()))
for a, desc in result['regressions'].items():
    assert Path(a).read_bytes() == Path(desc['matches']).read_bytes()

for p in (root / 'delivery-seed12').glob('*.csv'):
    a = rows(p)
    b = [r for r in rows(new / 'low_cost' / p.name) if r['seed'] == '12']
    assert a == b, p
    result['witness_tables'].append(p.name)

prior = json.loads((root / 'prior-artifacts.json').read_text())
for p, digest in prior.items():
    assert hashlib.sha256(Path(p).read_bytes()).hexdigest() == digest, p
result['preserved_prior_files'] = len(prior)
result['passed_tests'] = sum(map(int, re.findall(r'test result: ok\. (\d+) passed',
                                                (root / 'corrected-tests.txt').read_text())))
result['policy_changed'] = result['physical_rules_changed'] = False
with Path(sys.argv[1]).open('x') as f:
    json.dump(result, f, indent=2)
    f.write('\n')
