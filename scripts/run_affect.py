#!/usr/bin/env python3
"""Registered B2 physical histories and live policy comparison; immutable output."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
binary_hash=hashlib.sha256(Path('target/release/affect_probe').read_bytes()).hexdigest()
histories=['no_history','help_safe','help_costly','failed_attempt','refusal','remote_refusal','asymmetric','death_known','death_unknown','fear','stranger']
cases=[('history',h,a) for h in histories for a in ['off','on']]+[('live',h,a) for h in ['colocated','farming','farming_distant','construction'] for a in ['off','on']]
def run(task):
    mode,case,arm=task;name=f'{mode}_{case}_{arm}';dest=root/name
    with (root/(name+'.log')).open('x') as f:subprocess.run(['target/release/affect_probe',mode,case,arm,str(dest),'30'],stdout=f,stderr=f,check=True)
    rows=list(csv.DictReader((dest/'runs.csv').open()));assert len(rows)==30
    assert all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rows)
    for p in dest.glob('*.tsv'):
        with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
        p.unlink()
    print(name,rows[0]['initial_choice'],'food='+rows[0]['food_transferred'],flush=True)
    return {'name':name,'runs':len(rows)}
with ThreadPoolExecutor(max_workers=3) as pool:runs=list(pool.map(run,cases))
with (root/'provenance.json').open('x') as f:json.dump({'source_revision':revision,'binary_sha256':binary_hash,'seeds':list(range(30)),'runs':sum(x['runs'] for x in runs),'arms':runs},f,indent=2);f.write('\n')
