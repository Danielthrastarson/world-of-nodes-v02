# World of Nodes — current design contract

Updated 2026-09-22 following the owner's design change.

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
