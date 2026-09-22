#!/usr/bin/env python3
"""Keep same-input corrections, kernel conformance and trajectory divergence separate."""
from pathlib import Path
import csv,gzip,json,sys,statistics,collections
root=Path(sys.argv[1]);H=3600000

def rows(p):
 if p.exists():
  with p.open() as f:yield from csv.DictReader(f)
 else:
  with gzip.open(str(p)+'.gz','rt') as f:yield from csv.DictReader(f)
def counts(p):
 result=collections.defaultdict(dict)
 for r in rows(p):result[r['seed']][r['key']]=int(r['count'])
 return dict(result)
def distribution(values):
 if not values:return {'n':0}
 values=sorted(values)
 return dict(n=len(values),min=values[0],median=statistics.median(values),max=values[-1],mean=statistics.mean(values),exact=values.count(0))
summary={'static':{},'offline':{},'live':{}}
static=list(rows(root/'phase1'/'static.csv'))
for r in static:
 for k in ['ms','nutrition','fatigue','cargo']:assert r['forecast_'+k]==r['kernel_'+k]
primitive=list(rows(root/'phase1'/'primitives.csv'));assert all(r['exact']=='true' for r in primitive)
summary['static']={'composite_cases':len(static),'composite_exact':len(static),'primitive_cases':len(primitive),'primitive_exact':len(primitive)}
for p in sorted((root/'phase2').glob('*-counts.csv')):
 arm=p.name.removesuffix('-counts.csv');c=counts(p)['0'];summary['offline'][arm]=c
 comparisons=list(rows(root/'phase2'/f'{arm}-comparisons.csv'))
 summary['offline'][arm]['deltas']={key:distribution([int(r['new_'+key])-int(r['old_'+key]) for r in comparisons]) for key in ['ms','nutrition','fatigue','cargo']}
 summary['offline'][arm]['kernel_rows']=len(list(rows(root/'phase2'/f'{arm}-kernel.csv')))
 if arm=='reactive':
  c['self_only_affected_still_changed']=sum(r['self_only_changed']=='true' and r['all_changed']=='true' for r in comparisons)
  c['self_only_affected_rejection_to_pass']=sum(r['self_only_changed']=='true' and r['old_gate']!='PASS' and r['new_gate']=='PASS' for r in comparisons)
for arm in ['control','coherent']:
 runs=list(rows(root/'phase3'/f'{arm}-runs.csv'));c=counts(root/'phase3'/f'{arm}-counts.csv')
 assert all(int(r['queue_overtakes'])==0 and int(r['accounting_error'])==0 for r in runs)
 summary['live'][arm]={'runs':len(runs),'seed0':runs[0],'counts_seed0':c['0'],
  'outcome_ranges':{k:sorted(set(r[k] for r in runs)) for k in runs[0] if k not in ['seed','scenario']}}
 assert all(d.get('kernel_mismatch',0)==0 for d in c.values())
 episodes=list(rows(root/'phase3'/f'{arm}-shadow-episodes.csv'))
 summary['live'][arm]['forecast_execution_errors']={key:distribution([int(r[key]) for r in episodes if r[key]!='']) for key in ['time_error_ms','queue_error_ms','nutrition_error_ms','fatigue_error_ms']}
 summary['live'][arm]['completed_arrivals']=sum(r['status']=='Arrived' for r in episodes)
 summary['live'][arm]['queue_information_errors']=distribution([int(r['predicted_queue_ms'])-int(r['frozen_queue_ms']) for r in episodes if r['frozen_queue_ms']!=''])
 summary['live'][arm]['world_change_queue_errors']=distribution([int(r['frozen_queue_ms'])-int(r['realized_queue_ms']) for r in episodes if r['frozen_queue_ms']!='' and (r['status']=='Arrived' or (r['status']=='Died' and r['queue_error_ms']!=''))])
 # A current-demand wait is conditional; no admission remains censored.
 summary['live'][arm]['frozen_no_admission']=sum(r['frozen_queue_ms']=='' for r in episodes)
old={r['seed']:r for r in rows(root/'phase3'/'control-runs.csv')}
new={r['seed']:r for r in rows(root/'phase3'/'coherent-runs.csv')}
historical={r['seed']:r for r in rows(Path('research/acceptance/current-runs.csv'))}
for seed,r in old.items():
 for key in ['event_hash','state_hash','food_consumed','food_spoiled','food_delivered','starvation_person_hours']:assert r[key]==historical[seed][key],(seed,key)
summary['paired_live']={'pairs':len(old),'control_historical_hashes_preserved':True,
 'same_state_hash':sum(old[s]['state_hash']==new[s]['state_hash'] for s in old),
 'same_event_hash':sum(old[s]['event_hash']==new[s]['event_hash'] for s in old),
 'decision_comparisons':list(rows(root/'phase3'/'live-differences.csv'))}
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print('Validated',summary['static'],summary['paired_live']['pairs'],'live pairs')

# Revisit the exact earlier self-count subset, rather than comparing only totals.
prior=list(csv.DictReader(gzip.open('research/shadow/reactive-self_count.csv.gz','rt')))
current=list(rows(root/'phase2'/'reactive-comparisons.csv'))
by={(r['time_ms'],r['agent']):r for r in current}
assert len(by)==len(current), 'Ambiguous candidate join: add source/board identity'
gate_changes=[r for r in prior if r['old_gate']!=r['corrected_gate']]
passes=[r for r in prior if r['old_gate']!='PASS' and r['corrected_gate']=='PASS']
revisit={'prior_affected_candidates':len(prior),
 'affected_still_numerically_changed':sum(by[(r['time_ms'],r['agent'])]['all_changed']=='true' for r in prior),
 'prior_gate_changes':len(gate_changes),
 'prior_gate_changes_still_changes':sum(by[(r['time_ms'],r['agent'])]['old_gate']!=by[(r['time_ms'],r['agent'])]['new_gate'] for r in gate_changes),
 'prior_rejection_to_pass':len(passes),
 'prior_passes_retained':sum(by[(r['time_ms'],r['agent'])]['new_gate']=='PASS' for r in passes)}
(root/'phase2'/'self-count-revisit.json').write_text(json.dumps(revisit,indent=2)+'\n')
