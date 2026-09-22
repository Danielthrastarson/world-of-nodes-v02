#!/usr/bin/env python3
"""Independent lifecycle, physical event, reversal and paired-outcome audits."""
import collections as C
import csv
import gzip
import hashlib
import json
from pathlib import Path
import re
import statistics as S
import sys

root=Path(sys.argv[1] if len(sys.argv)>1 else 'research/persistence')
names=['reactive','remembered','resumable','committed','self_priority']
H=3600000
def rows(path,delimiter=','):
    with path.open() as f:return list(csv.DictReader(f,delimiter=delimiter))
def save(name,value): (root/name).write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
old=rows(Path('research/acceptance/meal_aware-runs.csv'))
cache={};allruns={};summary={};audits={};reversal_rows=[];excerpts=[]
for name in names:
    runs=rows(root/f'{name}-runs.csv');allruns[name]=runs
    assert [int(r['seed']) for r in runs]==list(range(30))
    for r in runs:
        assert r['accounting_error']==r['queue_overtakes']=='0'
        assert int(r['food_consumed'])+int(r['food_remaining'])+int(r['waste_remaining'])==3000
        assert r['jobs_accepted']==r['diag_jobs_accepted']
        assert r['food_delivered']==r['diag_delivered_quanta']
        if name in ['reactive','remembered'] or (name=='self_priority' and r['diag_ABLATION_CHANGED_DECISION']=='0'):
            for k,v in old[int(r['seed'])].items():
                if k not in ['scenario','shadow_meal_passes']:assert r[k]==v,(name,k,r[k],v)
    summary[name]={k:dict(mean=S.mean(float(r[k]) for r in runs),min=min(float(r[k]) for r in runs),max=max(float(r[k]) for r in runs))
                   for k in runs[0] if k not in ['scenario','seed','event_hash','state_hash']}
    index=json.loads((root/f'{name}-journal-index.json').read_text())
    assert [i['seed'] for i in index]==list(range(30))
    plan_rows=rows(root/f'{name}-plans.tsv','\t')
    for item in index:
        sha=item['canonical_sha256']
        if sha not in cache:
            with gzip.open(root/item['path'],'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==sha
            counts=C.Counter();reasons=C.Counter();plans={};observations={};reversals=[];selected=[];noinfo_cancel=C.Counter();queue_changes=C.Counter()
            row_count=0
            with gzip.open(root/item['path'],'rt') as f:
                for r in csv.DictReader(f,delimiter='\t'):
                    row_count+=1;kind=r['kind'];counts[kind]+=1
                    key=(int(r['agent']),int(r['plan'])) if r['plan'] else None
                    if kind in ['INTENT_CANCELLED','INTENT_PAUSED','DIRECTION_REVERSAL']:reasons[kind,r['reason']]+=1
                    if kind=='INTENT_CREATED':
                        assert key not in plans
                        assert 'board_node: 0' in r['detail'] and 'request_id: 0' in r['detail']
                        plans[key]='ACTIVE'
                    elif kind=='INTENT_PAUSED':
                        assert plans[key]=='ACTIVE',(name,key,plans[key])
                        plans[key]='PAUSED'
                    elif kind=='INTENT_RESUMED':
                        assert plans[key]=='PAUSED'
                        plans[key]='ACTIVE'
                    elif kind=='INTENT_CANCELLED':
                        assert plans[key] in ['ACTIVE','PAUSED']
                        plans[key]='CANCELLED:'+r['reason']
                        if 'changes={}' in r['detail']:noinfo_cancel[r['reason']]+=1
                    elif kind=='INTENT_COMPLETED':
                        assert plans[key] in ['ACTIVE','PAUSED'] and r['location']=='Node(0)'
                        plans[key]='COMPLETED'
                    if kind=='DIRECTION_REVERSAL':
                        informed='new_information=true' in r['detail']
                        assert informed != ('new_information=false' in r['detail'])
                        if informed:assert 'changes={}' not in r['detail']
                        queue_changes.update(re.findall(r'QUEUE_SERVICE:[^"}]+',r['detail']))
                        reversals.append(dict(agent=int(r['agent']),plan=r['plan'],time_h=int(r['time_ms'])/H,
                            nutrition_h=int(r['nutrition_ms'])/H,fatigue_h=int(r['fatigue_ms'])/H,
                            new_information=informed,reason=r['reason'],detail=r['detail']))
                    if kind!='DECISION' and (r['agent']=='0' or kind=='INTENT_COMPLETED'):
                        selected.append(r)
            assert row_count==item['rows']
            assert counts['UNINFORMED_REVERSAL']==sum(not r['new_information'] for r in reversals)
            assert counts['BOARD_REVERSAL']==sum(bool(r['plan']) for r in reversals)
            assert counts['UNINFORMED_BOARD_REVERSAL']==sum(bool(r['plan']) and not r['new_information'] for r in reversals)
            cache[sha]=dict(counts=counts,reasons=reasons,plans=plans,reversals=reversals,selected=selected,
                            noinfo_cancel=noinfo_cancel,rows=row_count,queue_changes=queue_changes)
        result=cache[sha];seed=item['seed']
        for k,v in runs[seed].items():
            if k.startswith('diag_') and k[5:].isupper():assert int(v)==result['counts'][k[5:]],(name,k,v)
        raw={(int(r['agent']),int(r['plan'])):r['status'] for r in plan_rows if int(r['seed'])==seed}
        assert raw==result['plans']
        for r in result['reversals']:reversal_rows.append(dict(scenario=name,seed=seed,**r))
        if seed==0:
            created=result['counts']['INTENT_CREATED'];completed=result['counts']['INTENT_COMPLETED']
            reversals=result['counts']['DIRECTION_REVERSAL'];board_reversals=result['counts']['BOARD_REVERSAL']
            audits[name]=dict(rows=result['rows'],counts=dict(result['counts']),
                reasons={f'{k[0]}:{k[1]}':v for k,v in result['reasons'].items()},
                no_material_information_cancellations=dict(result['noinfo_cancel']),
                plan_completion_rate=completed/created if created else None,
                uninformed_reversal_rate=result['counts']['UNINFORMED_REVERSAL']/reversals if reversals else None,
                uninformed_board_reversal_rate=result['counts']['UNINFORMED_BOARD_REVERSAL']/board_reversals if board_reversals else None,
                horizon_status=dict(C.Counter(result['plans'].values())))
            save(f'{name}-trace-selections.json',result['selected'])
            excerpts.extend([f'## {name}',''])
            for r in result['selected']:
                if r['kind'] in ['INTENT_CREATED','INTENT_REACHED_X','INTENT_COMPLETED','INTENT_CANCELLED','INTENT_PAUSED','DIRECTION_REVERSAL']:
                    detail=r['detail'] if r['kind'] in ['DIRECTION_REVERSAL','INTENT_CANCELLED'] else r['reason']
                    excerpts.append(f'- {int(r["time_ms"])/H:.6f}h · agent {r["agent"]} · plan {r["plan"]} · {r["kind"]} · nutrition {int(r["nutrition_ms"])/H:.6f}h · fatigue {int(r["fatigue_ms"])/H:.6f}h: {detail}')
            excerpts.append('')
    agents=rows(root/f'{name}-agents.csv');assert len(agents)==4500
    health=[]
    for t in [36,48,72]:
        group=[r for r in agents if int(r['time_h'])==t];live=[r for r in group if r['alive']=='true']
        health.append(dict(time_h=t,survivors_per_run=len(live)/30,
            zero_nutrition_hours_per_run=sum(float(r['zero_nutrition_h']) for r in group)/30,
            continuously_nourished_per_run=sum(r['continuously_nourished']=='true' for r in group)/30,
            alive_nutrition_min=min(float(r['nutrition_h']) for r in live),
            alive_nutrition_median=S.median(float(r['nutrition_h']) for r in live),
            alive_nutrition_max=max(float(r['nutrition_h']) for r in live)))
    for seed in range(30):
        end=[r for r in agents if int(r['time_h'])==72 and int(r['seed'])==seed]
        assert sum(int(r['consumed']) for r in end)==int(runs[seed]['food_consumed'])
        assert abs(sum(float(r['zero_nutrition_h']) for r in end)-float(runs[seed]['starvation_person_hours']))<1e-5
    save(f'{name}-health.json',health)

save('summary.json',summary);save('lifecycle-audits.json',audits)
with (root/'reversals.csv').open('w') as f:
    writer=csv.DictWriter(f,fieldnames=list(reversal_rows[0]));writer.writeheader();writer.writerows(reversal_rows)
paired={}
for name in names[1:]:
    paired[name]={}
    for k in summary[name]:
        delta=[float(a[k])-float(b[k]) for a,b in zip(allruns[name],allruns['reactive'])]
        paired[name][k]=dict(mean_delta=S.mean(delta),min_delta=min(delta),max_delta=max(delta))
save('paired-effects.json',paired)

event_pattern=re.compile(r'Event \{ id: (\d+), at: (\d+), agent: (None|Some\((\d+)\)), cause: (.*?), kind: "([^"]+)", detail: "(.*)" \}')
trace_audits={}
for name in names:
    queued={};moving={};last_leg={};positions={i:0 for i in range(50)};dead=set();counts=C.Counter();consumed=spoiled=0;queue_records=[];service=[]
    with gzip.open(root/f'{name}.trace.gz','rt') as f:
        for line in f:
            match=event_pattern.fullmatch(line.strip());assert match,line
            eid,at,_,actor,cause,kind,detail=match.groups();eid=int(eid);at=int(at);actor=int(actor) if actor is not None else None
            counts[kind]+=1
            def field(key):return int(re.search(rf'{key}=(\d+)',detail)[1])
            if kind=='Decision':assert actor not in dead
            if kind=='Queued':
                edge=field('edge');assert actor not in queued and actor not in moving
                assert positions[actor] in [(0,1),(1,2)][edge]
                queued[actor]=(edge,at,actor,field('target'))
            elif kind in ['Interrupted','Died']:
                q=queued.pop(actor,None)
                if q and q[3]==0:
                    queue_records.append(dict(agent=actor,joined_h=q[1]/H,left_h=at/H,wait_h=(at-q[1])/H,edge=q[0],end=kind,detail=detail))
                if kind=='Died':dead.add(actor);moving.pop(actor,None)
            elif kind=='Admitted':
                edge,joined,identity,target=queued.pop(actor)
                assert not any(q[0]==edge and q[1:3]<(joined,identity) for q in queued.values())
                start,to=field('from'),field('to');assert start==positions[actor]
                assert {start,to}==set([(0,1),(1,2)][edge])
                if last_leg.get(actor)==(edge,to,start):counts['OPPOSITE_TRAVERSAL']+=1
                moving[actor]=(edge,at,start,to)
                assert sum(v[0]==edge for v in moving.values())<=4
                if target==0:queue_records.append(dict(agent=actor,joined_h=joined/H,left_h=at/H,wait_h=(at-joined)/H,edge=edge,end='Admitted',detail=detail))
                if name=='committed' and edge==0 and 8*H<=at<=17*H:
                    service.append(dict(time_h=at/H,agent=actor,joined_h=joined/H,event=line.strip()))
            elif kind=='Arrived':
                edge,departed,start,to=moving.pop(actor)
                assert field('node')==to and field('edge')==edge and at-departed>=2*H
                if last_leg.get(actor)==(1,2,1) and (edge,start,to)==(1,1,2):counts['F_X_F']+=1
                last_leg[actor]=(edge,start,to);positions[actor]=to
                if to==0:counts['BOARD_ARRIVAL']+=1
            elif kind=='Consumed':consumed+=field('q')
            elif kind=='Spoiled':assert at==48*H;spoiled+=field('q')
    assert consumed+spoiled==3000
    for k in ['OPPOSITE_TRAVERSAL','F_X_F','BOARD_ARRIVAL']:assert counts[k]==int(allruns[name][0]['diag_'+k]),(name,k,counts[k])
    trace_audits[name]=dict(events=dict(counts),consumed=consumed,spoiled=spoiled,fifo_overtakes=0,capacity_violations=0)
    save(f'{name}-board-queues.json',queue_records)
    if name=='committed':save('committed-edge0-service.json',service)
save('physical-trace-audits.json',trace_audits)
source=Path('research/acceptance')
old_index=json.loads((source/'meal_aware-journal-index.json').read_text())[0]
feedback=[]
with gzip.open(source/old_index['path'],'rt') as f:
    for r in csv.DictReader(f,delimiter='\t'):
        if r['agent']=='0' and int(r['time_ms']) in [18120000,18180000,18182000]:feedback.append(r)
save('preserved-forecast-feedback.json',dict(source=str(source/old_index['path']),canonical_sha256=old_index['canonical_sha256'],records=feedback))
(root/'TRACE_EXCERPTS.md').write_text('# Temporal-consistency traces\n\n'+ '\n'.join(excerpts)+'\n')
print('Verified 150 runs, 90 exact frozen-policy replays, lifecycle transitions, reversal counts, food and nutrition accounting, FIFO and physical trajectories.')
for name in names:print(name,{k:summary[name][k]['mean'] for k in ['diag_INTENT_CREATED','diag_INTENT_COMPLETED','jobs_accepted','food_delivered','survivors_72h','starvation_person_hours']})
