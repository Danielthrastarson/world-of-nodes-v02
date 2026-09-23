# F8 — two settlements, existing institutions

F8 passes at the bounded scope of explaining economic interaction between two
inhabited locations in the existing Commons(0). Complementarity produces movement,
small physical food flows and, on short routes, occasional voluntary communal
deliveries. It does not produce reciprocal exchange, wood exports or efficient
allocation. Migration and personal consumption dominate. No default policy,
physical rule, property right or information channel changed. F7 is unchanged.

This is not validation of trade, sustainable settlements or independent political
communities. No barter protocol exists, so zero barter attempts is **not evidence
of rejected bargains or double coincidence of wants**. A new exchange/property
institution requires a project-owner decision; none was selected here. Money and
prices remain unimplemented and unjustified as an immediate remedy.

## Registered experiment and validation

[Specification](../../docs/TWO_SETTLEMENT_EXPERIMENT.md): 210 runs, seven arms,
paired seeds 0–29, 168h. Fifty people start 25 at A and 25 at B, knowing only their
local surroundings. A—X—B has capacity4/FIFO on each bidirectional edge. Two boards
use the existing local shortage protocol, with no preloaded request. Complementary
resources are A:2400 food/40 plots/0 wood; B:600 food/10 plots/500 wood. Symmetry
redistributes the same totals equally. All original food is created at0 and expires
at48h. Existing personal cultivation, harvest ownership, carts, sleep, starvation,
and ration rules remain unchanged. No stone demand is fabricated.

The distribution comparison changes initial food, fertile units and wood placement
together; it does not identify their separate effects. Travel arms change only
nominal edge time. Cart/path controls form a 2×2 comparison at baseline travel.
No assets are prebuilt, no workers assigned, and no quantities adjusted after results.

- **141 tests pass**, including seven new observer fixtures; clippy and builds pass.
- Conservation and FIFO overtakes remain exactly zero in every run; journal totals
  reconcile consumption, production, spoilage and all living-agent milliseconds.
- Observer-enabled/disabled test worlds have identical physical/event hashes and
  RNG. Research place labels cannot change behavior.
- Nine historical and nineteen adoption regression rows, the F3 farming replay and
  F7 dead-end seed13 replay match preserved outputs exactly.
- All **1,589 preceding research files** retain their SHA-256 checksums.
- [Validation](validation.json) compares all **1,470 checkpoints** between first-pass
  and corrected measurement runs: every physical/event hash and every non-flow
  result is identical. All seven full seed0 event traces also match byte-for-byte.

An instrumentation defect was found and repaired, without touching simulation:
arrival is logged before same-boundary spoilage, so the first observer counted eight
arrival records (48 quanta total) as edible at their exact expiry deadline. The
new deterministic regression first failed, then passed. The corrected low-cost
A→B edible-flow mean is35.1, not36.7. Preserve `matrix/`,
`analysis-first-pass.json` and `expiry-boundary-before.txt` as the failed measurement;
use **`matrix-corrected/` and `analysis.json`** for conclusions. Every reported edible
arrival now precedes its immutable expiry. The earlier scaffold compilation failure
is also retained in `initial-check.txt`.

## Results

Means per run at168h; food units are integer nutrition-hour quanta. Fractional
means describe populations of runs, not fractional physical goods. All arms have
50 survivors at36h and48h; this early survival does not establish long-term success.

| Arm | Edge time | Alive72h | Alive168h | Food produced | Food consumed | Food spoiled | Starvation person-h | Queue person-h |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Complementary |2h|39.80|15.07|2813.6|4879.8|924.6|914.33|339.10|
| Symmetric |2h|50.00|18.00|3696.0|6146.0|550.0|856.00|279.00|
| High cost |4h|35.27|13.27|2427.2|4310.6|1116.6|944.97|442.60|
| Low cost |0.5h|50.00|18.53|3302.4|5600.4|671.53|761.00|177.28|
| No carts |2h|39.80|15.07|2813.6|4879.8|924.6|914.33|339.10|
| Paths |2h base|39.80|13.57|2736.8|4795.93|924.6|930.07|322.56|
| Paths, no carts |2h base|39.80|13.57|2736.8|4795.93|924.6|930.07|322.56|

Deaths equal50 minus survivors; every recorded death is starvation under the
existing mechanics. Detailed checkpoints, nutrition distributions, inventories and
local deficits remain in raw files. Continuous starvation reset is unchanged;
being alive is not equivalent to continuous nourishment.

| Arm | Edible cargo arriving A→B | B→A | Communal food deposited | Jobs accepted | Inter-settlement visits |
|---|---:|---:|---:|---:|---:|
| Complementary |16.93|0|0|6.00|68.00|
| Symmetric |0|0|0|0|40.00|
| High cost |0|0|0|0|30.27|
| Low cost |35.10|8.00|2.70|0.90|90.33|
| No carts |16.93|0|0|6.00|68.00|
| Paths |15.87|0.03|0|4.23|69.20|
| Paths, no carts |15.87|0.03|0|4.23|69.20|

Wood flows are **zero in both directions in every arm**. Arrival cargo retains its
owner; it includes personal food accompanying movement and communal cargo that may
never be deposited. Waste movements are separately recorded. Gross arrivals can
count repeated passages; they are not unique goods, barter or net welfare. Visits
are physical relocations, not a declaration of permanent residence.

Only low-cost routes produce completed communal deliveries: **6/30 seeds**,81 total
quanta,2.7 mean,0–19 range. Seeds12/13/15/16/18/29 deliver19/8/13/8/14/19 respectively.
These are unconditional transfers under the existing commons, not reciprocal trade.

Paired effects relative to complementary, including full seed ranges:

- Low cost: survivors +3.47 (−2 to+4), consumption +720.6 (−305 to+853),
  starvation −153.33h (−176.02 to+58.16), queue −161.82h. Shorter travel helps on
  average but not in every paired seed. No isolated effect of delivery on survival
  is identified: travel also changes migration, farming and timing.
- High cost: survivors −1.8 (−2 to−1), consumption −569.2, queue +103.50h.
- Paths: survivors −1.5 (−2 to+1), consumption −83.87, starvation +15.74h,
  queue −16.54h. Faster movement again does not guarantee better nourishment.
- Symmetry: survivors +2.93, consumption +1266.2; complementarity creates more
  visits but does not establish a beneficial division of labor.
- Cart availability has zero measured outcome effect in the registered 2h controls
  because **no carts are built**. This is not an estimate of the effect of using a cart.

These are paired descriptive experiments over the registered seeds, not claims
about all geographies or a statistically sampled real population. Full ranges are
in [paired.json](matrix-corrected/paired.json).

## Causal chains

### Cargo reaches the destination but the policy does not deposit it

Complementary seed0 agent11 accepts B's4-quantum request at83.5h. It physically
travels B→X→A, loads four quanta of its own harvested food into Commons(0) custody
at87.663–87.7h, and returns to B at92.5533h. Ownership changes explicitly; there is
no appropriation of another cultivator's output.

At B it eats one communal quantum: nutrition10.4467→11.4467h. At92.5667h its
nutrition is11.4333h. Integer ration headroom does not permit another whole quantum
under the12h communal ceiling. The existing ordinary self-provision priority fires
before the job-delivery branch and selects remembered personal food at A. Agent11
leaves B with the remaining three communal quanta, consumes them at X at95.75h,
and cancels the unfulfilled job at103.7833h. Four agents follow this pattern; two
other seed0 carriers reverse at X before B. All six jobs eventually cancel.

The primary evidence is [arrival-without-deposit.json](arrival-without-deposit.json),
events389592–389634, and [causal-traces.json](causal-traces.json), agent timelines.
This is a witnessed preference/arbitration outcome, not proof that food cannot
physically reach B. No priority, ration or intention repair was made. Aggregate
baseline cancellations all use the explicit reason “accepted job cannot acquire
expected cargo”; that final reason alone would conceal the preceding transport and
carrier consumption.

### Voluntary communal delivery can complete on short routes

Low-cost seed12 is replayed independently in [delivery-seed12](delivery-seed12).
Seven jobs are accepted, two cancelled, and five deposits move19 quanta:

| Agent | Destination | Time h | Quanta |
|---|---|---:|---:|
|1|B|44.8107|4|
|3|B|45.2140|3|
|1|A|49.2778|3|
|3|A|49.5278|3|
|31|B|66.3111|6|

The later deliveries load the carrier's own loose harvest through the existing
accepted-job ownership transfer. The first deliveries gather communal job cargo
from finite original deposits. These legal acquisition paths, carriage and local
deposit are all traced; no home tether or forced contribution is introduced.
Both directions occur, but no promise of reciprocity or negotiated rate exists.
The witness replay matches every CSV record for seed12 in the main corrected run.

### People move to food more than food moves to people

Complementary seed0 people originally at B consume840 quanta while physically at A,
748 at B and12 at X. Movement gives them access to food at its location. Across
baseline seeds, A-origin food consumed at B averages only4.93 quanta, versus4111.77
at A and12.7 at X. On short routes, A-origin consumption at B rises to23.43, but
this remains small relative to total consumption5600.4. Origin tracking prevents
confusing locally harvested food with imports.

Movement is not universally beneficial: in every low-cost seed, all25 people
originally at A end dead at B. Fifteen of the B-origin people remain alive at A.
This is not a migration optimum or permanent-home rule. Endpoint cohorts, transit,
resident time and all visits are retained so the opposite population movements
are visible instead of hidden in a settlement total.

### Capability and allocation remain separate

At36h in the complementary arm A has mean1424.33 edible quanta and33 residents,
7.73 of them critically hungry; B has31.33 quanta and9 residents,2.73 critical.
These are physical stocks, not guaranteed free, immediately usable nourishment.
At120h B has no food,500 wood, two residents both critical, and all10 cultivation
units occupied by dead owners' crops. At168h it has four critical residents with
the same dead-owner plot lock-up. Existing rights prohibit a new cultivator from
replacing or harvesting those crops. No inheritance or abandonment mechanism exists.

A meanwhile has unused cultivation units (mean10.93 at168h) and dead-owner units
(14). Low-cost B at120h has3.93 critical agents alongside someone else's edible
loose food on average. This is measurable coexistence, not proof that any proposed
barter would benefit both owners. Land is not owned by implication; the active crop
claim is what blocks those occupied units. Death does not redistribute property.

Actual production is concentrated at A: baseline2663.2 versus150.4 at B. That
reflects geography and current behavior. It is **not evidence of export-oriented
specialization**, because no reciprocal wood supply or export response is present.

## Labor, capital, paths and churn

Every living millisecond belongs to one activity. Baseline mean hours are:
travel468.83, queues339.10, sowing199.64, harvesting155.82, gathering1.52,
sleep1784.17 and idle3313.19. Low cost gives travel170.46, queues177.28,
sowing225.65, harvesting188.87, food gathering1.73, wood gathering0.03,
cart construction0.31, sleep1971.45 and idle3963.88.

Travel plus waiting falls by460.19h; cultivation labor rises59.06h. These are
observed allocation differences. It would be false to assign all saved travel
hours to farming: rest, idle time and living-person-hours also change. Loaded travel
includes personal food and waste, not just communal relief. There is no abstract
opportunity-cost state in the kernel; exclusivity creates the displaced time.

No arm completes a cart. Baseline/asymmetric cart forecasts repeatedly encounter
insufficient local wood; symmetric worlds show no known carrying demand. In the
low-cost arm89 projects begin across30 runs, committing890 wood and9.3944 actual
labor-hours. At168h all89 remain incomplete and all their investors are alive.
Mean construction per project is0.10556h against4 nominal work-hours, so these are
interrupted investments, not usable carts. There is no completed-cart idle time,
cargo, access denial or dead-owner cart loss to attribute here. Gate-event counts
are emitted on changed forecast text, **not a count of every investment evaluation**.
See [capital-projects.json](capital-projects.json) and per-run `projects.csv`.

Paths retain existing laws. At168h their mean compaction is65.93 on A—X and54.47
on X—B, reducing nominal work from2h to1.7363h and1.7821h. Capacity remains4;
surface recovery still occurs. Path feedback changes flows and losses without
producing wood exports or improving aggregate nourishment in this comparison.

Churn is instrumented, not repaired. Complementary seed0 has157 queue exits for
sleep and two queued-route switches; low-cost seed0 has38 and0. Of106,181 baseline
sleep starts,91,671 interruption records describe already-completed sleep ticks;
only484 are early sleep interruptions. Counting all starts as oscillation would be
misleading. There is no churn-policy ablation here, so no causal percentage of
economic failure is assigned to churn. The F7 dead-end failure is preserved exactly.

## Economic institution decision evidence

| Question | Evidence and limit |
|---|---|
| Heterogeneous value comparison | B's wood remains local while food pressures persist. The policy has no bilateral comparison mechanism. Failure to compare offered bundles is not tested; no offers exist. |
| Double coincidence of wants | **Not testable here.** No reciprocal offer protocol; zero barter attempts/completions are structural absence, not failed bargaining. |
| Intertemporal exchange | Production and cart investment are intertemporal, but no promises or delayed-exchange contracts are present. No isolated contract failure was measured. |
| Labor valuation | Exclusive time and paired allocation differences are measured. More capability can worsen allocation, but no wage/value mechanism is compared or implied necessary. |
| Capital access | No completed carts in these fixtures, so no new evidence of cart sharing/rental obstruction. F5 dead-owner capital evidence is preserved. F8 directly shows crop/process ownership lock-up. |
| Local scarcity signaling | Boards are local, food cargo depends on remembered sources and personal priorities, and no wood-demand/offer institution exists. No global surplus map or new communication is introduced. A signaling treatment is not performed, so its separate contribution is not quantified. |

The supported result is **coordination failure under specified rights, preferences,
geography and local information**, alongside occasional communal transfers. Money
would not itself reopen dead-owner crops, change the delivery priority or create
transport time. It has not been shown necessary or sufficient. Keep F9 unstarted.
Before owner-to-owner barter, pooling, lending or abandoned-property recovery is
implemented, obtain the project-owner's institutional choice and isolate it in a
new study. No such choice is needed to preserve and complete this F8 result.

## Data and reproduction

Source revisions: original matrix `f2f16e5`; corrected observer and final matrix
`7c5e701`; trace classifier `30f6bd1`. The original matrix and corrected replay are
the **same210 experiments**, not420 independent samples. Seed12 is a reproducibility
witness, not an additional independent sample. Preregistration is `7128981`.

```sh
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_settlements.py /tmp/f8-new-matrix
python3 scripts/analyze_settlements.py /tmp/f8-new-matrix /tmp/f8-analysis.json
target/release/settlement_probe low_cost /tmp/f8-seed12 1 12
python3 scripts/verify_settlements.py /tmp/f8-verification.json
```

Output paths must be new. The verifier checks the checked-in original/corrected
artifacts and regressions; compare a fresh matrix to the corrected one by seed and
checkpoint. `parameters.txt`, provenance, summary and paired JSON accompany raw data.

Data dictionary: research places0=A,1=B,2=X,3=in transit; physical graph nodes0=A,
1=X,2=B. `runs` contains checkpoints; `places` physical need/stocks/crops; `people`
starting cohort, location and physiology; `inventory` physical batches and rights;
`accounts` located resource/time ledgers; `labor` exclusive activity durations;
`flows` actual arriving goods, unchanged owner, expiry and origin; `transfers`
explicit ownership transitions; `visits` relocation; `projects/carts` investment
and completed capital; `paths` unchanged F7 feedback; `counts` event/gate totals.
Actorless event counts use a fallback bucket2: do not interpret that bucket as
physical spoilage location. Use `accounts`/`inventory` for located spoilage instead.
Full first-seed traces and the delivery witness are gzip-compressed reproducibly.

This milestone explains no-flow, communal flow, migration, transport sensitivity,
crop lock-up and incomplete capital under existing rules. Reciprocal exchange and
productive specialization remain unestablished. No institution is silently added
to turn those negative findings into a success story.
