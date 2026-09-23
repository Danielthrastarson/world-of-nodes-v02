#!/usr/bin/env python3
"""Describe all registered F7 seeds without selecting favorable outcomes."""
import csv, json, statistics, sys
from pathlib import Path

root = Path(sys.argv[1])
output = Path(sys.argv[2])
cases = ['line', 'diamond', 'detour', 'dead_end', 'local_food', 'crossing', 'carts']


def rows(path):
    return list(csv.DictReader(path.open()))


def summary(v):
    return {'mean': statistics.mean(v), 'min': min(v), 'max': max(v)}


result = {}
for case in cases:
    result[case] = {}
    for arm in ['frozen', 'enabled']:
        directory = root / (case + '_' + arm)
        runs = rows(directory / 'runs.csv')
        trips = rows(directory / 'trips.csv')
        surfaces = rows(directory / 'surfaces.csv')
        report = {'checkpoints': {}, 'traffic': {}, 'seed_variability': {}}
        for hour in [12, 24, 36, 48, 72]:
            selected = [r for r in runs if int(r['hour']) == hour]
            report['checkpoints'][hour] = {
                k: summary([float(r[k]) for r in selected])
                for k in ['alive', 'consumed', 'spoiled', 'delivered', 'starvation_h',
                          'queue_h', 'at_c', 'at_food', 'visited_food', 'completed_trips']
            }
        for edge in sorted({int(t['edge']) for t in trips}):
            selected = [t for t in trips if int(t['edge']) == edge]
            completed = [t for t in selected if t['arrived_ms']]
            report['traffic'][edge] = {
                'admissions_per_seed': summary([sum(t['seed'] == str(s) for t in selected) for s in range(30)]),
                'completed_per_seed': summary([sum(t['seed'] == str(s) for t in completed) for s in range(30)]),
                'directional_admissions': {direction: sum(t['from'] + '->' + t['to'] == direction for t in selected)
                                          for direction in sorted({t['from'] + '->' + t['to'] for t in selected})},
                'mean_completed_elapsed_h': statistics.mean(
                    (int(t['arrived_ms']) - int(t['admitted_ms'])) / 3600000 for t in completed) if completed else None,
                'mean_admitted_nominal_h': statistics.mean(int(t['nominal_ms']) / 3600000 for t in selected),
                'food_quanta_edge_departures_per_seed': sum(int(t['food_departed']) for t in selected) / 30,
                'peak_observed_compaction': max([int(t['compaction']) for t in selected]
                                               + [int(s['compaction']) for s in surfaces if int(s['edge']) == edge]),
            }
        report['seed_variability'] = {
            k: len({r[k] for r in runs if r['hour'] == '72'})
            for k in ['alive', 'consumed', 'delivered', 'completed_trips']
        }
        if case in ['diamond', 'detour']:
            report['choice_at_c'] = {}
            for period, lo, hi in [('first_12h', 0, 12), ('12_to_48h', 12, 48), ('48_to_72h', 48, 73)]:
                selected = [t for t in trips if t['from'] == '0' and lo * 3600000 <= int(t['admitted_ms']) < hi * 3600000]
                report['choice_at_c'][period] = {
                    'total': len(selected), 'via_a': sum(t['edge'] == '0' for t in selected),
                    'via_b': sum(t['edge'] == '2' for t in selected),
                }
        end_surfaces = [s for s in surfaces if s['hour'] == '72']
        report['final_surfaces'] = {
            e: {k: summary([float(s[k]) for s in end_surfaces if s['edge'] == e])
                for k in ['compaction', 'passages', 'recovery_count', 'effective_ms', 'queue_h', 'occupancy_person_h']}
            for e in sorted({s['edge'] for s in end_surfaces})
        }
        result[case][arm] = report

with output.open('x') as out:
    json.dump(result, out, indent=2)
    out.write('\n')
