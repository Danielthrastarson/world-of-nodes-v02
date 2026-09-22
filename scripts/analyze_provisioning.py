#!/usr/bin/env python3
"""Verify physical balances and paired controls; summarize without a success target."""
import csv
import gzip
import hashlib
import json
from pathlib import Path
import statistics
import subprocess
import sys

root = Path(sys.argv[1] if len(sys.argv) > 1 else 'research/provisioning')
rows = list(csv.DictReader((root / 'runs.csv').open()))
assert len(rows) == 19 * 30, len(rows)
indexed = {(r['scenario'], int(r['seed'])): r for r in rows}
assert len(indexed) == len(rows)
for r in rows:
    assert r['accounting_error'] == '0'
    assert r['queue_overtakes'] == '0'
    assert int(r['food_initial']) == sum(int(r[k]) for k in ['food_consumed', 'food_remaining', 'waste_remaining'])
    assert int(r['food_spoiled']) == int(r['waste_remaining'])
    assert int(r['deaths']) + int(r['survivors_72h']) == 50
for seed in range(30):
    a, b = indexed['reference', seed], indexed['reverse_order', seed]
    assert a['state_hash'] == b['state_hash'], seed
    assert a['event_hash'] == b['event_hash'], seed
    for name in ['no_food', 'no_transport']:
        r = indexed[name, seed]
        assert all(r[f'survivors_{h}h'] == '0' for h in [36, 48, 72])
        assert r['food_consumed'] == '0'

# Agents can be stored as exact deterministic gzip after analysis.
agent_file = root / 'agents.csv'
stream = agent_file.open() if agent_file.exists() else gzip.open(root / 'agents.csv.gz', 'rt')
agent_rows = list(csv.DictReader(stream))
assert len(agent_rows) == 19 * 30 * 50 * 3
final_by_run = {}
for a in agent_rows:
    if a['time_h'] == '72': final_by_run.setdefault((a['scenario'], int(a['seed'])), []).append(a)
for name, seed in indexed:
    final = final_by_run[name,seed]
    assert len(final) == 50
    assert sum(int(a['food_consumed']) for a in final) == int(indexed[name,seed]['food_consumed'])
    for a in final:
        assert 0 <= float(a['nutrition_h']) <= 24
        assert 0 <= float(a['fatigue_h']) <= 24
        if a['alive'] == 'false':
            assert a['death_cause'] == 'starvation'
        if name in ['no_food','no_transport']:
            assert float(a['death_time_h']) == 36

lines = ['# Provisioning matrix summary', '', '30 paired seeds per scenario; survival is measured, not an acceptance target.', '',
         '| Scenario | Survivors 36h mean | 48h mean | 72h mean [range] | Consumed mean | Spoiled mean | Delivered mean | Starvation person-hours mean |',
         '|---|---:|---:|---:|---:|---:|---:|---:|']
summary = []
for name in dict.fromkeys(r['scenario'] for r in rows):
    values = [r for r in rows if r['scenario'] == name]
    def mean(key): return statistics.mean(float(r[key]) for r in values)
    alive = [int(r['survivors_72h']) for r in values]
    lines.append(f'| {name} | {mean("survivors_36h"):.2f} | {mean("survivors_48h"):.2f} | {mean("survivors_72h"):.2f} [{min(alive)}–{max(alive)}] | {mean("food_consumed"):.2f} | {mean("food_spoiled"):.2f} | {mean("food_delivered"):.2f} | {mean("starvation_person_hours"):.2f} |')
    summary.append({'scenario': name, 'survivors_mean': mean('survivors_72h'), 'survivors_min': min(alive), 'survivors_max': max(alive), 'jobs_mean': mean('jobs_accepted'), 'delivery_mean':mean('food_delivered'),'stale_failures_mean':mean('failed_stale_trips'),'spoiled_transit_mean':mean('spoiled_in_transit')})
(root / 'SUMMARY.md').write_text('\n'.join(lines)+'\n')
(root / 'summary.json').write_text(json.dumps(summary, indent=2)+'\n')
print('Verified 570 runs, 85,500 agent checkpoints, all material balances, death controls, and reversed-order equivalence.')
for s in summary: print(s)
