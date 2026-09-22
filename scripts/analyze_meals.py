#!/usr/bin/env python3
"""Summarize an immutable meal study; no policy calibration."""
import csv,json,re,gzip,hashlib,sys,statistics,shutil
from pathlib import Path
root=Path(sys.argv[1])
def reader(p):
 if not p.exists():return gzip.open(str(p)+'.gz','rt')
 return p.open()
def rows(p):
 with reader(p) as f:return list(csv.DictReader(f))
summary={}
for arm in ['no_meal','conditional_meals']:
 rs=rows(root/'live'/f'{arm}-runs.csv');assert len(rs)==30
 counts={int(r['seed']):{} for r in rs}
 for r in rows(root/'live'/f'{arm}-counts.csv'):counts[int(r['seed'])][r['key']]=int(r['count'])
 assert all(c==counts[0] for c in counts.values()),'report seed variability rather than using first run'
 stable=[k for k in rs[0] if k not in ['seed','state_hash']]
 assert all(all(r[k]==rs[0][k] for k in stable) for r in rs)
 assert all(r['queue_overtakes']=='0' and r['accounting_error']=='0' for r in rs)
 summary[arm]={'seeds':30,'identical_measured_outcomes':True,'per_run':rs[0],'counts_per_run':counts[0]}
 passrows=[];meals=[]
 with reader(root/'live'/f'{arm}-details.tsv') as f:
  for line in f:
   if '\tPASS\t' not in line:continue
   def val(k):return int(re.search(r'\b'+k+r'=(\d+)',line)[1])
   passrows.append({k:val(k) for k in ['final_nutrition','deliverable','meal_quanta']})
   for q,b,a in re.findall(r'q=(\d+) nutrition_before=(\d+) after=(\d+)',line):meals.append((int(q),int(b),int(a)))
 if passrows:
  summary[arm]['pass_distributions']={k:{'min':min(p[k] for p in passrows),'mean':statistics.mean(p[k] for p in passrows),'max':max(p[k] for p in passrows)} for k in passrows[0]}
  summary[arm]['meal_distributions']={k:{'min':min(p[i] for p in meals),'mean':statistics.mean(p[i] for p in meals),'max':max(p[i] for p in meals)} for i,k in enumerate(['quanta','nutrition_before_ms','nutrition_after_ms'])}
summary['paired']=rows(root/'live/paired-decisions.csv')
summary['offline']={r['key']:int(r['count']) for r in rows(root/'offline/old_meals-counts.csv')}
summary['old_artifacts_unchanged']=len(json.loads((root/'prior-artifacts.json').read_text()))
for p,h in json.loads((root/'prior-artifacts.json').read_text()).items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
for folder in ['live','offline','supplement']:
 for p in (root/folder).glob('*.tsv'):
  with p.open('rb') as src,p.with_suffix(p.suffix+'.gz').open('wb') as dst:
   with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as out:shutil.copyfileobj(src,out)
  p.unlink()
print(json.dumps({a:summary[a].get('pass_distributions') for a in ['no_meal','conditional_meals']},indent=2))
