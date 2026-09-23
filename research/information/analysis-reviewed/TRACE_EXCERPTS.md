# F12 source, receipt and changed-choice audit

All excerpts below are preserved seed-zero events. Thirty-seed counts come from `runs.csv` and `decisions.tsv.gz`, not multiplication of a representative trace. Historical panel and initial analysis remain unchanged. This review corrects a measurement-label ambiguity: selected aggregate equality does not establish identical physical execution.

## Matched fresh: accurate at receipt, depleted before arrival

At1h the report and actual legal availability both contain8 food quanta. At1h01m another Person consumes all8. The recipient acts on a condition that was true at receipt and becomes false before arrival. The failure is world change, not a hidden refresh or an impossible forecast.

```text
0.000000h  Some(0)  ReportCreated  report=0; node=1; observed_at=0; actual local observation
1.000000h  Some(1)  ReportReceived  Handoff { report: 0, author: 0, sender: 0, recipient: 1, node: 0, observed_at: 0, received_at: 3600000, spatial_belief_updated: true }; original observation age preserved; no authority transferred
1.016667h  Some(1)  Decision  intent=Travel { edge: 0, target: 1, food: true } nutrition_ms=39540000 starvation_ms=0 reason=seek remembered food; source=1 observed_at=0 believed_quantity=8 expires_at=172800000; predicted_ms=3600000 arrival_nutrition_ms=35940000 arrival_starvation_ms=0
1.016667h  Some(2)  Consumed  batch=0 q=8 owner=None nutrition_ms=68340000
2.016667h  Some(1)  Arrived  node=1 edge=0
2.016667h  Some(1)  StaleFoodTrip  node=1 remembered food absent/inaccessible
2.016667h  Some(1)  Decision  intent=Travel { edge: 0, target: 2, food: false } nutrition_ms=35940000 starvation_ms=0 reason=explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=28740000 arrival_starvation_ms=0
```

## Matched depleted: unavailable before receipt

At0h the source observation is recorded, then Person2 physically picks up the8 quanta. At1h the report still implies8 legally available quanta while current truth is0. Ownership/custody changed after observation; the recipient receives the stale original without access to that change.

```text
0.000000h  Some(0)  ReportCreated  report=0; node=1; observed_at=0; actual local observation
0.000000h  Some(2)  Pickup  batch=0 q=8
1.000000h  Some(1)  ReportReceived  Handoff { report: 0, author: 0, sender: 0, recipient: 1, node: 0, observed_at: 0, received_at: 3600000, spatial_belief_updated: true }; original observation age preserved; no authority transferred
1.016667h  Some(1)  Decision  intent=Travel { edge: 0, target: 1, food: true } nutrition_ms=39540000 starvation_ms=0 reason=seek remembered food; source=1 observed_at=0 believed_quantity=8 expires_at=172800000; predicted_ms=3600000 arrival_nutrition_ms=35940000 arrival_starvation_ms=0
2.016667h  Some(1)  Arrived  node=1 edge=0
2.016667h  Some(1)  StaleFoodTrip  node=1 remembered food absent/inaccessible
2.016667h  Some(1)  Decision  intent=Travel { edge: 0, target: 2, food: false } nutrition_ms=35940000 starvation_ms=0 reason=explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=28740000 arrival_starvation_ms=0
```

In both fresh/depleted cases, handoff changes the immediate typed intent in30/30 seeds. It changes immediate destination in17/30. The other13 already chose exploration toward node1 and now choose the same trip for remembered food (`food:false→true`). Selected aggregate travel, consumption and starvation measurements match at24h, but route order and endpoint need not. Seed-zero recipient arrivals:

| Case | Arm | Arrival sequence (hours, node/edge) |
|---|---|---|
| matched_fresh | off | 2.016667: node=2 edge=1; 3.016667: node=0 edge=1; 4.016667: node=1 edge=0 |
| matched_fresh | on | 2.016667: node=1 edge=0; 3.016667: node=0 edge=0; 4.016667: node=2 edge=1 |
| matched_depleted | off | 2.016667: node=2 edge=1; 3.016667: node=0 edge=1; 4.016667: node=1 edge=0 |
| matched_depleted | on | 2.016667: node=1 edge=0; 3.016667: node=0 edge=0; 4.016667: node=2 edge=1 |

## Matched spoiled: food knowledge also conveys visited-map knowledge

The8 food quanta expire at0.5h, before the1h handoff. The original report retains that deadline. The recipient correctly treats its food as inedible; research truth and believed legal availability are both0. Receipt also puts node1 into spatial memory, so exploration treats it as known. No false fresh-food promise is needed to alter behavior.

```text
0.000000h  Some(0)  ReportCreated  report=0; node=1; observed_at=0; actual local observation
0.500000h  None  Spoiled  batch=0 q=8 holder=Loose(1) created=0 deadline=1800000
1.000000h  Some(1)  ReportReceived  Handoff { report: 0, author: 0, sender: 0, recipient: 1, node: 0, observed_at: 0, received_at: 3600000, spatial_belief_updated: true }; original observation age preserved; no authority transferred
1.016667h  Some(1)  Decision  intent=Travel { edge: 1, target: 2, food: false } nutrition_ms=39540000 starvation_ms=0 reason=explore unknown location; no resource target; predicted_ms=3600000 arrival_nutrition_ms=35940000 arrival_starvation_ms=0
2.016667h  Some(1)  Arrived  node=2 edge=1
2.016667h  Some(1)  Decision  intent=Wait nutrition_ms=35940000 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed
```

All30 recipients in the on arm explore only node2 and then have no unknown remaining destination. Immediate destination changes in13/30 because17 already chose node2. Those17 still change later behavior by omitting the subsequent exploration of node1. Recipient travel falls3→1 person-hours in every seed. No consumption, starvation or death improvement is established.

## Unknown route: recipient arrives at the stationary source

Seed-zero courier decisions include682 `PAUSED_NO_KNOWN_ROUTE` evaluations. The courier does not discover a route or complete the planned trip to rendezvous0. The recipient later reaches the source node2 through its own ordinary exploration and can receive the report there. That recipient already has a newer direct observation, so the receipt does not replace its spatial belief.

```text
0.000000h  Some(0)  ReportCreated  report=0; node=2; observed_at=0; actual local observation
0.000000h  Some(0)  ReportDeliveryStarted  report=0; recipient=1; rendezvous=0; controlled purpose, no recipient location queried
11.366667h  Some(0)  ReportHandedOver  Handoff { report: 0, author: 0, sender: 0, recipient: 1, node: 2, observed_at: 0, received_at: 40920000, spatial_belief_updated: false }
11.366667h  Some(1)  ReportReceived  Handoff { report: 0, author: 0, sender: 0, recipient: 1, node: 2, observed_at: 0, received_at: 40920000, spatial_belief_updated: false }; original observation age preserved; no authority transferred
11.366667h  Some(0)  ReportDeliveryCompleted  report=0; recipient=1; physical contact
```

All30 unknown-route runs receive a report, at8.583333h (13seeds) or11.366667h (17seeds). All30 preserve the newer/equal recipient observation; belief updates are0. Delivery completion means physical handoff, not successful travel to the originally supplied rendezvous.

## Live on-arm completion / cancellation counts

Each row covers30 runs. These are complete event-derived counters, not the number of repeated policy evaluations. A receipt without a spatial update is still an actual local handoff.

| Case | Receipts | Belief updates | Old/equal report receipts | Completed undertakings | Recipient absent cancellations | Death cancellations |
|---|---:|---:|---:|---:|---:|---:|
| short | 30 | 30 | 0 | 30 | 0 | 0 |
| long | 0 | 0 | 0 | 0 | 30 | 0 |
| congestion | 0 | 0 | 0 | 0 | 30 | 0 |
| unknown_route | 30 | 0 | 30 | 30 | 0 | 0 |
| fatigue | 13 | 0 | 13 | 13 | 17 | 0 |
| hunger | 30 | 30 | 0 | 30 | 0 | 0 |
| absent | 0 | 0 | 0 | 0 | 30 | 0 |
| death | 0 | 0 | 0 | 0 | 0 | 30 |
| depleted | 30 | 30 | 0 | 30 | 0 | 0 |
| path_on | 30 | 30 | 0 | 30 | 0 | 0 |
| path_off | 30 | 30 | 0 | 30 | 0 | 0 |

Long/congested/absent runs each cancel30/30 because the recipient is not present when the courier reaches the fixed rendezvous. Fatigue yields13 receipts at9h, all older than recipient knowledge, and17 absent-recipient cancellations. Death cancels30/30. No report-not-held or explicit cancellation occurs. Repeated reports, successful delivery and successful belief update must remain separate measures.

Live assignment-versus-unassigned comparisons include the courier undertaking itself and physical movement costs. They do not isolate information-only effects. The six-hour recipient hold is a labeled research control, not an autonomous choice or institutional residence duty.
