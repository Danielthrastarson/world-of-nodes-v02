#!/usr/bin/env python3
"""Organization absent: exact old outcomes, authoritative hashes and raw traces."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
with (root/'older.log').open('x') as f:subprocess.run([sys.executable,'scripts/verify_mobilization.py',str(root/'older')],stdout=f,stderr=f,check=True)
def rows(p):return list(csv.DictReader(p.open()))
def run(d):
 case,arm=d.name.rsplit('_',1);dest=root/d.name
 with (root/(d.name+'.log')).open('x') as f:subprocess.run(['target/release/mobilization_probe',case,arm,str(dest),'1'],stdout=f,stderr=f,check=True)
 for p in dest.glob('*.csv'):
  assert rows(p)==[r for r in rows(d/p.name) if r['seed']=='0'],(d.name,p.name)
 for p in dest.glob('*.tsv'):
  lines=gzip.decompress((d/(p.name+'.gz')).read_bytes()).decode().splitlines(keepends=True)
  expected=''.join([lines[0]]+[l for l in lines[1:] if l.startswith('0\t')]).encode()
  assert p.read_bytes()==expected,(d.name,p.name)
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(d.name,'exact outcomes/hashes/traces',flush=True);return d.name
old=Path('research/mobilization/matrix-final');dirs=sorted(p for p in old.iterdir() if p.is_dir())
with ThreadPoolExecutor(max_workers=2) as pool:checks=list(pool.map(run,dirs))
prior=json.loads(Path('research/formation/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
with (root/'validation.json').open('x') as f:json.dump({'source_revision':revision,'historical_executions':113+len(checks),'f11a_checks':checks,'prior_files_unchanged':len(prior),'exceptions':'Wall runtime nondeterministic; optional formation:false changes Config debug schema only (older B2 derived physical_signature excluded by existing verifier). Authoritative physical outcomes, state/event hashes, raw traces exact.'},f,indent=2);f.write('\n')
