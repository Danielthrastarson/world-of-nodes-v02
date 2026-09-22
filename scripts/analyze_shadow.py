#!/usr/bin/env python3
"""Streaming, explicitly stratified analysis; never treat censored plans as failures."""
import csv
import gzip
import hashlib
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
import sys

root = Path(sys.argv[1])
HOUR = 3600000

def rows(arm, kind):
    p = root / f'{arm}-{kind}.csv'
    if p.exists():
        with p.open() as f:
            yield from csv.DictReader(f)
    else:
        with gzip.open(str(p) + '.gz', 'rt') as f:
            yield from csv.DictReader(f)

def dist(hist):
    n = sum(hist.values())
    if not n:
        return {'n': 0}
    ordered = sorted(hist.items())
    def percentile(p):
        target = max(1, (n * p + 99) // 100)
        total = 0
        for value, count in ordered:
            total += count
            if total >= target:
                return value
    return dict(n=n, exact=hist[0], min=ordered[0][0], p05=percentile(5), median=percentile(50), p95=percentile(95), max=ordered[-1][0], mean=sum(v*c for v,c in ordered)/n, mean_absolute=sum(abs(v)*c for v,c in ordered)/n)

ages = ['0-2h', '2-6h', '6-12h', '12-24h', '24h+']
def age_bin(ms):
    h = int(ms)/HOUR
    assert h >= 0
    return ages[next((i for i, stop in enumerate([2,6,12,24]) if h < stop),4)]

sources = {
 'reference':('research/provisioning/runs.csv','reference'),
 'contested_requests':('research/provisioning/runs.csv','contested_requests'),
 'stale_depletion':('research/provisioning/runs.csv','stale_depletion'),
 'current':('research/acceptance/current-runs.csv','current'),
 'known_source':('research/acceptance/known_source-runs.csv','known_source'),
 'reactive':('research/congestion/reactive-runs.csv','reactive'),
 'committed':('research/persistence/committed-runs.csv','committed'),
 'forecast_corrected':('research/congestion/forecast_corrected-runs.csv','forecast_corrected'),
}
summary = {}
for runfile in sorted(root.glob('*-runs.csv')):
    arm = runfile.name.removesuffix('-runs.csv')
    if arm not in sources:
        continue
    runs = list(rows(arm,'runs'))
    historical_file, historical_name = sources[arm]
    with open(historical_file) as f:
        old = {r['seed']:r for r in csv.DictReader(f) if r['scenario']==historical_name}
    for r in runs:
        assert r['identical']=='true'
        assert r['event_hash']==r['baseline_event_hash']==old[r['seed']]['event_hash']
        assert r['state_hash']==r['baseline_state_hash']==old[r['seed']]['state_hash']
        for k in ['food_consumed','food_spoiled','food_delivered','jobs_accepted','starvation_person_hours','queue_overtakes','accounting_error']:
            assert r[k]==old[r['seed']][k], (arm,r['seed'],k,r[k],old[r['seed']][k])
    static=list(rows(arm,'static'))
    assert all(r['exact']=='true' for r in static)
    counters = defaultdict(dict)
    with (root/f'{arm}-counts.csv').open() as f:
        for r in csv.DictReader(f): counters[r['seed']][r['key']]=int(r['count'])
    result = dict(pairs=len(runs), historical_hashes_match=True, static_cases=len(static),static_exact=len(static), counts_seed0=counters['0'], outcomes={k:runs[0][k] for k in ['survivors_36h','survivors_48h','survivors_72h','food_consumed','food_delivered','jobs_accepted','starvation_person_hours','event_hash','state_hash']})
    # Physical feasibility excludes the behavioral end-nutrition margin.
    # Derive the live forecast's physical claims from captured Plan fields,
    # rather than equating policy rejection with predicted physical failure.
    predictions={}
    bodies={}
    cp=root/f'{arm}-captures.tsv'
    opener=open if cp.exists() else gzip.open
    if not cp.exists(): cp=Path(str(cp)+'.gz')
    with opener(cp,'rt') as f:
        next(f)
        for line in f:
            columns=line.split('\t')
            now, agent=int(columns[0]),int(columns[1])
            body=re.search(r'nutrition: (\d+), fatigue: (\d+), starvation: (\d+)',columns[2])
            assert body
            bodies[(now,agent)]=tuple(map(int,body.groups()))
            for index, part in enumerate(columns[-1].split('Candidate { ')[1:]):
                match=re.search(r'plan: Some\(Plan \{ elapsed: (\d+), nutrition: (\d+), starvation: (\d+), source_starvation: (\d+), meal_quanta: (\d+), delivered: (\d+)',part)
                expiry=re.search(r'expiry: Some\((-?\d+)\)',part)
                if match and expiry:
                    elapsed,n,starvation,source_starvation,meals,delivered=map(int,match.groups())
                    predictions[(now,agent,index)]=(starvation<24*HOUR and source_starvation<24*HOUR and delivered>0 and int(expiry[1])>now+elapsed)
    hist=defaultdict(Counter)
    for r in rows(arm,'primitives'):
        for k in ['time_error_ms','nutrition_error_ms','fatigue_error_ms','cargo_error']:
            hist[f'actual_primitive:{r["action"]}:{k}'][int(r[k])]+=1
    episodes=list(rows(arm,'episodes'))
    for r in episodes:
        r['online_category']=r['category']
        if r['status']!='Arrived':
            n,f,starvation=bodies[(int(r['time_ms']),int(r['agent']))]
            dt=int(r['end_ms'])-int(r['time_ms'])
            critical=max(0,n-dt)<=6*HOUR or min(24*HOUR,f+dt)>=22*HOUR
            wait=int(r['realized_queue_ms'])
            frozen=int(r['frozen_queue_ms']) if r['frozen_queue_ms'] else None
            if wait>0 and critical:
                r['category']='CAPACITY_COMPETITION' if frozen is None or frozen>=wait else 'WORLD_CHANGED_AFTER_PREDICTION'
            elif r['status']=='Died':
                r['category']='MATCH_PREDICTED_DEATH' if int(r['predicted_total_ms'])==dt else 'UNRESOLVED_DEATH'
            elif critical:
                r['category']='UNRESOLVED_INTERRUPTION'
        for k in ['time_error_ms','nutrition_error_ms','fatigue_error_ms','queue_error_ms','policy_time_error_ms','policy_nutrition_error_ms','policy_fatigue_error_ms']:
            if r[k]:hist[f'actual_travel:{k}'][int(r[k])]+=1
        if r['frozen_queue_ms']:
            hist['queue:belief_minus_frozen_ms'][int(r['predicted_queue_ms'])-int(r['frozen_queue_ms'])]+=1
        if r['queue_error_ms'] and r['frozen_queue_ms']:
            hist['queue:frozen_minus_actual_ms'][int(r['frozen_queue_ms'])-int(r['realized_queue_ms'])]+=1
    if episodes:
        with (root/f'{arm}-episode-classifications.csv').open('w') as f:
            writer=csv.DictWriter(f,fieldnames=list(episodes[0]));writer.writeheader();writer.writerows(episodes)
    result['episodes']={'n':len(episodes),'status':dict(Counter(r['status'] for r in episodes)),'primary':dict(Counter(r['category'] for r in episodes))}
    for r in rows(arm,'routes'):
        for k in ['ordering_error_ms','nutrition_error_ms','fatigue_error_ms']:
            hist[f'route_arithmetic:{k}'][int(r[k])]+=1
    physical_confusion=Counter()
    gate=Counter(); classifications=Counter(); planfail=Counter(); eligible=Counter()
    for r in rows(arm,'plans'):
        gate[r['forecast_gate']]+=1
        classifications[r['classification']]+=1
        planfail[r['shadow_failure']]+=1
        if r['complete_tape']=='true':
            key=(int(r['time_ms']),int(r['agent']),int(r['candidate']))
            assert key in predictions, (arm,key)
            predicted=predictions[key]
            shadow=r['shadow_failure']=='NONE'
            label={(True,True):'CORRECT_FEASIBLE',(False,False):'CORRECT_INFEASIBLE',(True,False):'FALSE_FEASIBLE',(False,True):'FALSE_INFEASIBLE'}[(predicted,shadow)]
            physical_confusion[label]+=1
            if predicted:
                hist['claimed_feasible_plan:cargo_error'][int(r['cargo_error'])]+=1
                if r['shadow_failure'] in ['NONE','NO_EDIBLE_CARGO']:
                    for k in ['elapsed_error_ms','nutrition_error_ms','fatigue_error_ms']:
                        hist[f'claimed_feasible_plan:{k}'][int(r[k])]+=1


        if r['complete_tape']=='true' and r['shadow_failure']=='NONE':
            eligible[r['forecast_gate']]+=1
            for k in ['elapsed_error_ms','nutrition_error_ms','fatigue_error_ms','cargo_error']:
                hist[f'conditional_complete_plan:{k}'][int(r[k])]+=1
    result['physical_plan_confusion']=dict(physical_confusion)
    result['plans']={'primary_gate':dict(gate),'classification':dict(classifications),'shadow_failures':dict(planfail),'executable_belief_tapes_by_gate':dict(eligible)}
    result['route_feasibility']=dict(Counter(r['classification'] for r in rows(arm,'route_feasibility')))
    kernels=list(rows(arm,'kernel_plans'))
    supplement=root/f'{arm}-validation-kernel_plans.csv'
    if supplement.exists():
        with supplement.open() as f:
            merged={(r['time_ms'],r['agent'],r['candidate']):r for r in kernels}
            merged.update({(r['time_ms'],r['agent'],r['candidate']):r for r in csv.DictReader(f)})
            kernels=list(merged.values())
        with (root/f'{arm}-validation-runs.csv').open() as f:
            for r in csv.DictReader(f):
                assert r['event_hash']==old[r['seed']]['event_hash']
                assert r['state_hash']==old[r['seed']]['state_hash']
    assert all(r['shadow_kernel_exact']=='true' for r in kernels), arm

    result['kernel_samples']={'n':len(kernels),'shadow_exact':sum(r['shadow_kernel_exact']=='true' for r in kernels),'belief_success':sum(r['belief_failure']=='NONE' for r in kernels),'truth_success':sum(r['truth_failure']=='NONE' for r in kernels),'belief_failure':dict(Counter(r['belief_failure'] for r in kernels)),'truth_failure':dict(Counter(r['truth_failure'] for r in kernels))}
    truth_matrix=Counter()
    truth_disagreements=[]
    for r in kernels:
        key=(int(r['time_ms']),int(r['agent']),int(r['candidate']))
        assert key in predictions, (arm,key)
        predicted=predictions[key]
        truth=r['truth_failure']=='NONE' and int(r['truth_cargo'])>0
        label={(True,True):'CORRECT_FEASIBLE',(False,False):'CORRECT_INFEASIBLE',(True,False):'FALSE_FEASIBLE',(False,True):'FALSE_INFEASIBLE'}[(predicted,truth)]
        truth_matrix[label]+=1
        if label.startswith('FALSE_'):truth_disagreements.append(dict(r,classification=label))
    result['kernel_samples']['forecast_vs_frozen_truth']=dict(truth_matrix)
    result['kernel_samples']['disagreement_examples']=truth_disagreements[:8]

    memory=defaultdict(Counter)
    for r in rows(arm,'memory'):
        memory[(r['subject'],age_bin(r['age_ms']))][int(r['error'])]+=1
    result['memory_age']={subject:{b:dist(memory[(subject,b)]) for b in ages} for subject in ['food_quantity','food_freshness','queue_same_bank','request']}
    selfrows=list(rows(arm,'self_count'))
    result['self_count']={'affected':len(selfrows),'gate_changed':sum(r['old_gate']!=r['corrected_gate'] for r in selfrows),'action_changed':sum(r['action_changed']=='true' for r in selfrows),'changed_legs':dict(Counter(r['hypothetical_legs'] for r in selfrows))}
    witness_file=root/f'{arm}-self_count_witness.csv'
    if witness_file.exists():
        with witness_file.open() as f:witnesses=list(csv.DictReader(f))
        assert all(r['kernel_failure']=='NONE' and r['elapsed_exact']=='true' and int(r['kernel_delivered'])==int(r['corrected_cargo'])>0 and int(r['old_cargo'])==0 for r in witnesses), arm
        result['self_count']['conditional_false_zero_witnesses']=len(witnesses)

    for r in selfrows:
        hist['self_count:total_forecast_difference_ms'][int(r['old_plan_ms'])-int(r['corrected_plan_ms'])]+=1
        hist['self_count:queue_delay_difference_ms'][int(r['queue_delay_difference_ms'])]+=1
        for old_ms,new_ms in re.findall(r'(\d+)->(\d+)',r['hypothetical_legs']):
            hist['self_count:matched_traversal_delay_ms'][int(old_ms)-int(new_ms)]+=1

    result['errors']={k:dist(v) for k,v in sorted(hist.items())}
    result['unmet_need']=dict(Counter(r['category'] for r in rows(arm,'needs')))
    summary[arm]=result
    print(arm, result['pairs'], 'preserved pairs;', {k:result['kernel_samples'][k] for k in ['n','shadow_exact','forecast_vs_frozen_truth']},flush=True)

(root/'summary.json').write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n')
with (root/'error-distributions.csv').open('w') as f:
    writer=csv.DictWriter(f,fieldnames=['arm','comparison','n','exact','min','p05','median','p95','max','mean','mean_absolute']);writer.writeheader()
    for arm,s in summary.items():
        for k,v in s['errors'].items():writer.writerow(dict(arm=arm,comparison=k,**v))
with (root/'memory-age.csv').open('w') as f:
    writer=csv.DictWriter(f,fieldnames=['arm','subject','age_bin','n','exact','min','p05','median','p95','max','mean','mean_absolute']);writer.writeheader()
    for arm,s in summary.items():
        for subject, bins in s['memory_age'].items():
            for b,v in bins.items():writer.writerow(dict(arm=arm,subject=subject,age_bin=b,**v))
original=json.loads((root/'prior-artifacts.json').read_text())
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==v for p,v in original.items())
(root/'preservation.json').write_text(json.dumps({'prior_files_unchanged':len(original),'pairs':sum(s['pairs'] for s in summary.values()),'checkpoints_per_pair':73,'historical_event_and_state_hashes_match':True},indent=2)+'\n')
