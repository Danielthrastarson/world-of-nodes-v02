# F5 — personal capital: gate passed

Physical, personally owned carts are implemented. Autonomous investment occurs
without assignments or quotas, sometimes improves logistics, sometimes harms
nourishment, and sometimes leaves unusable private assets. No outcome was tuned
away. F5 establishes capital formation and capability, not efficient investment
or reliable settlement survival.

## Institution and implementation

A cart requires 10 personally owned wood and 4 nominal work-hours. Its physical
container adds 20 cargo quanta to the unchanged 10-quanta backpack. Construction
uses the same physiology-dependent work rate and authoritative clock. Materials
move into an explicit embedded-wood ledger, including unfinished projects. A cart
with stable ID and owner appears only at completion. Input gathering, construction,
transport, spoilage and death are actual kernel actions/transitions.

Owner and user are separate. The owner must be physically present to hitch it.
Parking removes carrying capability and leaves cargo in the actual cart. Cargo
retains its goods ownership and original food age. Dead owners retain title;
mid-edge carts remain mid-edge. No sharing, inheritance, land ownership, wages,
money, depreciation, priority admission or travel-speed benefit was introduced.
The cart uses its pulling agent's existing FIFO traveler slot; separate vehicle
footprint/terrain restrictions are outside this first physical abstraction.

Investment uses personal observations/memories of a food source, a communal
request, queues/routes and local wood. It compares repeated believed transport
work at backpack/cart capacities with the acquisition/construction cost, retaining
the six-hour construction nutrition margin and existing immediate survival rules.
There are no invented meals or remote updates. Remembered demand is neither a
promise of future work nor a reservation. Multiple agents can independently
credit the same demand and overinvest. Construction may be interrupted and resumed.

The repeated-trip estimate is a bounded investment heuristic, not a certified
multi-cycle execution plan: it repeats current believed route costs rather than
predicting future competition, changes of physiology or food availability. Its
short construction action forecast is checked against kernel mechanics. No claim
is made that agents correctly forecast their entire lifetime payback.

The demand signal is the existing communal hauling institution, although the
cart and its inputs are personal. This is not a demonstration of investment
motivated solely by future personal consumption. Capital is opt-in through its
world configuration; historical defaults, references and policies without capital
remain unchanged. In capital worlds, a delivery spanning backpack and cart keeps
the job active until both physical containers have unloaded; no cargo is duplicated.

## Paired experiment

Source revision: `097f37ff15f0e2fe3d7549a25fffa368ce66913b` (kernel implementation
`d0bf5e7`). Seven registered conditions × two arms × seeds 0–29 = **420 runs**,
each 72h. The arms differ only in cart investment availability; finite wood exists
in both. All use 50 agents, 3000 food at F, creation t=0/expiry t=48h, capacity-four
edges, backpack ten, initial nutrition twelve/fatigue four. Each edge's nominal
travel time and board demand are disclosed fixture factors, not tuned defaults.

Except discovery, agents start with explicit remembered source/route information,
identical in both arms. Discovery starts with only C and its adjacent edge. New
fixtures do not replace the original unknown-source 2h-edge reference.

Values below are per run, cart unavailable → available. Endpoint totals have zero
variation across the 30 paired seeds in each condition; these symmetric line
worlds provide reproducibility, not broad stochastic/geographic generalization.
Food/delivery quantities are quanta; starvation is person-hours at zero nutrition.

| Condition | Each edge | Initial request / wood | Carts completed | Delivered | Consumed | Starvation hours | Alive at 72h |
|---|---:|---:|---:|---:|---:|---:|---:|
| Short, low demand | 0.25h | 30 / 500 | 0 | 33 → 33 | 2491 → 2491 | 509 → 509 | 50 → 50 |
| Short, high demand | 0.25h | 600 / 500 | 50 | 230 → 286 | 2358 → 2406 | 642 → 594 | 50 → 50 |
| Medium, high demand | 0.5h | 600 / 500 | 50 | 60 → 432 | 2506 → 2262 | 494 → 738 | 50 → 50 |
| Long, high demand | 2h | 600 / 500 | 50 | 0 → 0 | 1288 → 880 | 920 → 1040 | 28 → 20 |
| Scarce wood | 0.25h | 600 / 5 | 0 | 230 → 230 | 2358 → 2358 | 642 → 642 | 50 → 50 |
| Contested wood | 0.25h | 600 / 10 | 0 | 230 → 92 | 2358 → 2410 | 642 → 590 | 50 → 50 |
| Discovery | 0.5h | 600 / 500 | 22 | 24 → 28 | 2456 → 2385 | 544 → 615 | 50 → 50 |

Demand can subsequently refresh through the existing board protocol; it is not a
lifetime delivery ceiling. All unconsumed food spoils at 48h. Alive at 72h does not
mean adequately nourished: the unchanged continuous-starvation reset can keep
agents alive while they spend substantial time without nutrition.

## Asset cost, use and physical payback

All 50 agents considered a remembered transport opportunity eventually in each
available arm; none was assigned a construction quota. Short/medium/long high
conditions each started and completed 50 carts: **500 wood and 200 labor-hours**.
Discovery started 36 projects, completed 22, embedded 360 wood and used 152.620
labor-hours. Fourteen unfinished investments persist. Low demand and both scarce
wood conditions complete no carts. In the contested case gathering still occurs.

| Available arm | Cart traversals completed | Cart food-quanta-edge arrivals | Backpack food-quanta-edge departures | Mean cart load utilization on completed traversals | Carts with edible flow | Dead-owner carts |
|---|---:|---:|---:|---:|---:|---:|
| Short high | 688 | 4318 | 954 | 31.38% | 40 / 50 | 0 |
| Medium high | 416 | 1258 | 1257 | 15.12% | 33 / 50 | 0 |
| Long high | 44 | 0 | 0 | 0% | 0 / 50 | 30 |
| Discovery | 100 | 930 | 1008 | 46.50% | 14 / 22 | 0 |

A quantum crossing multiple edges is counted multiple times; these flow measures
are not unique food produced or delivered. Backpack departure and cart edible
arrival measures are deliberately labeled separately. Utilization uses departure
load on completed cart traversals / (20 × completed traversals), including empty
trips. Mean completion-to-first-traversal delays among used carts: 1.493h short,
3.279h medium, 10.625h long, 0.954h discovery. Never-used carts are excluded from
that mean and retain an empty first-use field, not zero delay.

Every cart's record in `matrix/*/carts.csv` reports wood, labor, first-use delay,
idle hours, trips, cargo flow, and the excess over what a ten-unit backpack could
carry on those same crossings. Example short-high cart 0: 10 wood, 4 work-hours,
156 cart food-quanta-edges, 104 matched-crossing excess quanta and at least 14 extra
backpack crossings to carry that same load. It was idle for 61.999h of the run.
These are per-asset **capability witnesses**, not an attribution of aggregate
welfare to that particular cart. Mixed-agent interference prevents recovering
individual causal net payback from one observed trajectory.

Causal availability effects are the paired world deltas above: short-high buys
+56 delivered/+48 consumed for 500 wood/200h; medium buys +372 delivered but loses
244 consumed; long buys no extra delivery and loses 408 consumed/eight survivors.
No scalar subtracts wood from food or hours. Opportunity cost beyond other cart
investments is not modeled: no fictitious competing wood industry was added.

A separate fixed, research-only single-carrier schedule isolates one cart against
no cart under identical physical laws. Other 49 agents remain idle; this is neither
NPC behavior nor an optimum. At 48h, delivered quantities are:

| Edge / initial demand | No cart | One cart | Additional delivered | Cart cost |
|---|---:|---:|---:|---|
| 0.25h / 30 | 29 | 29 | 0 | 10 wood + 4h |
| 0.25h / 600 | 257 | 599 | 342 | 10 wood + 4h |
| 0.5h / 30 | 28 | 28 | 0 | 10 wood + 4h |
| 0.5h / 600 | 110 | 347 | 237 | 10 wood + 4h |
| 2h / 30 | 2 | 22 | 20 | 10 wood + 4h |
| 2h / 600 | 2 | 53 | 51 | 10 wood + 4h |

The long-distance carrier's starvation exposure rises from 3.206h to 5.114h despite
higher delivery. Background agents add 1176 starvation-hours to every schedule.
Small residual requests can be eaten by their carrier; completion is not forced.

## Causal traces and negative results

Exact event IDs and timestamps are retained in `causal-excerpts.txt`, with full
seed-zero tapes in each arm. All seeds retain per-person/checkpoint/asset tables.

- Short-high agent 0 locally gathers ten wood in 20s, begins project 0, completes
  cart 0 at 4.005556h, hitches locally and enters the normal FIFO queue. Its route
  still includes self-provisioning reversals; owning a cart does not force a job.
- Long-high agent 20 pays the same construction cost and dies at 36h while crossing
  X→F. Cart 20 is stranded at that exact edge progress with owner 20 and no food.
  All 50 invested, none moved edible cart cargo, and 30 assets end locked to dead
  owners. The paired result identifies harm from investment availability; the
  trace shows construction delay before exposure to unchanged constrained travel.
- Contested wood: agents 0–9 each gather exactly one quantum. No investor obtains
  the ten personally owned wood required. Agent 0 changes from INVEST to
  INSUFFICIENT_LOCAL_WOOD after its first quantum. Aggregate sufficient material
  does not imply institutionally accessible construction inputs. No pooling rule
  was introduced to remove this failure.
- Discovery agent 0 has NO_KNOWN_DEMAND at t=0. It reaches F at 1h, learns the source,
  later returns to C, begins local wood acquisition at 7.625h and completes its
  cart at 12.088148h. The investment rule did not secretly direct exploration to F.
- More logistics is not better nourishment: medium-high delivery increases while
  consumed food decreases and starvation rises. This is preserved without trying
  to select a more favorable construction cost or worker count.

The first research scheduler accidentally waited whenever its chosen meal could
not consume a whole quantum. That controller defect is preserved in
`witness-initial-controller.rs` and `physical-witness-initial.csv`; corrected results
are in `physical-witness.csv`. The correction excludes zero-quantum meals and has
a regression test. It did not change NPC policy, kernel or any paired matrix run.
The initial compile/lint failures and corrected logs are also retained.

## Validation and reproduction

The full suite after kernel integration passed: 100 library, 2 transport-binary,
10 integration tests. The final research-controller regression adds one passing
test. Clippy across all targets passes with warnings denied. Capital tests cover
rights/completion, input preservation, interruption/death, actual container load,
parking/locality, immutable age, mid-edge stranded cargo, spoilage, FIFO against
an earlier unloaded traveler, cross-container delivery, hidden remote materials,
low/high demand and 12 frozen construction-physiology cases.

Nine historical endpoints, 1387 hourly regression comparisons and the F3 seed-zero
0–168h state/event outputs remain identical. SHA-256 verification confirms **1066
pre-existing research files unchanged**. All 420 runs and matched schedules pass
material/exclusivity/locality audits and report zero queue overtakes. Food plus
consumption/waste and wood plus embedded inputs balance exactly.

Reproduce into new paths (never overwrite historical output):

```sh
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --bin capital_probe --bin capital_witness
python3 scripts/run_capital.py /tmp/world-nodes-capital-reproduction
target/release/capital_witness /tmp/world-nodes-capital-witness.csv
```

`matrix/provenance.json` records seeds and experiment revision; parameters are
written separately for every run. Final research-controller/probe source is `8c57a64`. Final probe changes only document/assert the
registered common-food-age assumption behind matched-crossing metrics. The final
controller refactor retains the fixed witness output byte for byte. The historical
preservation manifest, run tables, detailed analysis, full tapes and failure logs
are part of this study. Time-zero food means matched-crossing excess is not valid
as a generic mixed-age farming-output estimator.

## Gate decision and next dependency

F5 PASSED: physical formation, real capacity, voluntary investment, opposing
cost/benefit outcomes and explicit lock-up all occurred with preserved invariants.
This does not select a socially optimal capital institution or validate long-term
sustainability. No meal-aware or stronger intention policy was adopted.

F6 is prepared at `docs/INFRASTRUCTURE_RIGHTS_DECISION.md`. Fixed infrastructure
access is not implied by personal cart rights. Pending that genuine institutional
choice, no crossing, land title, storage bonus or housing need has been added.
