#!/usr/bin/env python3
"""Extract witness evidence and distinguish completed sleep ticks from churn."""
from pathlib import Path
from collections import Counter, defaultdict
import csv, gzip, json, re, sys

output = Path(sys.argv[1])
result = {}
for directory in map(Path, sys.argv[2:]):
    compressed = directory / 'events.tsv.gz'
    stream = gzip.open(compressed, 'rt') if compressed.exists() else (directory / 'events.tsv').open()
    locations = {i: 0 if i % 2 == 0 else 2 for i in range(50)}
    counts, consumption, samples, jobs, timelines = Counter(), Counter(), defaultdict(list), set(), defaultdict(list)
    for line in stream:
        kind = re.search(r'kind: "([^"]+)"', line).group(1)
        at = int(re.search(r'at: (\d+)', line).group(1))
        actor = re.search(r'agent: Some\((\d+)\)', line)
        actor = int(actor.group(1)) if actor else None
        counts[kind] += 1
        if kind == 'Admitted':
            locations[actor] = -1
        elif kind == 'Arrived':
            locations[actor] = int(re.search(r'node=(\d+)', line).group(1))
        elif kind == 'Consumed':
            consumption[f'cohort:{actor % 2}:node:{locations[actor]}'] += int(re.search(r'q=(\d+)', line).group(1))
        elif kind == 'JobAccepted':
            jobs.add(actor)
        elif kind == 'Interrupted':
            detail = line.split('detail: "', 1)[1]
            if 'was=Sleep' in detail:
                until = int(re.search(r'until: (\d+)', detail).group(1))
                counts['sleep_tick_already_complete' if until <= at else 'actual_early_sleep_interrupt'] += 1
            if detail.startswith('sleep; was=Queued'):
                counts['queue_exit_for_sleep'] += 1
            if detail.startswith('queue for route; was=Queued'):
                counts['queue_route_switch'] += 1
        if kind in ['JobAccepted', 'JobCancelled', 'Delivered', 'CargoLoaded', 'CartStarted', 'CartCompleted', 'Died', 'StaleFoodTrip']:
            if len(samples[kind]) < 12:
                samples[kind].append(line.strip())
        if actor in jobs and kind in ['JobAccepted', 'JobCancelled', 'Delivered', 'CargoLoaded', 'Consumed', 'Arrived', 'Admitted', 'Died']:
            if len(timelines[str(actor)]) < 160:
                timelines[str(actor)].append(line.strip())
    stream.close()
    seed = next(csv.DictReader((directory / 'runs.csv').open()))['seed']
    result[directory.name] = {'seed': seed, 'counts': dict(counts),
                              'consumed_by_starting_cohort_and_actual_node': dict(consumption),
                              'examples': dict(samples), 'job_actor_timelines': dict(timelines)}
with output.open('x') as f:
    json.dump(result, f, indent=2)
    f.write('\n')
