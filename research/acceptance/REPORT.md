# Communal acceptance decomposition — negative result preserved

**All four isolated variants accept zero jobs and deliver zero communal quanta.**
Prior food knowledge alone does not unblock acceptance. Meal credit and removing
the end margin unblock some *plans to visit the board*, but no such visit completes.
The failure is a sequence of gates, not one sufficient explanation. No default
policy, physical parameter, ownership rule, or information channel was changed.

## Experiment and preservation

Thirty paired seeds, 0–29, per arm; 120 executed 72h runs. Every arm retains 50
agents initially at C, a locally visible 600-quanta request, C–X–F with 2h edges
of shared capacity four, carrying capacity ten, and 3000 quanta at F created at
t=0 and expiring at 48h. No forced return, assigned workers, or replenishment.

* **Current:** preserved delivery fixture and policy, no inherited source memory.
* **Known source:** unchanged policy; stipulated F/route observation at -1h,
  quantity 3000, original expiry 48h. No remotely current observations.
* **Meal aware:** original unknown-source initial condition, six-hour end margin,
  local board claiming; only the acceptance forecast credits legal meals.
* **Zero margin:** original unknown-source initial condition and original no-meal
  estimator; end margin zero, predicted death still prohibited. The separate
  existing entry condition, nutrition above six hours, remains unchanged.

The [registered design](../../docs/ACCEPTANCE_EXPERIMENT.md) predates these runs.
Current versus known source ran first, then the two forecast variants. Zero margin
added zero jobs in all 30 pairs, so the registered trigger for a 3h sensitivity
was not met. No combined policy was run. The optional ExploreForRequest hypothesis
was not run: inspection already shows exploration discovers F and remembered
requests motivate attempted board visits. Whether an added persistent search
intention would change these outcomes remains **untested**, not disproved.

All 180 runs of the previous delivery decomposition were re-executed and matched
every preserved CSV field, including event/state hashes. The two previous research
directories remain unchanged. The original physical witnesses and bounds remain
evidence of feasibility, not an autonomous policy or a newly proven optimum.

Within each new arm, all seeds produced identical observable decision journals and
outcomes. The constrained map and deterministic tie-breaking leave no consequential
random branch in these runs. This is reproducibility across 30 seeds, **not 30
independent demonstrations across diverse worlds**; do not infer general confidence
intervals from the zero variance.

## Acceptance funnel

Values below are **per run**, identical in all 30 paired runs. Observations count
repeated local decisions, not distinct people; exactly 50 distinct people observe
the request in every arm. `ALL` includes decisions considering remembered requests
away from C. `LOCAL` means physically at the request board. Higher-priority
deferrals are reported separately from evaluations that reach job valuation.

| Measure | Current | Known source | Meal aware | Zero margin |
|---|---:|---:|---:|---:|
| Local request observations | 55,256 | 55,256 | 55,256 | 55,256 |
| Decisions with a known request, anywhere | 161,788 | 160,612 | 151,176 | 144,996 |
| Job-level evaluations, anywhere | 58,760 | 43,678 | 41,500 | 44,736 |
| Job-level evaluations at C | 15,132 | 50 | 15,132 | 15,132 |
| Passing forecasts away from C | 0 | 0 | 92 | 1,700 |
| Passing forecasts at C | 0 | 0 | 0 | 0 |
| Jobs accepted | 0 | 0 | 0 | 0 |
| Accepted jobs seeking/reaching F | 0 | 0 | 0 | 0 |
| Communal cargo acquisitions / return attempts | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| Communal deliveries / delivered quanta | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

Exclusive primary gates, as percentages of **all job-level evaluations**:

| Gate | Current | Known source | Meal aware | Zero margin |
|---|---:|---:|---:|---:|
| NO_KNOWN_SOURCE | 33.547% | 9.881% | 49.022% | 42.874% |
| SOURCE_PREDICTED_SPOILED | 0% | 0% | 27.692% | 37.116% |
| INSUFFICIENT_EXPECTED_QUANTITY | 0% | 0% | 1.195% | 0% |
| CANNOT_COMPLETE_PLAN_ALIVE | 4.588% | 6.163% | 0% | 16.211% |
| POST_PLAN_NUTRITION_MARGIN | 61.865% | 83.955% | 21.870% | 0% |
| PASS (remote board-visit forecast only) | 0% | 0% | 0.222% | 3.800% |

All other requested primary gates are 0% in this denominator:
SOURCE_MEMORY_TOO_STALE, CANNOT_REACH_SOURCE_ALIVE, FATIGUE_CONSTRAINT,
CARRY_CAPACITY_ZERO, REQUEST_ALREADY_CLAIMED, OTHER_EXPLICIT_REASON. There is no
hard memory-age cutoff; SOURCE_MEMORY_TOO_STALE is **inactive**, not a successful
test of such a cutoff. Confidence still decays with the existing 12h constant.
Some candidates violate multiple constraints; the complete journals and
`*-journal-audit.json` retain these independent violations. Exclusive percentages
are not estimates of each mechanism's causal contribution. Rounding may affect sums.

At **C alone**, 100% of current, meal-aware and zero-margin evaluations reject for
NO_KNOWN_SOURCE. With prior source memory, 100% of the 50 local evaluations reject
for POST_PLAN_NUTRITION_MARGIN. No source-informed agent subsequently reaches C.

Local decisions that never reach job valuation:

| Explicit deferral | Current | Known source | Meal aware | Zero margin |
|---|---:|---:|---:|---:|
| Entry requires nutrition >6h | 33,468 | 33,468 | 33,468 | 33,468 |
| Self-provisioning below 12h takes priority | 0 | 15,082 | 0 | 0 |
| Critical fatigue / ongoing sleep recovery | 6,656 | 6,656 | 6,656 | 6,656 |
| Total deferred | 40,124 | 55,206 | 40,124 | 40,124 |

These map to OTHER_EXPLICIT_REASON and FATIGUE_CONSTRAINT in the raw deferred
funnel. Across all locations, legal eating and self-provisioning add further
deferrals; their exact counts and selected actions are in `*-journal-audit.json`.
No rejection is explained merely as “score too low.” The archived `diag_gate_*`
fields are the older study's unchanged estimator diagnostics; use the new gate
journals and [funnel.csv](funnel.csv) for variant-specific acceptance forecasts.

## Causal traces

**Known source at C, agent 0, t=0.** Nutrition 12h; remembered food 3000 quanta,
observed -1h, expiry 48h. Original no-meal estimate:

| Predicted action | Hours |
|---|---:|
| C→X | 2.000000 |
| X→F | 2.000000 |
| Gather ten quanta | 0.005556 |
| F→X | 2.001389 |
| X→C | 2.556327 |
| Total | 8.563272 |
| Nutrition on completion | 3.436728 |

Predicted meals: zero. Required margin: 6h. Predicted alive, but rejected.
All 50 agents have this initial calculation. They then choose self-provisioning;
after time passes, its below-12h priority prevents further local job evaluation.
Source/board colocation is therefore **not sufficient** to produce acceptance.

**Meal aware, agent 0, t=5.033333h at F.** Nutrition 17.966667h; source observed
now with 2956 quanta. A conditional F→C→F→C plan predicts 15.883333h elapsed,
five quanta eaten, five delivered, and 7.083333h nutrition remaining. The meals
are two single-quanta meals at F after acquisition and three quanta at X on the
return. All come from remembered-source communal cargo, obey the 12h ceiling,
retain the 48h expiry, and reduce deliverable cargo. The otherwise identical
no-meal shadow leaves 1.148609h nutrition and fails the margin.

The agent joins the F→X queue to visit the board. At 5.05h its updated local
forecast rejects; personal gathering cancels that queue. It repeatedly gathers
and rejoins. It is eventually admitted at 6h and reaches X at 8h. It queues for
C at 8.016667h, but eats carried personal food at 11.016667h, cancelling the queue.
At 11.033333h it chooses food at F instead. It never arrives at the board.

**Zero margin, agent 0, t=4.083333h at F.** Nutrition 12.916667h. The old no-meal
forecast predicts 20.392224h for F→C→F→C, finishing with zero nutrition and
7.475557 continuous starvation-hours: alive but badly undernourished, so allowed
only by the zero-margin variant. It queues toward C, leaves to gather/eat at
5.016667h, rejoins, and reaches X at 8h with nutrition 10h. The unchanged
below-12h self-provisioning priority immediately sends it toward F. No claim occurs.

Full seed-0 event traces preserve event IDs and parent causes. The exact predicted
routes/meals are in `*-selected-forecasts.json`; [trace excerpts](TRACE_EXCERPTS.md)
link the sequence of decisions through time. Full decision journals cover all seeds.

Pre-claim transport, independently counted from physical events in seed 0
(all seeds have identical decision journals and outcomes):

| Measure | Current | Known source | Meal aware | Zero margin |
|---|---:|---:|---:|---:|
| Distinct agents attempting a board visit | 0 | 0 | 16 | 12 |
| Board-directed queue joins | 0 | 0 | 92 | 164 |
| Queue exits before admission | 0 | 0 | 84 | 156 |
| Of those: gathering / eating | 0 / 0 | 0 / 0 | 72 / 12 | 140 / 16 |
| F→X admissions toward board | 0 | 0 | 8 | 8 |
| X→C admissions toward board | 0 | 0 | 0 | 0 |
| Completed visits to claim at C | 0 | 0 | 0 | 0 |

Thus 91.30% and 95.12% of board-directed queue entries, respectively, end in
interruption. These are **pre-claim attempts**, not accepted-job cancellations,
communal cargo returns, or proof that changing queue commitment would solve this
fixture. Neither lease expiry nor stale claims can explain a run with zero claims.

## Forecast audit and attribution limits

The meal forecast only receives current local observation and personal memory.
It carries predicted food quantity, integer consumption, ownership ceilings,
carrying limits, physiological penalties, original expiry, and remembered queue
evidence through the plan. It never queries future congestion or actual remote
stock. Remembered availability is a conditional belief, not a reservation.

The original predictor approximates batch gathering and places queue delay after
travel. A legal meal replay instead simulates individual gathering work, policy
cadence after instant actions, and waiting before traversal. These arithmetic
differences are disclosed rather than attributed silently to eating. A read-only
no-meal shadow using the **same replay arithmetic** is recorded for each meal
candidate. None of the 92 passing meal forecasts per run also passes this shadow.
Food credit is necessary for those observed passes; a fully general separation
of all estimator approximations is not established by this result.

The forecast is a conditional physically legal plan, **not a promised execution
of the unchanged priority hierarchy**. In particular, forecast meal/return steps
do not install an intention that overrides self-provisioning, sleep, or queue
reevaluation. A runtime replay test verifies time, nutrition, cargo and consumption
for a controlled legal meal plan; a separate test verifies identical predictions
under changed unobserved remote truth. Neither test guarantees autonomous execution.

## Nutrition, migration and the starvation-reset finding

| Outcome per run | Current | Known source | Meal aware | Zero margin |
|---|---:|---:|---:|---:|
| Survivors at 36h / 48h / 72h | 28 / 28 / 28 | 28 / 28 / 28 | 20 / 20 / 20 | 20 / 20 / 20 |
| Food consumed | 1264 | 1288 | 900 | 924 |
| Food spoiled at 48h | 1736 | 1712 | 2100 | 2076 |
| Starvation-person-hours through 72h | 944 | 920 | 1020 | 996 |
| Continuously nourished survivors through 36h / 48h | 16 / 16 | 16 / 16 | 4 / 4 | 12 / 12 |
| Continuously nourished survivors through 72h | 0 | 0 | 0 | 0 |
| Survivors at C / F at 72h | 0 / 28 | 0 / 28 | 0 / 20 | 0 / 20 |

Every 72h survivor has **zero nutrition**, in every arm. Median nutrition among
survivors at 36h / 48h is 15 / 15h (current), 15.082222 / 15.082222h (known),
16.802222 / 17h (meal), and 14 / 14h (zero margin). Individual distributions,
current continuous starvation, cumulative time at zero, and consumption are in
`*-agents.csv`; checkpoint summaries include minima/maxima in `*-health.json`.
“Continuously nourished” here means alive with no positive-duration exposure to
zero nutrition since t=0; it is stricter than surviving the checkpoint.

The accepted finding remains: a tiny strategically timed meal resets continuous
starvation, permitting technical survival despite a large cumulative deficit.
The previous physical witness kept 47 stationary residents alive through 72h
using two strategically timed quanta each while accumulating severe starvation.
That rule is unchanged here. Survival before approximately 36h is still protected
by initial biological reserves; survival at 72h is not adequate provisioning.

Both forecast variants reduce survival by eight agents in every pair, increase
starvation exposure by 76h and 52h respectively, and increase spoilage. Their
board-directed traffic adds congestion but achieves no delivery. Migration remains
permitted and supplies all eventual survivors. This experiment does not demonstrate
that hauling outperforms migration, or that migration is universally optimal.

## Answers to the bounded research questions

1. **Percent rejected at each gate:** the exclusive table above and `funnel.csv`
   provide all gates and denominators. In current, margin 61.865%, unknown source
   33.547%, predicted death 4.588%; no passes. Deferrals are a separate population.
2. **Is source/board colocation sufficient to explain zero acceptance?** No.
   Removing that separation still gives zero jobs: the initial margin rejects all
   50 known-source agents. Information-action separation is real but not sufficient.
3. **Does ignoring meals independently block feasible plans?** It rejects some
   conditional plans that legally include eating: all 92 meal-aware passes fail
   their matched no-meal shadow. Removing this forecast restriction alone does not
   overcome the full acceptance chain. This is not evidence of 92 accepted jobs.
4. **Effect of the six-hour end margin?** Zero margin yields 1700 passing remote
   evaluations and 164 board-directed queue entries per run, but **zero additional
   accepted jobs or deliveries**, and eight fewer survivors. It changes willingness
   to attempt an undernourished plan, not the remaining priority or locality gates.
5. **Coherence corrections?** Representing legally available nourishment and its
   physical debit addresses forecast pessimism. The replay also exposes timing
   approximations. These are research-only candidate corrections; no default adopted.
   Forecasts still do not bind actual subsequent decisions.
6. **New assumptions?** Lowering the margin changes behavioral risk tolerance.
   Prior source memory changes an explicit initial information condition.
   ExploreForRequest would add a new intention, but was not implemented/run.
   Remote claiming or new communication would change the institution; neither exists.
7. **Any isolated voluntary autonomous delivery?** No, across all 120 runs.
8. **Does delivery outperform migration or coexist with it?** No delivery occurred.
   Only attempted board visits coexist with migration, with worse measured nutrition
   and survival in the two forecast variants. Provisioning remains unvalidated.

## Verification and reproduction

52 Rust tests pass; formatting and Clippy with warnings denied pass. Every run has
zero material-accounting error and zero FIFO overtakes. Independent event audits
check shared capacity, FIFO eligibility, adjacency, traversal duration, expiry at
48h, and consumed-plus-waste=3000. Journal gates reproduce the raw counts exactly;
predicted meal ceilings and cargo limits are audited; individual zero-nutrition
time sums to global starvation exposure. Provenance and hashes accompany the data.

Build with `cargo build --offline --release --bins`. For a **new** output directory:

```sh
python3 scripts/run_acceptance.py /tmp/acceptance-replay current 30
python3 scripts/run_acceptance.py /tmp/acceptance-replay known_source 30
python3 scripts/run_acceptance.py /tmp/acceptance-replay meal_aware 30
python3 scripts/run_acceptance.py /tmp/acceptance-replay zero_margin 30
python3 scripts/analyze_acceptance.py /tmp/acceptance-replay
```

The runner refuses to overwrite a case. Each `*-journal-index.json` maps every
executed seed to a complete, content-addressed gzip TSV. Only identical journals
are deduplicated; no decisions are sampled out. Canonical journals omit the two
constant scenario/seed columns, recoverable from the index. Hashes cover the
uncompressed canonical bytes. Four distinct journals represent the 120 runs
losslessly. Full physical event traces are retained for seed 0 of each arm.
