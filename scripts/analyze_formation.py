#!/usr/bin/env python3
"""Paired movement effect; no claim of protection, combat strength or optimality."""
from pathlib import Path
import csv,gzip,json,statistics,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
def rows(p):return list(csv.DictReader(p.open()))
def mean(xs):return statistics.mean(xs) if xs else None
meta=json.loads((root/'provenance.json').read_text());summary={}
for case in meta['cases']:
 arms={a:rows(root/(case+'_'+a)/'runs.csv') for a in ['hold','move']}
 assert [(r['seed'],r['pre_command_hash']) for r in arms['hold']]==[(r['seed'],r['pre_command_hash']) for r in arms['move']]
 numeric=[k for k in arms['move'][0] if k not in ['seed','case','arm','pre_command_hash','state_hash','event_hash']]
 means={a:{k:mean([float(r[k]) for r in arms[a] if r[k]!='']) for k in numeric} for a in arms}
 delta={k:mean([float(b[k])-float(a[k]) for a,b in zip(arms['hold'],arms['move']) if a[k]!='' and b[k]!='']) for k in numeric}
 pauses={a:{} for a in arms}
 for a in arms:
  for r in rows(root/(case+'_'+a)/'pauses.csv'):pauses[a][r['reason']]=pauses[a].get(r['reason'],0)+float(r['hours'])/len(meta['seeds'])
 summary[case]={'means':means,'paired_delta_move_minus_hold':delta,'pause_person_hours':pauses,'within_arm_ranges':{a:{k:[min(float(r[k]) for r in arms[a] if r[k]!=''),max(float(r[k]) for r in arms[a] if r[k]!='')] for k in numeric if len(set(r[k] for r in arms[a]))>1 and all(r[k]!='' for r in arms[a])} for a in arms}}
with (out/'paired.json').open('x') as f:json.dump(summary,f,indent=2);f.write('\n')
with (out/'TABLE.md').open('x') as f:
 f.write('| Case | Enlisted | Assembled at order | Received | Departed | Arrived in service | First / last arrival h | Spread h | Δ original civilian queue h | Δ starvation h |\n|---|---:|---:|---:|---:|---:|---|---:|---:|---:|\n')
 for case,s in summary.items():
  m=s['means']['move'];d=s['paired_delta_move_minus_hold'];fmt=lambda x:'—' if x is None else f'{x:.3f}'
  f.write(f"| {case} | {m['enlisted']:.1f} | {m['assembled_at_command']:.1f} | {m['receipts']:.1f} | {m['departed']:.1f} | {m['arrived']:.1f} | {fmt(m['first_arrival_h'])} / {fmt(m['last_arrival_h'])} | {fmt(m['arrival_spread_h'])} | {d['original_civilians_queue_h']:.3f} | {d['starvation_h']:.3f} |\n")
with (out/'TRACE_EXCERPTS.md').open('x') as f:
 for case in meta['cases']:
  f.write('\n## '+case+' move, seed0\n\n```text\n')
  lines=gzip.decompress((root/(case+'_move')/'events.tsv.gz').read_bytes()).decode().splitlines()
  selected=[l for l in lines if any('\t'+k+'\t' in l for k in ['UnitMoveIssued','UnitOrderReceived','UnitAssembled','UnitDeparted','UnitArrived','UnitEndpointAfterRelease','UnitServiceEnded','UnitActionState','CareTransferred'])]
  f.write('\n'.join(selected[:100])+'\n```\n')
with (out/'infrastructure.json').open('x') as f:
 on=summary['path_on']['means']['move'];off=summary['path_off']['means']['move'];json.dump({'path_on_minus_off_move':{k:on[k]-off[k] for k in numeric if on[k] is not None and off[k] is not None},'note':'Physical path effect only; same capacity/FIFO. No speed modifier attached to unit.'},f,indent=2);f.write('\n')
