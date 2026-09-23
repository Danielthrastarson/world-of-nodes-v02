#!/usr/bin/env python3
"""Re-run immutable 983b5b5 F11C for an exact pre-playable baseline."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import gzip, hashlib, json, subprocess, sys

binary, output = map(Path, sys.argv[1:])
output.mkdir(parents=True, exist_ok=False)
registered = json.loads(Path('research/combat/matrix/provenance.json').read_text())
tasks = [(case, arm) for case in registered['cases'] for arm in registered['arms']]

def run(item):
    case, arm = item
    name = case + '_' + arm
    with (output/(name+'.log')).open('x') as log:
        subprocess.run([str(binary),case,arm,str(output/name),'30'], stdout=log,stderr=log,check=True,timeout=300)
    for p in (output/name).glob('*.tsv'):
        p.with_suffix('.tsv.gz').write_bytes(gzip.compress(p.read_bytes(),mtime=0))
        p.unlink()
    print(name,flush=True)

with ThreadPoolExecutor(max_workers=2) as pool:
    list(pool.map(run,tasks))
registered.update(source_revision='983b5b5b522456358aa9c7f3d7adb408df77a89f',
                  binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                  source_materialization='git archive 983b5b5 Cargo.toml Cargo.lock src; unmodified baseline build',
                  runs=1440)
(output/'provenance.json').write_text(json.dumps(registered,indent=2)+'\n')
