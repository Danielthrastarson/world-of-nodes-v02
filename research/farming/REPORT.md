# F3 — cultivator-owned renewable production

Gate: PASSED for causal production and explainable nourishment, not indefinite
sustainability, optimal planning, or communal farming. No parameters were tuned.

## Implemented institution and physical model

The owner’s B decision is implemented: claim starts at sowing, crop and harvest
belong to cultivator, plot becomes available after harvest. No land title or
inheritance. Dead owners retain crops, including mature crops. Sow/growth/harvest
use the same clock as physiology and transport. Work is interruptible and retained.
Harvest creates loose food at the plot; normal ownership, carrying and spoilage apply.
See [specification](../../docs/FARMING_EXPERIMENT.md) and
[action/model boundaries](../../docs/FARMING_ACTIONS.md). Environmental input is
explicitly accounted, not mislabeled initial food. There is no closed mass claim
for unmodeled water/carbon/soil. Mature unharvested crops do not decay in this model.
The policy is a disclosed bounded investment heuristic, not learned intelligence.

## Paired evidence

300 runs: ten registered arms × seeds 0–29, 168h. Fifty people; 3000 initial quanta,
48h original shelf life, original physiology/transport, local discovery. Farming
adds suitable units and production; no automatic communal crop/output conversion.
Every boundary audits material/exclusivity/ownership; no accounting error or FIFO
overtake. Full parameters, checkpoints, people, crop counters, activity and seed-0
causal traces are in `matrix`. Min/max variation and all checkpoints: `summary.json`.

| Geography / plots / model | Alive 72h (mean) | Alive 168h (range) | Produced & consumed crop quanta (mean) | Starvation person-hours 168h (mean) | Inaccessible mature dead-owner crops (mean) |
|---|---:|---:|---:|---:|---:|
| Line / no farming | 28 | 0 | 0 | 1349.00 | 0 |
| Line / C / base | 30 | 4 | 1440 | 1318.77 | 38 |
| Line / F / base | 28 | 28 | 3360 | 694.50 | 0 |
| Line / C / slow | 28 | 0 | 0 | 1349.00 | 0 |
| Line / F / slow | 28 | 4 | 1248 | 1683.70 | 4.47 |
| Line / C / low yield | 20 | 0 | 624 | 1586.31 | 32 |
| Line / F / low yield | 28 | 0–1 | 1390.80 | 1504.34 | 0 |
| Branch / no farming | 48 | 0 | 0 | 1369.92 | 0 |
| Branch / C / base | 43.33 | 0–1 | 868 | 1518.90 | 43.13 |
| Branch / far F / base | 48 | 0–2 | 1284.80 | 1373.36 | 22.90 |

Base: sow1h, growth12h, harvest1h, yield24. Slow: growth36h only.
Low yield: output12 only. Nominal work is impaired by real hunger/fatigue.
All autonomous harvested output was consumed; observed harvested-output waste=0.
Original stock still spoiled normally. Zero output spoilage is an outcome of these
runs, not missing spoilage: the split/carry/death deadline test explicitly verifies it.
Survival is not adequate nourishment: the starvation reset rule is unchanged, and
slow-growth survival can coexist with *more* total starvation exposure than control.

## Causal findings

1. **Renewable output matters after original stock.** Line/F base: 140 completed
   harvests, 3360 new quanta consumed, ~403.13 labor-person-hours. At168h 28 living
   agents are at F. Initial food cannot account for this: it expired at48h. The same
   nonfarming world has no survivors at120/168h. Most early deaths still happen in
   migration before farming starts; farming does not repair transport starvation.
2. **Investment is voluntary, with opportunity costs.** At C, agents sow, then may
   explore toward F while growth proceeds. Seed0 agent0 starts0h, establishes1h,
   reachesF5h, returns toX16.08h, reachesC29.05h after queue/sleep interruption,
   harvests30.0625h. That crop matured13h. Four residents eventually continue local
   cycles, but38 mature crops belong to dead owners and stay inaccessible.
3. **Slow growth can block investment.** At C the no-meal cultivation candidate
   exceeds current-nutrition-plus-continuous-starvation reachability. No sowing occurs.
   Physical outcomes match finite-stock control. At F, eating existing food allows
   some longer candidates; that does not guarantee later nourishment or survival.
4. **Geography exposes a new failure, not a special-case repair.** Branch/F seed0
   agent0 harvests48.4333h; establishes a second crop68.5167h, due80.5167h.
   While it grows, exploration resumes: X79.9025h, C86.5133h, other branch92.552h;
   agent dies113h at node3, leaving the mature crop behind. Existing exploration,
   critical needs and return preference can dominate future crop use. The policy
   does not confer a compulsory home/crop tether. This history is preserved.
5. **Low output and action arbitration interact.** Line/F low yield leaves0–1
   survivors (20 runs none,10 runs one). Work interruptions average3835/run versus
   248/base. Yield12 is less than even the14h unimpaired labor+growth cycle; fatigue
   and interrupted sleep/work increase delay. Some agents cease new investment
   after falling below the existing critical threshold. No worker quota, extra
   food, extra carrying space, food-age reset or risk-threshold reduction was added.
6. **Ownership has a measured cost.** Dead-owner crops never pass to another agent.
   Branch/C leaves37–45 mature inaccessible crops. These are institutional allocation
   losses, not conservation errors. Paused/live remote crops are reported separately
   from dead-owner crops; no unimplemented abandonment/renunciation is assumed.

Exact selected events: `causal-traces.json`; full event tapes remain under each arm.
General plot IDs and remembered routes are used; no F/C identity exists in policy.

## Physical control and tests

A separate explicit global controller stays at C and executes legal sow/grow/harvest,
eating and sleep for50 agents. At168h:50 alive,12000 quanta produced,8400 consumed,
183.33 starvation-person-hours, initial F stock untouched then spoiled. This is a
feasible witness, not optimality or autonomous behavior. See `physical-witness.txt`.
No labor/no land controls produce no food. Frozen forecasts match kernel timing,
nutrition, fatigue and alive/completion classification in48 parameter combinations.
Eleven new tests cover these plus ownership, retained work, death, splitting/spoilage,
local memory, hidden-land noninterference, node relabeling and time partition.
Full suite:88 library +2 transport-binary +10 integration tests; fmt/clippy pass.
The initial build missed a research caller of the old zero-production audit wrapper;
fixed before experiments. Initial clippy found function placement after a test module;
fixed without behavior change. Both failed logs are preserved.

## Preservation and reproduction

931 prior research files remain SHA256-identical. Nine historical endpoints and
19×73 default/corrected hourly comparisons pass; both regression tables are byte-
identical to preserved F0. No-farming hashes use exactly their old representation;
farming hashes additionally cover crop state and personal crop memory.

Mechanics commit `79e3a5c`; registered matrix harness `ac1a91d`; witness/contracts
`6f87897`. `matrix/provenance.json` records repository revision when runs completed;
the later witness addition did not change the matrix executable. Seed0 full traces
and all per-seed measurements are retained. Commands use new output paths:

```
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_farming.py research/farming/reproduction-NEW
 target/release/farming_witness research/farming/witness-NEW.txt
 target/release/meal_probe regressions research/farming/regressions-NEW
```

Continue to F4: measure pressure-dependent voluntary labor allocation using these
same mechanics, without adding meaningless wood/stone jobs before they have a use.
