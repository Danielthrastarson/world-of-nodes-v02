#!/usr/bin/env python3
"""F9B preregistered paired direct/escrow comparison. Never overwrite evidence."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import csv,gzip,hashlib,json,shutil,subprocess,sys
root=Path(sys.argv[1]);root.mkdir(parents=True,exist_ok=False)
binary=sys.argv[2] if len(sys.argv)>2 else 'target/release/settlement_probe'
revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
fixtures=['complementary','symmetric','low_cost','high_cost','colocated','separated']
def run(pair):
    fixture,mode=pair;name=fixture+'_'+mode;out=root/name
    with (root/(name+'.log')).open('w') as log:
        subprocess.run([binary,fixture,str(out),'30','0',mode],stdout=log,stderr=log,check=True)
    rows=list(csv.DictReader((out/'runs.csv').open()))
    assert len(rows)==210 and all(r['accounting_error']==r['overtakes']=='0' for r in rows)
    if mode=='on':
        old=Path('research/barter/matrix')/name
        for p in old.iterdir():
            if p.name=='barter_counts.csv':
                before={(r['seed'],r['hour'],r['key']):r['value'] for r in csv.DictReader(p.open())}
                after={(r['seed'],r['hour'],r['key']):r['value'] for r in csv.DictReader((out/p.name).open())}
                assert all(after[k]==v for k,v in before.items()),(name,p.name)
            elif p.suffix=='.gz':
                assert gzip.decompress(p.read_bytes())==(out/p.stem).read_bytes(),(name,p.name)
            else:
                assert p.read_bytes()==(out/p.name).read_bytes(),(name,p.name)
    for p in out.glob('*.tsv'):
        with p.open('rb') as a,p.with_suffix('.tsv.gz').open('wb') as b:
            with gzip.GzipFile(fileobj=b,filename='',mode='wb',mtime=0) as z:shutil.copyfileobj(a,z)
        p.unlink()
    print(name+' complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,[(f,m) for f in fixtures for m in ['on','market']]))
prior=json.loads(Path('research/market/prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
(root/'provenance.json').write_text(json.dumps({'revision':revision,'runs':360,'seeds':list(range(30)),'fixtures':fixtures,'preserved_files':len(prior),'direct_f9a_runs_identical':180,'money':False,'offer_policy':'unchanged','market_nodes':[0,2],'offer_lifetime_h':12},indent=2)+'\n')
