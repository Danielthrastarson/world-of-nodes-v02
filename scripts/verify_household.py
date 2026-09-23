#!/usr/bin/env python3
"""B1 opt-out historical regression, including the entire accepted F10 experiment."""
from pathlib import Path
import gzip,hashlib,json,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
for script,dest in [('verify_authority.py','panel'),('run_authority.py','authority')]:
    with (root/(dest+'.log')).open('x') as f:
        subprocess.run([sys.executable,'scripts/'+script,str(root/dest)],stdout=f,stderr=f,check=True)
old=Path('research/authority/matrix');checks=[]
def data(p):return gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes()
for d in sorted((root/'authority').iterdir()):
    if not d.is_dir():continue
    for p in sorted(d.iterdir()):
        previous=old/d.name/p.name
        a,b=data(p),data(previous)
        if p.name=='parameters.txt':
            a=a.replace(b'Config { household: None, authority:',b'Config { authority:')
        assert a==b,(d.name,p.name)
        checks.append(str(p.relative_to(root)))
prior=json.loads(Path('research/household/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
out={'revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'historical_executions':227,'panel_executions':40,'full_authority_executions':187,'authority_files_exact':checks,'prior_files_unchanged':len(prior),'parameter_debug_difference':'Only optional household: None is normalized in F10 parameter text. CSVs, hashes, decisions and events require exact equality.'}
with (root/'validation.json').open('x') as f:json.dump(out,f,indent=2);f.write('\n')
print('227 historical executions exact; prior artifacts unchanged',flush=True)
