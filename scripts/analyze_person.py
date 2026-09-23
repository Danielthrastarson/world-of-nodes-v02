#!/usr/bin/env python3
"""B0 coverage from preserved decision/physical traces; not another policy input."""
from pathlib import Path
from collections import Counter
import csv,gzip,json,re,sys,itertools
root=Path(sys.argv[1]);out=Path(sys.argv[2])
validation=json.loads((root/'validation.json').read_text())
selected=Counter();events=Counter();coverage={}
for p in sorted(root.rglob('events.tsv.gz')):
    decisions=Counter();physical=Counter()
    with gzip.open(p,'rt') as f:
        first=next(f,'')
        if first.startswith('Event {'):
            def legacy_rows():
                for line in itertools.chain([first],f):
                    found=re.search(r'kind: "([^"\n]+)", detail: "(.*)" \}\s*$',line)
                    assert found,(str(p),'unrecognized legacy event',line[:120])
                    yield {'kind':found[1],'detail':found[2]}
            stream=legacy_rows()
        elif first:
            stream=csv.DictReader(itertools.chain([first],f),delimiter='\t')
            assert {'kind','detail'}<=set(stream.fieldnames),(str(p),stream.fieldnames)
        else:stream=[]
        for row in stream:
            kind=row['kind'];detail=row['detail']
            assert kind,(str(p),'empty event kind')
            if kind=='Decision':
                match=re.search(r'intent=(\w+)',detail)
                if match:decisions[match[1]]+=1
            else:physical[kind]+=1
    if decisions or physical:
        coverage[str(p.relative_to(root))]={'selected_commands':dict(decisions),'physical_events':dict(physical)}
        selected.update(decisions);events.update(physical)
# Method checks are recorded by their actual runtime consequences, not role labels.
families={
    'farming':('Cultivate','Harvested'),
    'capital':('BuildCart','CartCompleted'),
    'construction':('BuildCrossing','CrossingCompleted'),
    'provisioning':('Deliver','Delivered'),
    'barter':('AcceptBarter','BarterSettled'),
    'market':('OfferBarter','MarketDeposited'),
    'authority':('AppointSteward','StewardAppointed'),
}
result={
 'source_revision':validation['source_revision'],
 'run_executions_exact':validation['total_run_executions'],
 'trace_files':len(coverage),
 'coverage_note':'Counts pool only recorded trace seeds and include overlap between regression panels; they are coverage, not independent rates or new experimental outcomes.',
 'selected_command_counts':dict(selected),'physical_event_counts':dict(events),
 'family_labels':{k:{'example_command':c,'example_event_name':e,'command_seen':selected[c]>0,'event_seen':events[e]>0} for k,(c,e) in families.items()},
 'parser_formats':'Explicit TSV header or legacy Rust Event debug record; unknown schemas fail rather than silently counting blanks.',
 'authority_note':'Appointment/commission are controlled institutional commands, not autonomous selections; use StewardAppointed/CrossingCommissioned physical events and authority mechanism tests.',
 'files':coverage,
}
assert selected['Cultivate']>0 and selected['AcceptBarter']>0 and events['MarketDeposited']>0
assert events['CrossingCompleted']>0 and events['StewardAppointed']>0
with out.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
