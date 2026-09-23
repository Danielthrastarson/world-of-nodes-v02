#!/usr/bin/env python3
"""Immutable F13A integration controls; existing local authority only."""
import csv
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor


def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    if len(sys.argv) not in (2, 3):
        raise SystemExit('run_kingdom_readiness.py NEW_DIRECTORY [SEEDS=30]')
    root = Path(sys.argv[1]); root.mkdir(parents=True, exist_ok=False)
    seeds = int(sys.argv[2]) if len(sys.argv) == 3 else 30
    assert seeds > 0
    cases = ['alternate', 'scarce', 'long']; arms = ['pending', 'commissioned']
    binary = Path('target/release/kingdom_readiness_probe')
    provenance = {
        'source_revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
        'working_tree_status': subprocess.check_output(['git', 'status', '--porcelain'], text=True).splitlines(),
        'binary_sha256': digest(binary), 'runner_sha256': digest(Path(__file__)),
        'fixture_sha256': digest(Path('src/provisioning/kingdom_research.rs')),
        'registration_sha256': digest(Path('docs/KINGDOM_READINESS_EXPERIMENT.md')),
        'cases': cases, 'arms': arms, 'seeds': list(range(seeds)), 'runs': 6 * seeds,
        'duration_h': 72, 'timeout_s': 300,
        'intervention': 'One existing lawful local steward commission versus pending; all other seeded purposes and physical conditions paired.',
        'scope': 'F13A cross-system readiness only. Neither a ruler order nor autonomous government initiative.',
    }
    (root/'provenance.json').write_text(json.dumps(provenance, indent=2)+'\n')
    def run(item):
        case, arm = item; name = case+'_'+arm; out = root/name
        with (root/(name+'.log')).open('x') as log:
            subprocess.run([str(binary), case, arm, str(out), str(seeds)], stdout=log, stderr=log, check=True, timeout=300)
        with (out/'runs.csv').open() as f: rows = list(csv.DictReader(f))
        assert len(rows) == seeds and [int(r['seed']) for r in rows] == list(range(seeds)), name
        assert all(None not in r and None not in r.values() for r in rows), name
        assert all(r['overtakes'] == '0' for r in rows), name
        assert all(r['audit_ok'] == '1' and r['case'] == case and r['arm'] == arm for r in rows), name
        for p in out.glob('*.tsv'):
            with p.with_suffix('.tsv.gz').open('xb') as f: f.write(gzip.compress(p.read_bytes(), mtime=0))
            p.unlink()
        print(name, len(rows), 'audited runs', flush=True)
        return name
    try:
        with ThreadPoolExecutor(max_workers=2) as pool:
            completed = list(pool.map(run, [(c,a) for c in cases for a in arms]))
        (root/'completion.json').write_text(json.dumps({'status':'PASS', 'runs':6*seeds, 'completed':completed}, indent=2)+'\n')
    except BaseException as error:
        (root/'failure.json').write_text(json.dumps({'status':'INCOMPLETE', 'error':repr(error), 'partial_outputs':'retained'}, indent=2)+'\n')
        raise

if __name__ == '__main__': main()
