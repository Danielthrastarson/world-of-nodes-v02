# Forecast coherence correction — evidence before adoption

**The six demonstrated mechanics defects are corrected in an opt-in research
forecast. In the required live delivery fixture, this changes no selected actions,
physical outcomes, jobs or deliveries. The default forecast remains unchanged.**

## Scope and preservation

Baseline: `5093fdd`. The live comparison changes forecast mechanics only. It retains
50 agents at C, the physical 600-quanta request, unknown F, 3000 food quanta created
at t=0, 48h expiry, 2h edges, shared capacity four, carrying capacity ten, FIFO,
ownership, hunger, fatigue, starvation, information boundaries and all priorities.
No BDI/GOAP, new institution, commitment rule or information channel was introduced.

The current forecast's **no-meal-credit assumption remains in the live arm**.
Adding predicted meals was not one of the six specified corrections. The six-hour
end margin and personal buffer remain. Offline comparison with preserved meal-aware
histories retains their existing meals and tick pauses; those are not new live arms.

Eight historical seed-0 replays match preserved state/event hashes, and their
instrumented/uninstrumented versions agree at all 73 checkpoints. All 30 live
controls match the historical control hashes. Every corrected live endpoint has the
same physical state hash as its paired control. Event hashes differ because the
logged forecast values/reasons change, despite identical selected actions.
All **454 pre-existing research files** are preserved; checksums are recorded.

The experiment follows [the registered scope](../../docs/FORECAST_COHERENCE.md):
static validation, unchanged historical replay, then the isolated live arm. The
same seeds 0–29 are used. These fixtures have no consequential seed variation;
30 pairs demonstrate reproducibility, not 30 independent statistical histories.

## 1. Did the known static inconsistencies disappear?

**Yes, in the covered cases: 540/540 composite/source-prefilter forecasts match kernel execution,
and 130/130 preserved primitive contracts still match.** Targeted tests cover the
original queue-order counterexample, self-membership, expiry during atomic gathering,
and unchanged risk margin/no-meal behavior. The full suite has 82 passing tests.

Corrections are limited to:

| Defect | Corrected mechanics |
|---|---|
| Movement calculated before waiting | Advance physiology through remembered waiting, then integrate movement |
| Phantom self-membership on later legs | Remove one currently witnessed own queue membership once from copied evidence |
| Aggregate gathering rounding | Execute independently timed one-quantum work, including threshold crossings |
| Work based on unused capacity | Acquire the minimum of free capacity, remembered availability and request demand |
| Gathering charged for loose cargo | Use the existing immediate loading action |
| Observed reservations ignored | Limit expected stock by the observed unreserved amount |

Food deadlines and ownership remain attached to the same goods. Spoilage during
work stops that acquisition at the original deadline. No hidden reservation or
future queue departure is inferred.

The static matrix contains 540 configured input combinations: 481 candidate action
sequences and 59 rejected source-prefilter predictions. **All 540 now receive kernel
comparisons.** Final review found that the rejected-source explanatory path still
used legacy one-quantum arithmetic; it was corrected and included before the final
full run. The first-pass evidence is preserved separately in
[coherence-first-pass](../coherence-first-pass/REPORT.md), and is superseded by this
expanded validation. No remaining discrepancy was observed in covered comparisons.
This is not universal proof of all possible forecasts or complete queue knowledge.

A research-harness issue found during validation is retained in
[development-findings.txt](development-findings.txt): its old conditional wait could
advance the world beyond focal death. The new endpoint comparator stops at death;
the original audit comparator is preserved. The live phase began only after static
validation passed.

## 2. Historical offline comparison

All comparisons use the **same personal inputs and copied RNG**, with historical
execution unchanged. Across the eight histories:

- 273,842 candidate comparisons could be matched by source and board.
- 124,824 change a predicted numerical result or failure state.
- 111,472 change the primary rejection gate.
- Of 176,894 matched physical classifications, **2,332 change**: 2,312 become
  feasible and 20 become infeasible.

These are repeated decision evaluations, not counts of agents or attempted jobs.
Primary-gate changes include exposing physical failure before a nutrition-margin
rejection; they must not be interpreted as 111,472 changed choices.

| Preserved history | Physical classifications changed | Primary gates changed | Intent proposals changed |
|---|---:|---:|---:|
| Reference | 0 | 14,912 | 0 |
| Current delivery | 0 | 30,364 | 0 |
| Known source | 0 | 30,628 | 0 |
| Meal-aware reactive | 788 | 9,572 | 72 |
| Committed diagnostic | 748 | 5,848 | 748* |
| Earlier self-count-corrected diagnostic | 0 | 13,764 | 0 |
| Contested requests | 796 | 6,384 | 1,079 |
| Stale depletion | No matched complete classification | 0 | 5,440 |

*The committed row compares the underlying reactive proposal. It does not assert
that the preserved commitment controller would select every different proposal.
Stale depletion gains 4,444 newly evaluated rejected candidates, excluded from the
matched-classification change count. Candidate coverage can change when corrected
route timing changes source prefiltering.

### Revisit of the exact earlier self-count subset

For the preserved meal-aware reactive history:

| Earlier finding | Result after all six corrections |
|---|---:|
| 7,236 affected candidate comparisons | All 7,236 remain numerically different |
| 2,028 changed primary gates | 1,752 of that exact subset remain different |
| 788 rejection-to-PASS changes | All 788 remain PASS |
| 72 different intent values | All 72 remain different |

The combined corrections produce **10,156** changed candidates and **9,572** gate
changes in that history overall. The subset intersection is documented in
[phase2/self-count-revisit.json](phase2/self-count-revisit.json). Comparing totals
alone would miss the 276 earlier gate changes superseded by other corrections.

## 3. Corrected mechanical accuracy under frozen beliefs

Across historical corrected candidate sequences, including terminal failure
prefixes, the independent interpreter finds:

| Conditional classification | Count |
|---|---:|
| Correct feasible | 27,858 |
| Correct infeasible | 168,562 |
| False feasible | 0 |
| False infeasible | 0 |

Another 81,866 candidates are prefiltered/unresolved and are not counted as executed
sequences. This denominator differs from the previous audit's complete-delivery-tape
matrix. No exhaustive search over alternative plans is claimed.

All **371 sampled historical kernel witnesses** agree with corrected elapsed time,
nutrition, fatigue, starvation and delivery quantity. Both live seed-0 histories
add 57 exact kernel comparisons each. No sampled kernel mismatch or independent
terminal-state mismatch remains.

For each live arm, conditional candidates classify as 7,560 correct-feasible and
31,488 correct-infeasible, with zero false-feasible/false-infeasible. Every one of
the 7,560 physically executable candidates still fails the six-hour margin.

These results condition on the agent's remembered stock and estimated waiting.
They do not imply that the real world will supply that stock or that waiting time.

## 4. Live paired decision funnel

Thirty pairs, seeds 0–29. Every seed has the following values; quantities are per
run, not multiplied by thirty.

| Metric | Existing forecast | Corrected forecast |
|---|---:|---:|
| Unique request observers | 50 | 50 |
| Physical request observation events | 55,256 | 55,256 |
| Acceptance/forecast evaluations | 58,760 | 58,760 |
| Primary NO_KNOWN_SOURCE | 19,712 | 19,712 |
| Primary SOURCE_PREDICTED_SPOILED | 0 | 30,364 |
| Primary CANNOT_COMPLETE_PLAN_ALIVE | 2,696 | 1,124 |
| Primary POST_PLAN_NUTRITION_MARGIN | 36,352 | 7,560 |
| Forecast PASS | 0 | 0 |
| Board-directed travel intents | 0 | 0 |
| Board-directed queue joins | 0 | 0 |
| Board-directed X arrivals | 0 | 0 |
| Return arrivals at C to claim | 0 | 0 |
| Accepted communal jobs | 0 | 0 |
| F reached under accepted jobs | 0 | 0 |
| Communal cargo acquisition | 0 | 0 |
| Communal return attempts/completions | 0 | 0 |
| Completed deliveries / delivered quanta | 0 / 0 | 0 / 0 |

All fifty agents initially stand at the board; that initial presence is not a
successful return to claim after discovering food. X-arrival counts in this table
are board-directed arrivals, not ordinary outward migration.

The entire live comparison contains **4,853,640 matched decision points**
(161,788 per seed), **zero differing selected intents**, and zero unmatched decision
points. There are 41,910 changed reason strings per seed on identical inputs.

The old primary margin label often masked another failure already present in its
violations list. Exposing spoilage explicitly does not imply that 30,364 newly
rejected formerly accepted plans existed: the old arm also had zero PASS.

## 5. Survival, starvation and migration

| Metric | Existing | Corrected |
|---|---:|---:|
| Survivors at 36h | 28 | 28 |
| Survivors at 48h | 28 | 28 |
| Survivors at 72h | 28 | 28 |
| Deaths | 22 | 22 |
| Starvation-person-hours | 944 | 944 |
| Food consumed, quanta | 1,264 | 1,264 |
| Food spoiled, quanta | 1,736 | 1,736 |
| Unique agents physically reaching F | 28 | 28 |
| Living agents at F at 72h | 28 | 28 |
| Living agents at C at 72h | 0 | 0 |
| Accounting error / FIFO overtakes | 0 / 0 | 0 / 0 |

Survival is still not adequate nourishment. Starvation reset mechanics and the
previous severe undernourishment result are unchanged. Migration remains valid.

## 6. Mechanical error versus queue information versus world changes

Corrected conditional arithmetic matches the tested mechanics, but **actual travel
error remains**. The control and corrected arms produce byte-identical queue-episode
evidence and identical physical outcomes.

| Comparison | n | Mean signed error | Range |
|---|---:|---:|---:|
| Belief shadow total travel minus actual, hours | 60 arrivals | -3.870815 | -22.08 to 0 |
| Belief queue wait minus realized wait, hours | 68 admitted episodes | -2.823268 | -16.08 to 0 |
| Arrival nutrition prediction minus actual, hours | 60 | +2.792593 | 0 to +10 |
| Arrival fatigue prediction minus actual, hours | 60 | -3.597926 | -18 to 0 |

The separate comparison with frozen-current-demand kernel forks still exposes
missing queue information. Later divergence between that fork and real execution
records world changes after prediction. The complete distributions, sample counts
and no-admission censoring are in [summary.json](summary.json). Those forks are
conditional controls, not exact future admission oracles.

The accepted previous episode attribution remains applicable to these identical
physical episodes: among 60 completed arrivals, 32 match, four have a pre-existing
information mismatch, and 24 differ through later world changes. Another 6,620
queue episodes encounter critical physiology/death with current-demand capacity
still withholding admission. These repeated episodes are not additive shares of
deaths or delivery failure.

The earlier committed-arm example—roughly zero believed waiting versus 28h in a
current-demand fork—remains preserved. It is **not** a newly observed C-bound attempt
in this live arm: the corrected current policy never initiates one.

## 7. First dominant break and remaining causes

The initial upstream problem remains **request known → source known**: food is
unknown at C and discovered only through travel. Among evaluated candidates, 33.55%
are rejected for no known source.

After source discovery, the decisive persistent break is
**plan considered → plan judged acceptable**. No evaluation passes. Corrected
primary rejections divide into 51.67% spoilage, 12.87% end-nutrition margin and
1.91% predicted inability to finish alive, in addition to the no-source group.
These percentages describe evaluations, not independent people or causal deaths.

A concrete legal-belief example occurs for agent 0 at t=4.083333h at F. It remembers
the request at C and has about 12.9167 nutrition-hours. The modeled sequence is
F→C to claim, C→F to acquire ten quanta, F→C to deliver. Conditional completion takes
20.392228h and leaves zero nutrition with about 7.47556h continuous starvation.
It predicts arrival alive but fails the unchanged six-hour margin. Its observation,
memory and both forecasts are in [margin-example.json](margin-example.json).

Remaining explanations must stay separate:

- **Unavailable information:** unknown F initially; stale resource/request/queue
  memories and unseen opposite-bank traffic remain. No new information is granted.
- **Changing world state:** later arrivals and competing consumers still differ
  from the static hypothesis. Mechanical correctness does not remove this.
- **Preferences:** the six-hour margin rejects all 7,560 conditionally executable
  live candidates; existing survival, sleep and self-provision priorities remain.
- **Institutions:** physical board claiming requires the extra journey. No remote
  claim is permitted. Stale claims cannot explain this live result because zero
  jobs are accepted.
- **Capacity competition:** migration still produces physical waiting and survival
  loss. It cannot directly explain a failed C-bound execution here, because no
  C-bound provisioning attempt starts.

The no-meal forecast remains an explicit planning assumption rather than a claim
that the kernel forbids eating. Its isolated effect was investigated previously;
it was not silently combined with this mechanics-only live intervention.

## Outcome, limitations and reproduction

The evidence supports result C for the required live fixture: real forecast defects
were corrected, but they were not central to its selected-action failure. Their
behavioral significance in other preserved histories is visible offline and must
not be generalized into successful live provisioning.

**No default-policy adoption. No communal provisioning validation. No next NPC
architecture or institution selected.** All negative findings remain intact.

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --bin coherence_probe
python3 scripts/run_coherence.py NEW_DIRECTORY 30
```

The runner refuses to overwrite evidence, gates phases on static conformance,
checks historical replay hashes, and retains compressed decision/forecast traces.
The runner completed all phases again after extending static validation. The earlier
clean one-seed reproduction and the final expanded run are distinguished in the
provenance; retained first-pass evidence is not silently overwritten.
The final run reproduces **106 shared data files byte-for-byte** from the first pass;
the only changed shared data file is the static table expanded from 481 to 540 rows.
The report's counts, raw gate tables and kernel witnesses are reproduced by
`scripts/analyze_coherence.py`. Checksums and source revision are recorded alongside
this report. Exactness is limited to tested inputs and stipulated beliefs; it is
not omniscience, optimal planning, or proof that every possible model defect is gone.
