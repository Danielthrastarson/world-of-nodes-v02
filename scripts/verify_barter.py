#!/usr/bin/env python3
"""Recheck historical, witness, physical invariant and artifact preservation claims."""
from pathlib import Path
import csv,hashlib,json,re,sys
r=Path('research/barter');matrix=r/'matrix'
def rows(p): return list(csv.DictReader(p.open()))
checks={}
for fixture in ['complementary','symmetric','low_cost','high_cost']:
 old=Path('research/settlements/matrix-corrected')/fixture;new=matrix/(fixture+'_off')
 assert (old/'runs.csv').read_bytes()==(new/'runs.csv').read_bytes()
 assert (old/'events.tsv.gz').read_bytes()==(new/'events.tsv.gz').read_bytes()
 checks[fixture]={'checkpoints_identical':210,'full_seed0_trace_identical':True}
 if fixture!='low_cost':
  assert (new/'events.tsv.gz').read_bytes()==(matrix/(fixture+'_on')/'events.tsv.gz').read_bytes()
for witness,fixture,seed in [('colocated-witness','colocated_on','0'),('low-cost-seed18','low_cost_on','18')]:
 for p in (r/witness).glob('*.csv'):
  assert rows(p)==[x for x in rows(matrix/fixture/p.name) if x['seed']==seed],p
for d in matrix.iterdir():
 if not d.is_dir(): continue
 a=rows(d/'runs.csv');assert len(a)==210
 assert all(x['accounting_error']==x['overtakes']=='0' for x in a)
prior=json.loads((r/'prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
out={'source_revision':json.loads((matrix/'provenance.json').read_text())['revision'],'experiments':360,'checks':checks,'witnesses_reproduced':2,'prior_files_unchanged':len(prior),'tests_passed':sum(map(int,re.findall(r'test result: ok\. (\d+) passed',(r/'validation-tests.txt').read_text()))),'material_errors':0,'fifo_overtakes':0,'money_implemented':False}
with Path(sys.argv[1]).open('x') as f:json.dump(out,f,indent=2);f.write('\n')
