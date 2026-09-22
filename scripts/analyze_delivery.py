#!/usr/bin/env python3
"""Audit paired delivery decomposition, frozen regressions and physical traces."""
import collections
import csv
import gzip
import json
from pathlib import Path
import re
import statistics
import sys

root = Path(sys.argv[1] if len(sys.argv)>1 else 'research/delivery')
rows = list(csv.DictReader((root/'runs.csv').open()))
index = {(r['scenario'],int(r['seed'])):r for r in rows}
assert len(rows)==len(index)==180
old = {(r['scenario'],int(r['seed'])):r for r in csv.DictReader(Path('research/provisioning/runs.csv').open())}
for r in rows:
    assert r['accounting_error']==r['queue_overtakes']=='0'
    assert int(r['food_consumed'])+int(r['waste_remaining'])+int(r['food_remaining'])==3000
    assert r['food_spoiled']==r['waste_remaining']
    assert r['food_delivered']==r['diag_delivered_quanta']
    assert r['jobs_accepted']==r['diag_jobs_accepted']
    assert r['lease_expiries']==r['diag_claim_expiries']
    assert int(r['diag_jobs_delivered'])<=int(r['diag_returns_completed'])<=int(r['diag_returns_started'])<=int(r['diag_jobs_acquiring_cargo'])<=int(r['diag_jobs_reaching_f'])<=int(r['jobs_accepted'])
for seed in range(30):
    for name,legacy in [('reference','reference'),('contested','contested_requests')]:
        a,b=index[name,seed],old[legacy,seed]
        for key in b:
            if key!='scenario': assert a[key]==b[key],(name,seed,key,a[key],b[key])
    a=index['current',seed]
    assert a['diag_request_observers']=='50'
    for name in ['queue','job']:
        b=index[name,seed]
        assert a['event_hash']==b['event_hash'] and a['state_hash']==b['state_hash']
    for name in ['current','queue','sleep','job']:
        assert index[name,seed]['jobs_accepted']==index[name,seed]['food_delivered']=='0'

metrics=[k for k in rows[0] if k not in ['scenario','seed','event_hash','state_hash']]
summary={}
for name in ['current','queue','sleep','job','reference','contested']:
    group=[index[name,s] for s in range(30)]
    summary[name]={k:{'mean':statistics.mean(float(r[k]) for r in group),'min':min(float(r[k]) for r in group),'max':max(float(r[k]) for r in group)} for k in metrics}
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
paired={}
for name in ['queue','sleep','job']:
    paired[name]={}
    for key in metrics:
        delta=[float(index[name,s][key])-float(index['current',s][key]) for s in range(30)]
        paired[name][key]={'mean_delta':statistics.mean(delta),'min_delta':min(delta),'max_delta':max(delta)}
(root/'paired-effects.json').write_text(json.dumps(paired,indent=2)+'\n')

witnesses=list(csv.DictReader((root/'witnesses.csv').open()))
assert len(witnesses)==117
bounds=json.loads((root/'bounds.json').read_text())
for r in witnesses:
    assert r['accounting_error']==r['queue_overtakes']=='0'
    assert int(r['consumed'])+int(r['waste'])==3000
    assert int(r['delivered'])<=bounds['bulk_delivery_upper_bound']
assert min(int(r['first_delivery_ms']) for r in witnesses if r['first_delivery_ms'])==bounds['earliest_delivery_ms']
assert max(int(r['residents_alive_72']) for r in witnesses)==bounds['alive_stationary_residents_72h_max']
assert max(int(r['alive_at_c_72h']) for r in witnesses)==50

pattern=re.compile(r'Event \{ id: (\d+), at: (\d+), agent: (None|Some\((\d+)\)), cause: (.*?), kind: "([^"]+)", detail: "(.*)" \}')
excerpts=['# Delivery mechanism traces','','Times below are simulated hours. Full compressed traces preserve event IDs and parent causes.','']
trace_audits={}
paths=sorted(root.glob('*.trace')) or sorted(root.glob('*.trace.gz'))
assert len(paths)==11,len(paths)
for path in paths:
    name=path.name.removesuffix('.gz').removesuffix('.trace')
    queued={};moving={};positions={i:(1 if name.startswith('contested') and i>=25 else 0) for i in range(50)}
    kinds=collections.Counter();counts=collections.Counter();consumed=spoiled=0
    stream=gzip.open(path,'rt') if path.suffix=='.gz' else path.open()
    excerpts.extend([f'## {name}',''])
    for line in stream:
        match=pattern.fullmatch(line.strip())
        if not match:continue
        eid,at,_,actor,cause,kind,detail=match.groups()
        eid,at=int(eid),int(at);actor=None if actor is None else int(actor)
        kinds[kind]+=1
        def field(key):return int(re.search(rf'{key}=(\d+)',detail)[1])
        if kind=='Queued':
            assert actor not in queued and actor not in moving
            edge=field('edge');assert positions[actor] in [(0,1),(1,2)][edge]
            queued[actor]=(edge,at,actor)
        elif kind in ['Interrupted','Died']:
            queued.pop(actor,None)
            if kind=='Died':moving.pop(actor,None)
        elif kind=='Admitted':
            edge,joined,identity=queued.pop(actor)
            assert not any(q[0]==edge and q[1:]<(joined,identity) for q in queued.values()),(name,eid)
            assert field('from')==positions[actor]
            assert {field('from'),field('to')}==set([(0,1),(1,2)][edge])
            moving[actor]=(edge,at,field('to'))
            assert sum(v[0]==edge for v in moving.values())<=4
        elif kind=='Arrived':
            edge,departed,to=moving.pop(actor)
            assert field('node')==to and field('edge')==edge
            assert at-departed>=2*3600000
            positions[actor]=to
        elif kind=='Consumed':consumed+=field('q')
        elif kind=='Spoiled':spoiled+=field('q')
        selected=(kind in ['Delivered','LeaseExpired','JobAccepted','JobCancelled'] and counts[kind]<8)
        selected|=(kind in ['Admitted','Arrived','Consumed','Gathered','PerformanceChanged'] and actor==0 and counts['actor0']<20)
        selected|=(kind=='Spoiled' and counts[kind]<2)
        if selected:
            excerpts.append(f'- {at/3600000:.6f}h · event {eid} · agent {actor} · cause {cause} · **{kind}**: {detail}')
            counts[kind]+=1
            if actor==0:counts['actor0']+=1
    stream.close()
    assert consumed+spoiled==3000,(name,consumed,spoiled)
    if name.endswith('witness'):assert kinds['Rejected']==0,(name,kinds['Rejected'])
    trace_audits[name]={'events':sum(kinds.values()),'counts':dict(kinds),'fifo_overtakes':0,'capacity_violations':0,'consumed':consumed,'spoiled':spoiled}
    excerpts.append('')
    if path.suffix!='.gz':
        with path.open('rb') as src,gzip.GzipFile(filename=str(path)+'.gz',mode='wb',mtime=0) as dst:
            while block:=src.read(1024*1024):dst.write(block)
        path.unlink()
(root/'TRACE_EXCERPTS.md').write_text('\n'.join(excerpts)+'\n')
(root/'trace-audits.json').write_text(json.dumps(trace_audits,indent=2)+'\n')
print('Verified 180 runs; 60 frozen reference/contested replays byte-equal by field; 117 physical witnesses; 11 independent FIFO/capacity/travel/material trace audits.')
for name in summary:
    print(name,{k:summary[name][k]['mean'] for k in ['jobs_accepted','food_delivered','survivors_72h','starvation_person_hours','diag_commitment_churn','diag_replacement_attempts']})
print('bulk best',max(int(r['delivered']) for r in witnesses))
print('nourished best',max(int(r['residents_nourished_48']) for r in witnesses))
