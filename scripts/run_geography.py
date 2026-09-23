#!/usr/bin/env python3
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,json,hashlib,gzip,shutil,csv,statistics
r=Path(sys.argv[1]);r.mkdir(parents=True,exist_ok=False)
prior={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and r.resolve() not in p.resolve().parents and p.resolve()!=r.with_suffix('.log').resolve()}
(r/'prior-artifacts.json').write_text(json.dumps(prior,indent=2)+'\n')
maps=['line','branch','alternative_routes','dead_end','bottleneck','unequal_sources']
def run(name):
 with (r/f'{name}.log').open('w') as out:subprocess.run(['target/release/geography_probe',name,str(r/name),'30'],check=True,stdout=out,stderr=out)
 p=r/name/'events.tsv'
 with p.open('rb') as src,p.with_suffix('.tsv.gz').open('wb') as dst:
  with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
 p.unlink();print(name,'complete',flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(run,maps))
subprocess.run([sys.executable,'scripts/analyze_geography.py',str(r)],check=True)
