#!/usr/bin/env python3
"""Three gated research phases; never overwrite old evidence or select a new default."""
from pathlib import Path
import csv,gzip,hashlib,json,shutil,subprocess,sys
root=Path(sys.argv[1]).resolve()
if root.exists(): raise SystemExit('Use a new output directory')
root.mkdir(parents=True)
seeds=sys.argv[2] if len(sys.argv)>2 else '30'
prior={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in Path('research').rglob('*') if p.is_file() and root not in p.resolve().parents}
(root/'prior-artifacts.json').write_text(json.dumps(prior,indent=2)+'\n')
def run(phase,*args):
 with (root/('-'.join((phase,*args))+'.log')).open('w') as out:
  subprocess.run(['target/release/coherence_probe',phase,str(root/('phase1' if phase=='static' else 'phase2' if phase=='replay' else 'phase3')),*args],check=True,stdout=out,stderr=out)
def compress(directory):
 for p in directory.rglob('*'):
  if p.is_file() and (p.suffix=='.tsv' or 'comparisons.csv' in p.name or '-shadow-' in p.name):
   with p.open('rb') as src,p.with_suffix(p.suffix+'.gz').open('wb') as dst:
    with gzip.GzipFile(fileobj=dst,mode='wb',filename='',mtime=0) as z:shutil.copyfileobj(src,z)
   p.unlink()
run('static')
print('Static conformance passed',flush=True)
sources={
 'reference':('research/provisioning/runs.csv','reference'),
 'current':('research/acceptance/current-runs.csv','current'),
 'known_source':('research/acceptance/known_source-runs.csv','known_source'),
 'reactive':('research/congestion/reactive-runs.csv','reactive'),
 'committed':('research/persistence/committed-runs.csv','committed'),
 'forecast_corrected':('research/congestion/forecast_corrected-runs.csv','forecast_corrected'),
 'contested_requests':('research/provisioning/runs.csv','contested_requests'),
 'stale_depletion':('research/provisioning/runs.csv','stale_depletion')}
proof={}
for arm,(f,name) in sources.items():
 run('replay',arm)
 old=next(r for r in csv.DictReader(open(f)) if r['scenario']==name and r['seed']=='0')
 new=next(csv.DictReader((root/'phase2'/f'{arm}-runs.csv').open()))
 for key in ['event_hash','state_hash','food_consumed','food_spoiled','food_delivered','starvation_person_hours']:
  assert old[key]==new[key],(arm,key)
 counts={r['key']:int(r['count']) for r in csv.DictReader((root/'phase2'/f'{arm}-counts.csv').open())}
 assert counts.get('kernel_mismatch',0)==0,(arm,'kernel mismatch blocks live phase')
 proof[arm]={'event_hash':new['event_hash'],'state_hash':new['state_hash'],'historical_match':True,'kernel_exact':counts.get('kernel_exact',0)}
 print(arm,'replay preserved',flush=True)
(root/'phase2'/'preservation.json').write_text(json.dumps(proof,indent=2)+'\n')
compress(root/'phase2')
run('live',seeds)
compress(root/'phase3')
for p,digest in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==digest,p
subprocess.run([sys.executable,'scripts/analyze_coherence.py',str(root)],check=True)
