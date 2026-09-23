#!/usr/bin/env python3
"""Immutable B3 analysis: separate mechanism controls, live care and population limits."""
from pathlib import Path
import csv,gzip,hashlib,json,re,statistics,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
def rows(p):
 with p.open() as f:return list(csv.DictReader(f))
def events(p):
 with gzip.open(p/'events.tsv.gz','rt') as f:return list(csv.DictReader(f,delimiter='\t'))
def dump(name,v):
 with (out/name).open('x') as f:json.dump(v,f,indent=2);f.write('\n')
summary={'cases':{},'pairs':{},'checks':{}};allrows=[];same=[]
for d in sorted((root/'matrix-verified').iterdir()):
 if not d.is_dir():continue
 rs=rows(d/'runs.csv');assert len(rs)==30;allrows+=rs
 keys=[k for k in rs[0] if k not in ['seed','case','state_hash','event_hash']]
 item={'runs':len(rs),'ranges':{k:{'min':min(float(r[k]) for r in rs),'max':max(float(r[k]) for r in rs),'mean':statistics.mean(float(r[k]) for r in rs)} for k in keys},'generation3_runs':sum(int(r['generation_depth'])>=3 for r in rs)}
 ps=rows(d/'people.csv');item['birth_sexes']={sex:sum(p['sex']==sex and p['birth_time_ms']!='0' and int(p['birth_time_ms'])>0 for p in ps) for sex in ['Male','Female']}
 counts=rows(d/'counts.csv');item['counts_totals']={k:sum(int(r['count']) for r in counts if r['key']==k) for k in sorted(set(r['key'] for r in counts))}
 acts=rows(d/'activity.csv');item['activity_hours_mean']={k:sum(float(r['hours']) for r in acts if r['activity']==k)/30 for k in sorted(set(r['activity'] for r in acts))}
 # Audit primitive adult-work decision requests in the preserved seed0 child interval.
 byid={int(p['person']):p for p in ps if p['seed']=='0'}
 for e in events(d):
  if e['kind']!='Decision':continue
  m=re.fullmatch(r'Some\((\d+)\)',e['person'])
  if not m:continue
  p=byid[int(m[1])]
  if int(e['time_ms'])-int(p['birth_time_ms'])<24*3600000:
   assert not any('intent='+x in e['detail'] for x in ['Gather','Cultivate','BuildCart','BuildCrossing','Reproduce','OfferBarter','AcceptBarter','Claim']),e
 old=root/'matrix'/d.name/'runs.csv'
 if old.exists():
  orig=rows(old);ignore={'runtime_ms','initial_state_text_bytes','peak_state_text_bytes','child_deaths'}
  for a,b in zip(orig,rs):assert {k:v for k,v in a.items() if k not in ignore}=={k:v for k,v in b.items() if k not in ignore},d.name
  same.append(d.name)
 resource=(root/'matrix-verified'/(d.name+'.resources.txt')).read_text()
 rss=re.search(r'Maximum resident set size \(kbytes\): (\d+)',resource)
 item['process_peak_rss_kib']=int(rss[1]) if rss else None
 summary['cases'][d.name]=item
for name,a,b in [('birth_burden','infertile','dependency'),('social_zero_initial_attachment','dependency','dependency_social'),('production_burden','working_no_birth','working'),('dependent_competition','seeded_one','seeded_three'),('controlled_generational_support','multigeneration','generational_control')]:
 ra=rows(root/'matrix-verified'/a/'runs.csv');rb=rows(root/'matrix-verified'/b/'runs.csv');keys=['births','alive','food_transferred','food_consumed','food_spoiled','starvation_h','unsupported_h','reproduction_person_h','generation_depth']
 summary['pairs'][name]={'from':a,'to':b,'mean_paired_delta':{k:statistics.mean(float(y[k])-float(x[k]) for x,y in zip(ra,rb)) for k in keys}}
assert len(allrows)==510 and all(r['accounting_error']=='0' and r['overtakes']=='0' for r in allrows)
assert max(int(r['peak_persons']) for r in allrows)<=128
prior=json.loads((root/'prior-artifacts.json').read_text())
for p,h in prior.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
summary['checks']={'runs':510,'prior_files_unchanged':len(prior),'superseded_cases_physically_identical':same,'accounting_error':0,'fifo_overtakes':0,'safety_abort_count':0,'seed0_child_adult_work_requests':0}
dump('analysis.json',summary)
with (out/'TRACE_EXCERPTS.md').open('x') as f:
 f.write('# B3 focal causal chains\n\nSeed0; authoritative milliseconds. Full raw traces remain in each fixture. Controlled support and Partner seeding are research interventions, not NPC omniscience or autonomous mate search.\n')
 for case in ['basic','decline','dependency_social','parent_death','multigeneration','generational_control','working','seeded_three']:
  f.write('\n## '+case+'\n\n```text\n');n=0
  for e in events(root/'matrix-verified'/case):
   if e['kind'] in ['ConceptionTrial','PregnancyStarted','Born','Matured','Died','PregnancyEnded','ResearchPartnerSeed','ReproductionStarted'] or (e['kind'] in ['CareTransferred','ResearchAdultSupport','ReproductionConsent','ResearchFatalExposure'] and n<25):
    f.write('\t'.join(e.values())+'\n');n+=1
  f.write('```\n')
print(json.dumps(summary['checks']))
