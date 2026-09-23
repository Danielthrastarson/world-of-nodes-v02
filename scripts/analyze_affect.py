#!/usr/bin/env python3
"""Analyze sealed B2 CSVs/traces; never run or alter simulation decisions."""
from pathlib import Path
import csv, gzip, hashlib, json, re, sys
root=Path(sys.argv[1]); out=Path(sys.argv[2]); out.mkdir(parents=True,exist_ok=False)
def rows(path):
    with path.open() as f:return list(csv.DictReader(f))
def events(path):
    with gzip.open(path/'events.tsv.gz','rt') as f:return list(csv.DictReader(f,delimiter='\t'))
def physical_events(path):
    result=[]
    for r in events(path):
        if r['kind'] in {'SocialChoice','SocialAppraisal','OwnDangerObserved','DeathObserved'}:continue
        if r['kind']=='Decision':r['detail']=r['detail'].split('; social=Assessment')[0]
        result.append(r)
    return result
def ranges(rs,keys):return {k:[min(float(r[k]) for r in rs),max(float(r[k]) for r in rs)] for k in keys}
def dump(name,obj):
    with (out/name).open('x') as f:json.dump(obj,f,indent=2);f.write('\n')
summary={'history':{},'live':{},'cost':{},'checks':{}}
physical_keys=['alive','food_transferred','consumed','spoiled','starvation_h','care_travel_h','accounting_error','overtakes']
keys=physical_keys+['care_selected','care_refused','observed','anger_episodes','fear_episodes','grief_episodes','social_edges']
all_runs=[]
for mode in ['history','live']:
    cases=sorted(p.name[len(mode)+1:-3] for p in (root/'matrix').glob(mode+'_*_on') if p.is_dir())
    for case in cases:
        dirs={a:root/'matrix'/f'{mode}_{case}_{a}' for a in ['off','on']}
        arms={a:rows(p/'runs.csv') for a,p in dirs.items()}
        assert all(len(v)==30 for v in arms.values())
        all_runs.extend(arms['off']+arms['on'])
        item={'ranges':{a:ranges(rs,keys+['pressure','cost']) for a,rs in arms.items()}}
        if mode=='history':
            assert [r['physical_signature'] for r in arms['off']]==[r['physical_signature'] for r in arms['on']]
            neutral=rows(root/'matrix/history_no_history_on/runs.csv')
            assert [r['physical_signature'] for r in arms['on']]==[r['physical_signature'] for r in neutral]
            item['first_choices']={a:{v:sum(r['initial_choice']==v for r in rs) for v in sorted(set(r['initial_choice'] for r in rs))} for a,rs in arms.items()}
            item['paired_first_choice_changes']=sum(a['initial_choice']!=b['initial_choice'] for a,b in zip(arms['off'],arms['on']))
            item['seed0_history_relationships']=[r for r in rows(dirs['on']/'relationships.csv') if r['seed']=='0' and r['stage']=='history']
            item['seed0_people_on']=[r for r in rows(dirs['on']/'people.csv') if r['seed']=='0']
        else:
            item['paired_physical_changes']=sum(any(a[k]!=b[k] for k in physical_keys) for a,b in zip(arms['off'],arms['on']))
            item['activity_exact']=rows(dirs['off']/'activity.csv')==rows(dirs['on']/'activity.csv')
            personkeys=['seed','person','nutrition_ms','consumed','starvation_h','death_ms']
            project=lambda a:[{k:r[k] for k in personkeys} for r in rows(dirs[a]/'people.csv')]
            item['person_physics_exact']=project('off')==project('on')
            item['seed0_physical_events_exact']=physical_events(dirs['off'])==physical_events(dirs['on'])
            assert item['paired_physical_changes']==0 and item['activity_exact'] and item['person_physics_exact'] and item['seed0_physical_events_exact'],item
        summary[mode][case]=item
for case in ['scarce','surplus']:
    item={}
    for arm in ['off','on']:
        p=root/'cost'/f'{case}_{arm}'; rs=rows(p/'runs.csv');all_runs+=rs
        ps=rows(p/'people.csv')
        item[arm]={'runs':ranges(rs,keys),'seed0_people':[r for r in ps if r['seed']=='0']}
        for person in ['0','1']:
            subgroup=[r for r in ps if r['person']==person]
            assert len({tuple(r[k] for k in ['nutrition_ms','consumed','starvation_h','death_ms']) for r in subgroup})==1
    summary['cost'][case]=item
assert len(all_runs)==1020
assert all(r['accounting_error']=='0' and r['overtakes']=='0' for r in all_runs)
prior=json.loads((root/'prior-artifacts.json').read_text())
# Snapshot format is a direct path -> SHA256 mapping.
for path,expected in prior.items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==expected,path
summary['checks']={'runs':len(all_runs),'accounting_errors':0,'queue_overtakes':0,'historical_files_unchanged':len(prior),'matched_physics_equal':True,'live_activity_and_person_physics_equal':True,'live_seed0_physical_traces_equal':True}
dump('analysis.json',summary)
with (out/'TRACE_EXCERPTS.md').open('x') as f:
    f.write('# B2 focal causal chains\n\nSeed 0. Raw event/appraisal files preserve all 30 seeds of appraisal data and seed-0 physical traces. Histories are controlled kernel actions; matched later choices are autonomous. Times are authoritative milliseconds. Source-event IDs index the corresponding raw stage event stream.\n')
    selected={'CareTransferred','CareCompleted','Consumed','CareAttemptUnavailable','SocialAppraisal','MatchedPersonChoice','SocialChoice','OwnDangerObserved','Died','DeathObserved'}
    for case in ['help_safe','help_costly','failed_attempt','refusal','remote_refusal','fear','death_known','death_unknown','stranger']:
        f.write('\n## '+case+'\n\n```text\n')
        es=events(root/'matrix'/f'history_{case}_on')
        for r in es:
            if r['kind'] in selected:f.write('\t'.join(r.values())+'\n')
        f.write('```\n')
print(json.dumps(summary['checks']))
