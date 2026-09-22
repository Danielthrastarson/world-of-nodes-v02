#!/usr/bin/env python3
"""Audit every acceptance run and its losslessly archived decision journal."""
import collections as C
import csv
import gzip
import hashlib
import json
from pathlib import Path
import re
import statistics as S
import sys

root = Path(sys.argv[1] if len(sys.argv) > 1 else 'research/acceptance')
names = ['current', 'known_source', 'meal_aware', 'zero_margin']
H = 3600000


def csv_rows(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def save(name, data):
    (root / name).write_text(json.dumps(data, indent=2, sort_keys=True) + '\n')


old = {(r['scenario'], int(r['seed'])): r for r in csv_rows(Path('research/delivery/runs.csv'))}
allruns = {}
summary = {}
funnels = []
journal_cache = {}
excerpts = ['# Acceptance traces', '', 'Times are simulated hours. Nodes: C=0, X=1, F=2.',
            'Full journals retain every evaluated and deferred decision, not just these excerpts.', '']
for name in names:
    runs = csv_rows(root / f'{name}-runs.csv')
    assert [int(r['seed']) for r in runs] == list(range(30))
    allruns[name] = runs
    for r in runs:
        assert r['accounting_error'] == r['queue_overtakes'] == '0'
        assert int(r['food_consumed']) + int(r['food_remaining']) + int(r['waste_remaining']) == 3000
        assert r['food_spoiled'] == r['waste_remaining']
        assert r['jobs_accepted'] == r['diag_jobs_accepted']
        assert r['food_delivered'] == r['diag_delivered_quanta']
        assert r['diag_request_observers'] == '50'
        if name == 'current':
            for k, v in old['current', int(r['seed'])].items():
                if k != 'scenario':
                    assert r[k] == v, (name, r['seed'], k)
    metrics = [k for k in runs[0] if k not in ['scenario', 'seed', 'event_hash', 'state_hash']]
    summary[name] = {k: dict(mean=S.mean(float(r[k]) for r in runs),
                            min=min(float(r[k]) for r in runs),
                            max=max(float(r[k]) for r in runs)) for k in metrics}
    gates = csv_rows(root / f'{name}-gates.csv')
    per_seed = {(int(r['seed']), r['scope'], r['stage'], r['gate']): int(r['count']) for r in gates}
    for scope in ['LOCAL', 'ALL']:
        for stage in ['EVALUATION', 'DEFERRED']:
            counts = C.Counter()
            for r in gates:
                if r['scope'] == scope and r['stage'] == stage:
                    counts[r['gate']] += int(r['count'])
            total = counts.total()
            for gate, n in counts.items():
                funnels.append(dict(scenario=name, scope=scope, stage=stage, gate=gate,
                                    count=n, denominator=total, per_run=n / 30,
                                    percentage=100 * n / total if total else 0))
    index = json.loads((root / f'{name}-journal-index.json').read_text())
    assert [r['seed'] for r in index] == list(range(30))
    excerpts.extend([f'## {name}', ''])
    for item in index:
        key = item['canonical_sha256']
        if key not in journal_cache:
            with gzip.open(root / item['path'], 'rb') as f:
                digest = hashlib.file_digest(f, 'sha256').hexdigest()
            assert digest == key
            counts = C.Counter()
            detail_counts = C.Counter()
            deferrals = C.Counter()
            violations = C.Counter()
            observed_agents = set()
            passing_agents = set()
            passes_local = passes_remote = shadow_passes = meals = 0
            rows = 0
            selections = []
            selected_gates = set()
            previous_actor0 = None
            with gzip.open(root / item['path'], 'rt') as f:
                for r in csv.DictReader(f, delimiter='\t'):
                    rows += 1
                    at = int(r['time_ms'])
                    agent = int(r['agent'])
                    local = r['local_request'] == 'true'
                    stage, gate = r['stage'], r['primary_gate']
                    counts['ALL', stage, gate] += 1
                    if local:
                        observed_agents.add(agent)
                        counts['LOCAL', stage, gate] += 1
                    audit = r['audit']
                    detail = re.search(r'detail: "(.*?)", candidates:', audit)[1]
                    detail_counts[stage, detail.split(';')[0]] += 1
                    if stage == 'DEFERRED':
                        if r['intent'].startswith('Eat'):
                            reason = 'LOCAL_EATING_PRECEDES_JOB_EVALUATION'
                        elif r['intent'] == 'Sleep':
                            reason = 'FATIGUE_OR_ONGOING_RECOVERY'
                        elif int(r['nutrition_ms']) <= 6 * H:
                            reason = 'JOB_ENTRY_REQUIRES_NUTRITION_ABOVE_6H'
                        elif int(r['nutrition_ms']) < 12 * H and (
                                'remembered food' in detail or 'observed deposit' in detail):
                            reason = 'SELF_PROVISIONING_BELOW_12H_PRECEDES_JOBS'
                        elif 'atomic gathering' in detail:
                            reason = 'FINISH_ATOMIC_GATHER'
                        else:
                            raise AssertionError(('Unexplained deferral', name, r))
                        deferrals['ALL', reason] += 1
                        if local:
                            deferrals['LOCAL', reason] += 1
                    for observed in re.findall(r'observed_at: Some\((-?\d+)\)', audit):
                        assert int(observed) <= at
                    for group in re.findall(r'violations: \[(.*?)\]', audit):
                        violations.update(re.findall(r'"([A-Z_]+)"', group))
                    if gate == 'PASS':
                        passing_agents.add(agent)
                        if local:
                            passes_local += 1
                        else:
                            passes_remote += 1
                        assert 'Claim' in r['intent'] or 'visit remembered board' in detail
                        match = re.search(r'shadow: Some\(\(\d+, \d+, \d+, "(.*?)"\)\)', audit)
                        if match and match[1] == 'PASS':
                            shadow_passes += 1
                    for match in re.finditer(r'meal: t=(\d+) node=Node\((\d+)\) batch=\d+ owner=(Commons|Agent)\(\d+\) q=(\d+) nutrition_before=(\d+) after=(\d+) expiry=(\d+) load_after=(\d+) cargo_remaining=(\d+)', audit):
                        t, node, owner, q, before, after, expiry, load, cargo = match.groups()
                        assert int(after) == int(before) + int(q) * H
                        assert int(after) <= (12 if owner == 'Commons' else 24) * H
                        assert int(t) < int(expiry)
                        assert int(load) <= 10 and int(cargo) <= 10
                        meals += 1
                    # One example per gate, plus actor 0's first pre-claim loop.
                    signature = (r['location'], gate, detail.split(';')[0])
                    selected = stage == 'EVALUATION' and gate not in selected_gates
                    if selected:
                        selected_gates.add(gate)
                    selected |= agent == 0 and 4 * H <= at <= 14 * H and signature != previous_actor0
                    if selected:
                        selections.append(dict(time_h=at / H, agent=agent, location=r['location'],
                                               nutrition_h=int(r['nutrition_ms']) / H, gate=gate,
                                               intent=r['intent'], detail=detail, audit=audit))
                    if agent == 0:
                        previous_actor0 = signature
            journal_cache[key] = dict(counts=counts, rows=rows, details=detail_counts,
                                      deferrals=deferrals,
                                      violations=violations, observers=len(observed_agents),
                                      passing_agents=sorted(passing_agents), local_passes=passes_local,
                                      remote_passes=passes_remote, shadows=shadow_passes,
                                      audited_meals=meals, selections=selections)
        j = journal_cache[key]
        assert j['rows'] == item['rows']
        assert j['observers'] == 50
        for (seed, scope, stage, gate), n in per_seed.items():
            if seed == item['seed']:
                assert j['counts'][scope, stage, gate] == n
        assert j['shadows'] == int(runs[item['seed']]['shadow_meal_passes'])
        assert sum(n for (scope, _, _), n in j['counts'].items() if scope == 'LOCAL') == int(runs[item['seed']]['diag_request_observations'])
        if item['seed'] == 0:
            save(f'{name}-selected-forecasts.json', j['selections'])
            save(f'{name}-journal-audit.json', dict(
                rows=j['rows'], distinct_request_observers=j['observers'],
                local_passes=j['local_passes'], remote_passes=j['remote_passes'],
                passing_agents=j['passing_agents'], shadows=j['shadows'],
                audited_predicted_meals=j['audited_meals'],
                independent_violations=dict(j['violations']),
                deferrals=[dict(scope=k[0], reason=k[1], count=v) for k, v in j['deferrals'].most_common()],
                decision_details=[dict(stage=k[0], detail=k[1], count=v) for k, v in j['details'].most_common()]))
            for entry in j['selections']:
                if entry['agent'] == 0 and (entry['gate'] == 'PASS' or entry['location'] == 'Node(1)' or entry['time_h'] == 0):
                    excerpts.append(f'- {entry["time_h"]:.6f}h, agent 0, {entry["location"]}, nutrition {entry["nutrition_h"]:.6f}h: **{entry["gate"]}**; {entry["detail"]}')
            excerpts.append('')
    agents = csv_rows(root / f'{name}-agents.csv')
    assert len(agents) == 30 * 3 * 50
    health = []
    for t in [36, 48, 72]:
        group = [r for r in agents if int(r['time_h']) == t]
        live = [r for r in group if r['alive'] == 'true']
        health.append(dict(time_h=t, survivors_per_run=len(live) / 30,
                           zero_nutrition_hours_per_run=sum(float(r['zero_nutrition_h']) for r in group) / 30,
                           continuously_nourished_per_run=sum(r['continuously_nourished'] == 'true' for r in live) / 30,
                           alive_nutrition_min=min(float(r['nutrition_h']) for r in live),
                           alive_nutrition_median=S.median(float(r['nutrition_h']) for r in live),
                           alive_nutrition_max=max(float(r['nutrition_h']) for r in live),
                           alive_at_zero_per_run=sum(float(r['nutrition_h']) == 0 for r in live) / 30))
    for seed in range(30):
        endpoint = [r for r in agents if int(r['seed']) == seed and int(r['time_h']) == 72]
        assert abs(sum(float(r['zero_nutrition_h']) for r in endpoint) - float(runs[seed]['starvation_person_hours'])) < 1e-5
        assert sum(int(r['consumed']) for r in endpoint) == int(runs[seed]['food_consumed'])
    save(f'{name}-health.json', health)

with (root / 'funnel.csv').open('w') as f:
    writer = csv.DictWriter(f, fieldnames=list(funnels[0]))
    writer.writeheader()
    writer.writerows(funnels)
save('summary.json', summary)
paired = {}
for name in names[1:]:
    paired[name] = {}
    for metric in summary[name]:
        delta = [float(a[metric]) - float(b[metric]) for a, b in zip(allruns[name], allruns['current'])]
        paired[name][metric] = dict(mean_delta=S.mean(delta), min_delta=min(delta), max_delta=max(delta),
                                   positive_pairs=sum(v > 0 for v in delta), negative_pairs=sum(v < 0 for v in delta))
save('paired-effects.json', paired)
effect = paired['zero_margin']['jobs_accepted']
save('conditional-decisions.json', dict(
    three_hour_margin_triggered=effect['mean_delta'] >= 1 and effect['positive_pairs'] >= 15,
    mean_added_jobs=effect['mean_delta'], acceptance_increased_pairs=effect['positive_pairs']))
event_pattern = re.compile(r'Event \{ id: (\d+), at: (\d+), agent: (None|Some\((\d+)\)), cause: (.*?), kind: "([^"]+)", detail: "(.*)" \}')
trace_audits = {}
for name in names:
    queued, moving = {}, {}
    positions = {i: 0 for i in range(50)}
    counts = C.Counter()
    exits = C.Counter()
    board_agents = set()
    returned = set()
    consumed = spoiled = 0
    with gzip.open(root / f'{name}.trace.gz', 'rt') as stream:
        for line in stream:
            match = event_pattern.fullmatch(line.strip())
            assert match, line
            eid, at, _, actor, cause, kind, detail = match.groups()
            eid, at = int(eid), int(at)
            actor = None if actor is None else int(actor)

            def field(key):
                return int(re.search(rf'{key}=(\d+)', detail)[1])

            counts[kind] += 1
            if kind == 'Queued':
                assert actor not in queued and actor not in moving
                edge = field('edge')
                assert positions[actor] in [(0, 1), (1, 2)][edge]
                board = field('target') == 0
                queued[actor] = (edge, at, actor, board)
                if board:
                    counts['board_queue_joins'] += 1
                    board_agents.add(actor)
            elif kind in ['Interrupted', 'Died']:
                q = queued.pop(actor, None)
                if q and q[3]:
                    counts['board_queue_exits'] += 1
                    exits[detail.split(';')[0]] += 1
                if kind == 'Died':
                    moving.pop(actor, None)
            elif kind == 'Admitted':
                edge, joined, identity, board = queued.pop(actor)
                assert not any(q[0] == edge and q[1:3] < (joined, identity) for q in queued.values())
                assert field('from') == positions[actor]
                assert {field('from'), field('to')} == set([(0, 1), (1, 2)][edge])
                moving[actor] = (edge, at, field('to'), board)
                assert sum(v[0] == edge for v in moving.values()) <= 4
                if board:
                    counts['board_edge_admissions'] += 1
                    counts[f'board_admissions_edge_{edge}'] += 1
            elif kind == 'Arrived':
                edge, departed, to, board = moving.pop(actor)
                assert field('node') == to and field('edge') == edge
                assert at - departed >= 2 * H
                positions[actor] = to
                if board and to == 0:
                    counts['board_visits_completed'] += 1
                    returned.add(actor)
            elif kind == 'Consumed':
                consumed += field('q')
            elif kind == 'Spoiled':
                assert at == 48 * H
                spoiled += field('q')
    assert consumed + spoiled == 3000
    assert consumed == int(allruns[name][0]['food_consumed'])
    assert counts['JobAccepted'] == int(allruns[name][0]['jobs_accepted'])
    trace_audits[name] = dict(counts=dict(counts), distinct_board_attempt_agents=sorted(board_agents),
                              board_arrivals=sorted(returned), board_queue_exit_reasons=dict(exits),
                              consumed=consumed, spoiled=spoiled, fifo_overtakes=0,
                              capacity_violations=0, sub_nominal_traversals=0)
save('trace-audits.json', trace_audits)
(root / 'TRACE_EXCERPTS.md').write_text('\n'.join(excerpts) + '\n')
print('Verified 120 runs, 30 frozen control replays, all decision-journal gates, predicted meals, and individual nutrition accounting.')
for name in names:
    print(name, {k: summary[name][k]['mean'] for k in ['jobs_accepted', 'food_delivered', 'survivors_72h', 'starvation_person_hours']})
