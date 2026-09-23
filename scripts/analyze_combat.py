#!/usr/bin/env python3
"""Physical paired effects, without monetized welfare or casualty equivalence."""
from pathlib import Path
import csv,json,statistics,sys
root=Path(sys.argv[1]);out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=False)
rows={p.parent.name:list(csv.DictReader(p.open())) for p in root.glob('*/runs.csv')}
cases=sorted({r[0]['case'] for r in rows.values()});arms=['undefended','defended','peace','peace_defended']
metrics=['stolen','stock','produced','consumed','deaths','starvation_h','queue_h','care_food','household_separation_h','assembled','receipts','military_food','prospective_defenders_food','prospective_defenders_deaths','raiders_deaths','prospective_defenders_farming_h','prospective_defenders_gather_h','prospective_defenders_construction_h','prospective_defenders_travel_h','prospective_defenders_queue_h','prospective_defenders_military_duty_h','prospective_defenders_combat_h','other_civilians_queue_h','other_civilians_starvation_h','target_food_consumed','target_food_remaining_all_custodies','incapacitations','hits','engaged','attackers_reaching_stock','defenders_arriving_target']
summary={}
for case in cases:
 v={a:rows[case+'_'+a] for a in arms}
 assert all([r['seed'] for r in v[a]]==[r['seed'] for r in v['undefended']] for a in arms)
 summary[case]={a:{k:statistics.mean(float(r[k]) for r in v[a]) for k in metrics} for a in arms}
 summary[case]['paired_effects']={}
 for key in metrics:
  delta=[float(b[key])-float(a[key]) for a,b in zip(v['undefended'],v['defended'])]
  peaceful=[float(b[key])-float(a[key]) for a,b in zip(v['peace'],v['peace_defended'])]
  summary[case]['paired_effects'][key]={'defense_minus_undefended_mean':statistics.mean(delta),'min':min(delta),'max':max(delta),'peace_defense_minus_peace_mean':statistics.mean(peaceful),'interaction':statistics.mean(x-y for x,y in zip(delta,peaceful))}
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
with (out/'paired.csv').open('x') as f:
 w=csv.writer(f);w.writerow(['case','seed','theft_prevented','stock_delta','produced_delta','consumed_delta','defender_death_delta','starvation_h_delta','civilian_queue_h_delta','farming_h_delta','care_food_delta'])
 for case in cases:
  for a,b in zip(rows[case+'_undefended'],rows[case+'_defended']):
   w.writerow([case,a['seed'],float(a['stolen'])-float(b['stolen'])]+[float(b[k])-float(a[k]) for k in ['stock','produced','consumed','prospective_defenders_deaths','starvation_h','other_civilians_queue_h','prospective_defenders_farming_h','care_food']])
lines=['# F11C physical outcome summary','', 'Means per 24h run; identical paired seeds. Theft is cumulative unlawful custody transfer, not all food destroyed or consumed. Deaths and labor are not converted to food equivalents.','', '| Case | Theft without → with defense | Assembled | Hits | Defender deaths | Raider deaths | Incapacitations | Farming h without → with | Care food without → with |','|---|---:|---:|---:|---:|---:|---:|---:|---:|']
for c,s in summary.items():
 a,b=s['undefended'],s['defended'];fmt=lambda x:f'{x:.3g}'
 lines.append(f"| {c} | {fmt(a['stolen'])} → {fmt(b['stolen'])} | {fmt(b['assembled'])} | {fmt(b['hits'])} | {fmt(b['prospective_defenders_deaths'])} | {fmt(b['raiders_deaths'])} | {fmt(b['incapacitations'])} | {fmt(a['prospective_defenders_farming_h'])} → {fmt(b['prospective_defenders_farming_h'])} | {fmt(a['care_food'])} → {fmt(b['care_food'])} |")
(out/'SUMMARY.md').write_text('\n'.join(lines)+'\n')
print('\n'.join(lines))
