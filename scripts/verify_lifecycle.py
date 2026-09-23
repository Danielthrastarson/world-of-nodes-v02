#!/usr/bin/env python3
"""B3 absent: preserve B2 behavior and older representative regression panel."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
with (root/'older.log').open('x') as f:subprocess.run([sys.executable,'scripts/verify_affect.py',str(root/'older')],stdout=f,stderr=f,check=True)
def rows(p):
 with p.open() as f:return list(csv.DictReader(f))
def run(d):
 if d.parent.name=='cost':mode='cost';case,arm=d.name.rsplit('_',1)
 else:mode,tail=d.name.split('_',1);case,arm=tail.rsplit('_',1)
 name=mode+'_'+case+'_'+arm;dest=root/name
 with (root/(name+'.log')).open('x') as f:subprocess.run(['target/release/affect_probe',mode,case,arm,str(dest),'1'],stdout=f,stderr=f,check=True)
 for p in dest.glob('*.csv'):
  new=rows(p);old=[r for r in rows(d/p.name) if r['seed']=='0']
  # This debug hash includes Config's textual schema; lifecycle: None adds a field.
  if p.name=='runs.csv':
   for r in new+old:r.pop('physical_signature',None)
  assert new==old,(name,p.name)
 for p in dest.glob('*.tsv'):
  previous=gzip.decompress((d/(p.name+'.gz')).read_bytes())
  lines=previous.decode().splitlines(keepends=True)
  expected=''.join([lines[0]]+[l for l in lines[1:] if l.startswith('0\t')]).encode()
  assert p.read_bytes()==expected,(name,p.name)
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(name,'exact physical outcomes / state and event hashes / traces',flush=True);return name
old=Path('research/affect');dirs=[p for r in ['matrix','cost'] for p in (old/r).iterdir() if p.is_dir()]
with ThreadPoolExecutor(max_workers=2) as pool:checks=list(pool.map(run,sorted(dirs)))
prior=json.loads(Path('research/lifecycle/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
with (root/'validation.json').open('x') as f:json.dump({'source_revision':revision,'historical_executions':62+len(checks),'b2_checks':checks,'prior_files_unchanged':len(prior),'format_exception':'B2 research physical_signature hashes full debug Config; new lifecycle: None changes this formatting hash. All physical CSV fields, authoritative state/event hashes and raw event/appraisal traces match.'},f,indent=2);f.write('\n')
