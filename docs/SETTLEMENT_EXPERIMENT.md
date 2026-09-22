# Experiment S1 — can local rules sustain a settlement?

Status: settlement model and success measurements remain proposed. The clock, stock, travel, and gathering kernel is implemented and tested; a controlled transport probe is recorded in [the transport report](../research/TRANSPORT_REPORT.md). The settlement experiment has not run or passed.

## Falsifiable question

Can 50 identifiable agents, starting on undeveloped land with explicit finite reserves, gather and transport materials, cultivate food, and build usable shelter/storage without a scheduled build order, designated building coordinates, or a target settlement layout?

## Smallest causal model

Use a bounded 2D terrain with traversability, resource deposits, and cultivable patches. A seeded generator supplies only terrain/resources and a starting group location. A fixed reference map allows readable traces before geography sweeps.

Each person has one ID, position, current action, inventory with a carrying limit, hunger, fatigue, and a home assignment if available. One action owns their labor at any time. Movement consumes time at a defined speed. Arrival is required for gathering, transfer, work, eating from a store, and use of shelter. An initial implementation may omit crowd collision; record this approximation rather than suggesting simulated crowd forces.

Food, wood, and stone occupy deposits, the ground, inventories, or storage. Gathering transfers finite deposits into carried goods using labor and time. Farming transforms seed food, cultivation labor, land, growth time, and harvest labor into food through an explicit natural-production ledger. Unharvested crops are not edible stock. Consumption and spoilage are explicit sinks. Building recipes consume delivered goods and local labor; completion creates real shelter or storage capacity. Record materials embodied in buildings.

Hunger accumulates from elapsed time, eating reduces it, and prolonged deficit affects work capacity and ultimately survival under a documented rule. Fatigue accumulates from activity and recovers through sleep, which prevents simultaneous work. Safety initially means exposure and access to shelter, not an abstract military threat: unsheltered sleep recovers more slowly under a declared exposure parameter. Houses provide finite sleeping places. Storage offers finite protected capacity and a declared spoilage difference from exposed stock. These are provisional assumptions to ablate, not historical facts.

Agents choose from nearby observed opportunities using bounded need urgency, expected benefit, travel cost, and feasible capacity. They can gather, haul, cultivate, harvest, build, eat, sleep, or wait. Persist long enough to avoid per-tick job oscillation, with urgent need interruption. Reserve stock, work slots, and sleeping places transactionally; interrupted reservations expire without duplicating goods. Construction proposals sample feasible locally known sites and evaluate travel/access and space constraints. No global blueprint or hardcoded house/storehouse/field construction sequence.

Use local sight and remembered observations in the first model. Planning may use stale information; action validation checks the actual local state on arrival and can fail. No remote stock transfer or omniscient dispatch disguised as local behavior.

## Accounting and observation contract

Every transition records tick, actor, location, rule, relevant inputs/outputs, and cause/event IDs. Maintain per-resource balances: initial stock + explicit production = current free/carried/embodied stock + consumed/destroyed stock. People occupy exactly one location/action; reserved goods remain in one physical location until moved.

Record job choice, rejection, departure, arrival, pickup, delivery, construction work, completion, planting, growth, harvest, eating, sleep, shortage, and death. Rendering reads this state through observer snapshots. Headless and rendered runs under the same tick-stamped commands must have identical authoritative state hashes. Frame rate cannot affect outcome.

## First tests and implementation increments

1. Clock, seeded world, identities, inventories, and accounting. Test transfer conservation, local preconditions, and exclusive action/resource ownership.
2. Time-based travel, gathering, and carrying. Test that doubling route distance delays arrival, capacity limits load, and a person cannot work while traveling.
3. Needs, sleep, policies, and reservations. Test interruption/release, stale observations, finite shelter occupancy, and contention for the last stock item.
4. Farming, construction, and site choice. Test missing inputs/labor blocking completion, crop timing, and finite land/building capacity. Building completion must depend on delivered inputs and labor, not a timer alone.
5. Read-only primitive renderer and observation projection. Test hidden-state filtering and headless/rendered equivalence.
6. Execute experiments and publish raw results, summaries, causal traces, and failures before expanding.

## Runs and measurements

Use one simulation day as 24 simulation hours and initially run 180 days. Exact rates, units, recipes, initial reserves, and timestep must be versioned before running. Initial candidate timestep is one simulated minute; repeat selected runs at half and double that step to expose numerical artifacts. This is a numerical proposal, not an observed adequate resolution.

Run seeds 0–29 paired across configurations. Sweep initial food reserves (3, 10, 30 days at the declared per-person consumption rate), resource distance (near/far), agricultural yield (0.5x, 1x, 1.5x), and site-selection tie-breaking seeds. Separate environment, policy, and event random streams so paired changes remain interpretable. Keep all failed seeds.

Track daily population, hunger deficit person-hours, fatigue, sheltered sleep, food/wood/stone stocks by location, planted/harvested area, production/consumption/spoilage, completed houses/storage, occupancy, labor and travel by job, blocked jobs, job switches, transport load, and conservation residuals. Preserve positions/layout and the full parameter set, source commit, seed, command, and state hashes.

Provisional functioning-settlement gate for a run: at least 45 of 50 agents survive; during days 151–180, at least 95% of food demand is met, local food production covers consumption plus spoilage, at least 90% of sleep person-hours are sheltered, and at least one completed storehouse is actually used. Zero conservation/exclusivity violations is mandatory. These thresholds are engineering hypotheses, not gameplay balance targets. Report components and distributions, not just pass/fail; declare robustness only if at least 24/30 reference-configuration seeds pass, and publish sensitivity separately.

## Attempts to falsify the explanation

- No initial food: expose startup starvation/deadlock rather than secretly supplying food.
- Zero viable farmland or insufficient resources: expect unsustainability; explain any surviving run from actual remaining stocks.
- Disable transport: remote goods must not reach construction or consumers.
- Disable farm production: finite reserves and gathered food must explain all survival.
- Disable construction labor: no building may complete.
- Remove shelter's recovery benefit and protected storage's spoilage benefit separately: measure whether building demand changes; unchanged behavior may reveal scripted choices or other incentives requiring explanation.
- Increase travel distance or lower carrying capacity: measure delivered throughput and need failures; investigate unchanged outcomes.
- Repeat with altered seed/layout and timestep: report whether success depends on a lucky map, tie-breaking order, or numerical artifact.

For successful and failed runs, reconstruct measured event chains with IDs and timestamps. For example, only claim distance → late seed delivery → missed harvest → hunger if those exact links exist in the trace. Report alternative causes and unsupported links.

Before adding a second settlement, publish what sustained this one, what failed, which assumptions dominate, and the next falsifiable question. No results are asserted in this document.
