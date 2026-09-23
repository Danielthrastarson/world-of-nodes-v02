#!/usr/bin/env python3
"""Registered F9A paired fixture runner; immutable outputs and historical checks."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv, gzip, hashlib, json, shutil, subprocess, sys
root=Path(sys.argv[1]); root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
fixtures=['complementary','symmetric','low_cost','high_cost','colocated','separated']
def run(pair):
    fixture,mode=pair; name=fixture+'_'+mode
    with (root/(name+'.log')).open('w') as f:
        subprocess.run(['target/release/settlement_probe',fixture,str(root/name),'30','0',mode],stdout=f,stderr=f,check=True)
    rows=list(csv.DictReader((root/name/'runs.csv').open()))
    assert len(rows)==210 and all(r['accounting_error']==r['overtakes']=='0' for r in rows)
    if mode=='off' and fixture not in ('colocated','separated'):
        old=list(csv.DictReader((Path('research/settlements/matrix-corrected')/fixture/'runs.csv').open()))
        assert rows==old,(fixture,'disabled history changed')
    for p in (root/name).glob('*.tsv'):
        with p.open('rb') as a,p.with_suffix('.tsv.gz').open('wb') as b:
            with gzip.GzipFile(fileobj=b,filename='',mode='wb',mtime=0) as z: shutil.copyfileobj(a,z)
        p.unlink()
    print(name+' complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool: list(pool.map(run,[(f,m) for f in fixtures for m in ['off','on']]))
prior=json.loads(Path('research/barter/prior-artifacts.json').read_text())
for p,h in prior.items(): assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(root/'provenance.json').write_text(json.dumps({'revision':revision,'runs':360,'seeds':list(range(30)),'fixtures':fixtures,'preserved_files':len(prior),'disabled_f8_runs_identical':120,'money':False},indent=2)+'\n')
