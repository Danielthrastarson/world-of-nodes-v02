#!/usr/bin/env python3
"""B2 absent: exact F0-F10 panel and all 22 B1 fixture/arm controls at seed0."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
with (root/'panel.log').open('x') as f:subprocess.run([sys.executable,'scripts/verify_authority.py',str(root/'panel')],stdout=f,stderr=f,check=True)
old=Path('research/household/matrix-verified')
def read(p):return list(csv.DictReader(p.open()))
def run(d):
    case,arm=d.name.rsplit('_',1);dest=root/d.name
    with (root/(d.name+'.log')).open('x') as f:subprocess.run(['target/release/household_probe',case,arm,str(dest),'1'],stdout=f,stderr=f,check=True)
    checked=[]
    for p in dest.glob('*.csv'):
        assert read(p)==[r for r in read(d/p.name) if r['seed']=='0'],(d.name,p.name)
        checked.append(p.name)
    p=dest/'events.tsv';assert p.read_bytes()==gzip.decompress((d/'events.tsv.gz').read_bytes()),(d.name,'events')
    assert (dest/'social-final.txt').read_bytes()==(d/'social-final.txt').read_bytes()
    with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
    p.unlink();print(d.name,'exact',flush=True);return dict(name=d.name,csv=checked,events_exact=True)
with ThreadPoolExecutor(max_workers=3) as pool:checks=list(pool.map(run,sorted(p for p in old.iterdir() if p.is_dir())))
prior=json.loads(Path('research/affect/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
with (root/'validation.json').open('x') as f:json.dump({'revision':revision,'historical_executions':40+len(checks),'b1':checks,'prior_files_unchanged':len(prior),'parameter_difference':'New optional social: None only. All physical CSVs and events exact.'},f,indent=2);f.write('\n')
