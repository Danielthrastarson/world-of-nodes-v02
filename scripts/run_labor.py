#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import sys,json,subprocess,hashlib,gzip,shutil,csv,statistics
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
prior={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and 'labor' not in p.parts}
(root/'prior-artifacts.json').write_text(json.dumps(prior,indent=2)+'\n')
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
def run(q):
 with (root/f'{q}.log').open('w') as log:subprocess.run(['target/release/farming_probe','line','base','F',str(root/str(q)),'30',str(q)],stdout=log,stderr=log,check=True)
 for p in (root/str(q)).glob('*.tsv'):
  with p.open('rb') as src,p.with_suffix('.tsv.gz').open('wb') as dst:
   with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
  p.unlink()
 print(q,'complete',flush=True)
with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,[300,6000]))
paths={'scarce':root/'300','reference':Path('research/farming/matrix/line_base_F'),'abundant':root/'6000','plots_at_c':Path('research/farming/matrix/line_base_C')}
summary={}
with (root/'periods.csv').open('w') as out:
 writer=csv.writer(out);writer.writerow(['arm','seed','start','end','activity','person_hours','living_person_hours','fraction'])
 for arm,p in paths.items():
  rows=list(csv.DictReader((p/'runs.csv').open()));assert len(rows)==180
  assert all(x['accounting_error']=='0' and x['queue_overtakes']=='0' for x in rows)
  activity=list(csv.DictReader((p/'activity.csv').open()));index={(int(x['seed']),int(x['hour']),x['activity']):float(x['person_hours']) for x in activity}
  keys=sorted({x['activity'] for x in activity});summary[arm]={}
  for start,end in zip([0,36,48,72,120],[36,48,72,120,168]):
   values={k:[] for k in keys};fractions={k:[] for k in keys}
   for seed in range(30):
    d={k:index.get((seed,end,k),0)-index.get((seed,start,k),0) for k in keys};total=sum(d.values())
    for k,v in d.items():
     ratio=v/total if total else 0
     writer.writerow([arm,seed,start,end,k,v,total,ratio]);values[k].append(v);fractions[k].append(ratio)
   summary[arm][f'{start}-{end}']={k:{'mean_person_hours':statistics.mean(values[k]),'mean_fraction':statistics.mean(fractions[k])} for k in keys}
  last=[x for x in rows if x['hour']=='168'];summary[arm]['end']={k:statistics.mean(float(x[k]) for x in last) for k in ['alive','consumed','starvation_hours','jobs','delivered']}
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(root/'provenance.json').write_text(json.dumps({'revision':revision,'new_runs':60,'paired_seeds':list(range(30)),'baseline':'research/farming/matrix/line_base_F','prior_files_unchanged':len(prior)},indent=2)+'\n')
