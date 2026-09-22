#!/usr/bin/env python3
"""Capture full deterministic traces, independently audit FIFO, and extract readable events."""
import collections
import gzip
from pathlib import Path
import re
import subprocess
import sys

root = Path(sys.argv[1] if len(sys.argv) > 1 else 'research/provisioning')
root.mkdir(parents=True, exist_ok=True)
pattern = re.compile(r'Event \{ id: (\d+), at: (\d+), agent: (None|Some\((\d+)\)), cause: (.*?), kind: "([^"]+)", detail: "(.*)" \}')
lines = ['# Causal trace excerpts', '', 'Seed 0; timestamps are simulated hours. Full traces include every decision and event.', '']
for name in ['reference', 'contested_requests', 'stale_depletion', 'capacity_1']:
    process = subprocess.Popen(['target/release/provisioning_probe', '--trace', name, '0'], stdout=subprocess.PIPE)
    queued = {}
    count = collections.Counter()
    kinds = collections.Counter()
    selected = {0, 28} if name == 'reference' else {0}
    lines.extend([f'## {name}', ''])
    with gzip.GzipFile(filename=str(root / f'trace-{name}-0.txt.gz'), mode='wb', mtime=0) as output:
        for raw in process.stdout:
            output.write(raw)
            text = raw.decode().rstrip()
            match = pattern.fullmatch(text)
            if not match: continue
            event, at, _, actor, cause, kind, detail = match.groups()
            at, event = int(at), int(event)
            actor = None if actor is None else int(actor)
            kinds[kind] += 1
            if kind == 'Queued':
                edge = int(re.search(r'edge=(\d+)', detail)[1])
                assert actor not in queued
                queued[actor] = (edge, at, actor)
            elif kind in ['Interrupted', 'Died']:
                queued.pop(actor, None)
            elif kind == 'Admitted':
                edge, joined, identity = queued[actor]
                waiting = [record for record in queued.values() if record[0] == edge]
                assert (edge, joined, identity) == min(waiting), (name,event,actor,waiting)
                del queued[actor]
            if kind == 'JobAccepted' and len(selected) < 3: selected.add(actor)
            ordinary = kind in ['Admitted','Arrived','Consumed','PerformanceChanged','StarvationStarted','StaleFoodTrip','JobAccepted','JobCancelled','Delivered','CargoLoaded']
            show = actor in selected and ordinary and count[actor] < 32
            show |= kind in ['Died','LeaseExpired'] and kinds[kind] <= 8
            show |= kind == 'Spoiled' and kinds[kind] <= 3
            if show:
                count[actor] += 1
                lines.append(f'- {at/3600000:.6f}h · event {event} · agent {actor} · cause {cause} · **{kind}**: {detail}')
    assert process.wait() == 0
    lines.extend(['', 'Event counts: ' + ', '.join(f'{k}={v}' for k,v in sorted(kinds.items())), ''])
    print(name, 'FIFO trace audit passed;', sum(kinds.values()), 'events')
(root / 'TRACE_EXCERPTS.md').write_text('\n'.join(lines)+'\n')
