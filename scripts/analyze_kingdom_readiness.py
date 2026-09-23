#!/usr/bin/env python3
"""Paired existing-commission effects; no scalar welfare interpretation."""
import csv
import hashlib
import json
import math
from pathlib import Path
import statistics
import sys


def describe(values):
    xs = [v for v in values if v is not None]
    return {'n': len(xs), 'missing': len(values)-len(xs), 'mean': statistics.mean(xs) if xs else None,
            'min': min(xs) if xs else None, 'max': max(xs) if xs else None, 'sum':sum(xs) if xs else None}


def numeric(s):
    if s == '': return None
    if s in ('true','false'): return float(s == 'true')
    x = float(s)
    assert math.isfinite(x)
    return x


def fmt(x): return 'undefined' if x is None else f'{x:.3f}'.rstrip('0').rstrip('.')


def main():
    if len(sys.argv) != 3: raise SystemExit('analyze_kingdom_readiness.py PANEL NEW_ANALYSIS')
    panel, out = map(Path, sys.argv[1:]); out.mkdir(parents=True, exist_ok=False)
    provenance = json.loads((panel/'provenance.json').read_text())
    assert json.loads((panel/'completion.json').read_text())['status'] == 'PASS'
    result = {}; raw = []; hashes = {}
    for case in provenance['cases']:
        rows = {}
        for arm in provenance['arms']:
            p=panel/(case+'_'+arm)/'runs.csv'; hashes[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
            with p.open() as f: rows[arm] = list(csv.DictReader(f))
            assert [int(r['seed']) for r in rows[arm]] == provenance['seeds']
        metrics = set(rows['pending'][0])-{'seed','case','arm','state_hash','event_hash','runtime_ms','elapsed_ms'}
        result[case]={}
        for metric in sorted(metrics):
            before = [numeric(r[metric]) for r in rows['pending']]
            after = [numeric(r[metric]) for r in rows['commissioned']]
            delta = [b-a if a is not None and b is not None else None for a,b in zip(before,after)]
            result[case][metric]={'pending':describe(before),'commissioned':describe(after),'difference':describe(delta),
                                  'changed_pairs':sum(a!=b for a,b in zip(before,after))}
            raw.extend({'case':case,'seed':s,'metric':metric,'pending':a,'commissioned':b,'difference':d}
                       for s,a,b,d in zip(provenance['seeds'],before,after,delta))
    summary={'source_revision':provenance['source_revision'],'runs':provenance['runs'],'results':result,
             'input_sha256':hashes, 'analyzer_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
             'scope':'Existing local commission intervention; fixed report/security initial purposes. Readiness only; not sovereign delegation.',
             'interpretation':'Positive throughput or survival deltas alone do not establish net benefit. Undefined receipt/completion times are not zero-delay successes.'}
    (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    with (out/'paired.csv').open('x',newline='') as f:
        w=csv.DictWriter(f,fieldnames=['case','seed','metric','pending','commissioned','difference']);w.writeheader();w.writerows(raw)
    lines=['# F13A paired cross-system readiness','',f"{provenance['runs']} registered runs. Values are means; difference is commissioned minus pending. No welfare score.",'']
    for case,metrics in result.items():
        lines += ['## '+case,'','| Metric | Pending | Commissioned | Difference | Changed pairs |','|---|---:|---:|---:|---:|']
        for k,v in metrics.items():lines.append('| '+k+' | '+' | '.join(fmt(v[a]['mean']) for a in ['pending','commissioned','difference'])+' | '+str(v['changed_pairs'])+' |')
        lines += ['']
    (out/'SUMMARY.md').write_text('\n'.join(lines))
    print(provenance['runs'],'paired integration runs analyzed')

if __name__ == '__main__': main()
