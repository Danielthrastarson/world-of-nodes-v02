# Shadow forecast/action conformance: registered scope

This increment observes existing policies; it does not select or execute live actions.
All policy inputs, random streams, event order, physical parameters and prior artifacts
must be preserved. Baseline and instrumented runs must agree in state/event hashes.

Three comparisons are kept separate:

1. Existing forecast versus an independent **belief-only** action interpreter. The
   remembered queue delay is a conditional input, not a claim about exact service.
2. Authoritative kernel forks from captured real state, with no new autonomous
   commands. Existing occupants/queued agents continue under physiology/death. These
   are **conditional frozen-demand witnesses**, not omniscient optimal schedules.
3. Actual selected action episodes: admission, arrival, cancellation, death. Unexecuted
   plans have no observed completion error; interrupted waits are censored.

Every evaluated candidate is captured with observation, personal memory and the
   existing candidate sequence. The audit cannot pass true state into prediction.
Primitive conformance fixtures test independent descriptions against the kernel,
including threshold crossings, food deadlines, ownership, and integer rounding.

Primary classification order for a single matched mismatch: proven physical-model
inconsistency; institutional refusal; known shared-capacity competition; hidden
state discrepancy at prediction; later state change; explicit preference. Record
secondary evidence rather than pretending the causes are independent/additive.
Unproven feasibility is UNKNOWN, never FALSE_INFEASIBLE. Rejection by a six-hour
margin is preference, not a death-feasibility forecast.

Historical reconstruction uses the original deterministic fixtures and native
boundaries. Full audits are retained for seed 0; seeds 0–29 verify observational
noninterference and preserved historical hashes. No policy arm is newly adopted.

Memory bins: [0,2), [2,6), [6,12), [12,24), [24,infinity) hours; negative observation
ages are invalid. Freshness error compares absolute deadline, not unadjusted age.
All numerical prediction errors use prediction minus comparator. Denominators,
censored observations, unsupported cases and conditional assumptions must be explicit.

## Existing primitive contracts (description, not redesign)

| Action | Preconditions | Duration / effects | Completion / interruption |
|---|---|---|---|
| Queue for edge | Alive at adjacent node; valid target ID | Immediate FIFO membership `(join time, agent ID)`; no movement, food or ownership change | Admission when a shared slot is free; policy may cancel; death removes eligibility |
| Traverse | FIFO admission; shared occupancy below capacity | Nominal work divided by piecewise hunger × fatigue performance; nutrition -1/h, awake fatigue +1/h; carried batches retain age and owner | Endpoint after required work; atomic until endpoint or death, cargo remains at death position |
| Gather | Local edible deposit, legal access, one unreserved quantum, free carrying slot; communal action additionally requires an unmet accepted job | Reserve one quantum; 2000ms nominal work, same impairment as travel | On completion move one quantum with original age to carrier, personally/communally owned per declared action; cancellation/death/spoilage release reservation |
| Pickup | Local loose goods, legal owner, non-communal ownership, positive quantity and capacity | Immediate physical transfer; owner becomes acting agent; age unchanged | Immediate; cancels interruptible action. Existing kernel permits waste pickup too |
| Load communal cargo | Accepted unmet job, local loose/communal edible food away from destination, legal access, capacity | Immediate transfer into carrier custody, Commons ownership, increment acquired quantity | Immediate; age unchanged |
| Eat / communal withdrawal / eat communal cargo | At node; legal local/carried edible non-deposit batch; integer quantity within stock and headroom | Immediate quantity decrement and consumed ledger increment; +1 nutrition-hour/quantum; starvation reset; personal ceiling 24h, Commons ceiling 12h | Immediate; cancels interruptible action. No carried personal withdrawal from communal stock |
| Sleep | Alive at node | One-minute sleep action; hunger continues, fatigue -2/h, clamped | Boundary wakes policy; autonomous policy may extend or interrupt. Kernel-only fork ends tick at Idle |
| Claim / accept job | Communal institution, physically at board, no job, positive unclaimed demand fitting free capacity | Immediate board claim with fixed 12h expiry and persistent job; no goods reservation | Claim expiry does not cancel distant job. Acceptance and claim are one existing kernel operation, not independent primitives |
| Deliver / drop food | At accepted job destination, edible carried Commons goods | Immediate transfer to communal stock; age/owner retained; demand reduced; local claim and job cleared | Immediate. No generic personal-food Drop command exists in this kernel |
| Discard waste | Alive at node with carried waste | Immediate move to loose goods; owner/age retained | Immediate; no material deletion |
| Spoilage | Original absolute deadline reached | Positive edible batch becomes waste in place with same quantity/owner/custody | Boundary transition precedes gather completion/eating at equal timestamp |

Elapsed time advances only via the kernel clock. Immediate command effects are
not assigned a fabricated duration. A forecast's explicit wait until the next
policy tick is a scheduling assumption, recorded separately from command duration.

## Interpretation limits

The local edge observation reports applicants on the observer's bank and exposes
no in-flight occupancy (`occupied=0`). Equal observation age therefore does not
imply complete shared-edge knowledge. Unseen opposite-bank applicants are unavailable
information, not stale same-bank counts.

A current-state kernel fork retains other agents' current actions but gives them no
new commands. Its delay is a conditional current-demand estimate. It is neither an
exact prediction of future autonomous traffic nor a proof of global impossibility.
