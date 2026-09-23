#!/usr/bin/env python3
"""Registered paired F6 worlds; immutable output paths and prior-artifact verification."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,json,hashlib,csv,gzip,shutil,statistics
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
cases=['local_food'] if len(sys.argv)>2 and sys.argv[2]=='local_food' else ['closed','short','bottleneck','alternate','scarce','discovery']
def run(pair):
 name,arm=pair;key=name+'_'+arm
 with (root/(key+'.log')).open('w') as log:
  subprocess.run(['target/release/infrastructure_probe',name,arm,str(root/key),'30'],stdout=log,stderr=log,check=True)
 for p in (root/key).glob('*.tsv'):
  with p.open('rb') as a,p.with_suffix('.tsv.gz').open('wb') as b:
   with gzip.GzipFile(fileobj=b,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(a,z)
  p.unlink()
 print(key+' complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,[(n,a) for n in cases for a in ['unavailable','available']]))
summary={};paired={}
for name in cases:
 ends={}
 for arm in ['unavailable','available']:
  key=name+'_'+arm;rows=list(csv.DictReader((root/key/'runs.csv').open()))
  assert len(rows)==180 and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rows)
  last=[r for r in rows if r['hour']=='72'];ends[arm]={r['seed']:r for r in last}
  summary[key]={k:{'mean':statistics.mean(float(r[k]) for r in last),'min':min(float(r[k]) for r in last),'max':max(float(r[k]) for r in last)} for k in ['alive','consumed','spoiled','delivered','starvation_h','queue_h','visited_f','completed','at_c','at_x','at_f']}
  mm=[r for r in csv.DictReader((root/key/'metrics.csv').open()) if r['hour']=='72']
  for k in sorted({r['key'] for r in mm}):
   vv=[int(r['value']) for r in mm if r['key']==k];summary[key][k]={'mean':statistics.mean(vv),'min':min(vv),'max':max(vv)}
 paired[name]={k:{'mean':statistics.mean(float(ends['available'][s][k])-float(ends['unavailable'][s][k]) for s in ends['available'])} for k in ['alive','consumed','delivered','starvation_h','queue_h','visited_f']}
prior=json.loads(Path('research/infrastructure/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(root/'paired.json').write_text(json.dumps(paired,indent=2)+'\n')
(root/'provenance.json').write_text(json.dumps({'revision':revision,'runs':len(cases)*60,'seeds':list(range(30)),'prior_unchanged':len(prior)},indent=2)+'\n')
