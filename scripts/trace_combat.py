#!/usr/bin/env python3
"""Reconstruct local contact and unlawful custody events from preserved traces."""
from pathlib import Path
import csv,gzip,json,re,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
checks=[];chains={}
for d in sorted(p for p in root.iterdir() if p.is_dir()):
 rows=list(csv.DictReader((d/'runs.csv').open()));r=rows[0];case=r['case']
 count=17 if case=='congestion' else 10 if case=='household' else 9
 locations={i:1 if 6<=i<9 else 2 if case=='congestion' and i>=9 else 0 for i in range(count)}
 events=list(csv.DictReader(gzip.open(d/'events.tsv.gz','rt'),delimiter='\t'))
 started={};hits=0;stolen=0;reached=set();chain=[]
 for e in events:
  t=int(e['time_ms']);p=int(e['person'][5:-1]) if e['person'].startswith('Some(') else None;k=e['kind'];detail=e['detail']
  if k=='Admitted':locations[p]=('edge',int(re.search(r'edge=(\d+)',detail)[1]))
  if k=='Arrived':locations[p]=int(re.search(r'node=(\d+)',detail)[1])
  if k=='MusterArrived':reached.add(p)
  if k=='StrikeStarted':
   target=int(re.search(r'target=(\d+)',detail)[1]);assert isinstance(locations[p],int) and locations[p]==locations[target],(d.name,e)
   if p<6:assert p in reached,(d.name,'defender never assembled')
   started[p]=(t,target)
  if k=='StrikeHit':
   target=int(re.search(r'target=(\d+)',detail)[1]);assert p in started and started[p][1]==target
   assert t-started[p][0]>=15*60*1000,(d.name,'instant strike')
   assert isinstance(locations[p],int) and locations[p]==locations[target],(d.name,'nonlocal strike')
   hits+=1
  if k=='RaidStarted':assert locations[p]==2 and 6<=p<9;started[p]=(t,'raid')
  if k=='FoodStolen':
   assert locations[p]==2 and t-started[p][0]>=5*60*1000
   assert 'owner=Commons(0)' in detail and 'quantity=1' in detail;stolen+=1
  if k in ['Volunteered','MusterArrived','UnitOrderReceived','ProtectIssued','StrikeStarted','StrikeHit','Injured','Died','FoodStolen'] or k=='Arrived':chain.append(e)
 assert hits==int(r['hits']) and stolen==int(r['stolen']),(d.name,hits,stolen)
 checks.append({'case_arm':d.name,'seed':0,'local_timed_hits':hits,'local_timed_theft':stolen,'assembled':len(reached)})
 if d.name in ['local_defended','local_undefended','under_defended','late_defended','household_defended','congestion_defended']:chains[d.name]=chain
(out/'checks.json').write_text(json.dumps(checks,indent=2)+'\n')
(out/'causal-chains.json').write_text(json.dumps(chains,indent=2)+'\n')
print(len(checks),'traces independently checked for timed local contact/theft and assembly')
