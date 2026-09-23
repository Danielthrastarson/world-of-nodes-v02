#!/usr/bin/env python3
"""Military absent: exact historical mechanisms and immutable research."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
with (root/'older.log').open('x') as f:subprocess.run([sys.executable,'scripts/verify_lifecycle.py',str(root/'older')],stdout=f,stderr=f,check=True)
def rows(p):
 with p.open() as f:return list(csv.DictReader(f))
def run(d):
 dest=root/d.name
 with (root/(d.name+'.log')).open('x') as f:subprocess.run(['target/release/lifecycle_probe',d.name,str(dest),'1'],stdout=f,stderr=f,check=True)
 for p in dest.glob('*.csv'):
  new=rows(p);old=[r for r in rows(d/p.name) if r['seed']=='0']
  if p.name=='runs.csv':
   for r in new+old:r.pop('runtime_ms',None)
  assert new==old,(d.name,p.name)
 for p in dest.glob('*.tsv'):
  previous=gzip.decompress((d/(p.name+'.gz')).read_bytes())
  lines=previous.decode().splitlines(keepends=True)
  expected=''.join([lines[0]]+[l for l in lines[1:] if l.startswith('0\t')]).encode()
  assert p.read_bytes()==expected,(d.name,p.name)
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(d.name,'exact physical outcomes / state and event hashes / traces',flush=True);return d.name
old=Path('research/lifecycle/matrix-verified');dirs=sorted(p for p in old.iterdir() if p.is_dir())
with ThreadPoolExecutor(max_workers=2) as pool:checks=list(pool.map(run,dirs))
prior=json.loads(Path('research/mobilization/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
with (root/'validation.json').open('x') as f:json.dump({'source_revision':revision,'historical_executions':96+len(checks),'b3_checks':checks,'prior_files_unchanged':len(prior),'exceptions':'Measured wall runtime is nondeterministic. Optional military: None changes Config debug schema and older B2 research physical_signature; all physical fields, authoritative state/event hashes and raw traces match.'},f,indent=2);f.write('\n')
