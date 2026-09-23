#!/usr/bin/env python3
"""F10 historical opt-out compatibility and immutable-artifact audit."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
def read(p):return list(csv.DictReader(p.open()))
tasks=[]
for f in ['complementary','symmetric','high_cost','low_cost','colocated','separated']:
    for mode in ['on','market']:
        tasks.append((f+'_'+mode+'-0',['target/release/settlement_probe',f,'OUT','1','0',mode],Path('research/market/matrix-final')/(f+'_'+mode),0))
for mode in ['on','market']:
    tasks.append(('low_cost_'+mode+'-18',['target/release/settlement_probe','low_cost','OUT','1','18',mode],Path('research/market/matrix-final')/('low_cost_'+mode),18))
for f in ['closed','bottleneck','scarce','discovery']:
    tasks.append(('crossing_'+f,['target/release/infrastructure_probe',f,'available','OUT','1'],Path('research/infrastructure/matrix')/(f+'_available'),0))
for f in ['line','dead_end','carts']:
    tasks.append(('path_'+f,['target/release/paths_probe',f,'enabled','OUT','1'],Path('research/paths/matrix')/(f+'_enabled'),0))
def run(task):
    name,cmd,old,seed=task;d=root/name;cmd=[str(d) if x=='OUT' else x for x in cmd]
    with (root/(name+'.log')).open('x') as log:subprocess.run(cmd,stdout=log,stderr=log,check=True)
    checked=[]
    for p in d.glob('*.csv'):
        before=[r for r in read(old/p.name) if r.get('seed')==str(seed)]
        assert read(p)==before,(name,p.name)
        checked.append(p.name)
    # Every seed-zero historical event remains byte-for-byte identical after filtering.
    p=d/'events.tsv'
    if seed==0 and p.exists():
        compressed=old/'events.tsv.gz'
        before=gzip.decompress(compressed.read_bytes()) if compressed.exists() else (old/'events.tsv').read_bytes()
        assert p.read_bytes()==before,(name,'events')
    for p in d.glob('*.tsv'):
        with p.with_suffix('.tsv.gz').open('xb') as out:out.write(gzip.compress(p.read_bytes(),mtime=0))
        p.unlink()
    print(name+' exact',flush=True)
    return dict(name=name,seed=seed,csv_files=checked,seed0_events_exact=seed==0)
with ThreadPoolExecutor(max_workers=3) as pool:checks=list(pool.map(run,tasks))
# Historical finite-food baseline: all 19 scenarios, first paired seed.
p1=root/'p1'
with (root/'p1.log').open('x') as log:subprocess.run(['target/release/provisioning_probe','--matrix',str(p1),'1'],stdout=log,stderr=log,check=True)
assert read(p1/'runs.csv')==[r for r in read(Path('research/provisioning/runs.csv')) if r['seed']=='0']
prior=json.loads(Path('research/authority/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
with (root/'validation.json').open('x') as out:json.dump({'revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'prior_files_unchanged':len(prior),'regressions':checks,'p1_scenarios_exact':19,'parameter_debug_difference':'New optional authority: None field only; physical/event/state checks exact.'},out,indent=2);out.write('\n')
