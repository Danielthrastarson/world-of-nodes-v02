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
F5 passed under personal cart rights; [evidence](research/capital/REPORT.md) preserves inefficient investment and capital lock-up. F6 passed under [communal crossing rights](docs/INFRASTRUCTURE_RIGHTS_DECISION.md); [420-run evidence](research/infrastructure/REPORT.md) preserves unknown-resource inaction, unfinished inputs and congestion deaths. The [player action language](docs/PLAYER_ACTION_LANGUAGE.md) is conceptual only. No trade, money, authority, combat, graphics or multiplayer is implemented. Older scope statements below remain historical.

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

Long-term player control follows the [typed, compositional action language](docs/PLAYER_ACTION_LANGUAGE.md): stable top-level Q/W/E/R semantic categories, contextual 1–4 choices and simple mouse selection/targeting. Exact bindings remain provisional. Optional Jev assistance may rank permitted contextual actions using player knowledge; it cannot redefine core verbs, create semantics, reveal hidden truth or execute unconfirmed commands. This is a future UX architecture constraint, not authorization to implement Jev or the final UI during current simulation milestones.

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

## Current F7 authorization and future army controls

Proceed with [civilian path feedback](docs/PATHS_EXPERIMENT.md), preserving all F6 evidence. [Army player control](docs/ARMY_PLAYER_CONTROL.md) is a future architecture target: formation objectives and geometry, actual mobilized people/resources, stable categories and shared logistics. Do not implement military systems, their UI or a separate military movement layer during F7.

F7 now passes at the minimal use-created path scope: [420-run evidence](research/paths/REPORT.md).
Completed travel changes ground-edge compaction and later traversal work; disuse
recovers the surface. Capacity/FIFO, policy, institutions and historical worlds
remain unchanged. Paths improve some outcomes and worsen others; three dead-end
seeds expose existing queue/exploration/sleep churn with starvation deaths. Preserve
these findings. Engineered roads, maintenance jobs, F8 and military implementation
remain deferred. All 1,396 preceding research files retain their checksums.

## Current F8 result — supersedes the F8 deferral above

The owner accepted F7 and authorized two-settlement coordination under existing
institutions. [F8 evidence](research/settlements/REPORT.md) now passes that bounded
gate: 210 paired runs with complementary/symmetric resources, travel-cost and
cart/path controls. No F7 law, autonomous priority, institution or information
channel changed. All 1,589 preceding research artifacts remain byte-identical.

Both locations are in the existing Commons(0), not newly created sovereign groups.
Short routes allow voluntary communal deposits in6/30 seeds; no wood exports or
bilateral barter occur. Migration remains valid and dominant. Food can physically
arrive but be consumed by its carrier instead of deposited. Paths reduce waiting
while worsening average nourishment in this geography. No cart completes; some
low-cost investments remain unfinished. Dead-owner crop claims block production.

Preserve scarcity, productive capability and allocation as different layers.
Opportunity cost follows from exclusive physical labor/resource uses, not a new
kernel score. Instrumentation measures outcomes without entering decisions or hashes.
The observer's exact-arrival/expiry classification defect was repaired; both original
and corrected research outputs are preserved, with identical physical histories.

No new exchange/property institution was selected. Barter attempts are absent
because no bilateral protocol exists, not because bargaining was tested and failed.
Money/prices are not automatically justified. F9 remains unstarted; any barter,
pooling, lending or abandoned-property recovery rule requires an owner decision.
Military, final UI, engineered roads and staffed maintenance remain deferred.

## Current F9A property/exchange decision and result

The owner authorized voluntary alienation of personal goods and immediate atomic
local barter. This resolves the earlier barter-rights fork only: no money, credit,
forced sale, inheritance, employment, lending or other asset-title mechanism.
[Registered semantics](docs/BARTER_EXPERIMENT.md), [evidence](research/barter/REPORT.md).

F9A passes for mechanism correctness and explanatory research. The kernel validates
both owners, their co-location, idle availability and physical inputs before either
side changes. Outputs remain at the meeting node as recipient-owned loose goods;
age, quantities, travel/capacity and subsequent action requirements remain intact.
Offers are local and finite-lived; no global market or private counterparty-state
queries enter policy. Historical configurations retain barter disabled.

360 paired runs produce zero live barter settlements. The50-person enabled F8
worlds create just one offer. Seven recipients decline its food as unwanted; one
hungry recipient lacks owned payment goods. Controlled co-location creates offers
but work/departure prevents settlement. A static two-willing-policy witness does
settle and use the receipts; it is not a live market result. No qualifying live
double-coincidence failure is demonstrated. No money has been shown necessary.

The provisional willingness rule uses food needs/protected buffers and cart inputs
under personal beliefs. It is a new bounded behavioral assumption, not optimal
economic reasoning. No speculative resale, exchange-directed acquisition, trade
search or waiting commitment is added after the negative result. Preserve F8's
communal arrival-without-deposit arbitration exactly. All1,889 prior research files
and120 disabled F8 run histories remain unchanged;156 tests pass.

Keep capital (capability), allocation (exclusive uses), exchange (voluntary rights
transfer) and money (a possible later coordination institution) separate. F9B
monetary implementation remains unstarted; any currency requires an explicit owner
decision. Do not introduce a currency to bypass missing offers or missed meetings.


## F9B — physical local market (current)

The owner authorized persistent deposited local barter, not money. F9B now names
this market experiment; the older F9B monetary label above is superseded. Monetary
institutions still require a separate owner decision.

[Specification](docs/MARKET_EXPERIMENT.md), [final report](research/market/REPORT.md),
[validation](research/market/validation-final.json). Source 3168814; final matrix
research/market/matrix-final. F9B PASSED for mechanical correctness and explanatory
research. Existing default configurations and F9A direct barter remain available;
market_nodes is empty by default. No offer-generation, willingness, migration,
communal delivery priority, capital or farming rule was altered.

Market offers deposit actual personally owned goods; ownership and age persist.
Only local acceptance, local cancellation, spoilage or the fixed 12h expiry release
escrow. Acceptance transfers both owners atomically at that same market; payment
never follows the absent proposer. Deposited prior consent persists through remote
owner death until closure, with payment retaining that owner's identity. No inheritance.
One-outstanding-offer admission uses a private recorded commitment, not remote truth.
Normal goods merging is preserved; receipt accounting is observational.

360 final runs, six fixtures x two arms x 30 paired seeds. Co-location produces one
10-wood/2-food swap per run, after the seller departed. Seller later returns and eats
the payment locally. Acquired wood remains unused and eventually dead-owner property.
All 120 larger-world market runs together produce one offer and no swap, so broad
autonomous trade remains absent. Persistent offers do not create economic motivation.
No qualifying live double-coincidence failure or money need is established.

Preserve the new negative result: deposits can induce posting/cancellation churn.
Co-location has 32 posts per run, separation 356. Formation code was not tuned; the
existing capital evaluator loses its reason to want wood when escrow removes haulable
food and regains it on cancellation/release. Separation improves later farming and
survival with zero swaps; do not attribute that welfare effect to trade.

173 tests pass, 180 direct F9A histories reproduce, 2,195 prior research artifacts are
unchanged, and all final runs have zero material errors and FIFO overtakes. Two
implementation failures were caught: remote admission leaked settlement state, and
receipt instrumentation fragmented physical batches. Their failed tests, 720
superseded runs and preliminary traces remain archived. Use matrix-final for claims.

No money, new communication, merchant behavior, waiting rule, offer-formation fix
or further institution has been implemented. F9B ends with evidence; expansion waits
for the project owner's next direction.


## F9 closure / F10 current authority foundation (2026-09-23)

Owner accepted F9: PASSED, **MONEY NOT CURRENTLY JUSTIFIED**. Rare offer formation
is **OPEN ECONOMIC RESEARCH DEBT**. Preserve all barter/market research without
merchant work, trade search, speculation, stronger offer generation or currency.

F10 now PASSED at the owner-authorized minimal scope. [Specification](docs/AUTHORITY_EXPERIMENT.md),
[report](research/authority/REPORT.md), [validation](research/authority/validation.json).
Implementation `0f0f6d5`; study source `a4b42a8`; registration `9c3937f`.

Polity, existing embodied sovereign and steward office have distinct identities.
AppointSteward requires a living sovereign/candidate at the same node. It records
an assignment without routes, remote information, resources, travel or forced work.
Known personal assignments and local institutional observations are separate from
server/research true-office queries. Effective administration is derived from valid
office + living holder + physical presence at its settlement. Territory is the set
of administered settlements. Holder death/departure removes presence-based authority;
historical office records persist. Sovereign death prevents further appointments
but does not automatically revoke existing delegations. No succession/replacement.

In opt-in authority worlds, only an active local steward may commission the existing
public crossing opportunity. Donations/building then use unchanged F6 voluntary
mechanics. Commission persists after issuer departure/death; crossing stays Commons(0).
There are no taxes, seizure, compulsory labor, tolls, conscription or border rules.
Existing scenarios default to authority=None and retain their previous behavior.

Seven deployment mechanism controls: local authority starts0.5h; distant8.556h;
cap1 with eight earlier applicants causes32.08h wait and death mid-edge at36h,
without arrival. Departure after one hour residence yields just1 administration-hour.
Two real stewards establish two offices; no abstract radius or magical replacement.
Controlled deployment is research input, not a new sovereign/steward autonomous AI.

180 project runs (three conditions, two arms, seeds0–29): sufficient known-benefit
commissioned crossing completes30/30 at8.055556h using20 wood and8.005556 actual
labor-hours; pending controls0/30. Scarce wood stalls at5/20; discovery without
known benefit produces no contributions. All 30 seeds have identical measured
outcomes within each condition. Success does not show spontaneous government.
Successful fixture:40 alive72h,1991 food consumed,726.017 starvation-person-hours,
38 users/400 completed crossing trips. Steward departs14 times, leaving29.169h
without local authority. Existing policy, migration, congestion and starvation persist.

183 tests, clippy/fmt pass;40 historical scenario runs exactly reproduced;3,131
prior research files unchanged. New studies have zero material errors and FIFO
overtakes. Preserve all evidence in research/authority. F11 is not implemented;
no further institution or behavioral fix is adopted after the F10 report.


## Human foundation insertion — current authorization (2026-09-23)

The owner accepted F10 and paused the roadmap before F11. Insert:
F10 → B0 General Person → B1 Household/Kinship → B2 Relationships/Affect →
B3 Life Cycle/Reproduction → F11. Military must not resume before these gates pass.

B0 is a behavior-preserving refactor; [registered contract](docs/PERSON_FOUNDATION.md)
and [architecture](docs/PERSON_ARCHITECTURE.md). One embodied person must support
existing economic activities and institutional roles through bounded personal
information and the same action executor. Do not add separate farmer, trader,
steward or soldier brains. Preserve all old findings, rights and physical rules.

B1 begins with seeded kinship/households, no reproduction/inheritance or household
super-agent. Care/sharing must be explicit experimental assumptions; family does
not automatically pool property. B2 requires directed relationship history and
causal event appraisal, with demonstrated effects on later decisions. No decorative
emotion values. B3 requires only causal biological/life-cycle variables and an
explicit time-scale contract; no sex-based psychological or occupational stereotypes.
Social roles may depend on individual variation, constraints, geography, technology
and institutions. Culture is accumulated social information, not arbitrary bonuses.
None of B1–B3 is implemented by the B0 refactor.


## B0 result — common person foundation

B0 PASSED at behavior-preserving architectural scope. [Report](research/person/REPORT.md),
[replay validation](research/person/replay/validation.json), [architecture](docs/PERSON_ARCHITECTURE.md).
Registration `bcc57b6`; core refactor `2c640d2`; replay source `c35e729`.

`Person` is a compatibility alias of the existing embodied `Agent`, not a second
identity/body. One personal Context composes timestamped capability memories and
bounded domain opportunity evaluators. Runtime role-like farming/capital/construction
dispatch is removed. Hunger/fatigue predicates are derived from the same body and
used at the same priority points. Local barter choice is pure, with research-only
classification and observer effects applied separately in historical order.

Typed person inspection exposes current needs, beliefs, route goal, undertaking,
atomic action, own roles and recognized rights without refreshing remote knowledge.
An ActionPlan represents the selected next command and existing undertaking; it
does not turn reactive behavior into a newly committed multi-step planner. Shared
physical/institutional validation remains authoritative. F10 offices still do not
create an autonomous government policy, forced residence or special physiology.

192 tests pass, including nine new common-person checks. All 587 registered
regression executions match preserved physical metrics, state/event hashes and
recorded traces: full F10 (187), historical panel (40), full direct/market comparison
(360). Panels overlap; do not present this as 587 independent experiments. All
3,638 prior research files remain byte-identical. No expected physical outcome was
changed, no negative finding tuned away, and money remains unjustified.

B1–B3 remain unimplemented. B1 is the next dependency: seeded household/kinship
with an explicit bounded care/sharing experiment, not automatic pooling or
inheritance. F11 remains BLOCKED until B0–B3 pass. No military expansion began.

## Current increment: B1 household and directed care

B0 accepted. B1 is authorized with personal ownership, explicit caregiver obligations, local observation, and voluntary bounded care. No pooling, inheritance, compulsory sharing, emotions, reproduction or military. Registered rules: docs/HOUSEHOLD_EXPERIMENT.md. Historical results remain authoritative evidence, not outcomes to optimize.

### B1 result

B1 PASSED at seeded structural-care scope; research/household/REPORT.md records evidence. Personal property remains separate from household/kinship and directed Caregiver relations. 660 paired-condition runs, 210 tests, 227 exact historical replays and 4,410 unchanged old research files. Care can transfer food, shorten caregiver survival, fail after relatives pass on a road, and redistribute construction work. No pooling, inheritance, compulsory sharing, affect or reproduction. Source79515ee; original negative findings remain intact. B2/B3 and F11 are not implemented.
