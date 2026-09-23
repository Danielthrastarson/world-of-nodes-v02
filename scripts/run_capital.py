#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,json,hashlib,gzip,shutil,csv,statistics
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
scenarios=['short_low','short_high','medium_high','long_high','wood_scarce','wood_contested','discovery']
arms=[(s,a) for s in scenarios for a in ['unavailable','available']]
def run(pair):
 key='_'.join(pair)
 with (root/(key+'.log')).open('w') as log:subprocess.run(['target/release/capital_probe',*pair,str(root/key)],stdout=log,stderr=log,check=True)
 for p in (root/key).glob('*.tsv'):
  with p.open('rb') as src,p.with_suffix('.tsv.gz').open('wb') as dst:
   with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
  p.unlink()
 print(key,'complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,arms))
summary={}
for pair in arms:
 key='_'.join(pair);rows=list(csv.DictReader((root/key/'runs.csv').open()));assert len(rows)==180
 assert all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rows)
 last=[x for x in rows if x['hour']=='72'];summary[key]={}
 for k in ['alive','deaths','consumed','spoiled','delivered','jobs','starvation_hours','queue_hours']:
  vv=[float(x[k]) for x in last];summary[key][k]={'min':min(vv),'mean':statistics.mean(vv),'max':max(vv)}
 metrics=[x for x in csv.DictReader((root/key/'metrics.csv').open()) if x['hour']=='72']
 for k in sorted({x['key'] for x in metrics}):
  vv=[int(x['value']) for x in metrics if x['key']==k];summary[key][k]={'min':min(vv),'mean':statistics.mean(vv),'max':max(vv)}
 summary[key]['distinct_histories']=len({x['event_hash'] for x in last})
paired={}
for s in scenarios:
 aa={r['seed']:r for r in csv.DictReader((root/(s+'_available')/'runs.csv').open()) if r['hour']=='72'}
 bb={r['seed']:r for r in csv.DictReader((root/(s+'_unavailable')/'runs.csv').open()) if r['hour']=='72'}
 paired[s]={k:{'mean_difference':statistics.mean(float(aa[i][k])-float(bb[i][k]) for i in aa),'min_difference':min(float(aa[i][k])-float(bb[i][k]) for i in aa),'max_difference':max(float(aa[i][k])-float(bb[i][k]) for i in aa)} for k in ['alive','consumed','delivered','starvation_hours','queue_hours']}
prior=json.loads(Path('research/capital/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(root/'paired.json').write_text(json.dumps(paired,indent=2)+'\n')
(root/'provenance.json').write_text(json.dumps({'revision':revision,'arms':arms,'runs':420,'seeds':list(range(30)),'prior_unchanged':len(prior)},indent=2)+'\n')
