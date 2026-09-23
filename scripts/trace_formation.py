#!/usr/bin/env python3
"""Read-only seed0 event reconstruction; no new world runs or policy changes."""
from pathlib import Path
import csv,gzip,json,re,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
H=3600000
cases=json.loads((root/'provenance.json').read_text())['cases']
with (out/'edge-use-seed0.csv').open('x') as f:
 writer=csv.writer(f);writer.writerow(['case','arm','edge','from','to','cohort','admissions_after_command','completed_after_command','occupied_slot_hours_after_command'])
 for case in cases:
  for arm in ['hold','move']:
   d=root/(case+'_'+arm);start=int(re.search(r'command_time=(\d+)',(d/'parameters.txt').read_text())[1]);end=start+24*H
   rows=list(csv.DictReader(gzip.decompress((d/'events.tsv.gz').read_bytes()).decode().splitlines(),delimiter='\t'))
   pending={};stats={}
   def close(person,t,complete):
    if person not in pending:return
    began,key=pending.pop(person);s=stats.setdefault(key,[0,0,0]);s[1]+=int(complete and t>=start);s[2]+=max(0,min(t,end)-max(began,start))/H
   for e in rows:
    person=re.search(r'Some\((\d+)\)',e['person']);person=int(person[1]) if person else None;t=int(e['time_ms']);fields=dict(re.findall(r'(edge|from|to)=(\d+)',e['detail']))
    if e['kind']=='Admitted':
     assert person not in pending
     key=(fields['edge'],fields['from'],fields['to'],'original_recruits' if person<4 else 'original_civilians');stats.setdefault(key,[0,0,0])[0]+=int(t>=start);pending[person]=(t,key)
    elif e['kind'] in ['Arrived','Died']:close(person,t,e['kind']=='Arrived')
   for person in list(pending):close(person,end,False)
   for key,values in sorted(stats.items()):writer.writerow([case,arm,*key,*values])
with (out/'FOCAL_QUEUE_TRACE.md').open('x') as f:
 f.write('# Congested soldier3, seed0\n\nAbsolute clock, not hours since command. The order was issued at1h. These are observed events from the preserved final run.\n\n```text\n')
 lines=gzip.decompress((root/'congestion_move/events.tsv.gz').read_bytes()).decode().splitlines()[1:]
 for l in lines:
  v=l.split('\t');t=int(v[2]);kind=v[4]
  if v[3]=='Some(3)' and t<=43260000 and kind in ['UnitAssembled','UnitOrderReceived','UnitActionState','Queued','Interrupted','Admitted','Consumed','UnitDeparted','UnitArrived','UnitServiceEnded']:
   f.write(f'{t/H:.9f}h  event={v[1]}  {kind}  {v[5]}\n')
 f.write('```\n\nThe order persists across eating, but existing queue cancellation/reentry loses the old FIFO position. Expiry cancels the pending traversal. This trace supports an interaction between shared congestion, survival/action arbitration and finite commitment duration; it does not justify tuning any of them.\n')
