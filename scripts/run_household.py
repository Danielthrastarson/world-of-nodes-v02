#!/usr/bin/env python3
"""B1 immutable paired experiments. Policy never sees these case labels or analyses."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
cases=['colocated','self_conflict','multiple','near_known','far_known','unknown','caregiver_death','nonkin','farming','farming_distant','construction']
def run(task):
    case,arm=task;dest=root/f'{case}_{arm}'
    with (root/f'{case}_{arm}.log').open('x') as log:
        subprocess.run(['target/release/household_probe',case,arm,str(dest),'30'],stdout=log,stderr=log,check=True)
    for p in dest.glob('*.tsv'):
        with p.with_suffix('.tsv.gz').open('xb') as f:f.write(gzip.compress(p.read_bytes(),mtime=0))
        p.unlink()
    rows=list(csv.DictReader((dest/'runs.csv').open()))
    assert len(rows)==210 and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in rows)
    final=[r for r in rows if r['hour']=='48']
    result={k:{'mean':sum(float(r[k]) for r in final)/len(final),'min':min(float(r[k]) for r in final),'max':max(float(r[k]) for r in final)} for k in final[0] if k not in ['seed','hour','state_hash','event_hash']}
    print(case,arm,result['food_transferred']['mean'],flush=True)
    return f'{case}_{arm}',result
with ThreadPoolExecutor(max_workers=3) as pool: results=dict(pool.map(run,[(c,a) for c in cases for a in ['off','on']]))
with (root/'summary.json').open('x') as f:json.dump({'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'paired_seeds':list(range(30)),'runs':660,'results':results},f,indent=2);f.write('\n')
