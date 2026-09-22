# Shadow forecast/action-conformance audit

**The independent primitive model matches the kernel in the tested cases. The
existing composite forecast is not fully coherent: queue ordering, self-counting,
and acquisition assumptions produce measurable discrepancies.** These errors
coexist with missing information, changing traffic, institutional constraints and
explicit survival preferences. No NPC architecture, default policy, physical
parameter, ownership rule, FIFO rule, or institution was changed.

## Evidence and preservation

Eight **preserved** histories were reconstructed: reference, current delivery
fixture, known-source control, meal-aware reactive diagnostic, committed diagnostic,
previously corrected-forecast diagnostic, contested requests, and stale depletion.
These are existing experiments, not new policy interventions.

There are 240 audit-on/audit-off pairs (seeds 0–29), with matching state and event
hashes at all 73 checkpoints per pair and matching historical endpoint hashes.
All **297 pre-existing research files** retain their SHA-256 hashes. Additional
seed-0 replays broaden kernel validation and retain complete decision snapshots;
they also match the preserved hashes. Thirty repeated seeds are reproducibility
checks, not thirty statistically independent histories: these fixtures have no
consequential seed variation within an arm.

Full seed-0 observation, memory, selected-decision and candidate records are in
`*-captures.tsv.gz`. The independent interpreter receives only those personal
inputs. True-world comparisons run on separate kernel copies and never return
information or commands to the autonomous policy. See the
[registered audit and action contracts](../../docs/SHADOW_AUDIT.md).

We separate:

- **A0:** the existing policy's forecast.
- **A:** independent execution of the candidate under its remembered assumptions,
  including its stipulated queue-delay estimates.
- **B:** authoritative kernel witnesses. Belief fixtures validate A; separate
  real-state forks retain existing traffic/actions but permit no new outside
  commands. Their results are conditional, not a global physical optimum.
- **C:** actual historical execution, including interruption and changing traffic.

The summary data are in [summary.json](summary.json), full distributions in
[error-distributions.csv](error-distributions.csv), and memory bins in
[memory-age.csv](memory-age.csv). Units are integer milliseconds or integer quanta.
Errors are prediction minus comparator. Cancelled waits are censored, not completed
trips; unexecuted delivery plans do not receive an invented actual completion time.

## 1. Primitive conformance

**130/130 static contract cases matched exactly: 100%.** They cover hunger/fatigue
thresholds in travel and gathering, sleep ticks, integer eating, personal versus
communal ceilings, claim/acceptance, reservation cancellation, communal gathering,
loading, delivery, pickup, dead-owner exclusion, waste dropping, lease expiry and
original food deadlines through splitting/transfer.

Historical completed eating/gathering/sleep checks total **258,300**, all exact for
the measured elapsed time, nutrition, fatigue and carried quantity. Interrupted
sleep is excluded from that completion denominator. Movement also matches the
independent physiology calculation when conditioned on the **realized** waiting
time. This does not mean its waiting time was predictable.

The independent full-plan interpreter matches **395/395** sampled conditional
kernel witnesses in elapsed time, nutrition, fatigue and delivered quanta. These
are bounded, documented samples, not exhaustive verification of every world state.

## 2. Identified model inconsistencies

### Waiting is applied after movement in the original route estimator

The original estimator calculates movement impairment first, then adds queue time.
The kernel waits first; hunger/fatigue accumulated while waiting can slow movement.

Controlled example: nominal traversal 2h, known wait 2h, initial nutrition 8h,
fatigue 10h. Existing forecast: **4h**. Independent shadow and kernel:
**4h 46m 40s**. Arrival nutrition is overestimated by the same 46m 40s.
A separate deterministic fixture demonstrates a forecast of arrival alive while
the kernel kills the agent on the edge. No missing queue information is required
for this discrepancy.

Historical consequence: **1,260 contested-request candidate evaluations** predict
edible delivery but reach the destination after spoilage under the same assumed
queue delays. All are independently blocked by the existing nutrition margin;
none becomes an accepted job merely because the physical forecast is optimistic.

Concrete trace: t=24.1h, agent 12, candidate 1, target board X. The existing
forecast ends at **43.971111h** with four quanta. Independent execution and the
belief-conditioned kernel end at **48.8h**, after the original 48h deadline, with
zero edible delivery. The oracle row is in
[contested-request kernel validation](contested_requests-validation-kernel_plans.csv).

### The agent is counted again on hypothetical future traversals

Current-leg self-exclusion can be lost when the forecast changes the simulated
actor to Idle but reuses the remembered queue containing that same agent.
Affected matched traversals gain **exactly 2h** of phantom delay in these fixtures.
The affected legs and timestamps are retained in `*-self_count.csv.gz`.

For the preserved meal-aware reactive history:

- 7,236 candidate comparisons change.
- 2,028 primary gate classifications change.
- 788 rejected evaluations become PASS in the corrected shadow.
- 72 selected intent values would differ in the reactive policy comparison.
- 756 old zero-delivery forecasts have positive conditional kernel witnesses.

The committed history adds 748 such witnesses; probing the old estimator on the
preserved corrected-policy history adds 988. All **2,492** witnesses deliver the
corrected predicted quantity at the corrected predicted time in belief-conditioned
kernel execution. They demonstrate false-zero/false-infeasible forecasts under
those assumptions. They do **not** guarantee delivery against hidden real traffic.

On the corrected-policy history, this is a counterfactual probe of the **old**
estimator, not evidence that the corrected estimator still double-counts.
For committed policies, changed reactive proposals are not automatically changed
final committed decisions. No live decision changed during this audit.

The unchanged reference/current/known-source histories expose zero affected
self-count comparisons at their evaluated acceptance gates. The contested history
has 8,448 affected comparisons, 2,920 gate changes and 144 alternative intent values.

### Gathering is rounded as aggregate work rather than atomic quanta

The legacy forecast rounds a batch once and holds its starting performance rate
across that work; the kernel finishes separately timed one-quantum actions.
At 75% performance, ten quanta are predicted to take 26,667ms but actually take
26,670ms. In executable current-control tapes the measured elapsed discrepancy
ranges from 0 to -15ms. This rounding discrepancy is real but much smaller than
queue errors. Threshold crossings are covered by the independent interpreter;
these observations do not establish rounding as a provisioning bottleneck.

No demonstrated conformance error was found in the tested eating, ownership
transfer, food-age preservation, spoilage deadline or sleep-recovery mechanics.

### Acquisition amount, holder and reservations are not fully modeled

Three additional deterministic contract fixtures identify discrepancies in the
legacy forecast. These fixtures vary remembered action inputs, not the historical
reference world; their contribution to historical provisioning failure is not
established.

- With one quantum of request demand and ten free carrying slots, the forecast
  charges work for ten quanta. Predicted plan duration is 30,827,778ms versus
  30,802,778ms in the kernel; both deliver one quantum.
- With legally loadable loose cargo, the forecast still charges gathering work.
  Predicted duration is 30,827,778ms versus 30,800,000ms in the kernel; both deliver
  ten quanta. Loading is instantaneous under the existing action contract.
- With remembered quantity one but remembered **available quantity zero**, the
  forecast predicts one delivered quantum. The independent model rejects source
  availability and the belief-conditioned kernel refuses gathering. This is
  ignoring already-observed reservations, not an unknown future competitor.

These are separately tested counterexamples, not extra observations added to the
historical confusion matrices or the 130-case primitive denominator. Exact test
output is preserved in [forecast-contract-tests.txt](forecast-contract-tests.txt).

## 3. False feasibility: keep the comparators separate

Across seed-0 histories there are 273,842 communal candidate records. Of these,
139,898 contain complete action tapes suitable for physical classification.
Partial/prefiltered candidates remain unresolved, not automatically infeasible.
The physical matrix excludes the six-hour behavioral margin:

| Existing forecast versus independent execution under the SAME belief delays | Count |
|---|---:|
| Correct feasible | 24,306 |
| Correct infeasible | 114,332 |
| FALSE_FEASIBLE | 1,260 |
| FALSE_INFEASIBLE | 0 |

The 1,260 false-feasible cases are 0.90% of all complete tapes, or 8.91% of the
contested fixture's 14,148 complete tapes. Zero false-infeasible cases **in this
same-tape comparison** is not a finding that all rejection is justified: the
separate 2,492 normalized-self-membership witnesses above establish false-zero
predictions caused by faulty queue assumptions.

Against **395 sampled real-state, frozen-outside-command kernel forks**, the matrix
is instead: 3 correct-feasible, 307 correct-infeasible, 83 false-feasible and
2 false-infeasible. This includes epistemic error and capacity competition;
it is not a count of 85 arithmetic defects. The two false-infeasible witnesses
are contested-request plans that the real-state fork can deliver while remembered
queue assumptions predict no edible delivery. These are conditional witnesses,
not proof that the autonomous continuation would execute them.

No exhaustive search over alternative plans was performed. These counts must not
be presented as a complete test of every possible way to provision an agent.

The audit also probes known-node routes at captured decision states, independently
of whether the policy selected that route. Across **1,659,032 nonempty route
probes**, comparison with independent physiology under the same queue assumptions
gives 1,400,487 correct-feasible, 212,359 correct-infeasible, **46,186 false-feasible**
and zero false-infeasible predictions. The false-feasible route probes occur in
known-source (23,382) and stale-depletion (22,804) histories. They are repeated
hypothetical evaluations, not 46,186 attempted journeys, deaths or delivery plans.
They are reported separately from the complete-plan matrices above. The controlled
arrival-alive counterexample confirms that this class of timing discrepancy can
cross a real kernel death boundary.

## 4. Error distributions and information age

For the current delivery fixture, the independent belief shadow versus actual
completed movement yields:

| Measurement | n | Median | Mean signed error | Range |
|---|---:|---:|---:|---:|
| Travel including wait, hours | 60 | 0 | -3.870815 | -22.08 to 0 |
| Queue wait, admitted episodes, hours | 68 | 0 | -2.823268 | -16.08 to 0 |
| Arrival nutrition, hours | 60 | 0 | +2.792593 | 0 to +10 |
| Arrival fatigue, hours | 60 | 0 | -3.597926 | -18 to 0 |

The queue denominator includes agents admitted before subsequently dying, whereas
the arrival denominator requires completion. Neither includes cancelled waits as
though they were completed admissions. `policy_*` error columns separately retain
the original route estimator's prediction, so it is not silently replaced by A.

Completed primitive cargo changes have zero error. Counterfactual contested
spoilage can instead produce a four-quantum edible-delivery overprediction.
Actual full-plan delivery-error distributions are **not available**: no historical
autonomous delivery completed. Food deadline/remaining-life error is zero wherever
age metadata is comparable. Physical expiry uncertainty must not be confused with
uncertainty that food still exists when the agent arrives.

Selected mean absolute memory errors, ordered by age bins
[0–2h, 2–6h, 6–12h, 12–24h, 24h+]:

- Current-control same-bank queue count: **0, 3.47, 5.25, 7.33, 10.59 agents**.
- Known-source-control remembered food quantity: **0, 9.17, 152.22, 457.95,
  848.68 quanta**.
- Reference request quantity: **0, 223.10, 199.17, 170.33, 152.29 quanta**.
- Food remaining-life/deadline error: **0 in every populated bin**.
- The fixed 600-quanta current-control request remains exact at every age.

Thus request error is not monotonic with age, and freshness metadata behaves very
differently from quantity. Empty bins are not reported as evidence of zero error.
The stale-depletion history retains its original **28 failed stale-food trips**.

## 5. Causal classification without one misleading global percentage

Final episode attribution is in `*-episode-classifications.csv`; it uses captured
physiology and distinguishes a predicted death from a prediction mismatch. The
online `category` in raw episode logs is retained as `online_category` for audit,
not used as the final causal total. Four current-control deaths match the predicted
physical failure and are not mislabeled as preference rejections.

These are different observational units. Counts are repeated evaluations/episodes,
not independent agents or additive fractions of civilization-wide failure.

| Primary mechanism | Measured evidence |
|---|---|
| Physical-model inconsistency | Controlled queue-order, gather-rounding and acquisition counterexamples; 1,260 false-edible plans; 2,492 conditional self-count false-zero witnesses |
| Unavailable/stale information | Four of the current control's 60 completed edge episodes have a pre-existing information mismatch after current-demand replay; stale-depletion retains 28 failed source visits |
| World changed after prediction | 24 of those 60 completed episodes and 54 interrupted queue episodes differ from current-demand replay; the 24 completed trips are not failed deliveries |
| Risk/commitment preference | 7,560 current-control belief-executable candidate tapes fail the six-hour margin |
| Institutional restriction | Fourteen of 83 contested real-state kernel samples refuse claims because proposed quantity exceeds unclaimed demand; ownership/ceiling/lease contracts are separately tested |
| Shared-capacity competition | 6,620 current-control queue episodes reach a critical threshold/death while the current-demand replay still withholds admission; all eight committed X→C attempts exhibit the capacity/survival-override interaction |

For the 7,560 preference rejections, remembered assumptions are held fixed; this
is not a claim that 7,560 deliveries would succeed in the real congested world.
There is a direct real-state example at t=0 in the known-source control: the kernel
can deliver ten quanta in 8.563272h without meals, leaving less than six hours of
nutrition. The policy's margin rejects it. That is a preference, not a physical
impossibility or automatically a bug.

The fourteen claim refusals demonstrate protocol blocking at the claim step, not
that all fourteen full delivery plans would succeed if the rule were waived.
No dead-property exclusion is observed as the primary unmet-need category in these
histories; the static fixture verifies that exclusion exists. Do not claim it
caused the reference result. Ration rules and board locality remain unchanged.

The unmet-need journal also records UNKNOWN, KNOWN_BUT_TOO_FAR, CAPACITY_BLOCKED,
PHYSICALLY_UNAVAILABLE, SPOILED, and ACCESS_IN_PROGRESS where supported. It leaves
UNRESOLVED cases rather than inventing a proof of feasible access or voluntary
refusal. In the current control there are 56,206 UNKNOWN, 40,404 SPOILED, 776
ACCESS_IN_PROGRESS and 908 UNRESOLVED observations below 12h nutrition. These
need-state counts are not a causal partition of deaths.

## 6. Focal board-bound trace

For committed agents 0–3 arriving at X at t=8h, the point forecast gives zero
queue delay. A current-demand kernel fork gives **28h** before admission. Actual
agents wait **9h**, then leave at the existing critical-hunger threshold; they
never complete X→C.

For agents 4–7 arriving at t=13.2h, the point forecast gives **2h** queue delay;
the frozen-demand fork gives **30.8h**. Actual queue residence is **4.8h** before
critical fatigue interrupts it. The preserved intention trace subsequently reaches
the hunger override. Queue interruption and eventual intention cancellation are
different events.

The observer sees applicants on its own bank. Opposite-bank demand and in-flight
occupancy are not supplied; `occupied=0` in the observation is not a reliable
assertion that the shared edge is empty. This is missing information, separate from
the proven ordering defect. The previous recorded-background replay, which allows
others' actual cancellations, has shorter delays than the frozen-demand fork.
Neither is an exact future queue oracle available to the NPC.

## Outcome and limits

Every existing autonomous outcome is preserved, including zero delivered communal
quanta. The contested fixture still accepts and cancels eight jobs; the reference
and delivery-control acceptance failures are unchanged. Survival remains distinct
from adequate nourishment; starvation-reset mechanics and starvation-person-hours
are unchanged.

This audit establishes concrete forecasting defects and measures information and
capacity effects separately. It does not show that correcting forecasts alone
would create provisioning, justify BDI/GOAP, or select a new commitment rule.
**No default correction or architectural change is adopted.**

## Reproduction and validation

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --bin shadow_probe
python3 scripts/run_shadow.py /tmp/won-shadow-reproduction 30
```

Use a new output directory. The runner preserves prior research, performs paired
hash checks and retains compressed seed-0 evidence. Analysis uses exact integer
errors, stratified kernel samples, and explicit censoring. There are **77 passing
Rust tests** (65 library plus 12 integration tests), with formatting and Clippy clean.
An additional clean reference replay reproduced all 13 compared seed-0 evidence
files byte-for-byte, including full belief captures and final episode attribution.
SHA-256 provenance and replay commands are recorded alongside this report.
