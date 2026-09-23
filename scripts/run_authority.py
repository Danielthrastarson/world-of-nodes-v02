#!/usr/bin/env python3
"""Registered F10 physical mechanisms and paired autonomous participation."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,shutil,subprocess,sys,statistics
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
mechanisms=['local','distant','bottleneck','death_before','death_after','departure','multiple']
pairs=[(f,'command',1) for f in mechanisms]+[(f,a,30) for f in ['sufficient','scarce','discovery'] for a in ['pending','commissioned']]
def run(pair):
    f,a,n=pair;d=root/(f+'_'+a)
    with (root/(f+'_'+a+'.log')).open('x') as log:subprocess.run(['target/release/authority_probe',f,a,str(d),str(n)],stdout=log,stderr=log,check=True)
    rows=list(csv.DictReader((d/'runs.csv').open()))
    assert len(rows)==n*6
    assert all(r['accounting_error']==r['overtakes']=='0' for r in rows)
    for p in d.glob('*.tsv'):
        with p.open('rb') as src,p.with_suffix('.tsv.gz').open('xb') as dst:
            with gzip.GzipFile(fileobj=dst,filename='',mode='wb',mtime=0) as z:shutil.copyfileobj(src,z)
        p.unlink()
    print(d.name+' complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,pairs))
prior=json.loads(Path('research/authority/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
summary={};paired={}
for f,a,n in pairs:
    d=root/(f+'_'+a);rows=[r for r in csv.DictReader((d/'runs.csv').open()) if r['hour']=='72']
    summary[d.name]={k:{'mean':statistics.mean(float(r[k]) for r in rows),'min':min(float(r[k]) for r in rows),'max':max(float(r[k]) for r in rows)} for k in rows[0] if k not in ['seed','state_hash','event_hash']}
for f in ['sufficient','scarce','discovery']:
    arms={a:{r['seed']:r for r in csv.DictReader((root/(f+'_'+a)/'runs.csv').open()) if r['hour']=='72'} for a in ['pending','commissioned']}
    paired[f]={k:{'mean':statistics.mean(float(arms['commissioned'][s][k])-float(arms['pending'][s][k]) for s in arms['pending']),'changed_seeds':sum(arms['commissioned'][s][k]!=arms['pending'][s][k] for s in arms['pending'])} for k in ['completed','alive','consumed','starvation_h','queue_h','delivered','active_administration_h']}
for name,data in [('summary.json',summary),('paired.json',paired),('provenance.json',{'revision':revision,'mechanism_controls':7,'autonomous_runs':180,'paired_seeds':list(range(30)),'prior_files_unchanged':len(prior),'autonomous_appointment_policy':False,'money':False})]:
    with (root/name).open('x') as o:json.dump(data,o,indent=2);o.write('\n')
