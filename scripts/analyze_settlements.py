#!/usr/bin/env python3
"""F8 physical allocation evidence; no prices or inferred unobserved willingness."""
from pathlib import Path
from collections import defaultdict
import csv, json, statistics, sys

root, output = map(Path, sys.argv[1:3])
arms = ['complementary', 'symmetric', 'high_cost', 'low_cost', 'no_carts', 'paths', 'paths_no_carts']


def read(arm, name):
    with (root / arm / name).open() as f:
        return list(csv.DictReader(f))


def dist(values):
    return {'mean': statistics.mean(values), 'min': min(values), 'max': max(values)}


def totals(rows, key, value):
    result = defaultdict(lambda: [0.0] * 30)
    for r in rows:
        result[key(r)][int(r['seed'])] += value(r)
    return {k: dist(v) for k, v in sorted(result.items())}


result, raw_labor = {}, {}
for arm in arms:
    runs = read(arm, 'runs.csv')
    assert len(runs) == 210
    labor = read(arm, 'labor.csv')
    accounts = read(arm, 'accounts.csv')
    people = read(arm, 'people.csv')
    places = read(arm, 'places.csv')
    final = lambda rows: [r for r in rows if r['hour'] == '168']
    work = defaultdict(lambda: [0.0] * 30)
    for r in final(labor):
        work[r['activity']][int(r['seed'])] += float(r['person_hours'])
    raw_labor[arm] = dict(work)
    p = {'labor_h': {k: dist(v) for k, v in sorted(work.items())}}
    p['place_accounts'] = totals(final(accounts), lambda r: r['place'] + ':' + r['key'],
                                lambda r: float(r['value']) / (3600000 if r['key'].endswith('_ms') else 1))
    p['local_checkpoints'] = {}
    for hour in ['36', '48', '72', '120', '168']:
        p['local_checkpoints'][hour] = {
            place: {key: dist([float(r[key]) for r in places if r['hour'] == hour and r['place'] == place])
                    for key in ['alive', 'critical', 'nutrition_h', 'food', 'personal_food', 'commons_food',
                                'natural_food', 'wood', 'waste', 'dead_owner_food',
                                'critical_near_foreign_loose_food', 'plots_free', 'plots_active',
                                'plots_dead_owner', 'board_demand']}
            for place in ['0', '1', '2', '3']
        }
    p['endpoint_cohorts'] = totals(final(people),
        lambda r: r['cohort'] + '->' + r['place'] + ':' + ('alive' if r['alive'] == 'true' else 'dead'), lambda _: 1)
    p['event_counts'] = totals(final(read(arm, 'counts.csv')), lambda r: r['event'], lambda r: int(r['count']))
    flows = read(arm, 'flows.csv')
    p['flow_by_direction_material_rights'] = totals(flows,
        lambda r: r['from'] + '->' + r['to'] + ':' + r['material'] + ':' + r['owner'].split(':')[0],
        lambda r: int(r['quantity']))
    p['cart_flow'] = totals([r for r in flows if r['cart'] == 'true'],
        lambda r: r['from'] + '->' + r['to'] + ':' + r['material'], lambda r: int(r['quantity']))
    p['ownership_transitions'] = totals(read(arm, 'transfers.csv'),
        lambda r: r['kind'] + ':' + r['material'] + ':' + r['owner_before'].split(':')[0] + '->' + r['owner_after'].split(':')[0],
        lambda r: int(r['quantity']))
    carts = final(read(arm, 'carts.csv'))
    p['carts'] = {
        'count': dist([sum(r['seed'] == str(s) for r in carts) for s in range(30)]),
        'idle_h': dist([sum(float(r['idle_h']) for r in carts if r['seed'] == str(s)) for s in range(30)]),
        'trips': dist([sum(int(r['trips']) for r in carts if r['seed'] == str(s)) for s in range(30)]),
        'food_quanta_edges': dist([sum(int(r['food_quanta_edges']) for r in carts if r['seed'] == str(s)) for s in range(30)]),
    }
    dead = {(r['seed'], r['id']) for r in final(people) if r['alive'] == 'false'}
    p['carts']['dead_owner'] = dist([sum(r['seed'] == str(s) and (r['seed'], r['owner'].split(':')[1]) in dead for r in carts) for s in range(30)])
    paths = final(read(arm, 'paths.csv'))
    p['paths'] = {e: {k: dist([float(r[k]) for r in paths if r['edge'] == e])
                       for k in ['compaction', 'passages', 'recoveries', 'nominal_ms', 'capacity', 'queue_h', 'occupancy_person_h']}
                  for e in ['0', '1']}
    p['barter'] = {'protocol_present': False, 'attempts': 0, 'completed': 0,
                  'reason': 'No bilateral offer/transfer protocol in existing institutions. Not observed voluntary refusal.'}
    result[arm] = p

paired_labor = {}
for arm in arms[1:]:
    paired_labor[arm + '_minus_complementary'] = {
        k: dist([raw_labor[arm].get(k, [0]*30)[s] - raw_labor['complementary'].get(k, [0]*30)[s] for s in range(30)])
        for k in sorted(raw_labor[arm].keys() | raw_labor['complementary'].keys())
    }
with output.open('x') as f:
    json.dump({'arms': result, 'paired_labor_h': paired_labor}, f, indent=2)
    f.write('\n')
