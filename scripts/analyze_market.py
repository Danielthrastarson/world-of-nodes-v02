#!/usr/bin/env python3
"""F9B descriptive paired evidence; no inferred prices, bargains or trade travel."""
from pathlib import Path
from collections import defaultdict, Counter
from fractions import Fraction
import csv,json,statistics,sys
root,output=map(Path,sys.argv[1:3])
fixtures=['complementary','symmetric','low_cost','high_cost','colocated','separated']
def read(name,file): return list(csv.DictReader((root/name/file).open()))
def end(rows): return [r for r in rows if r['hour']=='168']
def dist(v): return dict(mean=statistics.mean(v),min=min(v),max=max(v))
def group(rows,key,value):
    out=defaultdict(lambda:[0.0]*30)
    for r in rows: out[key(r)][int(r['seed'])]+=value(r)
    return dict(out)
keys=['alive','deaths','produced','consumed','spoiled','delivered','jobs','starvation_h','queue_h','food_flows_ab','food_flows_ba','wood_flows_ab','wood_flows_ba','visits','barter_attempts','barter_completed']
result,raw={},{}
for fixture in fixtures:
 for mode in ['on','market']:
    name=fixture+'_'+mode;runs=read(name,'runs.csv');final=end(runs)
    assert len(final)==30
    rr={k:[float(r[k]) for r in final] for k in keys}
    assert [int(r['seed']) for r in final]==list(range(30))
    counts=group(end(read(name,'barter_counts.csv')),lambda r:r['key'],lambda r:int(r['value']))
    work=group(end(read(name,'labor.csv')),lambda r:r['activity'],lambda r:float(r['person_hours']))
    accounts=group(end(read(name,'accounts.csv')),lambda r:r['key'],lambda r:float(r['value'])/(3600000 if r['key'].endswith('_ms') else 1))
    events=group(end(read(name,'counts.csv')),lambda r:r['event'],lambda r:int(r['count']))
    offers=read(name,'offers.csv');ex=read(name,'exchanges.csv');ratios=Counter();pairs=Counter()
    traders=[Counter() for _ in range(30)]
    resources=defaultdict(lambda:[0]*30)
    for x in ex:
        s=int(x['seed']);traders[s][x['proposer']]+=1;traders[s][x['taker']]+=1
        q,p=int(x['give_q']),int(x['payment_q'])
        assert q>0 and p>0 and x['proposer']!=x['taker']
        if x['give']=='Food': assert int(x['at_ms'])<int(x['give_expiry'])
        if x['payment']=='Food': assert int(x['at_ms'])<int(x['payment_expiry'])
        ratios[f"{x['give']}:{x['payment']}={Fraction(q,p)}"]+=1
        pairs[x['give']+':'+x['payment']]+=1
        resources[x['give']][s]+=q;resources[x['payment']][s]+=p
    projects=end(read(name,'projects.csv'));carts=end(read(name,'carts.csv'));inventory=end(read(name,'inventory.csv'))
    result[name]={
      'population':2 if fixture in ('colocated','separated') else 50,
      'outcomes':{k:dist(v) for k,v in rr.items()},
      'checkpoints':{h:{k:dist([float(r[k]) for r in runs if r['hour']==h]) for k in ['alive','starvation_h','consumed','produced']} for h in ['36','48','72','120','168']},
      'barter_counters':{k:dist(v) for k,v in counts.items()},
      'offer_final_statuses':dict(Counter(r['status'] for r in offers)),
      'unique_traders':dist([len(t) for t in traders]),'repeat_traders':dist([sum(n>1 for n in t.values()) for t in traders]),
      'seeds_with_exchange':sum(bool(t) for t in traders),
      'ratios_counts':dict(ratios),'resource_pair_counts':dict(pairs),
      'exchanged_quantities':{k:dist(v) for k,v in resources.items()},
      'labor_h':{k:dist(v) for k,v in work.items()},'accounts':{k:dist(v) for k,v in accounts.items()},
      'event_counts':{k:dist(v) for k,v in events.items()},
      'capital':{'projects':dist([sum(r['seed']==str(s) for r in projects) for s in range(30)]),'completed':dist([sum(r['seed']==str(s) for r in carts) for s in range(30)]),'invested_wood':dist([sum(int(r['wood']) for r in projects if r['seed']==str(s)) for s in range(30)])},
      'physical_inventory':{k:dist(v) for k,v in group(inventory,lambda r:r['material']+':'+r['owner'].split(':')[0],lambda r:int(r['quantity'])).items()},
      'exchange_search_travel_h':0,
      'exchange_search_reason':'No trade-search route action exists; changes in realized travel are paired outcomes, not directly assigned trade labor.'
    }
    receipts=read(name,'market_receipts.csv')
    receipt_groups=group(receipts,lambda r:r['kind'],lambda r:int(r['initial']))
    peaks=[]
    for seed in range(30):
        ds=defaultdict(int)
        for o in offers:
            if o['seed']!=str(seed) or mode!='market': continue
            q=int(o['give_q']);ds[int(o['created_ms'])]+=q
            matched=[x for x in ex if x['seed']==str(seed) and x['offer']==o['id']]
            released=[x for x in receipts if x['seed']==str(seed) and x['offer']==o['id'] and x['kind'] not in ('purchase','payment')]
            at=int(matched[0]['at_ms']) if matched else int(released[0]['at_ms']) if released else 168*3600000
            ds[at]-=q
        current=peak=0
        for at,delta in sorted(ds.items()):current+=delta;peak=max(peak,current)
        peaks.append(peak)
    result[name]['market']={
      'peak_locked_quanta':dist(peaks),
      'receipt_initial':{k:dist(v) for k,v in receipt_groups.items()},
      'receipt_by_resource':{k:dist(v) for k,v in group(receipts,lambda r:r['kind']+':'+r['material'],lambda r:int(r['remaining'])).items()},
      'receipt_disposition':{k:dist(v) for k,v in group(receipts,lambda r:r['kind'],lambda r:int(r['remaining'])).items()},
      'retrieved':sum(int(r['retrieved']) for r in receipts),
      'used_locally':sum(int(r['used_locally']) for r in receipts),
      'redeposited':sum(int(r['redeposited']) for r in receipts),
      'other_removed':sum(int(r['other_removed']) for r in receipts),
      'commingled_remaining':sum(int(r['commingled']) for r in receipts),
      'payment_retrieved':sum(int(r['retrieved']) for r in receipts if r['kind']=='payment'),
      'payments_with_retrieval':sum(int(r['retrieved'])>0 for r in receipts if r['kind']=='payment'),
      'dead_owner_receipts':sum(int(r['remaining']) for r in receipts if r['owner_alive']=='false'),
      'latency_h':dist([(int(x['at_ms'])-int(next(o['created_ms'] for o in offers if o['seed']==x['seed'] and o['id']==x['offer'])))/3600000 for x in ex]) if ex else None,
      'stocks_at_checkpoints':{h:{k:dist(v) for k,v in group([r for r in read(name,'market_stocks.csv') if r['hour']==h],lambda r:r['key'],lambda r:int(r['quantity'])).items()} for h in ['0','12','36','48','72','120','168']}
    }
    raw[name]=(rr,work)
paired={}
for f in fixtures:
 a,aw=raw[f+'_market'];b,bw=raw[f+'_on']
 paired[f]={'outcomes':{k:dist([x-y for x,y in zip(a[k],b[k])]) for k in keys},'labor_h':{k:dist([aw.get(k,[0]*30)[s]-bw.get(k,[0]*30)[s] for s in range(30)]) for k in aw.keys()|bw.keys()}}
with output.open('x') as f: json.dump({'arms':result,'paired_market_minus_direct':paired},f,indent=2);f.write('\n')
