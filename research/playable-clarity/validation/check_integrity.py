#!/usr/bin/env python3
"""Recheck P4.1's immutable root-code and previous-research boundary."""
import hashlib,json,subprocess,sys
from pathlib import Path

root=Path(__file__).resolve().parent
output=root/(sys.argv[1] if len(sys.argv)>1 else 'integrity.json')
if output.exists(): raise FileExistsError(output)
def digest(p):
    h=hashlib.sha256()
    with p.open('rb') as stream:
        while block:=stream.read(1024*1024):h.update(block)
    return h.hexdigest()
def compare(name):
    baseline=json.loads((root/name).read_text())
    failures=[]
    for name,expected in baseline.items():
        p=Path(name)
        actual=digest(p) if p.is_file() else None
        if actual!=expected:failures.append({'path':name,'old':expected,'current':actual})
    return {'files':len(baseline),'differences':failures,'status':'FAIL' if failures else 'PASS'}
source=compare('root-source-baseline.json')
prior=compare('prior-research-sha256.json')
expected=set(json.loads((root/'root-source-baseline.json').read_text()))
actual={str(p) for p in Path('src').rglob('*') if p.is_file()}|{'Cargo.toml','Cargo.lock'}
source['extra_root_source_files']=sorted(actual-expected)
source['missing_root_source_files']=sorted(expected-actual)
if actual!=expected:source['status']='FAIL'
diff=subprocess.check_output(['git','diff','0f8c18b','--','src','Cargo.toml','Cargo.lock'])
result={'baseline_revision':'0f8c18bc9b16f47dbf75276f577e20f850518f36',
        'root_source':source,'prior_research':prior,'git_diff_root_vs_baseline_bytes':len(diff),
        'git_head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        'status':'PASS' if source['status']=='PASS' and prior['status']=='PASS' and not diff else 'FAIL'}
output.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(result['status'],source['files'],'rootsourcefiles;',prior['files'],'priorresearchfiles')
raise SystemExit(result['status']!='PASS')
