#!/usr/bin/env python3
"""F11B paired local commands. Immutable output; all persons move autonomously."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
seeds=int(sys.argv[2]) if len(sys.argv)>2 else 30
cases=['short','sequence','unknown_route','congestion','long','food_stress','fatigue_stress','bottleneck','path_on','path_off','partial','late','care','shared_route','crossing','cart']
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
binary=hashlib.sha256(Path('target/release/formation_probe').read_bytes()).hexdigest()
def run(item):
 case,arm=item;name=case+'_'+arm;d=root/name
 with (root/(name+'.log')).open('x') as f:subprocess.run(['target/release/formation_probe',case,arm,str(d),str(seeds)],stdout=f,stderr=f,check=True)
 rows=list(csv.DictReader((d/'runs.csv').open()));assert len(rows)==seeds and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rows)
 for p in d.glob('*.tsv'):
  with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
  p.unlink()
 print(name,'received',sum(int(r['receipts']) for r in rows),'departed',sum(int(r['departed']) for r in rows),'arrived',sum(int(r['arrived']) for r in rows),flush=True)
with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,[(c,a) for c in cases for a in ['hold','move']]))
for case in cases:
 arms=[list(csv.DictReader((root/(case+'_'+a)/'runs.csv').open())) for a in ['hold','move']]
 assert [(r['seed'],r['pre_command_hash']) for r in arms[0]]==[(r['seed'],r['pre_command_hash']) for r in arms[1]],case
with (root/'provenance.json').open('x') as f:json.dump({'source_revision':revision,'binary_sha256':binary,'seeds':list(range(seeds)),'runs':len(cases)*2*seeds,'duration_h_after_command':24,'cases':cases,'intervention':'one Move order delivered only to present assembled recipients versus none; same pre-command hashes','initial_conditions':'see parameters per case; stressed command states explicit; construction prepared through existing physical actions; no individual military moves','protective_benefit':'unmodeled; F11A full gate remains open'},f,indent=2);f.write('\n')
