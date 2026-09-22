# Shared-edge congestion and forecast self-counting

**The evidence supports an interaction: real transport competition reaches existing
survival overrides, and a separate forecast defect changes autonomous behavior.**
The committed agent does not simply forget its goal. Reducing pending migration
traffic lets the first returning cohort reach C. Correcting forecast self-counting
alone changes routes and survival but still produces no board arrivals, jobs or
food deliveries. Neither intervention is adopted as a default.

Direction itself has no additional physical service cost in this kernel: equal
applicants with equal physiology consume the same shared capacity in either
direction. In the preserved world, most of that competing demand happens to be
migration in the opposite direction. The single C—X—F route forces competition.
This is a result about this topology and road model, not all possible geography.

## Scope and evidence

All physical parameters, default policy, prior results, ownership, starvation,
request locality and FIFO remain unchanged. The committed diagnostic is a preserved
reference, not a proposed NPC architecture. Its seed-0 world was rehydrated at native
event boundaries to recover states, and its complete original event/state hashes
match: `d821bd7a701bbfab` / `9461f7349827e231`.

The study contains:

- Eight reconstructed board-bound arrivals at X, with full focal memories,
  intentions, original food batches, ordered queue membership and occupancy.
- 328 conditional micro-fixtures: two actual physiological cohorts, four queue
  compositions, every integer applicant count from 0 through 40.
- Forty load replays: eight focal agents × five pending-migration fractions.
- Eight recorded-background service projections that keep only the focal queued;
  these are physical diagnostics and do not use its autonomous survival policy.
- Sixty full 72h autonomous runs: meal-aware reactive control versus forecast
  self-exclusion, paired seeds 0–29. No load intervention or commitment is combined
  with this forecast comparison.

The [registered design](../../docs/CONGESTION_EXPERIMENT.md) distinguishes these
conditional fixtures from a counterfactual whole-population policy. They preserve
50 identities and physical material. They do not demonstrate an independently
emerged settlement, delivery network or adequately nourished population.

## 1. What can each agent tolerate at X?

Times below are simulated hours. Every agent has zero carried food at the fork.
All four current edge occupants are C→X migrants. Queue counts include only
applicants preceding the focal under `(joined_at, AgentId)`.

| Agent | Arrives X | Nutrition | Fatigue | Earlier C→X | Earlier X→C | Total ahead | Occupants |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8.0 | 15.0 | 12.0 | 34 | 0 | 34 | 4 |
| 1 | 8.0 | 15.0 | 12.0 | 34 | 1 | 35 | 4 |
| 2 | 8.0 | 15.0 | 12.0 | 34 | 2 | 36 | 4 |
| 3 | 8.0 | 15.0 | 12.0 | 34 | 3 | 37 | 4 |
| 4 | 13.2 | 11.8 | 17.2 | 26 | 4 | 30 | 4 |
| 5 | 13.2 | 11.8 | 17.2 | 26 | 5 | 31 | 4 |
| 6 | 13.2 | 11.8 | 17.2 | 26 | 6 | 32 | 4 |
| 7 | 13.2 | 11.8 | 17.2 | 26 | 7 | 33 | 4 |

Every preceding applicant's identity, direction and join time is in
[queue-ahead.json](queue-ahead.json); occupants and admission times are in
[occupancy.json](occupancy.json). Raw agent, memory, intention and goods snapshots
are in [snapshot-details.txt](snapshot-details.txt).

| Physiological delay budget at X | Agents 0–3 | Agents 4–7 |
|---|---:|---:|
| Until critical hunger, without food | 9.0 | 5.8 |
| Until critical fatigue if continuously awake | 10.0 | 4.8 |
| Until zero nutrition, without food | 15.0 | 11.8 |
| Until starvation death, without food | 39.0 | 35.8 |
| Direct traversal duration with immediate admission | 2.222222 | 2.666667 |
| Earliest arrival with an empty edge/queue | 10.222222 | 15.866667 |
| Requested budget: hunger window − immediate traversal | 6.777778 | 3.133333 |
| Same formula using the first hunger/fatigue override | 6.777778 | 2.133333 |
| Exact wait allowing arrival before critical hunger, integrating fatigue | 6.0 | 2.133333 |
| Exact wait allowing arrival before either threshold | 6.0 | 1.466667 |

Food expiry remains absolute t=48h. The death deadlines without further food are
absolute t=47h and t=49h, respectively. These do not mean waiting that long is an
autonomous choice.

The requested subtraction formula is a **snapshot-speed bound**, not an exact
maximum: travel slows as waiting adds fatigue. More importantly, admission makes
traversal atomic. An agent admitted before an override may cross the threshold
on the edge and still reach C. Thus arrival-before-critical budgets must not be
used as exact admission cutoffs or death limits. The immediate-admission bounds
above condition on an empty edge; they do not ignore occupancy in the actual replay.
See [physiological-budgets.json](physiological-budgets.json).

## 2. What delay did actual FIFO produce?

**None of the eight was admitted X→C in the preserved autonomous trace.** Actual
waiting-to-admission is therefore censored, not an observed 16h or infinite delay.

- Agents 0–3 queue at 8h, remain queued for 9h, and reverse toward food at 17h with
  nutrition 6h and fatigue 21h.
- Agents 4–7 queue at 13.2h, leave the queue for critical fatigue at 18h after 4.8h
  actually queued, sleep for 1h, and reverse toward food at 19h with nutrition 6h
  and fatigue 20h. Their elapsed 5.8h to reversal is not 5.8h of queue occupancy.

Earlier migrants receive admission in batches at 8.555556h, 11.333333h and 16.08h.
Waiting migrants subsequently leave/rejoin queues for sleep. FIFO ordering alone
cannot predict an unconditional admission time without assumptions about that
future eligibility.

Two explicitly omniscient projections expose the distinction:

| Projection | Agents 0–3 | Agents 4–7 |
|---|---|---|
| Recorded background actions, only focal held queued | Admit 24.08h; reach C 32.08h | Admit 24.08h; reach C 31.528h |
| All current applicants remain queued, no new actions | Admit 36h; reach C 44h | Admit 44h; no arrival by 48h |

The first projection implies delays of 16.08h and 10.88h before admission, beyond
the actual survival overrides. It also shows that the crossing is **not literally
fatal under the locked starvation grace period**: a carrier can arrive with zero
nutrition after roughly 9.08h or 6.528h of starvation. Holding a goal through those
conditions is not the existing autonomous policy and is not adopted. This is a
service calculation, not successful provisioning or a recommendation to tolerate
more starvation. No global projection is passed to an agent.

## 3–4. How many applicants cause failure, and does direction matter?

Micro-fixtures preserve the actual focal agent, time, food age, knowledge and
committed diagnostic policy. Background travelers use the physiology of an actual
unprovisioned waiting migrant at the same fork: nutrition/fatigue 4h/12h at 8h,
and 0h/17.2h at 13.2h. They queue once, age, slow and can die. Their conditional
initial endpoints and earlier queue submissions are explicit experimental inputs;
no in-run teleportation, privileged admission or new physical capacity occurs.

The edge starts empty for this threshold test. All 50 identities and existing
material remain represented. Background actors are scheduled transport demand,
not a new autonomous population model.

| Actual focal state | Largest successful ahead count | First failing count | Outcome at failure |
|---|---:|---:|---|
| Agent 0 at 8h | 11 | **12** | Critical-hunger reversal |
| Agent 4 at 13.2h | 3 | **4** | Sleep interruption, then critical-hunger reversal |

These thresholds hold for all-same-direction, all-opposite-direction, 50/50 and
observed-order compositions. At zero load the agents reach C at 10.222222h and
15.866667h. For the first cohort, eight ahead still permits arrival at 18.85h,
after critical hunger is crossed during atomic travel.

There is one policy-visible directional difference at the extreme 40-ahead case:
the later agent with all applicants locally at X cancels for `FOOD_TIMING_INVALID`
and reverses at 17.4h with 7.6h nutrition. Other compositions reverse at 19h. The
larger locally visible queue changes its forecast; this is not an opposing-direction
service penalty. All compositions still fail there, and thresholds are unchanged.

In the actual trace, opposing migrants constitute 34 of 34–37 earlier applicants
for the first cohort (100% down to 91.9%), and 26 of 30–33 for the later cohort
(86.7% down to 78.8%). The observed bottleneck is predominantly migration, but
same-direction returners also consume the same capacity. See
[micro.csv](micro.csv) and [queue-thresholds.json](queue-thresholds.json).

## 5–6. Does the recorded migration wave reproduce failure? What load permits C arrival?

**Yes: all eight 100% replays reproduce every focal decision and physical event
through reversal**, including times and physiology, against the preserved trace.

The counterfactual denominator is the **pending C→X demand at the X-arrival fork**:
34 applicants for agents 0–3; 26 for agents 4–7. Retain the lowest IDs with
`floor(fraction × applicants)`. Suppressed applicants remain physically at C,
continue physiology, and submit no further commands in this instrument. Travelers
already on an edge finish normally. Other agents' recorded commands are replayed
through the existing kernel; illegal changed-timing commands are rejected normally.
This is a conditional load replay, not a restriction on autonomous migration or a
simulation in which 25% of the original population migrates from t=0.

| Nominal retained pending migration | Counts, first/later cohort | First cohort outcome | Later cohort outcome |
|---|---|---|---|
| 100% | 34 / 26 | Reverse at 17h | Reverse at 19h |
| 75% | 25 / 19 | Reverse at 17h | Reverse at 19h |
| 50% | 17 / 13 | Reverse at 17h | Reverse at 19h |
| 25% | 8 / 6 | Admit 16.08h; arrive **20.85h**, nutrition **2.15h** | Reverse at 19h |
| 0% | 0 / 0 | Admit 8.555556h; arrive **10.777778h**, nutrition **12.222222h** | Reverse at 19h |

The first cohort waits 8.08h in the nominal-25% condition and 0.555556h at 0%.
Actual retained fractions are 8/34=23.53% and 6/26=23.08% in the nominal-25% rows.
The highest successful tested percentage is 25% for the first cohort; this grid
does not identify an exact continuous percentage threshold.

For the later cohort, even zero **pending migrants** leaves four in-flight migrants
and earlier X→C returners. The earlier returners take the next slots at 16.08h;
the focal reaches critical fatigue at 18h before another slot opens. That is why
its zero-pending-migration replay still fails, although its empty-edge micro-fixture
succeeds. Do not label this case an empty-road control.

The replay therefore demonstrates sensitivity to accumulated transport demand.
It does not show that reducing migration is a desirable solution, nor establish
that migration alone explains every later failure. Earlier physical occupancy,
same-direction competition, fatigue and survival arbitration also matter.

The few changed-timing background commands rejected in reduced-load replays are
attempted interruptions of atomic travel. The kernel rejects them; no body is
moved illegally. All are counted in [replay-audits.json](replay-audits.json).
No reduced-load result is presented as a full autonomous-world trajectory.

## 7. Does correcting forecast self-counting matter?

Yes, but it does not produce board arrivals. The isolated variant uses the
meal-aware reactive policy with the existing six-hour end margin. It removes the
observer's known own membership once from a **copied** current queue observation
and consistently reuses that copy on hypothetical future traversals. It does not
subtract again on the current leg, infer departures or arrivals, improve remote
knowledge, change actual queues, or add intention priority.

At t=5.05h, agent 0 has the same state and source observation in both arms:

- Old forecast: future hypothetical traversals reuse a four-person queue including
  itself, add two 2h waits, consume all ten projected communal quanta, and reject
  for insufficient deliverable quantity. The agent leaves to gather personally.
- Corrected forecast: the same observed queue contains three other applicants;
  current-leg delay remains unchanged. Five projected quanta remain deliverable,
  the forecast passes, and the agent stays queued.

This is a concrete coherence defect with a demonstrated first behavioral divergence.
The correction does not repair all subsequent arbitration. Corrected agent 0 still
reaches X at 8h with 15h nutrition, then immediately chooses F to obtain a personal
buffer. It has no committed goal in this isolated reactive arm.

| Per-run outcome, 30 paired seeds | Reactive control | Self-excluded forecast |
|---|---:|---:|
| Acceptance-stage forecast evaluations | 41,500 | 50,096 |
| Passing forecasts | 92 | 1,076 |
| Passing fraction | 0.222% | 2.148% |
| Board-directed episodes created / cancelled | 92 / 92 | 36 / 36 |
| Distinct agents forming episodes | 16 | 12 |
| Episodes reaching X | 8 | 4 |
| C arrivals / accepted jobs / deliveries | 0 / 0 / 0 | 0 / 0 / 0 |
| All route reversals | 24 | 20 |
| Queue-person-hours | 1,136.936291 | 964.113821 |
| Survivors at 36h / 48h / 72h | 20 / 20 / 20 | 24 / 24 / 24 |
| Starvation-person-hours | 1,020 | 992 |
| Food consumed / spoiled | 900 / 2,100 | 1,072 / 1,928 |

A passing reevaluation while already queued does not create a new episode or board
claim. Hence more passing forecasts and fewer episodes are compatible. Corrected
cancellations are all 36 `PERSONAL_BUFFER_PROVISIONING`; the control has 72 buffer,
16 eating and four no-current-action episode endings. Migration remains valid.
All endpoint survivors remain at F with zero nutrition. More survivors does not
establish adequate provisioning or validate communal delivery.

All 30 seeds have the same observable outcome within each arm; state hashes differ
with private seeded RNG state. This is paired reproducibility on a constrained map,
not 30 diverse ecological outcomes. Complete lifecycle journals are deduplicated
without dropping emitted records. Full forecast journals are retained for seed 0
of each arm, with forecast gate counts for all 60 runs. See
[forecast-first-divergence.json](forecast-first-divergence.json),
[autonomous-summary.json](autonomous-summary.json) and
[paired-effects.json](paired-effects.json).

## 8. Best-supported explanation

**Interaction between physical congestion, survival arbitration and forecasting.**
The committed reference preserves its goal, but queued migration and other returners
occupy a finite shared route until existing survival overrides intervene. Less
pending load causes board arrival in the first cohort; an empty queue allows both
cohorts to arrive. Direction does not add physical cost beyond shared demand.

The forecast defect is independently behaviorally significant and is a planning
coherence correction, not a new preference or transport institution. Correcting it
alone still yields zero board arrivals and deliveries. The experiments do not
identify a percentage share of the overall failure for each interacting cause.

The strongest statement supported is: **independent movement toward food creates
traffic competition that prevents these autonomous board visits under their current
survival priorities.** It is too strong to say that the traffic makes every physical
return impossible: the starvation-grace service projections demonstrate otherwise.
Nor have these experiments demonstrated a relief shipment being blocked; these
agents have not yet reached the board, accepted jobs or acquired communal cargo.

No policy is adopted, no migration is discouraged, and no alternative transport
institution or topology is introduced. Previous negative findings remain intact.

## Verification and reproduction

66 Rust tests pass; formatting and Clippy with warnings denied pass. Tests cover
single self-exclusion without fresh knowledge or double subtraction, unchanged
memory, directional physical service symmetry, exact focal-state preservation and
inert research recording. Existing conservation, ownership, death, locality, FIFO,
transport and frozen reference tests still pass.

Independent analysis verifies all eight exact focal replays, 40 replay capacity/FIFO
histories, 28 selected micro traces, and both full autonomous seed-0 physical traces.
It cross-checks journals against every run's counters, consumption and starvation
exposure. All autonomous runs retain zero material error and overtakes. Thirty
reactive runs match every field of their frozen persistence results. The full
committed reconstruction matches its frozen physical fields/hashes. Previous
research files remain byte-identical; their hashes are archived separately.

The development audit caught and corrected an instrumentation error before these
results were frozen: checking only the focal's endpoint location missed a C arrival
followed by another legal same-boundary action. Arrival is now measured from the
actual `Arrived` event and independently checked against the trace. The budget
label was also clarified after recognizing that ongoing fatigue makes the simple
subtraction optimistic. Neither correction changed autonomous rules.

After `cargo build --offline --release --bins`, reproduce into a new directory:

```sh
python3 scripts/run_congestion.py /tmp/congestion-reproduction
```

The report, raw results, snapshots, compressed journals/traces, independent audits,
provenance and SHA256 manifest are preserved together. No subsequent research
increment is selected by this report.
