# F7 — civilian paths and shared transport feedback

F7 passes for **use-created paths and maintenance through continued use**. Actual
passages change edge state; that state changes later physical movement, locally
observed route costs, traffic and economic outcomes. The existing autonomous
policy is unchanged. This is not a claim that engineered roads, maintenance jobs,
efficient routing or reliable provisioning have been completed.

The result is mixed. More movement and more deliveries can accompany worse
nourishment. Faster paths can also interact with existing exploration/sleep/queue
churn to leave an individual completely unprovisioned. These failures are preserved.

## Model and experiment

[Registered specification](../../docs/PATHS_EXPERIMENT.md). One completed person
passage adds one compaction unit, capped at100. After6h without passage, one unit
recovers every6h; subsequent use resets the timer. Nominal traversal work is
`ceil(base × (500-compaction)/500)`, at most20% lower. These are provisional coarse
physical assumptions, registered before outcomes and not tuned afterward.

Only explicitly eligible existing ground edges change. No commodity, connection,
ownership right or capacity is created. Queuing alone creates no path. A cart's
pulling person counts once. Partial travel ending in death earns no whole-edge
passage in this coarse model. A path cannot build a crossing. F6 projects and their
rights remain separate; completed crossings use the same scheduler.

Movement work is fixed at admission. Later wear/recovery cannot accelerate or
reverse an in-flight traversal. Agents observe adjacent effective costs through
the existing observation boundary; remote memories receive no updates. No future
traffic or surface change is supplied to forecasts. Disabled path configuration
preserves historical behavior. The frozen-surface experimental control records
traffic without changing costs.

Seven registered worlds × two arms × seeds0–29 = **420 runs**, each72h. Every run
has50 agents, initial nutrition12h, food3000 quanta created at0 and expiring48h,
backpack capacity10, and request600 at C. This finite-food stress experiment does
not claim indefinite sustainability. Topologies include line, diamond alternatives,
unequal-capacity detour and empty dead end; controls include local food, F6 crossing
construction and F5 personal carts. Complete per-seed parameters are preserved.

## Paired outcomes

Means over30 seeds; each cell is **frozen → paths enabled**. Food units are integer
nutrition quanta; fractional values below are population-of-runs averages.

| World | Completed edge trips | Delivered | Consumed | Starvation person-hours | Queue person-hours |
|---|---:|---:|---:|---:|---:|
| Line |480 →588|24 →96|2456 →2416|544 →584|1310.15 →1296.43|
| Diamond |789.17 →1019.70|47 →119.47|2478.63 →2496.10|521.37 →503.90|1106.56 →1066.41|
| Detour |523 →584|0 →43|2508 →2494|492 →506|1311.42 →1331.62|
| Dead end |597.93 →685.47|33.60 →76.23|2423.83 →2429.87|576.17 →566.53|1252.40 →1310.35|
| Local food |0 →0|0 →0|2450 →2450|550 →550|0 →0|
| Crossing |564 →597|83 →169|2417 →2332|583 →668|1147.87 →1015.72|
| Carts |416 →484|432 →339|2262 →2297|738 →703|1058.23 →1058.16|

At36h,48h and72h all controls have50 survivors. Enabled paths also have50 except
dead-end seeds13,23,28, each with49 from36h onward. Thus dead-end mean survival is
49.9. Seed13/23 agent49 and seed28 agent48 consume zero food and die at36h. No other
registered run has a death by72h. All survivors have zero nutrition at72h; being
alive is not adequate provisioning.

Dead-end aggregate starvation falls partly because death stops exposure counting:
3.6 of the mean9.63-person-hour reduction is lost post-death observation time.
Do not present that reduction as an unqualified welfare improvement. Spoilage is
explicit, and every unconsumed initial quantum becomes waste at48h. No fresh food
remains at72h.

Diamond consumption differences range from-43 to+85 quanta per paired run; the
mean gain is17.47, not universal benefit. Full ranges and paired differences are in
[summary](matrix/summary.json) and [paired results](matrix/paired.json). Line, detour,
local-food, crossing and cart endpoints are identical across these30 seeds; their
repeated seeds demonstrate reproducibility, not30 independent noisy observations.
Diamond/dead-end route choices do vary with seed.

## Causal paths and geography

**Physical effect.** Line seed0's first four passages finish at0.5h, changing
compaction0→4. Agent4, admitted afterward, receives0.496h nominal movement work
instead of0.5h. No earlier admission or priority is created. Its observed arrival
is0.996h rather than1h. Both in-flight work and FIFO remain intact. Eventually the
line edges reach the100-unit cap; disuse after the food deadline brings both to96
by72h. Unused local-food edges remain bare with zero passages in both arms.

**Spatial differentiation.** Dead-end paths also form: traversal is physical use
even when it serves unsuccessful exploration. Their mean final compaction is73.57,
versus98 on the two source-route edges. The system does not label useful roads or
withhold wear from unproductive trips. On the detour, final compactions are66/87 on
the narrow route and98/98 on the wider route.

**Route feedback.** Between12h and48h, detour admissions from C via the narrow A
route rise from360/1530 (23.53%) to690/1830 (37.70%) across30 runs. Wider B admissions
change1170→1140. Route choices, queues and traversal timing all evolve together;
this is the total effect of the surface intervention, not an isolated estimate of
one route-scoring term. Shared capacity remains binding: detour queue exposure
increases20.19 person-hours despite shorter nominal work. No transport priorities
or node-specific policy were introduced.

**More delivery, less nourishment.** In line seed0, paths produce80 job acceptances
versus62, and16 completed deliveries versus6. Yet gathered food falls2744→2658.
At48h, source spoilage rises256→342, carried spoilage falls284→222, and communal
stock spoilage rises4→20. These physical locations reconcile the544→584 total
waste increase. Earlier/more deliveries do not imply that people ate more. The
paired intervention changes travel, decision and allocation timing; this study
does not claim to have independently decomposed every mediator of the40-quanta
nourishment loss.

**Failure replay.** The first death in seed order is dead-end seed13, agent49.
Full replay reproduces matrix state/event hashes and all tables. In the frozen
arm it first reaches X at6.0208h, explores the dead end, returns through X and reaches
F at15.0611h; it first eats at15.0626h. With paths, initial queue switching postpones
its first C→X admission to6.7255h and arrival to7.2805h. It repeatedly changes route
queues, visits the empty branch, and returns to X at25.8667h. It never observes F.
Between its later exploration attempts, critical fatigue repeatedly interrupts
queue membership for sleep, then exploration interrupts sleep. By36h it has739
queue joins and546 sleep starts, no food, and dies at X. The frozen arm has331 queue
joins and has already eaten17 times by36h.

This is **changed traffic timing interacting with existing reactive arbitration,
limited knowledge and physiology**, not proof that the road itself blocks access
or FIFO  overtakes someone. Queue cancellation loses position by the existing rule.
No persistence fix or quota was added to remove the result. The late forecast
traces include desperate exploration ending at the death boundary; those are not
reported as viable food plans. See [compact failure trace](failure-13-summary.json),
[decision excerpts](failure-13-agent49.json), and full replay journals.

**Shared existing assets.** Frozen crossing and cart arms reproduce the earlier
F6/F5 physical checkpoints and event hashes for all30 seeds. Enabling the ground
path in the crossing fixture increases deliveries but reduces consumption by85.
In the cart fixture it raises consumption35 while deliveries fall93. Cart ownership,
construction inputs and crossing access are unchanged. Neither asset receives a
separate movement scheduler or capacity exemption.

The matched-trip nominal-work reduction (line48.382h, diamond mean81.132h, detour
46.545h, dead-end mean52.993h, crossing9.139h, carts37.980h) is a **conditional
same-trip capability measure**. It is not net realized time saved or a welfare
estimate: the intervention also changes which trips occur and physiology.

## Gate, tests and preservation

- 134 tests pass; strict Clippy and formatting pass. Ten new focused tests cover
  passage causality, saturation, admission contracts, physiology/forecast agreement,
 recovery, dead-edge cargo, stale memory, crossing exclusion, clock partitioning,
 bidirectional FIFO and graph relabeling.
- Every hourly audit passes across420 runs; every reported material error and FIFO
  overtake count is zero. Original food age, death ownership and carried cargo remain
  governed by the existing kernel.
- Nine historical research scenarios and all19 adoption-equivalence scenarios match
 preserved outputs. Farming, capital and crossing regressions match their prior
 tables including physical state/event hashes. A repeated autonomous seed0 line run
 reproduces its full event journal and CSV tables exactly.
- All1396 earlier research files retain their recorded SHA-256. No previous report,
 negative result, default policy, physiology, institution or reference parameter
 was replaced. The path feature requires explicit scenario configuration.
- A new relabeling test initially failed because the research topology helper did
 not remap path/crossing edge identities. The bounded helper correction and failed
 test output are preserved; historical scenarios remain unchanged. No policy was
 tuned in response to the matrix.

**Gate decision: PASSED at the minimal path scope.** Movement causes persistent
terrain state, local observations expose changed cost, subsequent movement and
economic behavior change, and disuse reverses part of that state. Multiple
geographies use the same policy and scheduler. Efficient provisioning is not a
gate requirement. Paving, staffed maintenance, new rights and military systems are
not implemented. Existing reactive churn remains a documented limitation; this
gate does not endorse it as the final agent architecture.

The [future army control target](../../docs/ARMY_PLAYER_CONTROL.md) is documentation:
stable command categories, formation objectives/front geometry and explicit
player issuance. Actual soldiers/resources must later use this same geography,
capacities, FIFO and travel system. No military feature, final UI, Jev or F8 system
was introduced in this increment.

## Reproduction and provenance

Specification commit38aa0e1; tested physical implementationa9d4100; matrix source
991b6e5; analysis scripts5b07763; exact-seed trace selection9251917. The last adds
only research CLI trace/range selection. Matrix parameters and hashes identify the
actual source used. Outputs are immutable; use new directories when reproducing.

```sh
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --bin paths_probe
python3 scripts/run_paths.py /tmp/won-paths-new
python3 scripts/analyze_paths.py /tmp/won-paths-new /tmp/won-paths-analysis-new.json
python3 scripts/path_causal_excerpts.py /tmp/won-paths-new /tmp/won-paths-excerpts-new.json
target/release/paths_probe dead_end enabled /tmp/won-paths-failure-new 1 13
```

Per-arm files include runs at0/12/24/36/48/72h, edge surfaces, all admissions/arrivals,
food loads at departure, individual endpoint consumption, asset state, parameters
and full seed0 traces. Food-quanta edge departures count transport legs, not unique
goods or final deliveries. `visited_food` counts actual arrivals at the designated
source; local-food agents start there and make zero such trips. `t=0` rows follow
initial policy admission, so some agents already occupy edges. State hashes include
optional path history; use physical fields/event hashes when comparing frozen
instrumented controls to worlds without that recording state.

[Analysis](analysis.json), [causal excerpts](causal-excerpts.json),
[historical comparison](regression-comparison-complete.json),
[frozen-control/repeat validation](frozen-and-repetition-complete.json), and
[matrix provenance](matrix/provenance.json) preserve the audit trail.
