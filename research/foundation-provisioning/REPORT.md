# F2: autonomous provisioning foundation

**PASSED as a causal-capability gate; communal efficiency is not validated.**
Source revision ba7f994. Specification: `docs/PROVISIONING_FOUNDATION.md`.
180 autonomous/control runs (30 seeds × two travel scales × communal/self-only/idle),
plus two kernel-executed globally informed schedule witnesses. All prior research,
including the negative locked reference, remains unchanged.

The compact geometry was registered before execution:0.5h per edge versus the
regional reference's2h. There was no parameter search. Both retain50 agents,
3000 quanta at F, t0 age,48h spoilage, capacity4, carrying10, initial nutrition12h,
request600, no inherited source knowledge, same no-meal forecast/six-hour margin,
same reactive priorities, ownership and FIFO. No workers are assigned in autonomous
arms. The controller's4 carriers/46 residents exist only in the labeled research
witness, never NPC behavior.

## Main outcomes, per run

| Geometry / institution | Survivors36/48/72h | Delivered | Consumed | Spoiled | Starvation-person-hours |
|---|---|---:|---:|---:|---:|
| Regional communal |28/28/28|0|1264|1736|944|
| Regional self-only |28/28/28|0|1264|1736|944|
| Regional idle |0/0/0|0|0|3000|1200|
| Compact communal |50/50/50|24|2456|544|544|
| Compact self-only |50/50/50|0|2524|476|476|
| Compact idle |0/0/0|0|0|3000|1200|

Per-seed CSVs are authoritative. Each of these line arms has one measured event
history across its30 seeds; this is deterministic replication, not30 independent
stochastic outcomes. All physical audits pass; material error and overtakes are0.
Idle remote food never reduces hunger. Survival still hides undernourishment under
the unchanged starvation grace/reset rule.

## Compact communal funnel

50 request observers →50 source discoverers →46,928 candidate evaluations →
9,680 PASS →66 completed pre-claim board approaches →62 accepted jobs →
62 source arrivals/acquisitions →62 started returns →10 returns reaching C →
6 completed deposits →24 delivered quanta →20 quanta consumed from communal stock.

Counts describe different units (agents, decisions, job episodes and goods); they
are not interchangeable conversion-rate denominators. There are9,618 board-directed
decisions, including repeated queue waits. The6 successful jobs are completed by6
agents; all six communal-stock consumers are also contributors in this history.
We demonstrate shared physical stock and voluntary deliveries, not proof of
altruistic support for distinct non-carrier residents or a general food network.

All62 jobs acquire452 communal quanta in total. Carriers consume328;24 reach C;
100 remain in carried inventories and ultimately become waste. Of the24 at C,
20 are consumed and4 spoil there. No cargo is duplicated or replenished after eating.
No communal cargo spoils while physically traversing an edge in this arm; that
narrow metric does not mean no transported cargo is wasted at nodes.

56 jobs abandon:34 cannot acquire further cargo after their acquisition target
has been spent/consumed;22 reject the remaining delivery as survival-infeasible or
predicted spoiled. There are56 claim expiries,30 replacement/duplicate attempts,
554 commitment-churn events and622 repeated job/edge joins. These lease effects
are relevant here because jobs actually exist; they still do not explain the
zero-acceptance regional reference.

First delivery:40.993930h, agent42,4 quanta, following its accepted job at34.61h.
Agent43 delivers simultaneously; agents28/29 deliver at42.660597h and0/1 at44.792408h.
All food retains its original48h expiry. Detailed event/job records are preserved.
Consumption from actual Commons(node) custody is counted by a read-only observer,
separately from eating Commons-owned cargo in an agent's inventory.

## Benefit, competition and the physical control

Compact self-only migration outperforms communal participation on nourishment:
same50 survivors,68 more quanta consumed,68 fewer wasted and68 fewer starvation
person-hours. Communal queue time is1310.149792 person-hours versus144 in self-only;
785.367493 of the communal arm's queue hours are loaded. At72h, communal has10 alive
at C,36 at F and4 elsewhere, while self-only hasall50 at F. Migration remains valid.
Delivery's existence is not evidence that maintaining C is beneficial.

The regional physical controller delivers120 quanta, first at8.015432h; compact
controller delivers600, first at2.012222h. These are constructive feasibility
witnesses through the real kernel, not exact optimal schedules and not comparable
welfare optima: they deliberately hold46 residents at C. Endpoint survival is4 and14
respectively. They demonstrate a physical delivery gap, not better social outcomes.
The material-only upper bound is3000 distinct original quanta transported from F to C (not repeated recirculation counts). Earlier tighter reference
bounds/280-quanta witness remain preserved; this different resident-consumption
schedule does not replace them. An exact bulk optimum remains unresolved.

## Gate and next dependency

F2 now distinguishes zero regional acceptance, geography-dependent voluntary jobs,
carrier consumption/queue losses, and completed compact deliveries that coexist
with migration yet worsen nourishment. No policy or institution is altered to
improve this. The simulator can execute and explain the full physical chain.

F3 is blocked on a genuine property-rights choice: who owns a crop produced by
labor while it is still growing? Open-access natural-food rules do not decide that.
`docs/RENEWABLE_PRODUCTION_DECISION.md` specifies the bounded next model and options.
No farming, capital, construction, money or higher system is implemented before
that decision. This is an owner-level institution gate, not a request for permission
to perform an ordinary bug fix.

## Reproduction and interpretation safeguards

`cargo test --all-targets`; `cargo build --release --bins`;
`python3 scripts/run_foundation_provisioning.py NEW_DIRECTORY`.
See `matrix/provenance.json`, parameters, counts, job journals and events. Primary
acceptance gates are the active-estimator research journal. Legacy decomposition
`at_job_priority`/`gate_*` estimates exist solely as historical comparison instruments
and are not used to interpret this new funnel. Global research knowledge never
feeds an autonomous decision. Previous outputs are hash-checked unchanged.
