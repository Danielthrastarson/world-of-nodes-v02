#!/usr/bin/env python3
"""Immutable F11A paired worlds; no injected service commands."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
cases=['small','large','food_constrained','caregiver','farmer','distant','bottleneck','unknown','history_help','history_refusal','civilian_traffic']
binary=hashlib.sha256(Path('target/release/mobilization_probe').read_bytes()).hexdigest()
def run(item):
 case,arm=item;name=case+'_'+arm;d=root/name
 with (root/(name+'.log')).open('x') as f:subprocess.run(['target/release/mobilization_probe',case,arm,str(d),'30'],stdout=f,stderr=f,check=True)
 rs=list(csv.DictReader((d/'runs.csv').open()));assert len(rs)==30 and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rs)
 for p in d.glob('*.tsv'):
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(name,'volunteers',sum(int(r['volunteers']) for r in rs),'arrivals',sum(int(r['arrivals']) for r in rs),flush=True)
with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,[(c,a) for c in cases for a in ['off','on']]))
with (root/'provenance.json').open('x') as f:json.dump({'source_revision':revision,'binary_sha256':binary,'seeds':list(range(30)),'runs':len(cases)*60,'duration_h':24,'cases':cases,'initial_conditions':'local requests; history cases import preserved B2 controlled appraisal histories; no injected recruitment/movement commands','security_benefit':'not modeled, not inferred from attendance'},f,indent=2);f.write('\n')
