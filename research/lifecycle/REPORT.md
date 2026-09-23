# B3 — embodied life cycle and reproduction

**PASSED at bounded mechanism scope.** Persons can be born, acquire persistent parentage and care obligations, remain dependent, mature without identity replacement, and produce a third generation through mutually accepted reproduction. This establishes a generational mechanism, **not autonomous demographic sustainability**.

Core source: `97b2f33408bc68733de74c52e1619f87fbd56c21`. Final research/reporting source: `9af6d34d8897f668706638d719851751dbf1c22f`. Baseline B2: `fdbc037`. [Registered assumptions](../../docs/LIFECYCLE_EXPERIMENT.md), [analysis](analysis/analysis.json), [focal chains](analysis/TRACE_EXCERPTS.md), [regressions](regressions/validation.json). Earlier negative results remain unchanged.

## Implemented rules

Biology belongs to the existing embodied Person ID. Birth time determines age on the one authoritative clock; a one-time maturation marker records the capability transition. No replacement Person, new brain or erased memory at adulthood. There are two simplified reproductive roles, used for compatibility and pregnancy only. Sex does not enter care, work, psychology, risk, relationship appraisal or general policy randomness.

The accelerated defaults are:

| Parameter | Value |
|---|---:|
| Childhood |24 simulated hours|
| Reproductive window |Age24h through strictly before96h|
| Mutual reproductive action |1h|
| Interval between started undertakings |32h, for both participants|
| Gestation |8h|
| Conception probability |25% per completed eligible undertaking|
| Birth sex |Seeded50/50|
| Newborn nutrition / food |0 /0|

These are research intervals, not human demographic calibration. Children retain the existing1 nutrition-hour/hour metabolism, sleep, fatigue and24h continuous-starvation threshold. No lactation, genetics, senescence mortality or extra pregnancy metabolism is modeled. Existing starvation mortality remains authoritative.

Each participant independently chooses a visibly present adult opposite-role Partner. Own drive/readiness, current needs and existing activity determine willingness. Both consents must occur in the same decision batch/time. One consent, an absent partner, a declining partner or a child cannot create pregnancy. Consent is not a remotely redeemable promise. The completed joint action creates only a probabilistic conception trial, using an independently mixed per-person biological RNG, separate from behavioral randomness.

Reproduction is an explicit physical action with an elapsed-time ledger, not idle time. Existing eating, critical survival, sleep and care arbitration precede it. Starting requires idle/no job, at least12h nutrition, fatigue below16h and interval elapsed; continuing can be interrupted by another actual action. It creates no workers, food or ownership rights. The registered priority is a behavioral hypothesis, not a demographic optimization.

Pregnancy remains with the gestating person. Death ends it. A due delivery during atomic edge traversal waits until the next node; delay is recorded. This narrow **node-delivery convention** avoids inventing infant carrying or adding a newborn traversal that bypasses edge capacity. It is a limitation, not realistic obstetrics. Maternal death wins a coincident death/birth boundary.

Birth creates a new stable Person at that physical node, with no goods/reserve; extends all enabled capability memory stores; records both parents; and adds both structural caregiver obligations. The child joins the gestating parent's household. If that parent has none, a group for that parent and child is created; the other household is not merged. No property is transferred by birth or death.

Children may eat legally accessible food, sleep and wait through Person. They do not perform adult labor, trade, hold offices or reproduce. Direct commands are checked by the same kernel capability boundary; explicit physical travel remains possible. Maturation lifts the restriction without replacing identity or history.

New parent–child social strength starts0. Structural kinship/care does not remotely report hunger, birth observations or death. A local encounter creates the personal social record; B2's unchanged appraisal updates it through history. Remote parentage exists institutionally, but the absent parent receives no sighting, emotional update or automatic journey. A dead caregiver cannot act, no substitute is appointed, and historical kin/care records remain inspectable.

## Experiment accounting

Final matrix: **510 runs**,17 cases×30 paired seeds,60h each. `basic` and `dependency` intentionally duplicate the same configuration; they are not independent evidence. Most mechanism fixtures use100% conception to isolate eligibility/execution; `stochastic` uses the25% default, and no-birth controls use0%. No parameter was tuned toward population growth.

The preserved first450-run matrix is superseded only for a reporting/coverage repair. The age-only death counter mislabeled a death exactly at the adult threshold even though death correctly occurred before maturation. The final counter uses whether the maturation transition actually occurred. Two seeded-dependent controls were also added because the original multiple-family case did not isolate simultaneous competition. All original450 runs reproduce **identical authoritative state/event hashes and all substantive metrics except the corrected child-death count**. Runtime/serialization-size metadata are excluded from that comparison.

Pilots remain available, including failure to reach Generation3. They are development observations, not pooled with the final matrix. During pilot review, reproductive RNG initialization was separated from the existing policy stream to avoid shifted overlapping streams across people. Probabilities stayed25% and50%.

## Birth, consent, maturation and death

| Case,30 seeds | Total births | Maturation events per run | Generation3 runs | Interpretation |
|---|---:|---:|---:|---|
| Basic / dependency, each |60|1|0|Autonomous mutual consent and physical care support one maturation|
| Stochastic25% |12|Mean0.267|0|12 successes from60 completed trials;48 failures preserved|
| Zero-fertility control |0|0|0|Mutual undertakings do not guarantee conception|
| Separated partners |0|0|0|No reproductive travel or remote conception|
| One partner declines |0|0|0|No unilateral conception|
| Severe hunger |0|0|0|Survival pressure prevents undertakings; both adults die|
| B2 enabled, zero birth attachment |60|0|0|Care is refused in this configuration; first child dies|
| Two households |120|2|0|Multiple families, not pooled property|
| Less-assisted multigeneration |240|4|0|Descendants mature but do not reproduce|
| Same with B2 |240|4|0|Prescribed child support still needed; worse nourishment|
| Controlled generational support |147|4|27|Biological chain reaches Generation3 under explicitly controlled support|

A first basic birth occurs at9h after a1h joint action and8h pregnancy. Its Person ID matures at33h. Tests preserve its memory, RNG, consumption history, birth time and parentage across that boundary. Fourteen consumed quanta or a birthday do not guarantee future survival: in the working world a descendant matures and later starves.

One important boundary is preserved: in `dependency_social`, the first child remains unfed and dies at33h, exactly when it would otherwise mature. Death is processed first. It is therefore one **unmatured death**, not a successful maturation followed by an adult death. The later newborn remains alive but hungry at60h; survival at that endpoint is not adequate care.

## The generational proof and its limits

In the less-assisted multigeneration worlds, eight founders produce four initial descendants. At33h these mature. A research-only Partner link is seeded between unrelated opposite-role descendants if such a pair exists; this is explicit fixture input, not mate-search behavior. The descendants still accept no reproductive undertaking: nourishment maintained near the care ceiling, sleep and elapsed time keep them outside the12h readiness gate. **No gate was lowered.**

The separate `generational_control` uses the same initial people, food and physical laws. A research coordinator issues local support commands to living parents: pick up their existing personal food when necessary and give six quanta to a descendant below18h nutrition. It does not create goods, set nutrition, change expiry, teleport custody or override either reproductive consent. It does override ordinary care selection, which is why this is a physical-mechanism control rather than an autonomous-family success claim.

In seed0:

- Four Generation2 Persons are born at9h.
- Their same IDs mature at33h.
- The controlled unrelated Partner association is recorded at33h.
- Descendants8 and9 independently accept a joint undertaking; conception occurs at40h03m.
- Person12, Generation3, is born at48h03m.

Generation3 occurs in27/30 seeds. In seeds6,18,24, all four first descendants are female; there is no eligible unrelated descendant pair. The fixture does not alter sex, invent a partner or skip maturation. Initial food spoils at48h, just before the focal third-generation birth. Mean starvation exposure is10.822 person-hours and no sustainability claim follows.

The controlled arm transfers552 food quanta per run and can interrupt the founders' reproductive undertakings. Its mean4.9 births are fewer than the less-assisted arm's8. The change is not simply “more help → more population.” Research-directed care consumes existing resources and excludes other activity.

## Care, B2 and physical burden

The no-birth/basic comparison holds the initial pair, food and rights fixed. Both arms spend4 person-hours on reproductive actions. With births,48 quanta reach dependents, total consumption rises84→132 and spoilage falls328→280. New people add8.933 starvation-person-hours. The newborn has no food-generating privilege and contributes no adult labor during childhood.

B2 creates a consequential negative result. With zero initial parent→child attachment and unchanged six-quanta care scoring, severe-need pressure6 ties cost6, so care is declined. Across30 runs, `dependency_social` records64,620 care refusals, transfers0 food, and loses the first child in every seed. Both parents remain alive. There is no automatic grief from a zero-attachment edge. This is a **specific arbitration/initial-history interaction**, not proof that all B2 families always refuse care: quantities and other history can change the comparison. It was not repaired by adding birth attachment, compulsory sharing or stronger altruism.

|60h case|Food transferred|Consumed|Starvation-person-hours|Survivors|
|---|---:|---:|---:|---:|
| Dependency, B1 care |48|132|8.933|4|
| Dependency, B2 enabled |0|84|29.933|3|
| Less-assisted multigeneration |192|528|35.733|16|
| Same, B2 enabled |132|468|95.733|16|

In the multigeneration rows, the coordinator supplies child-support commands in both arms. This is not an autonomous B2 rescue. Equal survival conceals a60-person-hour increase in starvation exposure.

The parent-death fixture applies a labeled fatal-starvation initial-state intervention to both founders at10h. The existing death kernel executes it; property is not redistributed. The child has already received12 quanta. It has23 unsupported childhood-hours, matures, then dies at45h01m. No replacement caregiver appears. The externally imposed exposure is not counted as previously experienced starvation time; the reported integration measures time actually spent in the run.

### Several dependents competing for the same food

`seeded_one` and `seeded_three` give the same two parents60 total personal food quanta. Only the number of initially hungry, newborn-age dependents differs. Both subsequently have one additional birth.

| Case | Care food | Parent consumption, each | Parent death | Total starvation-hours | Unmatured deaths |
|---|---:|---:|---:|---:|---:|
| One seeded dependent |36|12|54h|96.033|0|
| Three seeded dependents |54|3|45h|146.150|1|

In the three-dependent seed0 trace, the seeded dependents consume12,20,22 quanta; the later newborn receives0 and dies before maturation. This unequal allocation is the output of existing local care/survival decisions, not an optimal household allocator. All ultimately die within60h in this finite-food comparison. No pooling or magical support was added.

### Work and opportunity cost

The working pair uses the existing two cultivation units and F3 recipe, with six carried quanta per adult and no large loose reserve. Only conception success differs.

- No births: mean founder farming12.3525h, consumption108, starvation0.
- Births: mean founder farming14.7337h, consumption108, care transfers14, starvation35.914h (range35.203–36.230).
- Descendants perform0 farming hours in these runs; the first descendant matures but later dies.
- Seed0 produces96 quanta in each arm through four real24-quanta harvests.

Thus care does not necessarily reduce measured farming time. More time is required under worsened physiology while nourishment is redistributed. Both founders consume47 quanta rather than54 in seed0, while the child consumes14. Reproductive action time falls4→2 person-hours as later opportunity conditions change. Do not attribute every paired difference solely to the instantaneous gift action: these are coupled feedback effects. Local transfers have no invented duration; care travel is0 in these colocated worlds. Exact per-person activity ledgers preserve the actual opportunity costs.

## Information and B2 across generations

Tests cover a locally known attached descendant death versus the same remote death. With explicitly seeded attachment20, local knowledge produces grief10 and unknown death produces0. That attachment is an experimental initial condition, not automatically assigned at birth. Ordinary B2 appraisal is reused; there is no child-death script. A remote father receives no new personal social edge or need sighting merely because birth occurs elsewhere. Parentage and caregiver records remain structural history; they grant no remote condition knowledge or property access.

The initial birth relation strength remains0 as requested. Consequently, affection and grief are not guaranteed merely by genealogy. This preserves the distinction between biological/structural connection and learned interpersonal history.

## Computation and safety

The harness advances a bounded60h horizon, checks population each hour and aborts/reports above128 Persons. This is not a kernel fertility limit: births are never vetoed to satisfy a benchmark. No safety abort occurred. Peak population across the matrix is16 Persons; the generational-control arm peaks at12–13.

The final510 runs used about28.051 seconds of summed per-run elapsed measurement on this host, including the probe's state-size sampling. Parallel scheduling means this is not total wall time. Peak process RSS across30-seed case invocations is22,632KiB, including executable, tracing and harness overhead. In the generational-control case, a serialized Person/lifecycle/household/social-state proxy grows from about5.4KB to29KB. This is **not an allocator heap measurement** and excludes some other world components. Both this proxy and actual process RSS are retained; do not extrapolate a population-scale performance claim from16 people.

Sparse relationship counts peak at40 directed edges in the matrix; no all-to-all social graph is introduced. Existing household observer bookkeeping can still scale with household size, so larger-family performance needs separate evidence. Recorded birth-sex totals are not independent demographic samples: paired fixture aliases deliberately repeat seeds and conditions. No sex-ratio correction was applied.

## Validation and gate

- Full suite:242 tests passed. A subsequent targeted17-test lifecycle run also passes, adding one new boundary test:243 distinct covered tests. This is not a claim that a second full243-test suite was run.
- Formatting and all-target clippy pass; release binaries built.
- All510 final runs have zero material-accounting errors and zero FIFO overtakes.
- Ninety-six historical executions reproduce physical metrics, authoritative state/event hashes and raw recorded traces:62 earlier panel/B1 cases plus34 B2 arms atseed0.
- One diagnostic formatting exception is explicit: B2's research `physical_signature` hashes full debug Config, whose new `lifecycle: None` field changes that formatting hash. Actual physical values and authoritative hashes/traces match.
- All6,349 pre-B3 research files remain byte-identical.
- The final repeated450 cases have the same physical state/event hashes as their preserved reporting-predecessor outputs.

B3's twelve gates are met at this scope: real birth and persistent kinship; dependent Persons; physical care; identity-preserving maturation; mutual adult reproduction; death affecting support; B2-compatible information/appraisal; a controlled third generation; reproductive-only sex mechanics; no sex-linked psychology/occupation; and measurable growth with harness-only safety termination.

No inheritance, marriage, exclusivity law, divorce, property pooling, genetic traits or military powers were selected. **B3 passes; sustainable autonomous families remain unproven.** The newborn-care/B2 gate and descendant readiness bottleneck are preserved research findings, not reasons to add more family complexity now. F11 is next, subject to the explicit owner decision on military service obligations; F10 did not authorize conscription or seizure.

## Reproduce

Use new output directories. For exact binaries, check out the source recorded in each provenance file. The final matrix uses9af6d34; historical replays use97b2f33 because subsequent changes affect only research fixtures/reporting/tests.

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_lifecycle.py /tmp/won-b3-matrix
python3 scripts/verify_lifecycle.py /tmp/won-b3-regressions
python3 scripts/analyze_lifecycle.py research/lifecycle /tmp/won-b3-analysis
```

The analyzer reads the preserved directory layout (`matrix`, `matrix-verified`, prior checksum snapshot), validates the reporting repair, and writes new analysis files without overwriting evidence.
