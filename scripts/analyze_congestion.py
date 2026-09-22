#!/usr/bin/env python3
"""Independent audits of conditional transport, frozen replay and paired forecasting."""
import collections as C
import copy
import csv
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys

csv.field_size_limit(10**7)
root=Path(sys.argv[1] if len(sys.argv)>1 else 'research/congestion')
H=3600000

def read(path,sep=','):
    with path.open() as f:return list(csv.DictReader(f,delimiter=sep))
def save(name,x): (root/name).write_text(json.dumps(x,indent=2,sort_keys=True)+'\n')
def opening(path):return gzip.open(path,'rt') if path.suffix=='.gz' else path.open()
def pathof(name):
    p=root/name
    return p if p.exists() else p.with_suffix(p.suffix+'.gz')
pattern=re.compile(r'^Event \{ id: (\d+), at: (\d+), agent: (None|Some\((\d+)\)), cause: .*?, kind: "([^"\n]+)", detail: "(.*)" \}$')
def events(path):
    with opening(path) as f:
        for line in f:
            m=pattern.match(line.strip())
            if m:yield dict(t=int(m[2]),agent=int(m[4]) if m[4] else None,kind=m[5],detail=m[6])
def blank():return dict(nodes={i:0 for i in range(50)},queue={},travel={},dead=set())
def apply(state,e,audit=True):
    t,i,k,d=e['t'],e['agent'],e['kind'],e['detail']
    if k=='Decision':assert i not in state['dead']
    if k=='Queued':
        edge=int(re.search(r'edge=(\d+)',d)[1]);assert i not in state['travel']
        assert state['nodes'][i] in [(0,1),(1,2)][edge]
        state['queue'][i]=(t,i,edge)
    elif k=='Interrupted':state['queue'].pop(i,None)
    elif k=='Admitted':
        edge,a,b=map(int,re.findall(r'(?:edge|from|to)=(\d+)',d))
        assert state['nodes'][i]==a and {a,b}==set([(0,1),(1,2)][edge])
        q=state['queue'][i]
        assert q[2]==edge
        eligible=[v for v in state['queue'].values() if v[2]==edge]
        assert q==min(eligible),('FIFO',e,eligible)
        assert sum(v[0]==edge for v in state['travel'].values())<4
        state['queue'].pop(i);state['travel'][i]=(edge,t,a,b)
    elif k=='Arrived':
        node,edge=map(int,re.findall(r'(?:node|edge)=(\d+)',d))
        x=state['travel'].pop(i);assert x[0]==edge and x[3]==node
        assert t-x[1]>=2*H,('faster than nominal',e,x)
        state['nodes'][i]=node
    elif k=='Died':
        state['dead'].add(i);state['queue'].pop(i,None);state['travel'].pop(i,None)

original=list(events(Path('research/persistence/committed.trace.gz')))
bottleneck=read(root/'bottleneck.csv');replays=read(root/'replays.csv');micro=read(root/'micro.csv')
assert len(bottleneck)==8 and len(replays)==48 and len(micro)==328
frozen=read(Path('research/persistence/committed-runs.csv'))[0]
provenance=(root/'rehydration.txt').read_text()
assert 'event_hash='+frozen['event_hash'] in provenance and 'state_hash='+frozen['state_hash'] in provenance
rehydrated=dict(zip(frozen.keys(),provenance.splitlines()[-1].split(',')))
# Rehydration emits base experiment fields, while frozen persistence CSV also has counters.
for k,v in rehydrated.items():assert frozen[k]==v,(k,frozen[k],v)

snapshots={}
queue_details=[];occupied_details=[]
for row in bottleneck:
    t=round(float(row['arrival_x_h'])*H);i=int(row['agent'])
    if t not in snapshots:
        s=blank()
        for e in original:
            if e['t']>t:break
            apply(s,e)
        snapshots[t]=s
    s=snapshots[t];key=s['queue'][i]
    ahead=sorted(v for v in s['queue'].values() if v[2]==0 and v<key)
    opposite=[v for v in ahead if s['nodes'][v[1]]==0]
    assert len(ahead)==int(row['ahead']) and len(opposite)==int(row['opposite'])
    assert sum(v[0]==0 for v in s['travel'].values())==int(row['occupants'])
    for rank,(joined,j,edge) in enumerate(ahead):queue_details.append(dict(focal=i,fork_h=t/H,rank=rank+1,agent=j,joined_h=joined/H,from_node=s['nodes'][j],to_node=1-s['nodes'][j]))
    for j,(edge,admitted,a,b) in s['travel'].items():
        if edge==0:occupied_details.append(dict(focal=i,fork_h=t/H,agent=j,admitted_h=admitted/H,from_node=a,to_node=b))

save("queue-ahead.json",queue_details);save("occupancy.json",occupied_details)
physical_audits={};exact=[]
for row in replays:
    if not row['case'].startswith('load_'):continue
    i=int(row['agent']);pct=int(row['case'].split('_')[1]);t=round(float(row['start_h'])*H)
    s=copy.deepcopy(snapshots[t]);key=s['queue'][i]
    migrants=sorted(j for j,v in s['queue'].items() if v[2]==0 and v<key and s['nodes'][j]==0)
    keep=len(migrants)*pct//100
    for j in migrants[keep:]:s['queue'].pop(j)
    ev=list(events(pathof(f'replay-{i}-{pct}.trace')))
    for e in ev:apply(s,e)
    rejected=C.Counter(e['detail'] for e in ev if e['kind']=='Rejected')
    if pct==100:
        end=round(float(row['stop_h'])*H)
        # Exact focal decision reasons and physical events, excluding unstable event/cause IDs.
        a=[e for e in original if t<e['t']<=end and e['agent']==i]
        b=[e for e in ev if e['agent']==i]
        assert a==b,('focal replay mismatch',i,next(((x,y) for x,y in zip(a,b) if x!=y),None),len(a),len(b))
        exact.append(dict(agent=i,events=len(a),stop_h=row['stop_h']))
    arrivals=[e for e in ev if e['agent']==i and e['kind']=='Arrived' and e['detail']=='node=0 edge=0']
    assert bool(arrivals)==bool(row['arrival_h'])
    if arrivals:assert abs(arrivals[0]['t']/H-float(row['arrival_h']))<1e-8
    physical_audits[f'{i}-{pct}']=dict(events=len(ev),rejected=dict(rejected),fifo_overtakes=0)
save('replay-audits.json',dict(exact_focal_replays=exact,physical=physical_audits))

thresholds={};direction_differences=[]
for i in ['0','4']:
    groups={d:[r for r in micro if r['agent']==i and r['case']==d] for d in ['same','opposite','mixed','observed']}
    for d,rows in groups.items():
        assert [int(r['ahead']) for r in rows]==list(range(41))
        bad=[int(r['ahead']) for r in rows if not r['arrival_h']]
        threshold=min(bad)
        assert all(bool(r['arrival_h'])==(int(r['ahead'])<threshold) for r in rows)
        thresholds[f'{i}-{d}']=threshold
    for n in range(41):
        values={d:{k:v for k,v in rows[n].items() if k!='case'} for d,rows in groups.items()}
        if any(v!=values['opposite'] for v in values.values()):direction_differences.append(dict(agent=i,ahead=n,outcomes=values))
save('queue-thresholds.json',dict(first_failure=thresholds,directional_differences=direction_differences))

# Selected micro traces independently verify the synthetic initial queues, shared
# capacity and the event-based C-arrival metric (arrival may immediately start a
# new legal action at the same boundary, so endpoint location alone is insufficient).
micro_audits=[]
for i in [0,4]:
    t=round(float(next(r for r in bottleneck if int(r['agent'])==i)['arrival_x_h'])*H)
    for direction in ['same','opposite']:
        for count in [0,3,4,8,11,12,40]:
            st=blank();st['nodes'][i]=1;st['queue'][i]=(t,i,0)
            for rank,j in enumerate(j for j in range(50) if j!=i):
                if rank<count:
                    st['nodes'][j]=1 if direction=='same' else 0
                    st['queue'][j]=(t-1,j,0)
            ev=list(events(pathof(f'micro-{i}-{direction}-{count}.trace')))
            for e in ev:apply(st,e)
            row=next(r for r in micro if int(r['agent'])==i and r['case']==direction and int(r['ahead'])==count)
            arrivals=[e for e in ev if e['agent']==i and e['kind']=='Arrived' and e['detail']=='node=0 edge=0']
            assert bool(arrivals)==bool(row['arrival_h'])
            if arrivals:assert abs(arrivals[0]['t']/H-float(row['arrival_h']))<1e-8
            micro_audits.append(dict(agent=i,direction=direction,ahead=count,events=len(ev)))
save('selected-micro-audits.json',micro_audits)

# Physiology-aware waiting budget, independent of Rust prediction implementation.
def travel(n,f,delay=0):
    n=max(0,n-delay);f=min(24,f+delay);left=2.;dt=0
    while left>1e-10:
        rate=(1 if n>6+1e-10 else .8 if n>1e-10 else .5)*(1 if f<12-1e-10 else .9 if f<16-1e-10 else .75 if f<20-1e-10 else .5)
        bounds=[left/rate]
        bounds += [x-f for x in [12,16,20,22,24] if x-f>1e-9]
        bounds += [n-x for x in [6,0] if n-x>1e-9]
        step=min(bounds);left-=step*rate;dt+=step;n=max(0,n-step);f=min(24,f+step)
    return dt
budgets=[]
for r in bottleneck:
    n=float(r['nutrition_h']);f=float(r['fatigue_h']);u=travel(n,f)
    assert abs(u-float(r['unqueued_travel_h']))<1e-8
    lo,hi=0,n-6
    for _ in range(60):
        mid=(lo+hi)/2
        if mid+travel(n,f,mid)<=n-6:lo=mid
        else:hi=mid
    lo_any,hi_any=0,min(n-6,22-f)
    for _ in range(60):
        mid=(lo_any+hi_any)/2
        if mid+travel(n,f,mid)<=min(n-6,22-f):lo_any=mid
        else:hi_any=mid
    budgets.append(dict(agent=int(r['agent']),requested_hunger_budget_h=n-6-u,
                        requested_first_override_budget_h=min(n-6,22-f)-u,
                        latest_wait_for_arrival_before_critical_h=lo,
                        latest_wait_for_arrival_before_first_override_h=lo_any,
                        first_node_override_without_meal_h=min(n-6,22-f),hunger_override_h=n-6,fatigue_override_h=22-f))
save('physiological-budgets.json',budgets)

summary={};paired={};journal_audits={};forecast_examples={}
for arm in ['reactive','forecast_corrected']:
    rows=read(root/f'{arm}-runs.csv');assert len(rows)==30 and [int(r['seed']) for r in rows]==list(range(30))
    if arm=='reactive':
        old=read(Path('research/persistence/reactive-runs.csv'))
        for r,prior in zip(rows,old):
            for k,v in prior.items():assert r[k]==v,(k,r[k],v)
    for r in rows:
        assert r['accounting_error']==r['queue_overtakes']=='0'
        assert int(r['food_consumed'])+int(r['food_remaining'])+int(r['waste_remaining'])==3000
        assert r['jobs_accepted']==r['food_delivered']=='0'
    assert len({tuple((k,v) for k,v in r.items() if k not in ['seed','state_hash']) for r in rows})==1
    summary[arm]=rows[0]
    index=json.loads((root/f'{arm}-journal-index.json').read_text());assert len(index)==30
    unique={x['path']:x for x in index};assert len(unique)==1
    record=next(iter(unique.values()));counts=C.Counter();reasons=C.Counter();agents=set();selected=[]
    with gzip.open(root/record['path'],'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==record['canonical_sha256']
    with gzip.open(root/record['path'],'rt') as f:
        for r in csv.DictReader(f,delimiter='\t'):
            counts[r['kind']]+=1
            if r['kind']=='INTENT_CREATED':agents.add(r['agent'])
            if r['kind']=='INTENT_CANCELLED':reasons[r['reason']]+=1
            if r['kind'] in ['BOARD_REVERSAL','INTENT_REACHED_X'] or (r['agent']=='0' and r['kind'] in ['INTENT_CREATED','INTENT_CANCELLED']):selected.append(r)
    for k,v in counts.items():
        if 'diag_'+k in rows[0]:assert int(rows[0]['diag_'+k])==v
    journal_audits[arm]=dict(counts=dict(counts),cancellations=dict(reasons),agents_forming_intentions=len(agents))
    save(f'{arm}-selected-plans.json',selected)
    gates=read(root/f'{arm}-forecast-counts.csv')
    canonical=lambda seed:{(r['scope'],r['stage'],r['gate']):int(r['count']) for r in gates if int(r['seed'])==seed}
    assert all(canonical(s)==canonical(0) for s in range(30))
    observed=C.Counter();examples=[]
    with opening(pathof(f'{arm}-forecasts.tsv')) as f:
        for r in csv.DictReader(f,delimiter='\t'):
            observed[('ALL',r['stage'],r['primary_gate'])]+=1
            if r['local_request']=='true':observed[('LOCAL',r['stage'],r['primary_gate'])]+=1
            if r['agent']=='0' and r['time_ms'] in ['18120000','18180000']:examples.append(r)
    assert observed==canonical(0)
    forecast_examples[arm]=examples
    # Independent capacity/FIFO/death/minimum-travel audit for full autonomous seed 0.
    st=blank();n_events=0
    for e in events(root/f'{arm}.trace.gz'):apply(st,e);n_events+=1
    journal_audits[arm]['physical_events_audited']=n_events
    ar=read(root/f'{arm}-agents.csv')
    for seed,r in enumerate(rows):
        last=[a for a in ar if int(a['seed'])==seed and a['time_h']=='72']
        assert len(last)==50
        assert abs(sum(float(a['zero_nutrition_h']) for a in last)-float(r['starvation_person_hours']))<1e-6
        assert sum(int(a['consumed']) for a in last)==int(r['food_consumed'])
for k in summary['reactive']:
    if k not in ['scenario','seed','event_hash','state_hash']:
        paired[k]=float(summary['forecast_corrected'][k])-float(summary['reactive'][k])
save('autonomous-summary.json',summary);save('paired-effects.json',paired);save('journal-audits.json',journal_audits);save('forecast-first-divergence.json',forecast_examples)
print('Validated 328 micro-fixtures, 40 load replays, 8 conditional service projections, 60 autonomous runs, 8 exact focal replays and 30 frozen control matches.')
