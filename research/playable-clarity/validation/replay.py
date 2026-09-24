#!/usr/bin/env python3
"""P4.1 exact headless replays; retain fingerprints or complete failed output."""
import csv
import gzip
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parent
MODE = sys.argv[1]

def sha(data):
    return hashlib.sha256(data).hexdigest()

def raw(path):
    data = path.read_bytes()
    return gzip.decompress(data) if path.suffix == '.gz' else data

def write(name, value):
    with (ROOT/name).open('x') as f:
        json.dump(value, f, indent=2, sort_keys=True)
        f.write('\n')

def files(folder):
    return {str(p.relative_to(folder)).removesuffix('.gz'):p
            for p in folder.rglob('*') if p.is_file()}

def compare(reference, output, mode):
    result = {'mode':mode,'reference':str(reference),'checks':[],'differences':[],
              'exception':'Only results.csv run_wall_ns/replay_wall_ns values in playable scenarios. F13B has no semantic exceptions. Provenance compared separately from experimental artifacts.'}
    def check(name, before, after):
        a,b=raw(before),raw(after)
        entry={'path':name,'reference':str(before),'old_content_sha256':sha(a),
               'new_content_sha256':sha(b),'old_bytes':len(a),'new_bytes':len(b)}
        if mode=='scenarios' and name=='results.csv':
            ar=csv.DictReader(io.StringIO(a.decode()));br=csv.DictReader(io.StringIO(b.decode()))
            entry['fields_equal']=ar.fieldnames==br.fieldnames
            x,y=list(ar),list(br)
            entry['old_rows'],entry['new_rows']=len(x),len(y)
            diffs=[]
            for i,(old,new) in enumerate(zip(x,y)):
                for field in ar.fieldnames:
                    if field not in ('run_wall_ns','replay_wall_ns') and old.get(field)!=new.get(field):
                        diffs.append({'row':i,'field':field,'old':old.get(field),'new':new.get(field)})
            equal=entry['fields_equal'] and len(x)==len(y) and not diffs
            entry['comparison']='All ordered fields exact excepttwo walltimes'
            if not equal: result['differences'].append({'path':name,'changed_values':diffs})
        else:
            equal=a==b
            entry['comparison']='Exact uncompressed bytes'
            if not equal:
                index=next((i for i,(x,y) in enumerate(zip(a,b)) if x!=y),min(len(a),len(b)))
                result['differences'].append({'path':name,'first_byte':index,
                    'old_context':a[max(0,index-100):index+200].decode(errors='replace'),
                    'new_context':b[max(0,index-100):index+200].decode(errors='replace')})
        entry['equal']=equal
        result['checks'].append(entry)
    old_dirs={p.name for p in reference.iterdir() if p.is_dir()}
    new_dirs={p.name for p in output.iterdir() if p.is_dir()}
    if old_dirs!=new_dirs:
        result['differences'].append({'path':'directories','missing':sorted(old_dirs-new_dirs),'extra':sorted(new_dirs-old_dirs)})
    for folder in sorted(old_dirs&new_dirs):
        old,new=files(reference/folder),files(output/folder)
        if old.keys()!=new.keys():
            result['differences'].append({'path':folder,'missing':sorted(old.keys()-new.keys()),'extra':sorted(new.keys()-old.keys())})
        for name in sorted(old.keys()&new.keys()): check(folder+'/'+name,old[name],new[name])
    if mode=='scenarios':
        check('results.csv',reference/'results.csv',output/'results.csv')
        result['runs']=48
        result['additional_dated_input_replays']=48
    else:
        # The per-run native results are exact above. Retain aggregate outcomes
        # as a small convenience, separate from the 3000 native artifactchecks.
        check('runs.csv',reference/'runs.csv',output/'runs.csv')
        result['runs']=600
    result['status']='FAIL' if result['differences'] else 'PASS'
    return result

if MODE not in ('scenarios','commands'):
    raise SystemExit('replay.py scenarios|commands')
binary=Path('target/release/playable_probe' if MODE=='scenarios' else 'target/release/command_chain_probe')
reference=Path('research/playable/probes/scenarios-3seeds' if MODE=='scenarios' else 'research/command-chain/matrix')
write(MODE+'-provenance.json',{
    'baseline_revision':'0f8c18b','head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
    'reference':str(reference),'binary':str(binary),'binary_sha256':sha(binary.read_bytes()),
    'validator_sha256':sha(Path(__file__).read_bytes()),'root_source_manifest':'root-source-baseline.json',
    'scope':'Headless regression only. No new autonomous-behavior claim or UI comprehension claim.'})
with tempfile.TemporaryDirectory(prefix='won-clarity-'+MODE+'-') as scratch:
    output=Path(scratch)/'replay'
    command=([str(binary),'scenarios',str(output),'3','24'] if MODE=='scenarios' else
             [sys.executable,'scripts/run_command_chain.py',str(output),'30','--workers','2','--compress-traces'])
    try:
        with (ROOT/(MODE+'-run.log')).open('x') as log:
            subprocess.run(command,stdout=log,stderr=log,check=True,timeout=1800)
        result=compare(reference,output,MODE)
        write(MODE+'-comparison.json',result)
        # Keep native new run parameters/provenance even for an exact replay.
        names=('parameters.txt',) if MODE=='scenarios' else ('provenance.json','completion.json')
        for name in names:
            (ROOT/(MODE+'-'+name)).write_bytes((output/name).read_bytes())
        if result['status']!='PASS': raise RuntimeError('strict replay mismatch')
        write(MODE+'-retention.json',{'status':'PASS','command':command,
            'discarded_exact_replay_files':{str(p.relative_to(output)):sha(p.read_bytes()) for p in sorted(output.rglob('*')) if p.is_file()},
            'raw_reference':str(reference),'meaning':'Duplicate successful scratchoutput discarded only after allordered semanticcomparisons; originalrawdata remains authoritative.'})
        print(MODE,result['status'],result['runs'],'runs',len(result['checks']),'artifactchecks',flush=True)
    except BaseException as e:
        with tarfile.open(ROOT/(MODE+'-failed-output.tar.gz'),'x:gz') as archive:
            archive.add(scratch,arcname='replay')
        write(MODE+'-failure.json',{'status':'FAIL','error':repr(e),'command':command,'raw':'failed-output.tar.gz'})
        raise
