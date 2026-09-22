#!/usr/bin/env python3
"""Reproduce F0 in a new directory; never overwrite evidence."""
from pathlib import Path
import subprocess,sys,json,hashlib,csv
r=Path(sys.argv[1]);r.mkdir(parents=True,exist_ok=False)
prior={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and r.resolve() not in p.resolve().parents}
(r/'prior-artifacts.json').write_text(json.dumps(prior,indent=2)+'\n')
for phase in ['static','regressions','offline','live']:
 with (r/f'{phase}.log').open('w') as f:subprocess.run(['target/release/meal_probe',phase,str(r/phase),*(['30'] if phase=='live' else [])],stdout=f,stderr=f,check=True)
 if phase=='regressions':
  expected=list(csv.DictReader(open('research/meals/regressions/historical.csv')))
  actual=list(csv.DictReader((r/phase/'historical.csv').open()))
  assert actual==expected,'historical regression mismatch blocks live phase'
 print(phase,'passed',flush=True)
subprocess.run([sys.executable,'scripts/analyze_meals.py',str(r)],check=True)
(r/'provenance.json').write_text(json.dumps({'revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'command':' '.join(sys.argv)},indent=2)+'\n')
