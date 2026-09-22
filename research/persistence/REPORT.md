# Plan persistence / temporal consistency — bounded experimental result

**No arm reaches C to claim.** Remembered intention alone preserves the reactive
physical history exactly. Resumable and committed goals reduce repeated goal
creation, but do not produce board arrivals, jobs or delivery. In the committed
arm, board-bound agents persist until critical survival interrupts amid real FIFO
delay. The stronger claim that forgetting the plan alone explains zero arrivals
is not supported by this experiment.

There is local arbitration churn without material new observations. It must be
distinguished from physical route reversals, which all follow materially changed
queue evidence under the registered definition. Evidence being present does not
prove it caused a reversal, and predictable hunger can justify a reversal without
new external information. Neither statistic alone decides rationality.

## Scope and interventions

150 executed runs: five arms, paired seeds 0–29, 72h each. The unchanged fixture
has 50 agents initially at C, a 600-quanta request at C, 3000 food quanta at F,
two nominal 2h edges with shared capacity four, carrying capacity ten, and food
created at zero and expiring at 48h. No initial source knowledge, assigned workers,
forced return, priority lanes, remote claims or replenishment.

The default autonomous policy, starvation reset, physical rules, ownership and
three previous research directories are unchanged. All 180 earlier delivery runs
and all 120 acceptance runs were re-executed; every original run CSV field,
including event/state hashes, matches. The unchanged default delivery policy is
an external reference: 28 survivors and 944 starvation-person-hours, with zero jobs
or delivery. It is not the new diagnostic control.

The [registered design](../../docs/PERSISTENCE_EXPERIMENT.md) specifies these arms:

| Arm | Research arbitration |
|---|---|
| A — Reactive | Existing meal-aware policy; observational episodes, no persistent goal |
| B — Remembered | Same decisions; remember the chosen goal through interruptions, without priority |
| C — Resumable | Resume after eating, critical hunger, sleep, atomic work and ordinary self-provisioning; before fresh job valuation and optional personal-buffer gathering |
| D — Committed | Resume after eating, critical hunger, sleep and atomic work; before the ordinary below-12h preference |
| E — Self-priority ablation | Skip only the ordinary below-12h branch on an uninterrupted, previously selected board route; no goal retained across an interruption |

C explicitly cancels an ordinary self-provisioning trip away from the board but
pauses for local work. D retains its goal during critical food-seeking. Both permit
legal eating, preserve sleep and use the existing **nutrition ≤6h** critical
threshold and **fatigue ≥22h** sleep trigger, with existing recovery semantics.

A saved goal is a memory of the agent's own decision. It creates no claim, goods,
reservation or board update. Arrival completes the visit; job acceptance remains
a separate local decision using the unchanged meal-aware six-hour-end-margin rule.
Continuation checks do not rerun that entire acceptance margin each tick: they
check known connectivity, death before the next node, locally learned invalidation
and a necessary food-timing bound. The timing bound requires estimated arrival at
the board plus even nominal board→source→board travel and one gathering unit to fit
before remembered expiry. Passing this bound does not guarantee a job is feasible.

Known request expiry is inactive because these requests have no deadline; claim
leases are not request deadlines. A cancelled invalid route uses the existing
kernel interruption mechanism to leave its queue. B's bookkeeping never changes
physical actions. No combined intervention or default adoption was performed.

## Lifecycle and completion

All numbers below are per run and identical across all 30 seeds within each arm.

| Measure | A | B | C | D | E |
|---|---:|---:|---:|---:|---:|
| Intentions/observational episodes created | 92 | 16 | 16 | 12 | 92 |
| Distinct agents forming them | 16 | 16 | 16 | 12 | 16 |
| Intentions physically reaching X | 8 | 8 | 8 | 8 | 8 |
| Pauses | 0 | 88 | 4 | 8 | 0 |
| Resumptions | 0 | 76 | 4 | 0 | 0 |
| Cancelled/ended episodes | 92 | 16 | 16 | 12 | 92 |
| Completed intentions / board arrivals | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| Completion rate | 0% | 0% | 0% | 0% | 0% |
| Unresolved intentions at 72h | 0 | 0 | 0 | 0 | 0 |
| Jobs / cargo acquisitions / job returns / deliveries | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 |

A and E have no retained intention: a non-serving action ends an observed episode;
the next voluntary board decision starts another. B retains identity across these
same actions. Its smaller creation count is **bookkeeping, not improved behavior**.
Pauses/resumptions are transitions, not distinct intentions. A, B and E have identical
physical event/state hashes for each paired seed. The B intervention deliberately
cannot influence action choice; it measures remembered versus abandoned episodes
and voluntary resumption, not a new use of intention memory in scoring.

Cancellation/end reasons:

| Reason | A | B | C | D | E |
|---|---:|---:|---:|---:|---:|
| PERSONAL_BUFFER_PROVISIONING | 72 | 0 | 0 | 0 | 72 |
| EATING | 16 | 0 | 0 | 0 | 16 |
| NO_JUSTIFIED_CURRENT_ACTION | 4 | 0 | 0 | 0 | 4 |
| ORDINARY_SELF_PROVISION_OVERRIDE | 0 | 0 | 8 | 0 | 0 |
| FOOD_TIMING_INVALID | 0 | 16 | 8 | 12 | 0 |

All other terminal reasons are zero, including predicted death, target unreachable,
new local invalidation, known request expiry and death. Critical hunger/fatigue
are real **pauses** here, not terminal cancellations: D has four pause transitions
for each. Eight D agents eventually reverse for critical hunger; some were already
paused for fatigue, so those reversals do not create another pause transition.
B pauses 72 times for buffer provisioning, 12 for eating and four for no current
action. C pauses four times for ordinary local self-provisioning and resumes four.

## Reversals and information

A direction reversal is a new choice to traverse back along the last completed
edge, counted once per completed leg even if its queue is subsequently cancelled.
Actual opposite traversals and completed F→X→F trips are separately counted.

Material new information means observed changes to topology/capacity, queue service
buckets, accessible food relative to the intended quantity/expiry, or request
availability. Time stamps, people counts, small surplus-stock changes, predictable
hunger/fatigue and known expiry passage do not qualify. Current queue evidence
excludes the observer's own membership. Explicit changes accompany every reversal.

| Measure | A | B | C | D | E |
|---|---:|---:|---:|---:|---:|
| All-route direction reversals | 24 | 24 | 24 | 20 | 24 |
| Reversals of board-directed legs | 8 | 8 | 8 | 8 | 8 |
| Uninformed board reversals | 0 | 0 | 0 | 0 | 0 |
| Uninformed board-reversal rate | 0/8 | 0/8 | 0/8 | 0/8 | 0/8 |
| Uninformed all-route reversal rate | 0/24 | 0/24 | 0/24 | 0/20 | 0/24 |
| Physically admitted opposite traversals | 16 | 16 | 16 | 16 | 16 |
| Completed F→X→F trips | 8 | 8 | 8 | 8 | 8 |

All eight board-leg reversals per arm follow changed observed queue-service costs.
Other reversals include returning from newly discovered F, so discovering that
node supplies new information. The presence-based uninformed rate therefore cannot
establish that the eight board reversals were optimal or caused by new information.

| Board-leg reversal physiology | A / B / E | C | D |
|---|---|---|---|
| Nutrition range | 17.783333–17.966667h | 11.8–11.983333h | exactly 6h |
| Fatigue range | 15.033333–17.216667h | 15.016667–17.2h | 20–21h |
| Selected-action reason | Personal food buffer | Ordinary below-12h self-provisioning | Critical hunger |

Every reversal's time, nutrition, fatigue, originating intention and evidence is
in [reversals.csv](reversals.csv). The important correction to the motivating
example is that **the meal-aware reactive control turns back after eating, with
almost 18h nutrition**, to rebuild its personal food buffer. The previously reported
approximately-10h reversal was in the zero-margin arm, which is not this baseline.

## Three causal traces

### Reactive / remembered / ablation: buffer and forecast churn

Agent 0 chooses C at 5.033333h with 17.966667h nutrition. At 5.05h it leaves the
queue for personal gathering, then rejoins after one unit. This repeats until it
has its personal buffer. It is admitted F→X at 6h and reaches X at 8h, with 15h
nutrition. It queues toward C at 8.016667h, eats its six carried personal quanta
at 11.016667h, leaves the queue, and chooses F at 11.033333h to refill that buffer.

The reactive record has 72 buffer-related episode cancellations **without a
registered material observation change**, plus eight eating-related ones without
such a change. These are local goal/action churn, not uninformed route reversals.
Eating itself is legal and useful; forgetting the episode is the bookkeeping
distinction that B removes. B later voluntarily resumes 76 times but follows the
same physical trajectory.

The preserved acceptance forecasts expose a candidate source of the local churn.
At 5.033333h the forecast delivers five quanta. At 5.05h it delivers zero, rejects
for insufficient expected cargo, and the policy gathers personally. The later
forecast reuses a remembered four-person queue on future traversals of X–F,
adding two separate 2h waits. The current traversal excludes the agent's own queue
membership; future hypothetical traversals do not. Source stock remains 2956
quanta between those two decisions. The extra forecast time increases predicted
food consumption until no communal delivery remains.

This is an inspectable **forecast feedback concern**, not proof that removing it
would produce board arrivals. No queue-forecast repair or extra intervention was
run. The exact old forecasts are copied with their source hash in
[preserved-forecast-feedback.json](preserved-forecast-feedback.json); the original
artifact remains unchanged. The registered material-information definition excludes
one's own current queue membership, which is relevant to interpreting this churn.

### Resumable: ordinary preference still overrides the goal

Agent 0 keeps its original F→C intention and does not gather the optional personal
buffer during the queue. It still enters F→X at 6h and reaches X at 8h: suppressing
that early churn gains no earlier admission in this trace. It queues for C at 8h.
At 11.016667h nutrition is 11.983333h; ordinary self-provisioning takes priority,
the goal is explicitly cancelled, and it selects F. Eight C-arm cancellations
have this cause. C changes arbitration but preserves the ordinary preference.

### Committed: nine hours of FIFO delay before critical hunger

Agent 0 follows the same initial departure and queues X→C at 8h. It continues past
12h nutrition. It remains queued until 17h: **nine simulated hours**, then nutrition
reaches the unchanged critical threshold of 6h. It pauses its intention and heads
toward food. The remembered intention is eventually cancelled at 25.88h when its
food-timing bound fails. The board has not received a claim or remote update.

During that wait, edge 0 admits earlier C→X applicants in groups at 8.555556h,
11.333333h and 16.08h. All had queued at t=0, before agent 0's t=8h entry. FIFO
correctly serves them first. These events are retained in
[committed-edge0-service.json](committed-edge0-service.json). The journal identifies
locally observed queue changes; this retrospective trace can additionally inspect
the earlier applicants, information the agent did not receive remotely.

Thus the D-arm failure is not simply forgetting C at the next tick. The goal
persists, but competing migration traffic and physiological time consumption prevent
admission before an allowed survival override. A globally feasible delivery schedule
does not guarantee feasibility for an autonomous plan competing with all other
agents' concurrently chosen routes.

## Isolating the ordinary 12h branch

E evaluates the isolated ablation 16 times per run. It changes **zero decisions**.
All physical metrics and hashes remain identical to A. The eligibility counter
includes moments when a higher-priority meal already wins; skipping the lower
branch then has no effect. Eating/gathering/sleep clear this arm's uninterrupted
route marker, as registered; it is not persistent intention under another name.

The result is specific to this ablation and this meal-aware history. It does not
show that the 12h preference never matters: it explicitly cancels eight goals in
C. D postpones those reversals to critical hunger, but still yields no arrivals.
The isolated measured contribution of removing the 12h branch to C arrivals is
zero; these data do not identify a percentage share of the entire failure for
each interacting mechanism.

## Nutrition and downstream outcomes

| Outcome per run | A | B | C | D | E |
|---|---:|---:|---:|---:|---:|
| Survivors at 36h / 48h / 72h | 20 / 20 / 20 | 20 / 20 / 20 | 16 / 16 / 16 | 16 / 16 / 16 | 20 / 20 / 20 |
| Food consumed | 900 | 900 | 732 | 740 | 900 |
| Food spoiled at 48h | 2100 | 2100 | 2268 | 2260 | 2100 |
| Starvation-person-hours through 72h | 1020 | 1020 | 1044 | 1036 | 1020 |
| Queue-person-hours | 1136.936291 | 1136.936291 | 1156.377778 | 1142.377778 | 1136.936291 |
| Continuously nourished survivors through 48h | 4 | 4 | 4 | 4 | 4 |
| Continuously nourished survivors through 72h | 0 | 0 | 0 | 0 | 0 |
| Alive at C at 72h | 0 | 0 | 0 | 0 | 0 |

Every survivor is at F with zero nutrition at 72h. Starvation reset is unchanged;
technical survival remains distinct from adequate provisioning. C and D reduce
survival by four relative to the meal-aware control, with 24 and 16 additional
starvation-person-hours. Less intention churn is not itself better provisioning.
The personal buffer they forgo can be useful even when the behavior creating it
looks oscillatory. Migration remains a permitted strategy.

No arm accepts a job. Consequently no communal cargo, return or delivery stage is
reached, and this increment supplies no new test of downstream hauling or claims.

## Answers to the research questions

1. **Intentions created:** A 92, B 16, C 16, D 12, E 92 per run; distinct agents
   16, 16, 16, 12, 16. A/E count nonpersistent episodes, as disclosed above.
2. **Reach C:** zero in every arm; completion rate zero throughout.
3. **What cancels them:** A/E buffer provisioning, eating and no current action;
   B known food timing; C ordinary self-provisioning or timing; D timing, after
   critical hunger/fatigue pauses. Exact counts are in the lifecycle table.
4. **Reversals without new information:** zero of eight board-leg reversals per
   arm under the registered definition. Separately, A/E have 72 buffer cancellations
   without material new observations. Do not equate these different events or infer
   rationality merely from whether information was present.
5. **How much is specifically the 12h rule:** its isolated ablation changes zero
   decisions or arrivals in A's history. It cancels eight C-arm plans; removing its
   precedence in D delays reversal but does not produce arrivals. No general causal
   share is identified from these interacting trajectories.
6. **Does persistence produce accepted jobs:** no. B is behaviorally identical;
   C/D alter trajectories but never complete the required physical board visit.
7. **Downstream blocker after acceptance:** not reached. The remaining observed
   barrier is pre-claim transport delay plus survival/priority arbitration and
   forecast timing. Do not attribute these runs to lease or cargo behavior.
8. **Coherence versus assumptions:** explicit memory and lifecycle logging improve
   inspectability without changing B's decisions. Giving a saved intention priority
   (C/D) or suppressing a preference (E) introduces behavioral assumptions. Existing
   queue-forecast feedback warrants being recorded as a possible coherence issue,
   but was not corrected or causally isolated here. No architecture is adopted.

## Evidence and limits

62 Rust tests pass, including memory-only regression, eating/sleep resumption,
critical survival, local invalidation, no remote knowledge, and material-observation
classification. Formatting and Clippy with denied warnings pass. Every run has zero
material error and FIFO overtakes. Independent event audits check shared capacity,
admission fairness, adjacency, minimum traversal time, death inactivity, spoilage
at 48h, and consumed+waste=3000. Journal lifecycle counts match the endpoint plans;
individual zero-nutrition time sums to aggregate starvation exposure.

All 30 seeds within each arm yield identical observable journals and outcomes.
This constrained map exercises no consequential random branch; 30 seeds demonstrate
reproducibility, not broad statistical generality. These are particular definitions
of remembered/resumable/committed behavior, not every possible intention architecture.
The uninformed rate depends on the documented relevance classifier; raw changes and
observations are retained so that interpretation can be audited.

Full decision/lifecycle journals are losslessly compressed and deduplicated by
canonical content hash, with a per-seed index. No decisions are sampled out of the
journals emitted by this research instrumentation. It journals retained-goal
decision states and all lifecycle/reversal events, not every unrelated agent decision;
full physical event traces are also retained for seed 0 of each arm. Individual
checkpoint nutrition and consumption, complete plans and jobs, paired deltas,
provenance and artifact hashes accompany this report. Neither the default policy
nor prior evidence was changed.

Reproduce into a new directory after `cargo build --offline --release --bins`:

```sh
python3 scripts/run_persistence.py /tmp/persistence-replay reactive 30
python3 scripts/run_persistence.py /tmp/persistence-replay remembered 30
python3 scripts/run_persistence.py /tmp/persistence-replay resumable 30
python3 scripts/run_persistence.py /tmp/persistence-replay committed 30
python3 scripts/run_persistence.py /tmp/persistence-replay self_priority 30
python3 scripts/analyze_persistence.py /tmp/persistence-replay
```
