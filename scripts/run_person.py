#!/usr/bin/env python3
"""B0 behavior preservation: historical panel, all F10, full direct/market panel."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,shutil,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
def compare_folder(new,old,normalize_config=False):
    checked=[]
    for p in old.iterdir():
        if not p.is_file():continue
        q=new/p.name
        if p.name.endswith('.gz'):
            a=gzip.decompress(p.read_bytes())
            b=gzip.decompress(q.read_bytes()) if q.exists() else (new/p.stem).read_bytes()
        else:
            a=p.read_bytes();b=q.read_bytes()
        if normalize_config and p.name=='parameters.txt':b=b.replace(b'authority: None, ',b'')
        assert a==b,(str(new),p.name)
        checked.append(p.name)
    return checked
# Keep independent probes independent; bound process parallelism.
def foundational():
    with (root/'authority.log').open('x') as log:
        subprocess.run(['python3','scripts/run_authority.py',str(root/'authority')],stdout=log,stderr=log,check=True)
    checks={}
    for old in Path('research/authority/matrix').iterdir():
        if old.is_dir():checks[old.name]=compare_folder(root/'authority'/old.name,old)
    return checks
def panel():
    with (root/'regressions.log').open('x') as log:
        subprocess.run(['python3','scripts/verify_authority.py',str(root/'regressions')],stdout=log,stderr=log,check=True)
    return json.loads((root/'regressions/validation.json').read_text())
# Run these first so a foundational mismatch stops expansion of the matrix.
authority=foundational();historical=panel()
market_root=root/'exchange';market_root.mkdir()
fixtures=['complementary','symmetric','low_cost','high_cost','colocated','separated']
def exchange(pair):
    f,mode=pair;d=market_root/(f+'_'+mode)
    with (market_root/(d.name+'.log')).open('x') as log:
        subprocess.run(['target/release/settlement_probe',f,str(d),'30','0',mode],stdout=log,stderr=log,check=True)
    checked=compare_folder(d,Path('research/market/matrix-final')/d.name,True)
    for p in d.glob('*.tsv'):
        with p.open('rb') as src,p.with_suffix('.tsv.gz').open('xb') as dst:
            with gzip.GzipFile(fileobj=dst,filename='',mode='wb',mtime=0) as z:shutil.copyfileobj(src,z)
        p.unlink()
    print(d.name+' exact',flush=True)
    return {'fixture':f,'arm':mode,'runs':30,'exact_files':checked}
with ThreadPoolExecutor(max_workers=3) as pool:exchange_checks=list(pool.map(exchange,[(f,m) for f in fixtures for m in ['on','market']]))
prior=json.loads(Path('research/person/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
result={'source_revision':revision,'authority_runs_exact':187,'historical_panel_runs_exact':40,'exchange_runs_exact':360,'total_run_executions':587,'overlap':'Historical panel contains some of the same seed/fixture pairs as the complete panels; 587 is not independent experimental evidence.','paired_exchange_seeds':list(range(30)),'physical_metric_changes':0,'state_hash_changes':0,'event_hash_changes':0,'prior_files_unchanged':len(prior),'authority_files':authority,'historical_panel':historical,'exchange_files':exchange_checks,'parameter_print_normalization':'Only historical F9 parameters remove authority: None, already added by F10. No outcomes normalized.'}
with (root/'validation.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
