#!/usr/bin/env python3
"""F10 lifecycle, construction and cost interpretation; observations never feed policy."""
from pathlib import Path
from collections import Counter
import csv,gzip,json,re,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);trace=Path(sys.argv[3])
def rows(p):return list(csv.DictReader(p.open()))
def tsv(p):return list(csv.DictReader(gzip.decompress(p.read_bytes()).decode().splitlines(),delimiter='\t'))
result={};excerpts=[]
for d in sorted(p for p in root.iterdir() if p.is_dir()):
    rr=rows(d/'runs.csv');oo=rows(d/'offices.csv');last=[r for r in rr if r['hour']=='72']
    assert all(r['accounting_error']==r['overtakes']=='0' for r in rr)
    changes=tsv(d/'authority.tsv.gz');events=tsv(d/'events.tsv.gz')
    instants=Counter()
    # Independently reconstruct each office's active duration from transition timestamps.
    for o in oo:
        key=(o['seed'],o['office']);active=False;start=0;elapsed=0
        for t in changes:
            if (t['seed'],t['office'])!=key:continue
            now=int(t['at_ms']);on=t['active']=='true'
            assert on!=active
            if on:start=now
            else:elapsed+=now-start
            active=on
        if active:elapsed+=72*3600000-start
        assert abs(elapsed/3600000-float(o['active_h']))<1e-8,(d.name,key)
    projects=gzip.decompress((d/'projects.tsv.gz').read_bytes()).decode().splitlines()
    completed_times=[];failures=Counter()
    for p in projects:
        found=re.search(r'completed_at: Some\((\d+)\)',p)
        if found:completed_times.append(int(found[1])/3600000)
        else:
            wood=int(re.search(r', wood: (\d+)',p)[1]);cost=int(re.search(r'wood_cost: (\d+)',p)[1])
            # Classifications use intervention+actual mechanics; no hidden welfare score.
            if d.name.endswith('_pending'):failures['NO_AUTHORIZED_OBJECTIVE']+=1
            elif d.name.startswith('discovery_'):failures['NO_KNOWN_BENEFIT_NO_CONTRIBUTIONS']+=1
            elif wood<cost:failures['INSUFFICIENT_PHYSICAL_MATERIAL']+=1
            else:failures['INCOMPLETE_WORK']+=1
    result[d.name]={
        'runs':len(last),'appointments':len(oo),'commissions':sum(int(r['commissions']) for r in last),
        'completed_projects':len(completed_times),'completion_times_h':sorted(set(completed_times)),
        'incomplete_projects_at_72h':dict(failures),
        'offices_never_active':sum(not o['first_active_ms'] for o in oo),
        'holders_dead_unreplaced':sum(o['dead_without_replacement']=='true' for o in oo),
        'departure_events':sum(int(o['departures']) for o in oo),
        'replacement_attempts':0,'replacement_reason':'No autonomous replacement institution implemented; no research replacement command issued.',
        'seed0_event_kinds':dict(Counter(e['kind'] for e in events)),
        'seed0_offices':[o for o in oo if o['seed']=='0'],
        'population_checkpoints':{h:[{k:r[k] for k in ['alive','consumed','starvation_h','active_administration_h']} for r in rr if r['hour']==h] for h in ['36','48','72']},
        'outcome_variation':{k:len(set(r[k] for r in last)) for k in ['alive','consumed','starvation_h','completed','active_administration_h']},
    }
    for e in events:
        keep=e['kind'] in ['StewardAppointed','CrossingCommissioned','CrossingCompleted']
        if d.name.endswith('command'):
            keep|=e['agent'] in ['Some(9)','Some(10)'] and e['kind'] in ['Queued','Admitted','Arrived','Died','StarvationStarted','PerformanceChanged']
        elif d.name.endswith('commissioned'):
            keep|=e['kind'] in ['CrossingDonation','CrossingWorkStarted']
            keep|=e['agent']=='Some(1)' and e['kind']=='Decision' and int(e['at_ms']) in [0,60000,120000,180000,3720000]
        if keep:excerpts.append((d.name,e))
    for t in changes:
        if t['seed']=='0':excerpts.append((d.name,dict(at_ms=t['at_ms'],agent='office '+t['office'],kind='AuthorityTransition',detail=t['reason']+' active='+t['active'])))
with out.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
with trace.open('x') as f:
    f.write('# F10 causal excerpts\n\nSeed 0; exact millisecond records from compressed primary traces. Authority transitions are an observer journal, never remote messages.\n')
    for name in sorted(result):
        f.write('\n## '+name+'\n\n```text\n')
        for _,e in sorted((x for x in excerpts if x[0]==name),key=lambda x:int(x[1]['at_ms'])):
            f.write(f"{int(e['at_ms'])/3600000:.9f}h ({e['at_ms']}ms) {e['agent']} {e['kind']} {e['detail']}\n")
        f.write('```\n')
