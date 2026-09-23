#!/usr/bin/env python3
"""Registered F11C factorial raid/voluntary-defense controls; immutable outputs."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False);seeds=int(sys.argv[2]) if len(sys.argv)>2 else 30
cases=['local','under','over','late','bottleneck','hungry','congestion','path_on','path_off','crossing','household','farming'];arms=['undefended','defended','peace','peace_defended']
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip();binary=hashlib.sha256(Path('target/release/combat_probe').read_bytes()).hexdigest()
def run(item):
 case,arm=item;name=case+'_'+arm;d=root/name
 with (root/(name+'.log')).open('x') as f:subprocess.run(['target/release/combat_probe',case,arm,str(d),str(seeds)],stdout=f,stderr=f,check=True,timeout=300)
 rows=list(csv.DictReader((d/'runs.csv').open()));assert len(rows)==seeds and all(r['accounting_error']=='0' and r['overtakes']=='0' and None not in r for r in rows)
 for p in d.glob('*.tsv'):
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(name,'stolen',sum(int(r['stolen']) for r in rows),'hits',sum(int(r['hits']) for r in rows),'deaths',sum(int(r['deaths']) for r in rows),flush=True)
with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,[(c,a) for c in cases for a in arms]))
with (root/'provenance.json').open('x') as f:json.dump({'source_revision':revision,'binary_sha256':binary,'cases':cases,'arms':arms,'seeds':list(range(seeds)),'runs':len(cases)*len(arms)*seeds,'duration_h':24,'initial_conditions':'controlled raider purpose; voluntary defender request; local protect order; disclosed route memories; parameters per arm; crossing constructed through existing actions','interpretation':'gross theft prevention distinct from net food/household/labor outcomes; no welfare scalar'},f,indent=2);f.write('\n')
