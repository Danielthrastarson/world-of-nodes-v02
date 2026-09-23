#!/usr/bin/env python3
"""Post-run descriptive metrics. Does not change parameters or autonomous decisions."""
import csv,gzip,json,re,collections,sys
from pathlib import Path
root=Path(sys.argv[1]);out={}
for arm in sorted(p for p in root.iterdir() if p.is_dir() and (p/'runs.csv').exists()):
 rows=[r for r in csv.DictReader((arm/'runs.csv').open()) if r['seed']=='0']
 trips=[r for r in csv.DictReader((arm/'trips.csv').open()) if r['seed']=='0']
 with gzip.open(arm/'projects.tsv.gz','rt') as f:p=next(f)
 m=re.search(r'completed_at: Some\((\d+)\)',p);completion=int(m[1]) if m else None
 q={};peak=collections.Counter();waits=collections.defaultdict(list);deaths_queued=collections.Counter();kinds=collections.Counter();excerpt=[]
 with gzip.open(arm/'events.tsv.gz','rt') as f:
  for line in f:
   m=re.search(r'at: (\d+), agent: (?:Some\((\d+)\)|None).*?kind: "([^"]+)"',line)
   if not m:continue
   at=int(m[1]);a=m[2];kind=m[3];kinds[kind]+=1
   if kind=='Queued':
    edge=int(re.search(r'edge=(\d+)',line)[1]);q[a]=(edge,at);peak[edge]=max(peak[edge],sum(e==edge for e,t in q.values()))
   elif kind=='Admitted':
    if a in q:
     edge,start=q.pop(a);waits[edge].append((at-start)/3600000)
   elif kind=='Interrupted' and 'Queued' in line:q.pop(a,None)
   elif kind=='Died':
    if a in q:deaths_queued[q.pop(a)[0]]+=1
   if kind in ['CrossingCompleted','CrossingWorkStarted','CrossingDonation'] and (len(excerpt)<28 or kind=='CrossingCompleted'):excerpt.append(line.strip())
 routes=collections.Counter();byagent=collections.defaultdict(list)
 for t in trips:
  routes[(t['edge'],'after' if completion is not None and int(t['admitted_ms'])>=completion else 'before')]+=1
  byagent[t['agent']].append(t)
 reversals=sum(a['edge']==b['edge'] and a['from']==b['to'] and a['to']==b['from'] for ts in byagent.values() for a,b in zip(ts,ts[1:]))
 out[arm.name]={'checkpoints':rows,'completion_h':completion/3600000 if completion is not None else None,'project_seed0':p.strip(),
 'queue_peak':dict(peak),'admitted_wait_h':{str(e):{'n':len(w),'mean':sum(w)/len(w),'max':max(w)} for e,w in waits.items()},
 'deaths_while_queued':dict(deaths_queued),'admissions_by_edge_phase':{str(k):v for k,v in routes.items()},
 'consecutive_edge_reversals':reversals,'event_counts':dict(kinds),'construction_events':excerpt}
(root/'analysis.json').write_text(json.dumps(out,indent=2)+'\n')
for name,a in out.items():print(name,'completed',a['completion_h'],'peak',a['queue_peak'],'waits',a['admitted_wait_h'],'queued deaths',a['deaths_while_queued'])
