#!/usr/bin/env python3
"""Paired physical effects; no inferred protective benefit or counterfactual job assignment."""
from pathlib import Path
import csv,gzip,json,statistics,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
def rows(p):return list(csv.DictReader(p.open()))
def mean(a):return statistics.mean(a)
cases=json.loads((root/'provenance.json').read_text())['cases'];summary={}
for case in cases:
 arm={a:rows(root/(case+'_'+a)/'runs.csv') for a in ['off','on']}
 assert [r['seed'] for r in arm['off']]==[r['seed'] for r in arm['on']]
 numeric=[k for k in arm['on'][0] if k not in ['seed','case','enabled','state_hash','event_hash']]
 data={a:{k:mean([float(r[k]) for r in arm[a]]) for k in numeric} for a in arm}
 delta={k:mean([float(b[k])-float(a[k]) for a,b in zip(arm['off'],arm['on'])]) for k in numeric}
 activity={a:{} for a in arm}
 for a in arm:
  for r in rows(root/(case+'_'+a)/'activity.csv'):
   k=r['activity'];activity[a][k]=activity[a].get(k,0)+float(r['hours'])/30
 counts={}
 for r in rows(root/(case+'_on')/'counts.csv'):
  k=r['key'];counts.setdefault(k,{'count_per_run':0,'unique_people_per_run':0});counts[k]['count_per_run']+=int(r['count'])/30;counts[k]['unique_people_per_run']+=int(r['unique_people'])/30
 summary[case]={'means':data,'paired_delta_on_minus_off':delta,'activity_person_hours':activity,'activity_delta_on_minus_off':{k:activity['on'].get(k,0)-activity['off'].get(k,0) for k in sorted(set(activity['on'])|set(activity['off']))},'gates_and_counts':counts,'within_arm_physical_variation':{a:{k:len(set(r[k] for r in arm[a])) for k in numeric if len(set(r[k] for r in arm[a]))>1} for a in arm}}
with (out/'paired.json').open('x') as f:json.dump(summary,f,indent=2);f.write('\n')
with (out/'TABLE.md').open('x') as f:
 f.write('| Case | Volunteers | Arrivals | Duty h | Δ farming h | Δ food produced | Δ starvation h | Δ queue h |\n|---|---:|---:|---:|---:|---:|---:|---:|\n')
 for c,s in summary.items():
  m=s['means']['on'];d=s['paired_delta_on_minus_off'];a=s['activity_delta_on_minus_off'];f.write(f"| {c} | {m['volunteers']:.2f} | {m['arrivals']:.2f} | {m['duty_h']:.3f} | {a.get('farming',0):.3f} | {d['food_produced']:.2f} | {d['starvation_h']:.3f} | {d['total_queue_h']:.3f} |\n")
with (out/'TRACE_EXCERPTS.md').open('x') as f:
 for case in cases:
  f.write('\n## '+case+' enabled, seed0\n\n```text\n')
  lines=gzip.decompress((root/(case+'_on')/'events.tsv.gz').read_bytes()).decode().splitlines()
  selected=[l for l in lines if any('\t'+k+'\t' in l for k in ['Volunteered','MusterArrived','ServiceEnded','CareTransferred','ServiceEvaluation'])]
  f.write('\n'.join(selected[:55])+'\n```\n')
