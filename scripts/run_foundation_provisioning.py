#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,json,hashlib,gzip,shutil,csv,statistics
r=Path(sys.argv[1]);r.mkdir(parents=True,exist_ok=False)
prior={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and r.resolve() not in p.resolve().parents and p.resolve()!=r.with_suffix('.log').resolve()}
(r/'prior-artifacts.json').write_text(json.dumps(prior,indent=2)+'\n')
arms=[(s,a) for s in ['regional','compact'] for a in ['communal','self_only','idle','witness']]
def run(pair):
 s,a=pair;key=f'{s}_{a}'
 with (r/f'{key}.log').open('w') as out:subprocess.run(['target/release/provisioning_foundation_probe',s,a,str(r/key)],check=True,stdout=out,stderr=out)
 for p in (r/key).glob('*.tsv'):
  with p.open('rb') as src,p.with_suffix('.tsv.gz').open('wb') as dst:
   with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
  p.unlink()
 print(key,'complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,arms))
summary={}
for s,a in arms:
 key=f'{s}_{a}';rows=list(csv.DictReader((r/key/'runs.csv').open()));assert len(rows)==(1 if a=='witness' else 30)
 assert all(x['queue_overtakes']=='0' and x['accounting_error']=='0' for x in rows)
 summary[key]={k:{'min':min(float(x[k]) for x in rows),'mean':statistics.mean(float(x[k]) for x in rows),'max':max(float(x[k]) for x in rows)} for k in ['survivors_36h','survivors_48h','survivors_72h','jobs_accepted','food_delivered','food_consumed','food_spoiled','starvation_person_hours','queue_person_hours','alive_at_c','alive_at_f','diag_jobs_acquiring_cargo','diag_returns_started','diag_returns_completed','diag_jobs_delivered','diag_carrier_cargo_consumed','diag_communal_cargo_spoiled_transit']}
 summary[key]['distinct_event_histories']=len({x['event_hash'] for x in rows})
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(r/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(r/'provenance.json').write_text(json.dumps({'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'arms':arms,'seeds':list(range(30)),'prior_files_unchanged':len(prior)},indent=2)+'\n')
