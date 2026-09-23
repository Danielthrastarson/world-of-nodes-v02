#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
sha=hashlib.sha256(Path('target/release/affect_probe').read_bytes()).hexdigest()
def run(x):
    case,arm=x;d=root/(case+'_'+arm)
    with (root/(d.name+'.log')).open('x') as f:subprocess.run(['target/release/affect_probe','cost',case,arm,str(d),'30'],stdout=f,stderr=f,check=True)
    rows=list(csv.DictReader((d/'runs.csv').open()));assert len(rows)==30 and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rows)
    for p in d.glob('*.tsv'):
        with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
        p.unlink()
    print(d.name,'complete',flush=True)
with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,[(c,a) for c in ['scarce','surplus'] for a in ['off','on']]))
with (root/'provenance.json').open('x') as f:json.dump({'revision':revision,'binary_sha256':sha,'runs':120,'seeds':list(range(30)),'arm_semantics':'on = one prescribed gift; off = no gift; no further care in either arm; not a policy change'},f,indent=2);f.write('\n')
