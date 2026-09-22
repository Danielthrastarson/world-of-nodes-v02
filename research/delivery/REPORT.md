# Delivery decomposition: feasible physics, no autonomous acceptance

The accepted P1 negative result is preserved. This increment changes no default
policy or physical parameter. Source commit: `40d5f79` (full SHA in provenance).
The new evidence consists of 180 autonomous runs (six cases, seeds 0–29), 117
research schedules, and 11 complete independently audited traces.

## Answers to the research questions

1. **C-bound delivery is physically feasible.** The earliest first quantum is
   delivered at **8h 8.334s**. A validated bulk schedule delivers **280 quanta**
   before 48h. Both execute actual movement, FIFO admission, work, meals, sleep,
   ownership transfers and immutable-age food in the original runtime.
2. **The demonstrated gap is at least 280 quanta versus zero.** The analytical
   bulk upper bound is **430 quanta**. The exact maximum remains unproved:
   **280 <= physical maximum <= 430**. Do not call the finite search an optimizer
   with a global optimality certificate. This remaining uncertainty does not
   affect the finding that physical impossibility cannot explain zero delivery.
3. **The delivery-specific autonomous chain fails at acceptance.** All 50 agents
   observe the initial C request; no agent accepts it in any of the 120 primary
   paired runs. Source-seeking under a job, job arrival at F, communal acquisition,
   return, C arrival and delivery are all zero. Ordinary migration to F continues.
4. **None of the tested isolated commitment variants closes any delivery gap.**
   Queue commitment has no accepted jobs on which to act. Existing jobs already
   persist across ordinary decisions, so the job variant is an equivalence
   control. Sleep commitment changes behavior substantially but still produces
   no jobs. Traces identify the upstream known-source/local-claim/unfed-trip
   valuation gate as binding. We have not ablated those separate acceptance
   prerequisites and cannot assign a causal percentage to each.
5. **A change to that upstream gate would introduce a behavioral/planning
   assumption, not merely repair a lost intention.** Crediting planned future
   meals, accepting exploratory requests, or changing the survival margin would
   each be another decision rule. None was implemented. Coherent sleep is itself
   an explicit persistence rule; it improves action continuity here but worsens
   starvation exposure. No research variant is promoted to the default policy.

Communal provisioning remains unvalidated. The reference result—consumers migrate
rather than haul food to C—is retained. Stale claims cannot explain a run with
zero accepted jobs.

## Physical envelope and support

The [bound derivations](../../docs/DELIVERY_BOUNDS.md) separate certificates,
relaxed upper bounds and attained witnesses. The reference physical world remains
50 people at C, two 2h edges with shared capacity four, carrying capacity ten,
3000 quanta at F created at t=0, expiry at 48h, existing physiological penalties
and two-second gathering work. No transport or material parameter is relaxed in
an executed witness. Global knowledge and central scheduling occur only in the
research harness with autonomous decisions disabled.

The best bulk witness uses twelve centrally scheduled carriers, personal source
meals toward 18h nutrition, and rest at F from fatigue 12h toward 4h. These are
research controller choices, **not autonomous worker assignments**. Seven groups
of forty quanta arrive at C:

| Arrival time (h) | Quanta |
|---:|---:|
| 10.222222 | 40 |
| 15.866667 | 40 |
| 22.000000 | 40 |
| 29.000000 | 40 |
| 39.053272 | 40 |
| 41.053272 | 40 |
| 45.053272 | 40 |

This is a bulk-transport objective with no assigned stationary recipients; stock
can subsequently spoil at C. It demonstrates movement, not welfare. It consumes
564 quanta in carrier meals; 2436 quanta eventually become waste. All material is
accounted for. There is no stock replenishment or recirculation of delivered goods.

The first binding restriction in simultaneous dispatch is edge capacity at t=0.
Fatigue then slows otherwise fed uninterrupted work at 8h. The 48h expiry imposes
the final deadline. Source depletion never binds these witnesses. The 430 bound
relaxes physiology and work to their best possible rates; it is not an assertion
that transport alone determines the exact optimum.

“Agents supportable at C” has different answers depending on what support means:

| Definition | Demonstrated | Upper bound / status |
|---|---:|---|
| Stationary C residents with no starvation exposure through 48h | 4 | 11; exact maximum unresolved |
| Stationary communal C residents continuously nourished through 72h | 0 | 0: expiry at 48h and 12h communal ceiling |
| Stationary C residents alive at 72h, allowing starvation | 47 | 47; bound attained |
| Total agents alive and physically at C at 72h, including returned carriers | 50 | 50; population bound attained |

The final two rows exploit the **existing starvation-reset rule**, not nutritional
sufficiency. Three carriers deliver 120 quanta; each of 47 residents consumes one
quantum near 34h and one at 47h 1m. The residents consume only 94 quanta combined,
end with zero nutrition and almost 24h current starvation, and accumulate **2726
starvation-person-hours**. Total population exposure is 2756h. The carriers also
remain at C after their last viable trip. No claim of adequate provisioning follows
from 50 survivors. A minimum of two quanta per stationary 72h survivor plus the
fatigue/work bound rules out supporting 48 stationary residents with only two
carriers; the derivation is in the bound document.

## Registered fixture and interventions

The delivery fixture changes one initial institutional fact: a request for **600
quanta** is posted at C at t=0. This is 50 times the 12h communal ceiling, a stated
experimental target rather than a query of future hunger. An outstanding backlog
persists until delivered; local shortage refresh may increase demand but cannot
silently erase this initial request. All 50 people physically observe the board.
They still know only C and C–X initially; source contents require travel.

* **Current:** original pure decision policy, unmodified.
* **Queue:** an accepted-job queue wait persists through noncritical scoring and
  fatigue changes. Existing critical food/survival conditions and predicted
  impossibility may interrupt. Admission order and capacity are unchanged.
* **Sleep:** ongoing recovery toward fatigue 4h persists unless an existing critical
  food condition applies. Rates, minute sleep ticks and fatigue thresholds stay
  unchanged. Routine meals below the normal 12h target can be deferred.
* **Job:** an explicit equivalence control. Code inspection and tests found P1
  already preserves Job across ordinary evaluations; there was no score-driven
  job reset to remove. No invented stronger obligation was substituted.
* **Claim:** no redundant policy variant. P1 already expires claims after 12h,
  unblocks new attempts, and leaves the original worker's intention intact.
  Accepted-job fixtures measure this directly. No combined intervention was run.

Each primary comparison uses the same seeds 0–29. Thirty reference and thirty
existing contested-request runs are supplementary preservation/claim controls.
The line-map primary outcomes happen to be identical across seeds; these are
paired reproducibility checks, not thirty independent worlds or evidence of
statistical generality.

## Paired outcomes

All numbers below are per-run means; every primary variant has 28 survivors at
36h, 48h and 72h.

| Fixture policy | Request observers | Jobs | Delivered | Queue exits | Sleep interruptions | Decision reversals | Starvation-person-hours |
|---|---:|---:|---:|---:|---:|---:|---:|
| Current | 50 | 0 | 0 | 6656 | 6764 | 26558 | 944 |
| Queue commitment | 50 | 0 | 0 | 6656 | 6764 | 26558 | 944 |
| Sleep commitment | 50 | 0 | 0 | 22 | 62 | 1380 | 1032 |
| Job equivalence | 50 | 0 | 0 | 6656 | 6764 | 26558 | 944 |

Queue and job runs match the current fixture's event and state hashes for every
seed. Sleep produces 88 fewer consumed quanta (1176 versus 1264), 88 more spoiled
quanta (1824 versus 1736), and 88 additional starvation-person-hours. Reduced
churn is not evidence that the provisioning mechanism improved.

The primary chain is exactly:

**50 request observers → 0 accepted jobs → 0 job source searches → 0 job arrivals
at F → 0 communal acquisitions → 0 returns → 0 return admissions → 0 C arrivals
under a job → 0 deliveries.** Twenty-eight ordinary migrants still reach F.

## Why acceptance fails

Before discovery, there is no usable remembered source at C. The current policy
requires a known source to value a job and a physical board visit to accept it.
Once at F, the policy estimates F→C (to claim), C→F (to acquire), and F→C (to
fulfil), including remembered queues and physiological slowdown. It deducts this
whole time from current nutrition, **without crediting planned source meals**, and
requires at least six nutrition-hours left and no starvation on the forecast.
This is a disclosed P1 policy heuristic, not a physical restriction.

For seed 0, actor 0 at **4.083333h**, after self-provisioning and when the job
priority is actually eligible, has **12.916667h nutrition**. Its remembered-board
candidate predicts **20.392224h** total time and **7.475557h** starvation on arrival.
The nutrition-margin gate rejects it. This audit uses the same personal memory,
route estimator and candidate arithmetic as the policy; it performs no world query
and changes no decision or random stream. Across each current run there are 39048
candidate evaluations at job priority rejected for the nutrition margin, 1174
with no usable remembered source, and zero passing candidates. Repeated evaluations
are not independent attempted jobs.

There is also a simple optimistic bound on this interaction. Before any job is
accepted in this fixture, the policy normally builds a six-quantum personal buffer
and eats below 12h, so a meal leaves nutrition below 18h. From F, claiming and
fulfilling the C request requires at least 12h of nominal movement plus positive
gathering work. The gate demands another 6h left over: even before congestion or
fatigue, this requires more than 18h starting nutrition. Its forecast gives no
credit to the carrier's meals at intermediate nodes. Thus the six-quantum buffer,
local-only claim procedure and six-hour no-meals forecast margin jointly block
this candidate. This is an interaction of existing decision rules, not a shortage
of physically available calories, and not an estimate of the isolated effect of
changing any one of those rules.

The counters locate the upstream obstruction. They do not establish whether a
different local acceptance/planning rule would voluntarily deliver, nor whether
such a rule would be desirable. Migration remains a valid outcome.

## Claims and accepted-job failures: separate evidence

The **existing contested fixture** starts 25 people at C and 25 at X, with boards
at both. It is not the locked C-start reference or the new delivery fixture. Its
accepted deliveries target the X board; do not mislabel these as C-bound returns.
Across seeds 0–29:

| Accepted-job stage | Mean per run |
|---|---:|
| Accepted | 8 |
| Source sought | 8 |
| F reached | 7.2 |
| Communal cargo acquired (jobs) | 4 |
| Returns started | 4 |
| Return-edge admissions | 0.8 |
| Destination reached | 0.8 |
| Delivered | 0 |
| Expired claims | 8 |
| Replacement attempts while an expired original job remains active | 0.8 |
| Commitment churn cycles | 24 |

Six seeds (5, 13, 15, 17, 19, 25) have four measured duplicate/replacement attempts
each. This confirms that expiry already stops claim suppression; original jobs
continue. It does not estimate the causal effect of another lease duration, which
was not changed. The reference has no leases to expire.

Two inspectable failure chains:

* **Seed 0, actor 12:** claims at 30.266667h (event 162707), reaches F, acquires ten
  communal quanta, and begins the return queue at 41.189508h (214684). Eating one
  cargo quantum at 43.083333h removes it from the queue; subsequent personal
  provisioning and rejoining lose its place. The lease expires at 42.266667h
  without cancelling the job. At 44.466667h it abandons the return because the
  cargo is predicted to expire before arrival (221450). The trace audit finds
  zero predicted-fatal returns here: the combined textual cancellation reason
  must not be interpreted as evidence of imminent death.
* **Seed 5, actor 8:** accepts a four-quantum job at 13.9h (72068), persists after
  expiry, and reaches X at 39.078827h (211691). It consumes its communal cargo in
  four one-quantum meals, each within the ration ceiling (211696, 212053, 214466,
  216855). It cancels without a delivery at 42.333333h (218860). Thus some returns
  finish physically but the cargo becomes carrier nutrition. Ownership and
  material accounting remain exact.

These are additional mechanisms in accepted-job cases, not retroactive explanations
for zero acceptance at C. No isolated commitment comparison in the contested
world was used to infer a causal effect that was not measured.

## Metric definitions and verification

`runs.csv` contains every requested aggregate, including stage counts, accepted-job
churn, queue joins/exits/rejoins, sleep starts/renewals/interruptions, reversals,
claim expiry/replacement, cargo meals/spoilage, abandonment, fatal-return checks,
48h waste, all survival checkpoints and total starvation exposure. `jobs.tsv`
records every accepted job, its acquired/consumed/delivered quanta, stage flags,
expiry, reason for ending, queue joins by edge and churn count.

A churn cycle is a transition away from direct job-serving actions followed by
resumption under the **same accepted job identity**. Meals and sleep can be
legitimate reasons for those transitions. Decision reversals count changed action
categories, including legitimate eat/gather alternation. Repeated joins count
same-agent/job/edge joins, including the necessary outward/return use of one edge;
full traces distinguish voluntary queue losses from necessary traversal. Neither
counter alone proves bad decisions. Normal one-minute sleep continuation is counted
as renewal, separately from an interruption to another activity. Missing first
arrival/delivery is encoded as zero milliseconds in autonomous diagnostic columns.
Carrier deaths/fatal returns refer to an active hauling job; a later unrelated death
after abandonment is not assigned to that completed/abandoned attempt.

All 180 runs have zero material-accounting error and zero queue overtakes. All 60
frozen reference/contested replays match **every original CSV result field**, both
hashes included. Eleven full traces independently pass FIFO reconstruction, shared
edge capacity, node continuity, minimum travel time and final food/waste/consumption
checks. The four physical witness traces contain zero rejected intents. Forty-six
Rust tests pass, including new locality, queue commitment, critical-food interruption,
sleep semantics, expiry/duplicate effort, earliest-bound attainment and starvation
support tests. Formatting and Clippy with warnings denied pass.

A new queue test initially supplied a goal absent from the worker's memory; it
correctly exercised the missing-route exit instead of ordinary fatigue commitment.
The fixture was corrected to supply its explicitly observed test route. No physical
parameter was changed in response. Analysis also corrected claim attribution to
actual lease expiry events rather than merely old acceptance timestamps, and
records cargo consumption/spoilage by actual custody and owner rather than inferring
it from trace text. These diagnostics do not change the preserved events or states.

## Reproduction and limitations

```sh
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
cargo fmt --check
cargo build --offline --release --bins
target/release/delivery_probe --matrix /tmp/delivery-replay 30
target/release/delivery_probe --envelope /tmp/delivery-replay
python3 scripts/analyze_delivery.py /tmp/delivery-replay
```

The analyzer checks preservation against `research/provisioning/runs.csv`, writes
full metric summaries and paired differences, audits traces, and losslessly
compresses them. `parameters.txt` records each autonomous input; `witnesses.csv`
records the 117 centrally scheduled inputs and their outcomes. `bounds.json`
explicitly marks bulk optimality uncertified. Provenance includes source/toolchain
and SHA-256 artifact hashes. FNV state/event hashes retain their existing
same-toolchain regression meaning, not a cross-platform guarantee.

Limits: this finite search does not determine the exact bulk optimum or maximum
continuously nourished population at 48h. The auxiliary 600-quantum persistent
request is an explicit fixture assumption. The queue intervention is unexercised
by accepted jobs in the primary fixture; its null result is not evidence that
queue commitment could never matter elsewhere. The job variant is intentionally
redundant because the requested intention persistence already exists. There is no
new institution, information channel, food source, farming, construction, market,
combat, renderer, or default policy change. No combined intervention or V0.3 is
proposed. The accepted negative finding remains intact.
