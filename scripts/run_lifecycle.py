#!/usr/bin/env python3
"""B3 bounded paired seeds; immutable outputs and per-process resource measurements."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
binary=hashlib.sha256(Path('target/release/lifecycle_probe').read_bytes()).hexdigest()
cases=['basic','stochastic','infertile','separated','decline','hungry','dependency','dependency_social','parent_death','multiple','multigeneration','multigeneration_social','generational_control','working','working_no_birth','seeded_one','seeded_three']
def run(case):
 d=root/case
 with (root/(case+'.log')).open('x') as f:subprocess.run(['/usr/bin/time','-v','-o',str(root/(case+'.resources.txt')),'target/release/lifecycle_probe',case,str(d),'30'],stdout=f,stderr=f,check=True)
 rs=list(csv.DictReader((d/'runs.csv').open()));assert len(rs)==30 and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rs)
 for p in d.glob('*.tsv'):
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(case,'births',sum(int(r['births']) for r in rs),'generation3',sum(int(r['generation_depth'])>=3 for r in rs),flush=True)
with ThreadPoolExecutor(max_workers=2) as p:list(p.map(run,cases))
with (root/'provenance.json').open('x') as f:json.dump({'source_revision':revision,'binary_sha256':binary,'seeds':list(range(30)),'runs':len(cases)*30,'cases':cases,'horizon_hours':60,'population_abort_above':128,'basic_dependency_alias':'same fixture intentionally, not independent evidence','controls':'100% conception except stochastic25%, infertile0%; explicitly controlled support only in named generational arms'},f,indent=2);f.write('\n')
