# B2 — directed history, appraisal and transient affect

**PASSED at bounded mechanism scope.** An observed interpersonal event can change a later autonomous choice through personal appraisal and directed relationship/affect state. This does not establish a general psychological model, frequent betrayal, or a new aggregate economic effect.

Baseline: `a72e269` (accepted B1). Primary simulation/replay source: `21035f3089aa9ca090cbac0c32027055bd27d51a`. Follow-up cost audit source: `42c9f757c070cc73c0f04fc345a9a1003e2a4c8b`. Each dispatch records its binary SHA256 and seeds. [Registered rules](../../docs/RELATIONSHIP_EXPERIMENT.md), [machine-readable analysis](analysis/analysis.json), [exact focal chains](analysis/TRACE_EXCERPTS.md), [historical replay validation](regressions/validation.json).

## What changed

The common Person capability stage can now read a sparse personal social memory. Four directed persistent dimensions—trust, attachment, grievance, obligation—are separate from lazily decaying anger, fear and grief. There is no friendship, loyalty, betrayal or rebellion variable. Historical fixtures keep the optional social configuration absent, preserving previous decisions and event hashes. B2-enabled worlds explicitly opt into the registered behavioral hypothesis; this is not a silent replacement of every historical policy.

Actual local food transfers, visible use of food despite a care expectation, failed local transfer attempts, learned deaths and personally observed starvation danger generate typed evidence. A pure appraisal function receives that evidence, not World. Recipient-to-giver updates do not mirror automatically. Care expectations come from explicit caregiver obligations; family does not pool goods or broadcast needs. Local visible nutrition is a simplified observational assumption, not access to private goals or remote stock.

Initial explicit kin/care links have attachment20 and other dimensions0. Help strength uses quantity and the giver's locally observed nutritional stress, an imperfect scarcity proxy. Six quanta produce trust/obligation +6 at18h nutrition versus +12 at7h. Attachment rises by3 versus6. An observed unmet expectation with a believed alternative reduces trust, raises grievance and produces anger; an apparent inability produces disappointment without grievance/anger. These numerical assumptions were registered before the matrix and were not fitted afterward.

Affect influences discretionary care, not physiology or a new coercive action. It adds inspectable reluctance terms after B1's existing legality, custody, location, survival, fatigue, job and atomic-action gates. Each relationship dimension contributes separately to support pressure or reluctance. No conditional such as `if betrayed: refuse` exists. No stronger care commitment or survival-priority change was added.

The graph stores O(N+E) state, with explicit adjacency for event witnesses. Death/observation checks inspect existing contacts, not every possible pair. Appraisals occur on relevant evidence or danger transitions; relationship state has no tick decay. Affect is read lazily with one provisional intensity unit/hour of linear fading. A 100,000-person storage test produces zero edges without explicit relationships; it is not a 100,000-person simulation performance benchmark.

## Experimental design and interpretation

The primary matrix has **900 runs**, seeds0–29 per arm:

- Eleven controlled histories × influence off/on ×30 =660 runs.
- Four existing B1 worlds × social absent/present ×30 =240 runs,48h each.

Controlled history actions are prescribed commands executed by the physical kernel. They test appraisal of real occurrences, not how often people autonomously produce those histories. A research-only matched-state transplant then puts the resulting social memories into an otherwise identical later state. Full serialized non-social inputs are checked equal, including bodies, goods, location, personal spatial/care memories, rights and RNG. Hashes are supplementary evidence. The later choice and continuation are made by Person, not scripted.

This is a conditional-state control, not an assertion that each combined history/snapshot arose as one uninterrupted world. The matched donor has18h nutrition and six personally owned quanta; its current care recipient has0h. Influence-off retains history but excludes it from selection. The zero-relation control has identical later formal care rights but no relational priors; it is not a test of new stranger-transfer rights.

A separately registered **120-run cost audit** compares one physical gift/no gift under scarcity and abundance. It was added after the primary matrix to distinguish measured sacrifice from the appraisal proxy. No coefficients, physical rules or primary outputs changed. All30 seeds reproduce the reported scalar outcomes; these are deterministic mechanism replications, not30 independently sampled psychologies.

## History → appraisal → later choice

All influence-off controls initially choose GIVE,30/30. With influence on:

| Earlier history | Support pressure / reluctance at later decision | Initial GIVE /30 | Quanta transferred by2h | Quanta consumed by2h |
|---|---:|---:|---:|---:|
| No episode; existing kin/care priors |8 /6|30|6|6|
| Six-quanta support, giver18h |10 /6|30|6|6|
| Same support, giver7h |12 /6|30|6|6|
| Failed physical help attempt |8 /6|30|6|6|
| Locally observed refusal-like act |7 /8|0|0|0|
| Same act, unobserved remotely |8 /6|30|6|6|
| Directed/asymmetric costly-help history |12 /6|30|6|6|
| Attached death learned locally |8 /7|30|6|6|
| Same death unknown remotely |8 /6|30|6|6|
| Recent observed starvation danger |8 /8|0|6|0|
| No relationship priors |6 /6|0|0|0|

The critical contrast is costly support versus observed refusal under identical later physical inputs: **30/30 give versus0/30 give**. Neutral priors already permit giving, so the observed binary divergence here comes from the negative history; costly versus safe support changes appraisal strength, not this particular binary choice. Do not claim the experiment separately identifies every dimension or proves learned loyalty above a neutral control.

In the refusal episode, the caregiver consumes six personal quanta at18h nutrition while its severely hungry dependent is present. The dependent infers an available alternative. Trust becomes−6, grievance6, anger12. At the later1h decision anger is11; these separate terms produce pressure7 versus cost8. Person chooses its ordinary WAIT fallback. There is no `betrayal` outcome command and no assertion of perfect knowledge of motive.

In the inability episode, food is across a capacity0 edge. The giver actually queues toward it and cannot transfer food it does not hold. The recipient sees the failed attempt; trust becomes−1, grievance and anger remain0. The same later eligible care candidate is accepted. A self-preservation unit control also yields disappointment rather than intentional-harm appraisal.

Recent fear delays help by one policy minute: intensity12 falls just below its integer scoring boundary, allowing a gift at1h01m. The recipient has already joined an exploration queue and then travels away; the gift remains personally owned loose food. By2h, six quanta have transferred but none were eaten. **Transfer is not nourishment.** This is a coarse integer-score boundary effect, not evidence for a natural one-minute psychological timescale.

Known attached death produces grief10; unknown death produces0. The known-death matched case still gives: grief exists without forcing a different choice. Isolated unit ablations show that anger and grief can each change a marginal eligible care choice without changing rights or physics. These are mechanism sensitivity tests, not evidence of their prevalence. A later physical visit can discover a previously remote death once; remote death cannot trigger grief or unrelated self-observation.

## Cost is physical, not a social score

The follow-up audit runs48h with ordinary self-provision after one prescribed gift/no gift. Further care is disabled identically in both arms; appraisal is recorded but cannot change behavior. Scarce giver:7h nutrition and six personal quanta. Surplus giver:18h nutrition, six carried and60 personally owned loose quanta at the same node. No production; all initial food spoils at48h.

| Giver condition | Gift | Giver consumption | Giver death | Giver starvation-hours | Nutrition at48h | Recipient death | Waste at48h |
|---|---|---:|---:|---:|---:|---:|---:|
| Scarce |No|6|37h|24|0|24h|0|
| Scarce |Six quanta|0|31h|24|0|30h|0|
| Surplus |No|42|Alive|0|12h|24h|24|
| Surplus |Six quanta|48|Alive|0|18h|30h|12|

The scarce gift transfers six hours of survival from giver to recipient. The abundant gift causes no giver starvation or death within48h. It also changes which food is available to ordinary consumption, so the giver consumes six more quanta and ends with more reserve; the states are not identical. Call this **no measured nourishment loss in the observed window**, not zero universal opportunity cost. Total starvation-hours alone misses the scarce giver's earlier death because both eventually accumulate24h continuous starvation. Death timing, individual consumption and physical stocks are essential.

The primary `help_safe` label denotes lower observed nutritional stress, not proof of no long-term cost. The appraisal proxy does not solve counterfactual welfare or inspect hidden future opportunities.

## Autonomous B1 worlds: preserved negative result

All four live worlds have **zero changed physical outcomes across30 paired seeds**. Every person's activity ledger, nutrition, consumption, starvation and death timing match. Seed0 physical/action traces also match after removing only new social-observation records and the appended social score in decision explanations. Unfiltered social-enabled hashes naturally differ.

| World,48h | Care food | Total consumed | Spoiled | Starvation-person-hours | Survivors | Care travel-hours |
|---|---:|---:|---:|---:|---:|---:|
| Colocated |6|6|0|48.016667|0|0|
| Farming |10|54|0|36.416667|1|0|
| Distant farming |0|47|0|24|1|1|
| Construction |6|2408|598|30.382967|50|0|

Values are identical with social influence absent/present. Therefore additional farming/construction/other work displaced by B2 is0 in these comparisons; B1's existing care costs remain. No new relationship-induced travel is observed. Detailed per-person activity, affect exposure and physical metrics remain in each run directory.

Per enabled run, observed appraisal counts are4,7,1,1 respectively; anger episodes0 in all; fear episodes2,1,1,0; grief episodes1,1,0,0. All live eligible care candidates selected by the baseline remain selected; no social refusal occurs. Positive history often belongs to the recipient toward the caregiver, without a later reciprocal food opportunity. The current local event repertoire and existing survival/arbitration gates limit where history can affect decisions. **Do not amplify coefficients to manufacture drama.**

Affect durations in `people.csv` are calculated analytically from pulse intervals and capped at death. Matched-state records include the transplanted pulse's original clock span, not only the post1h execution window. Episode counters in `runs.csv` concern execution; controlled prelude appraisals are separately retained in `relationships.csv` and `appraisals.tsv.gz`. Do not read a0 execution grief count as no prior grief.

## Validation and bounded gate

- Full suite:225 passing tests; subsequent research-only cost fixture:one additional targeted passing test (226 distinct covered tests, not a claimed second full-suite run).
- Formatting and all-target clippy pass. Release probes built successfully.
- 1,020 new runs:zero accounting error and zero FIFO overtakes.
- 62 historical regression executions:40 F0–F10 panel cases plus22 B1 fixture/arm replays atseed0; preserved CSVs, state/event hashes and recorded traces match.
- All5,469 pre-B2 research files retain their SHA256 hashes.
- Tests cover directed asymmetry, late local death discovery, remote independence, no dead-person appraisal, deterministic/partitioned execution, journal independence, sparse storage, each relationship dimension, each affect, and matched-input choice divergence.

The ten B2 gates are met at this scope: sparse directed records persist; local events are explicitly appraised; affects have causal selection effects; ignorance prevents remote updates; observed history changes a later autonomous choice; B0 remains the decision interface; no all-pairs social update or new physical/social right was introduced. Primitive material, ownership, locality and queue invariants remain intact.

## Limits and deferred work

This is a deliberately narrow social substrate. It does not infer arbitrary intentions, learn cultural expectations, create new caregiver rights, reward official performance, add confrontation, punish, take revenge or extend grief into productivity penalties. Visible personal carried/cart food use is appraised; commons consumption and personally owned loose-food self-consumption are not yet equivalent social evidence channels. Failed local attempts conservatively suggest inability and can be misinterpreted. These are explicit coverage limits, not general models of morality.

Social capability remains optional for exact historical reproduction. Enabling it is a behavioral hypothesis; appraisal arithmetic is not a validated human model. No claim is made that costly loyalty, broad distrust networks or social conflict emerge spontaneously at population scale. The strongest proof is controlled history changing an autonomous later choice; the live economic effect is negative in the tested worlds.

No inheritance, marriage, pooled property, sex-linked psychology, culture, punishment, revenge rights, reproduction or military was added. **B3 remains NOT STARTED; F11 remains BLOCKED.** Existing scarcity, migration, care harm, inaccessible property and institutional failures are preserved.

## Reproduction

Use fresh output directories; runners refuse overwrite. Check out the indicated source revision to reproduce a particular binary/dispatch. The analysis script in the final evidence revision reads both source-specific datasets.

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_affect.py /tmp/won-b2-primary
python3 scripts/verify_affect.py /tmp/won-b2-regressions
python3 scripts/run_affect_cost.py /tmp/won-b2-cost
# For the analysis layout, place primary and cost outputs under ROOT/matrix and ROOT/cost,
# and preserve ROOT/prior-artifacts.json with repository-relative historical paths.
python3 scripts/analyze_affect.py research/affect /tmp/won-b2-analysis
```
