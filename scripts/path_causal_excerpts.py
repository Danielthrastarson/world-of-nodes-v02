#!/usr/bin/env python3
"""Seed-zero causal excerpts and exact event tallies, not selected best seeds."""
from pathlib import Path
from collections import Counter
import csv, gzip, json, re, sys

root, output = map(Path, sys.argv[1:3])
results = {}
for directory in sorted(root.iterdir()):
    if not directory.is_dir():
        continue
    events = directory / 'events.tsv.gz'
    if not events.exists():
        continue
    counts, spoiled, examples, focal = Counter(), Counter(), {}, []
    with gzip.open(events, 'rt') as stream:
        for line in stream:
            kind = re.search(r'kind: "([^"]+)"', line).group(1)
            counts[kind] += 1
            if kind == 'Spoiled':
                spoiled[re.search(r'holder=(\w+)', line).group(1)] += int(re.search(r'q=(\d+)', line).group(1))
            if kind in ['PathTread', 'PathRecovery', 'JobAccepted', 'Delivered', 'CrossingCompleted', 'Death']:
                examples.setdefault(kind, [])
                if len(examples[kind]) < 4:
                    examples[kind].append(line.strip())
            if directory.name.startswith('line_') and 'agent: Some(0)' in line:
                if kind in ['JobAccepted', 'JobCancelled', 'Delivered', 'Arrived', 'Admitted']:
                    focal.append(line.strip())
    trips = [t for t in csv.DictReader((directory / 'trips.csv').open()) if t['seed'] == '0']
    results[directory.name] = {
        'event_counts': dict(counts), 'spoiled_by_holder': dict(spoiled),
        'first_events': examples, 'line_agent_0': focal, 'first_twelve_admissions': trips[:12],
    }
with output.open('x') as out:
    json.dump(results, out, indent=2)
    out.write('\n')
