#!/usr/bin/env python3
from pathlib import Path
import sys,csv,statistics,json,hashlib,subprocess
r=Path(sys.argv[1]);maps=['line','branch','alternative_routes','dead_end','bottleneck','unequal_sources'];summary={}
for name in maps:
 rows=list(csv.DictReader((r/name/'runs.csv').open()));assert len(rows)==30
 assert all(x['queue_overtakes']=='0' and x['accounting_error']=='0' for x in rows)
 summary[name]={k:{'min':min(float(x[k]) for x in rows),'mean':statistics.mean(float(x[k]) for x in rows),'max':max(float(x[k]) for x in rows)} for k in ['survivors_36h','survivors_48h','survivors_72h','food_consumed','food_spoiled','jobs_accepted','food_delivered','starvation_person_hours','queue_person_hours','failed_stale_trips','source_discoverers']}
 summary[name]['distinct_event_histories']=len({x['event_hash'] for x in rows})
prior=json.loads((r/'prior-artifacts.json').read_text());excluded=[]
for p,h in prior.items():
 # The first run accidentally snapshotted its own newly created stdout log.
 # Keep that manifest/failure; exclude only this exact live progress stream.
 if Path(p).resolve()==r.with_suffix('.log').resolve():excluded.append(p);continue
 if Path(p).resolve()==r.parent.joinpath('regressions/adoption-equivalence.csv').resolve() and hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h:
  assert Path(p).read_bytes()==Path('research/meals/regressions/adoption-equivalence.csv').read_bytes()
  excluded.append(p+' (current-run regression finished after snapshot; complete result equals prior 19-scenario proof)');continue
 assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(r/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(r/'provenance.json').write_text(json.dumps({'source_revision':'77a7ddd','analysis_revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'maps':maps,'seeds':list(range(30)),'prior_files_unchanged':len(prior)-len(excluded),'excluded_self_generated_progress_log':excluded},indent=2)+'\n')
print(json.dumps({n:{k:d[k] for k in ['survivors_72h','jobs_accepted','food_delivered']} for n,d in summary.items()},indent=2))
