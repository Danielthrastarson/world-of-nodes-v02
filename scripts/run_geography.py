#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,json,hashlib,gzip,shutil,csv,statistics
r=Path(sys.argv[1]);r.mkdir(parents=True,exist_ok=False)
prior={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and r.resolve() not in p.resolve().parents}
(r/'prior-artifacts.json').write_text(json.dumps(prior,indent=2)+'\n')
maps=['line','branch','alternative_routes','dead_end','bottleneck','unequal_sources']
def run(name):
 with (r/f'{name}.log').open('w') as out:subprocess.run(['target/release/geography_probe',name,str(r/name),'30'],check=True,stdout=out,stderr=out)
 p=r/name/'events.tsv'
 with p.open('rb') as src,p.with_suffix('.tsv.gz').open('wb') as dst:
  with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
 p.unlink();print(name,'complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,maps))
summary={}
for name in maps:
 rows=list(csv.DictReader((r/name/'runs.csv').open()));assert len(rows)==30
 assert all(x['queue_overtakes']=='0' and x['accounting_error']=='0' for x in rows)
 summary[name]={k:{'min':min(float(x[k]) for x in rows),'mean':statistics.mean(float(x[k]) for x in rows),'max':max(float(x[k]) for x in rows)} for k in ['survivors_36h','survivors_48h','survivors_72h','food_consumed','food_spoiled','jobs_accepted','food_delivered','starvation_person_hours','queue_person_hours','failed_stale_trips','source_discoverers']}
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(r/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(r/'provenance.json').write_text(json.dumps({'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'maps':maps,'seeds':list(range(30)),'prior_files_unchanged':len(prior)},indent=2)+'\n')
