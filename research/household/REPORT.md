# B1 — Household and kinship: PASSED at registered scope

Households and directed care obligations now affect the existing Person's choices without pooling property. A willing caregiver can transfer real personally owned food to a dependent at a physical meeting place. The recipient must then eat normally. Care may fail, displace other activity, or reduce the caregiver's nourishment. No family emotion, reproduction, inheritance, compulsory sharing or military behavior was introduced.

## Scope, assumptions and provenance

- Owner-authorized institution: individual property, explicit directed Caregiver relations. Kinship and care are separate. Parent/Child have directed inverse views; Partner/Sibling are symmetric. Household records have IDs, members and optional home association, no inventory or controller. This minimal representation allows one seeded household membership per person; kinship/care can cross households. It is not a land/home entitlement or marriage rule.
- Registered rules: [specification](../../docs/HOUSEHOLD_EXPERIMENT.md); implementation remains in the [common Person architecture](../../docs/PERSON_ARCHITECTURE.md). Exact local physiology observation, severe need ≤6h, maximum memory age12h, caregiver forecast arrival reserve ≥6h, and gifts ≤6 quanta toward an observed12h need are provisional experimental choices. They are not universal psychology.
- Known-distant fixtures use explicit counterfactual initial person memories timestamped t=0; they do not replay a physically reconstructed prehistory of the meeting. They test decisions conditional on that initial belief, not how likely an agent is to acquire it naturally. Runtime observations and the unknown-need control remain strictly local.
- Existing self-eating, critical hunger, sleep, atomic gathering and accepted-job priorities remain. There is no new care-specific search for food, no household pooling, no remote location/death updates, no synchronization of relatives' travel. Recipient-owned visible local food reduces apparent unmet need; the caregiver cannot inspect a hidden backpack.
- Baseline B0: `df3ed3b`. Preregistration `cb2a290`. Care implementation `356fadd`. Final experiment source **`79515eeeb911c014df30d3626b9dc212fe2f0412`**, with executable SHA256 in [matrix provenance](matrix-verified/summary.json).
- Eleven configurations × two arms ×30 paired seeds0–29 = **660 final runs /330 pairs**, each48h. No claim of indefinite sustainability. Within each arm the reported aggregate outcomes were identical across seeds; these are reproducibility checks across seeds, not evidence of statistical generality over varied populations/topologies.
- `matrix/` preserves the initial event-header error and original physical results; `matrix-final/` preserves its corrected-writer replay; `matrix-verified/` is authoritative after the observer correction. Replays are not additional independent experiments. All154 final per-fixture files match the original after normalizing only its documented event header. [Testing/provenance notes](TESTING_NOTES.md) retain both discovered errors.

## Physical outcomes

Entries are per-run values (also the30-seed means). Off retains the same family structure but disables the care motive; the nonkin fixture separately removes kinship/obligations. All accounting errors and FIFO overtake counts were zero.

| Fixture | Care food transferred, off → on | Care travel h (on) | Starvation-person-h, off → on | Alive at 48h, off → on |
|---|---:|---:|---:|---:|
| colocated | 0 → 6 | 0.000 | 48.000 → 48.017 | 0 → 0 |
| self_conflict | 0 → 0 | 0.000 | 48.000 → 48.000 | 0 → 0 |
| multiple | 0 → 6 | 0.000 | 72.000 → 72.017 | 0 → 0 |
| near_known | 0 → 0 | 1.000 | 48.000 → 48.000 | 0 → 0 |
| far_known | 0 → 0 | 10.222 | 48.000 → 48.000 | 0 → 0 |
| unknown | 0 → 6 | 0.000 | 48.000 → 50.017 | 0 → 0 |
| caregiver_death | 0 → 0 | 0.000 | 25.000 → 25.000 | 0 → 0 |
| nonkin | 0 → 0 | 0.000 | 48.000 → 48.000 | 0 → 0 |
| farming | 0 → 10 | 0.000 | 24.000 → 36.417 | 1 → 1 |
| farming_distant | 0 → 0 | 1.000 | 24.000 → 24.000 | 1 → 1 |
| construction | 0 → 6 | 0.000 | 33.995 → 30.383 | 50 → 50 |

Raw checkpoint metrics, people, retained personal food, activity hours and reason counts are in [matrix-verified](matrix-verified). Paired differences and ranges are in [analysis](analysis/paired.json). `observed_opportunities` counts observation/evaluation occasions, not unique people or independent emergencies. Successful transfers are separately measured from eating. Gate rejections are not counted as failed physical transfers.

## Causal findings

**Care changes allocation through sacrifice.** Colocated, the caregiver gives6 quanta at t=0; the dependent consumes them at the next decision, not instantaneously. With care disabled the caregiver consumes all6; with care enabled it consumes none. Caregiver death moves from48h to42h. Dependent death moves from24h to30h1min. Both ultimately die. The extra minute of starvation is the real delay before consumption and the unchanged continuous-starvation reset; no biological rule was changed.

**Self-survival is not overridden by perfect altruism.** Starting the caregiver at4h nutrition causes it to eat its six quanta under the existing policy. No care goal or transfer follows. With two starving dependents and only six quanta, the first deterministic tie winner receives all6; the other receives none. This is inspectable nonoptimal allocation, not a worker/food quota or implicit shared inventory. Nonkin and care-disabled controls transfer zero.

**Known need can cause an unsuccessful trip.** The near-known caregiver leaves for the last observed node while the dependent independently explores in the opposite direction. They pass on the edge, where no new social observation or transfer is allowed. At1h the caregiver reaches the remembered location, observes absence, and cancels. The10h nominal-distance case takes10.222222h after fatigue impairment and also finds absence. No new location is supplied. These are measured care-associated travel costs, not claims that total48h travel necessarily rises: later exploration also differs between arms.

**Unknown need does not cause remote intervention.** There is no care action before the dependent physically arrives at the caregiver's node at2h. The caregiver then observes hunger and transfers6 quanta. This succeeds where the prior-information journey misses the dependent. It does not establish that ignorance is generally beneficial: it identifies the causal interaction between local knowledge, independent movement and meeting timing. Unit tests also hold the caregiver's personal history fixed while changing a remote dependent's hunger/death and verify identical choices.

**Helping does not guarantee adequate nourishment.** With farming, care transfers10 quanta over5 actions; production stays48. Total consumption rises47→54, but starvation exposure rises24→36.416667 person-hours. The caregiver incurs3.683333h at zero nutrition; the dependent consumes10 and dies at42.733333h instead of24h. Its cumulative starvation reaches32.733333h because later meals reset continuous starvation. Increased exposure partly reflects living longer while underfed; it is not by itself a total-welfare ranking. One person survives48h in both arms. This preserves the known distinction between being alive and being adequately nourished.

**Care has measurable activity costs and indirect effects.** In distant farming, the care trip uses1h that the control spends idle during the first6h; early farming labor remains1h in both arms. Across48h caregiver farming changes6.047917→5.175h and consumption50→47, with equal48-quanta output and no successful gift. These later differences include changed location and sleep, not just the direct trip.

In the construction fixture, by6h the caregiver's construction labor changes4.483333→0.966667h, queue time rises1.733333h, and idle time rises1.783333h. Other workers compensate: aggregate construction is down only1min at6h and is identical by48h. Both crossings finish at9.703704h. Thus the3.5h reduction in this person's eventual construction is largely redistribution across people, not3.5h less infrastructure work. The care action changes entry timing into the existing FIFO network; it gets no priority.

Construction-world consumption changes2404→2408, starvation33.995336→30.382967h, and all50 survive48h in both arms. The six gifted quanta remain at C after the dependent leaves; they are not consumed until37.794444h. Earlier nourishment comes from other physically acquired food. Do not attribute all aggregate changes to immediate feeding: transfer completion is not consumption completion.

**Death removes a structural source of support without redistribution.** The death fixture records the caregiver dying at1h and a living dependent losing that directed obligation; no replacement is appointed, no property is inherited, and household history persists. This fixture begins without food, so it establishes structural loss, not the quantity of otherwise feasible future care prevented. A separate material test confirms that the dead owner's property remains theirs in loose custody. Simultaneous caregiver/dependent deaths do not count as a surviving dependent losing support.

[Exact focal traces](TRACE_EXCERPTS.md) include transfers, delayed eating, absent-target cancellations, own-survival decisions and death. Full traces remain preserved, including negative outcomes.

## Verification and gate

- **210 tests pass**, including18 new B1 tests: directionality; individual property; real transfer then separate consumption; age conservation; locality/hidden death; critical self-survival; stale and infeasible travel; absent-target observation; no mid-edge gift; queue cancellation; multiple-dependent scarcity; death/no inheritance; pause/resume; deterministic replay and partition invariance; observer independence; simultaneous-death accounting.
- Full test log: [tests-verified.log](tests-verified.log). Formatting and all-target clippy with warnings denied pass.
- **227 historical executions** reproduce exactly:40 cross-domain scenarios plus the full187-run F10 experiment. Event/state hashes and output metrics remain identical. Only `household: None` in configuration debug text is normalized. See [regression validation](regressions/validation.json). The final observer fix is household-only; final B1 replays match every previous metric/event row.
- **4,410 prior research files unchanged**, verified by SHA256. [Final analysis validation](analysis/validation.json).
- Persistent kinship/membership survives separation/death; identity, ownership, locality, physical goods, FIFO and authoritative time remain distinct. Care acts through the common Person evaluator and the existing action executor.
- Both successful and failed care occur, with measured food/time sacrifices and effects on other activity. No pooling, inheritance, compulsory sharing or marriage-property decision was needed.

**B1 PASSED at the seeded structural-care scope.** This does not validate realistic family psychology, optimal caregiving, strong goal commitment, endogenous households, childhood physiology or welfare improvement. B2 and B3 remain unimplemented; F11 remains blocked.

## Reproduction

Use fresh output directories; writers refuse to overwrite result files. Run from repository root:

```sh
cargo test --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_household.py /tmp/won-b1-new-matrix
python3 scripts/analyze_household.py /tmp/won-b1-new-matrix /tmp/won-b1-new-analysis
python3 scripts/verify_household.py /tmp/won-b1-new-regressions
```

`analyze_household.py` compares against the preserved initial matrix, accounting explicitly for its header error. Research helpers can seed documented initial observations and endowments; they are never imported by autonomous selection. Observer-only metrics do not feed choices.
