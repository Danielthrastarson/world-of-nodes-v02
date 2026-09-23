# World of Nodes — current design contract

Updated 2026-09-22 following the owner's design change.

## Current authorization — foundation roadmap

The owner now authorizes [the gated foundation roadmap](docs/ROADMAP.md), starting
with F0 mechanical adoption and conditional-meal research, then F1 geography after
F0 passes. Earlier “no expansion” statements below describe historical study scopes;
they do not override this newer authorization. Preserve all published evidence.
Only the six mechanical corrections become normal prediction. Conditional meals,
resumable/committed intentions and new institutions are not default behavior.
Proceed through passed gates; stop expansion at a failed foundation or an owner-level
institution/game-design decision. See [the registered F0 study](docs/CONDITIONAL_MEALS.md).

## Foundation status — 2026-09-23

F0, F1 and F2 passed their causal-capability gates. See [the roadmap](docs/ROADMAP.md)
for evidence and exact definitions. Mechanical forecasting is adopted; meal-aware
planning is still research-only and worsened the locked fixture. Six geography
fixtures use one policy; a literal-node-ID request backlog defect was repaired.
A separately registered compact line yields voluntary communal deliveries, but
self-only migration still provides better nourishment. No claim of sustainable
settlement or validated efficient cooperation follows.

F3 now passes under the owner's cultivator-owned crop/output decision, with no land
title or inheritance. See [production evidence](research/farming/REPORT.md). Output
requires local labor, growth time and harvest; finite-food controls remain unchanged.
F4 also passes its pressure-dependent labor gate; see [evidence](research/labor/REPORT.md).
F5 is blocked on [cart rights](docs/CAPITAL_RIGHTS_DECISION.md). No capital, construction, trade, money, authority, combat, graphics or
multiplayer is implemented. Older scope statements below remain historical.

## Authority and scope

This document supersedes the earlier requirement to implement V0.2 security first. The long-term first slice is a single emergent settlement. The immediate authorized increment is now [local provisioning under scarcity](docs/PROVISIONING_SPEC.md), whose locked rules take precedence for this work. Farming, construction, safety, rendering and later systems remain deferred. Military pressure follows settlement, transport, and economic validation. This is a scope change, not evidence that the security loop passed and not a claim of V0.3 completion.

The original handoff is preserved verbatim in [the security handoff archive](docs/archive/WORLD_OF_NODES_HANDOFF_SECURITY.md). Its V0.1 findings remain unverified inherited claims: no original source, seeds, or results were supplied. Do not reconstruct them and label that reproduction.

## Current bounded research follow-up

The accepted provisioning failure is preserved. The delivery decomposition separates
physical feasibility from autonomous acceptance and commitment; see the
[research contract](docs/DELIVERY_DECOMPOSITION.md), [physical bounds](docs/DELIVERY_BOUNDS.md)
and [results](research/delivery/REPORT.md). The reference world and default policy
remain unchanged. Research schedulers may inspect global state only as explicit
counterfactual instruments, never as autonomous policies. No later systems or
V0.3 are authorized by these results.

The subsequent [acceptance experiment](docs/ACCEPTANCE_EXPERIMENT.md) and
[results](research/acceptance/REPORT.md) isolate prior source knowledge, meal-aware
forecasting and the end nutrition margin. All 120 runs retain zero accepted jobs
and deliveries. Some forecasts permit pre-claim board visits, but none completes;
the two forecast variants worsen survival. No diagnostic variant becomes the
default. Preserve the previous evidence, physical rules, board locality, migration
and starvation-reset behavior; this result authorizes no broader systems.

The bounded [temporal-consistency experiment](docs/PERSISTENCE_EXPERIMENT.md) now
has [150 paired results](research/persistence/REPORT.md). None completes a board
visit. Goal memory alone is behaviorally identical to the meal-aware control;
resumption/commitment change arbitration but encounter FIFO delay and survival
interrupts. The isolated ordinary-12h-priority ablation changes no decisions in
that history. Local goal churn exists, while the registered uninformed board-route
reversal count is zero. These observations do not establish that forgetting alone
causes the failure or select a final intention architecture. Preserve all earlier
evidence and default behavior. No further behavior change or scope expansion is
authorized by this report.

The subsequent [shared-edge congestion experiment](docs/CONGESTION_EXPERIMENT.md)
has [conditional transport and paired forecast results](research/congestion/REPORT.md).
Pending migration competes for shared FIFO capacity until existing hunger/fatigue
overrides interrupt board visits. Equal physiological demand has the same physical
cost in either direction. Reducing pending load allows the earlier cohort to reach
C; later arrivals also encounter existing occupants and earlier returners. A separate
forecast self-count correction changes behavior and survival but still produces zero
board arrivals, jobs or deliveries. Traffic does not make all physical returns fatal
under the starvation grace rule. These results support interacting transport and
policy mechanisms, not adoption of commitment, reduced migration or a transport
priority rule. All default behavior and earlier evidence remain preserved.

The [shadow forecast audit](docs/SHADOW_AUDIT.md) now has
[observational evidence](research/shadow/REPORT.md). All 240 audit-on/off pairs
preserve historical hashes and all 297 earlier research files remain unchanged.
Primitive contracts conform in the tested fixtures, while composite forecasts show
queue-order, self-counting and acquisition-model discrepancies. Conditional kernel
witnesses separate these defects from information gaps, capacity competition and
explicit margins. No BDI/GOAP architecture, new institution, default forecast fix or
policy change is adopted. Existing negative provisioning findings remain intact.

The [forecast coherence increment](docs/FORECAST_COHERENCE.md) now has
[static, offline and live evidence](research/coherence/REPORT.md). The six proven
mechanics defects are corrected only in an explicit research forecast mode. All
540 covered composite/source-prefilter kernel comparisons and 130 primitive checks pass. Offline
forecasts change, but the isolated 30-pair live delivery comparison changes zero
selected actions and preserves physical state hashes: zero board return arrivals,
jobs and deliveries; 28 survivors and 944 starvation-person-hours. The current
no-meal-credit assumption and six-hour end margin remain intact. No correction is
adopted as default; no new architecture or institution is authorized by the result.

## Principle

**The simulation is the source of the graphics.** The rendered world is a projection of underlying simulated state. Do not program a desired settlement outcome; program causal rules that can produce it. Do not animate a result when its cause can be simulated.

Preserve nodes, edges, stocks, identifiable people, travel time, capacity, conservation, exclusivity, locality, transformation, agency, and one coherent clock. Simulation must run headlessly. Geometry may be continuous while routes and institutions use a graph.

Low geometric complexity, high physical complexity is the long-term direction. Primitive geometry is sufficient. Detailed meshes, textures, prerecorded animations, and expensive art pipelines are not priorities.

## Architecture

1. **World simulation:** authoritative identities, location, resources, production, construction, settlements, needs, information, and time. Later: trade, territory, military. Explicit transformations and natural production account for every resource change.
2. **Decision system:** cheap rules or utility functions select among available actions. Intent is validated and executed by the world; a decision cannot create goods, teleport agents, or bypass capacity. Settlement decisions use the same boundary. Jev may later support higher-order decisions; no neural model per NPC or frame.
3. **Physics:** a replaceable subsystem, potentially Rapier later. Forces, collisions, impulses, joints, and raycasts only where justified. Cheap movement remains sufficient for most agents. Promotion to detailed bodies preserves identity, position, carried stocks, and clock ownership; it never creates a second authoritative person. Physics results enter world transitions, not renderer-only effects.
4. **Rendering:** Rust with a lightweight renderer, initially Macroquad or an alternative selected at implementation time. Keep it independent from simulation. Later wgpu/instancing is an option, not a current dependency. Draw circles, lines, boxes, polygons, and particles from read-only snapshots.

Proposed dependency direction: decision policies read observations and submit intents to the world; world advances validated actions and optional physics; observation projection produces snapshots; renderer reads snapshots. Renderer input becomes commands for a future simulation tick. Rendering never advances the world or mutates resources. Fixed simulation ticks share one clock; rendering may interpolate known positions without executing extra actions.

## Visual truth and information

- A visible person has a real agent ID. Army formations eventually display actual soldiers at actual positions.
- A field represents cultivated land and its crop state. Buildings show delivered materials and completed labor, not elapsed animation time.
- Goods carried or stored correspond to unique stock locations. Aggregate glyphs expose their represented count; decorative people or goods are prohibited.
- Roads require actual construction, maintenance, or accumulated traffic. They cannot appear merely because two settlements need a visual connection.
- Player snapshots contain observed state and timestamped remembered/inferred state. Stale information must be distinguishable. An explicitly labeled research/debug view may inspect omniscient state. Policies cannot read hidden world data through that view.
- Interpolation and visual simplification are allowed; they cannot imply unmodeled damage, harvest, delivery, ignition, or collapse.

## First vertical slice

One simple or procedural map, about 50 agents, food, wood, stone, hunger, sleep, safety, gathering, carrying, farming, construction, dynamic job allocation, houses, a storehouse, fields, and primitive rendering.

The map provides terrain and natural resource opportunities, not a scripted finished layout. Construction sites and fields must emerge from local choices under needs and physical constraints. Initial food and tools, if any, are explicit experimental inputs.

Use [the settlement experiment contract](docs/SETTLEMENT_EXPERIMENT.md) before implementing systems. Failure is useful evidence. Do not widen scope to hide it.

## Later progression

After the settlement experiment is understood:

1. Two settlements.
2. Local scarcity/value.
3. Physical goods transport.
4. Profitable trade routes emerging from decisions.
5. Traffic tracking.
6. Roads emerging from traffic and maintenance.
7. Migration and settlement growth.
8. Authority and territory.
9. Military pressure and the deferred security/economy hypothesis.
10. Richer physics, lighting, fire, and destruction incrementally.

## Future physical causes, not immediate requirements

Light sources should eventually supply both visibility and rendered illumination using the same state. A candidate attenuation law is I(r) = P / (1 + k r²); choose units, occlusion, and thresholds experimentally.

Combustion should eventually depend on temperature, fuel, moisture, and ignition conditions. Heat can cause light, damage, panic, structural failure, and economic loss only when those links are modeled. No scripted fire masquerading as this model.

Detailed motion may use F = ma, v(t+dt) = v(t) + a dt, x(t+dt) = x(t) + v dt, with solver stability and collision handling verified before use. Horse impacts should eventually produce real impulses and physical reactions. Structural collapse should eventually follow failure and forces. Neither horses nor structural physics belongs in the first slice.

Torch lighting must originate in real light sources. Weather, shadows, particles, crowds, material state, and destruction may improve visuals as their causal models earn inclusion.

## Evidence discipline

Separate parameters, state, policies, and seeded randomness. Use small tested commits, conservation and exclusivity checks, paired-seed experiments, causal traces, and explicit assumptions. Record failures, dead mechanics, sensitivity, and numerical artifacts. Never silently adjust rules or success thresholds to obtain an attractive outcome.
