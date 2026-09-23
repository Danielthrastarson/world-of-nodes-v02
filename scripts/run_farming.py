#!/usr/bin/env python3
"""Execute only the pre-registered F3 matrix; never overwrite prior evidence."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,json,hashlib,gzip,shutil,csv,statistics
r=Path(sys.argv[1]);r.mkdir(parents=True,exist_ok=False)
prior=json.loads(Path('research/farming/prior-artifacts.json').read_text())
arms=[('line','none','C')]+[('line',m,s) for m in ['base','slow','low_yield'] for s in ['C','F']]+[('branch','none','C')]+[('branch','base',s) for s in ['C','F']]
def run(a):
 key='_'.join(a)
 with (r/(key+'.log')).open('w') as log:subprocess.run(['target/release/farming_probe',*a,str(r/key)],stdout=log,stderr=log,check=True)
 for p in (r/key).glob('*.tsv'):
  with p.open('rb') as src,p.with_suffix('.tsv.gz').open('wb') as dst:
   with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
  p.unlink()
 print(key,'completed',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,arms))
summary={}
for a in arms:
 key='_'.join(a);rows=list(csv.DictReader((r/key/'runs.csv').open()));assert len(rows)==180
 assert all(x['accounting_error']=='0' and x['queue_overtakes']=='0' for x in rows)
 summary[key]={}
 for h in [36,48,72,120,168]:
  rr=[x for x in rows if int(x['hour'])==h]
  summary[key][h]={k:{'min':min(float(x[k]) for x in rr),'mean':statistics.mean(float(x[k]) for x in rr),'max':max(float(x[k]) for x in rr)} for k in ['alive','at_c','at_f','consumed','spoiled','starvation_hours','queue_hours','jobs','delivered']}
  cc=list(csv.DictReader((r/key/'crops.csv').open()))
  for k in sorted(set(x['key'] for x in cc)):
   vv=[int(x['value']) for x in cc if int(x['hour'])==h and x['key']==k]
   summary[key][h][k]={'min':min(vv),'mean':statistics.mean(vv),'max':max(vv)}
 summary[key]['distinct_histories']=len({x['event_hash'] for x in rows if x['hour']=='168'})
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(r/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(r/'provenance.json').write_text(json.dumps({'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'arms':arms,'seeds':list(range(30)),'hours':168,'prior_files_unchanged':len(prior)},indent=2)+'\n')
