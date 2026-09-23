#!/usr/bin/env python3
"""Verify F9B conservation, exact controls, paired repair replay and preservation."""
from pathlib import Path
import csv,gzip,hashlib,json,re,sys
root=Path('research/market');matrix=root/'matrix-final';first=root/'matrix-corrected'
def rows(p):return list(csv.DictReader(p.open()))
fixtures=['complementary','symmetric','low_cost','high_cost','colocated','separated']
changed=[];checks={}
for f in fixtures:
    direct=matrix/(f+'_on');old=Path('research/barter/matrix')/(f+'_on')
    assert rows(direct/'runs.csv')==rows(old/'runs.csv')
    assert (direct/'events.tsv.gz').read_bytes()==(old/'events.tsv.gz').read_bytes()
    for mode in ['on','market']:
        d=matrix/(f+'_'+mode);rr=rows(d/'runs.csv');assert len(rr)==210
        assert all(r['accounting_error']==r['overtakes']=='0' for r in rr)
        pre=rows(first/(f+'_'+mode)/'runs.csv')
        for a,b in zip(pre,rr):
            assert a.keys()==b.keys()
            keys=[k for k in a if k!='state_hash']
            if any(a[k]!=b[k] for k in keys):changed.append((f,mode,a['seed'],a['hour']))
        # Counters/receipts do not claim a disposition beyond physical quantity.
        for r in rows(d/'market_receipts.csv'):
            assert int(r['initial'])==sum(int(r[k]) for k in ['remaining','retrieved','used_locally','redeposited','other_removed'])
        for x in rows(d/'exchanges.csv'):
            assert x['proposer']!=x['taker'] and int(x['give_q'])>0 and int(x['payment_q'])>0
            for side in ['give','payment']:
                if x[side]=='Food':assert int(x['at_ms'])<int(x[side+'_expiry'])
    checks[f]={'direct_runs_exact':30,'full_direct_seed0_trace_exact':True}
for witness,fixture,seed in [('colocated-final-witness','colocated_market','0'),('low-cost-final-seed18','low_cost_market','18')]:
    for p in (root/witness).glob('*.csv'):
        assert rows(p)==[r for r in rows(matrix/fixture/p.name) if r['seed']==seed],p
prior=json.loads((root/'prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
out={'revision':json.loads((matrix/'provenance.json').read_text())['revision'],'primary_runs':360,'superseded_development_runs':720,'independent_seeds':30,'controls':checks,'changed_checkpoints_after_receipt_coalescing_repair':changed,'preserved_files':len(prior),'tests_passed':sum(map(int,re.findall(r'test result: ok\. (\d+) passed',(root/'tests-final.txt').read_text()))),'material_errors':0,'fifo_overtakes':0,'money':False,'witnesses_reproduced':2}
with Path(sys.argv[1]).open('x') as f:json.dump(out,f,indent=2);f.write('\n')
