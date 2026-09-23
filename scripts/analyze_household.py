#!/usr/bin/env python3
"""B1 paired evidence and trace checks; research only, no policy output."""
from pathlib import Path
import csv,gzip,hashlib,json,statistics,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
old=Path('research/household/matrix')
def read(p):return list(csv.DictReader(p.open()))
def avg(xs):return statistics.mean(xs)
summary={};paired={};equality=[];traces=[]
for d in sorted(root.iterdir()):
    if not d.is_dir():continue
    r=[x for x in read(d/'runs.csv') if x['hour']=='48'];assert len(r)==30
    assert all(x['accounting_error']=='0' and x['overtakes']=='0' for x in r)
    keys=[k for k in r[0] if k not in ['seed','hour','event_hash','state_hash']]
    summary[d.name]={k:{'mean':avg(float(x[k]) for x in r),'min':min(float(x[k]) for x in r),'max':max(float(x[k]) for x in r)} for k in keys}
    summary[d.name]['distinct_outcomes']=len({tuple(x[k] for k in keys) for x in r})
    m=read(d/'metrics.csv');ks={x['key'] for x in m};maps={s:{} for s in range(30)}
    for x in m:maps[int(x['seed'])][x['key']]=int(x['count'])
    summary[d.name]['counts']={k:avg(maps[s].get(k,0) for s in range(30)) for k in sorted(ks)}
    # Every rerun output must agree with the first matrix, except the documented TSV header.
    for p in d.iterdir():
        a=p.read_bytes();b=(old/d.name/p.name).read_bytes()
        if p.suffix=='.gz':
            a=gzip.decompress(a);b=gzip.decompress(b).replace(b'seed,time_ms,agent,kind,detail\n',b'seed\ttime_ms\tagent\tkind\tdetail\n',1)
        assert a==b,(d.name,p.name)
        equality.append(str(p.relative_to(root)))
    events=list(csv.DictReader(gzip.open(d/'events.tsv.gz','rt'),delimiter='\t'))
    care=[e for e in events if e['kind'].startswith('Care')]
    if d.name=='unknown_on':assert care and all(int(e['time_ms'])>=2*3600000 for e in care)
    if d.name=='unknown_off':assert not care
    if d.name.endswith('_on'):
        traces.append({'case':d.name,'care':care[:18],'first_consumptions':[e for e in events if e['kind']=='Consumed'][:8],'death':[e for e in events if e['kind']=='Died'][:5]})
for name in sorted({d.name.rsplit('_',1)[0] for d in root.iterdir() if d.is_dir()}):
    arms={a:{x['seed']:x for x in read(root/(name+'_'+a)/'runs.csv') if x['hour']=='48'} for a in ['off','on']}
    paired[name]={}
    for k in ['food_transferred','consumed','produced','starvation_h','alive','care_travel_h','crossings_completed']:
        delta=[float(arms['on'][str(s)][k])-float(arms['off'][str(s)][k]) for s in range(30)]
        paired[name][k]={'mean_delta':avg(delta),'min_delta':min(delta),'max_delta':max(delta),'changed_seeds':sum(x!=0 for x in delta)}
    activity={}
    for a in ['off','on']:
        table=read(root/(name+'_'+a)/'activity.csv');activity[a]={}
        for x in table:activity[a][(int(x['seed']),int(x['hour']),int(x['agent']),x['activity'])]=float(x['hours'])
    diffs={}
    for h in [6,48]:
        for group in ['caregiver','all']:
            for k in ['farming','construction','travel','queue','sleep','idle','gather']:
                vals=[]
                for s in range(30):
                    totals={a:sum(v for (seed,hour,agent,act),v in activity[a].items() if seed==s and hour==h and act==k and (group=='all' or agent==0)) for a in ['off','on']}
                    vals.append(totals['on']-totals['off'])
                diffs[f'{h}h:{group}:{k}']=avg(vals)
    paired[name]['activity_deltas_h']=diffs
prior=json.loads(Path('research/household/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
for name,obj in [('summary.json',summary),('paired.json',paired),('traces.json',traces),('validation.json',{'final_runs':660,'paired_seeds':list(range(30)),'prior_artifacts_unchanged':len(prior),'rerun_files_exact_except_documented_header':equality,'unknown_need_no_care_before_physical_meeting':True})]:
    with (out/name).open('x') as f:json.dump(obj,f,indent=2);f.write('\n')
print('660 runs validated; header-only correction preserves every metric and event row',flush=True)
