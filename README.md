# World of Nodes

**The simulation is the source of the graphics.** A causal kingdom simulation with primitive rendering of real agents, stocks, movement, work, and observed state.

Current work follows the [gated foundation roadmap](docs/ROADMAP.md). F0–F10 and B0–B3 are complete at their registered scopes; F11A mobilization and F11B physical unit movement have passed mechanism gates. F11A’s protective-benefit gate remains open. F11C begins with a concrete threat/control specification; combat is not implemented. Money remains unjustified and unimplemented. Earlier reports below retain their historical policies and negative findings.

- [Current design contract](WORLD_OF_NODES_HANDOFF.md)
- [Locked provisioning specification](docs/PROVISIONING_SPEC.md)
- [Preserved provisioning results](research/provisioning/REPORT.md)
- [Delivery mechanism decomposition](research/delivery/REPORT.md)
- [Acceptance gate decomposition](research/acceptance/REPORT.md)
- [Plan persistence and temporal consistency](research/persistence/REPORT.md)
- [Physical feasibility bounds](docs/DELIVERY_BOUNDS.md)
- [Deferred settlement experiment](docs/SETTLEMENT_EXPERIMENT.md)
- [Preserved earlier security handoff](docs/archive/WORLD_OF_NODES_HANDOFF_SECURITY.md)

The 2026-09-22 design change defers the earlier V0.2 military/security work until settlement and transport foundations are validated. It does not establish that the security/economy loop passed.

Reproduce the historical P1 model with `cargo run --offline --release --bin provisioning_probe -- reference 0`. New `World::new(Config::default())` uses the adopted coherent estimator. Run checks with `cargo test --offline`. The original `transport_probe` and its evidence remain preserved as a legacy baseline. The new provisioning model is versioned separately in the same crate.

The reference produces migration toward food, 28 survivors at 72h, and zero communal deliveries. All those survivors have zero nutrition at 72h. This is a measured failure of communal provisioning, not a validated sustainable settlement.

The delivery follow-up demonstrates 280 physically transported quanta against zero autonomous delivery, with a 430-quantum upper bound (exact bulk optimum unresolved). A request visible at C still produces no accepted jobs across 30 paired seeds per isolated policy variant. The default policy and original negative evidence remain unchanged. Run `delivery_probe --matrix OUTPUT_DIRECTORY 30` and `delivery_probe --envelope OUTPUT_DIRECTORY` after building the release binaries; see the linked report for reproduction and limits.

The acceptance follow-up ran 120 paired experiments. Prior source knowledge still produces no jobs; meal-aware and zero-margin forecasts permit some trips toward the board, but none reaches C to claim. Both forecast variants reduce 72h survivors from 28 to 20, with zero deliveries. Complete gate journals distinguish forecast passes from actual acceptance. These remain diagnostic variants; the default policy and both earlier evidence sets are preserved.

The persistence follow-up ran 150 paired experiments against the meal-aware control. Remembering goals alone and the isolated 12h-priority ablation leave physical behavior unchanged. Resumable and committed goals still produce zero board arrivals, jobs and deliveries, with 16 survivors versus the diagnostic control's 20. Traces distinguish local goal churn from route reversals with changed queue evidence; committed agents can remain queued until critical hunger intervenes. No intention architecture or diagnostic policy is adopted as the default.

V0.1 source, seeds, tests, and raw outputs were never supplied, so its reported negative results remain inherited claims, not reproduced evidence. The archived handoff preserves them unchanged.

The [congestion research report](research/congestion/REPORT.md) isolates shared-edge
load, directional demand and forecast self-counting. It preserves all earlier
results and the default policy; no transport priority or new NPC architecture is
adopted. Reproduce with `python3 scripts/run_congestion.py NEW_OUTPUT_DIRECTORY`
after an offline release build.


The observational [shadow forecast/action-conformance audit](research/shadow/REPORT.md)
compares personal forecasts with independent action arithmetic, kernel witnesses and
preserved execution. Run it with `cargo build --release --bin shadow_probe` followed
by `python3 scripts/run_shadow.py /tmp/won-shadow-reproduction 30` (new directory).
It changes no autonomous policy; contract assumptions are in
[SHADOW_AUDIT.md](docs/SHADOW_AUDIT.md).

The [forecast coherence correction](research/coherence/REPORT.md) is an opt-in
research variant. Covered composite forecasts now conform to the kernel; the
30-pair live delivery comparison changes no selected actions or physical outcomes
and still delivers zero communal food. The default policy remains unchanged.
Reproduce with `cargo build --release --bin coherence_probe` and
`python3 scripts/run_coherence.py NEW_OUTPUT_DIRECTORY 30`.

The [F0 adoption/conditional-meal report](research/meals/REPORT.md) adopts mechanical
corrections while retaining no-meal planning by default. The gated roadmap continues
through geography and provisioning diagnostics; each stage records its gate in
[ROADMAP.md](docs/ROADMAP.md). Historical probe commands stay explicitly reproducible.

Foundation gates F0–F8 passed at their documented scopes. [F1 geography](research/geography/REPORT.md) and
[F2 provisioning](research/foundation-provisioning/REPORT.md) preserve failures and
explain the first autonomous deliveries. [F3 farming](research/farming/REPORT.md)
uses cultivator-owned crops without land titles. [F4 labor](research/labor/REPORT.md)
measures voluntary responses to scarcity. [F5 personal capital](research/capital/REPORT.md)
implements physical carts and reports 420 paired runs: some delivery gains,
worse nourishment in other conditions, and dead-owner capital lock-up. Historical
worlds remain unchanged; farming and capital require explicit configuration.
[F6 communal crossings](research/infrastructure/REPORT.md) are physically built and
shared under finite capacity. The 420-run study preserves incomplete projects,
unknown-resource inaction and fatal queues. The future [player action language](docs/PLAYER_ACTION_LANGUAGE.md)
is documentation only; no ruler command API or traffic-generated roads were added during F6.

[F7 civilian paths](research/paths/REPORT.md) adds optional use-created ground
surface state and disuse recovery to the shared transport kernel. Across 420 runs,
paths change routes, traffic and nourishment in both favorable and harmful ways.
The autonomous policy, FIFO and capacities remain unchanged. Engineered roads and
staffed maintenance are deferred. Reproduce with `cargo build --release --bin paths_probe`
then `python3 scripts/run_paths.py NEW_OUTPUT_DIRECTORY`.

[F8 two-settlement coordination](research/settlements/REPORT.md) studies existing
institutions across 210 paired runs. Short routes permit occasional communal food
deposits; migration dominates, wood exports remain zero, cart projects stay
incomplete, and dead-owner crops remain inaccessible. No policy, property or F7
rule changes. Reproduce with `cargo build --release --bin settlement_probe` then
`python3 scripts/run_settlements.py NEW_OUTPUT_DIRECTORY`.

[F9A local barter](research/barter/REPORT.md) adds explicit alienable personal goods,
typed local offers and atomic ownership swaps. Policy uses only local holdings,
visible offers and personal beliefs. Successful static conformance does not erase
the live negative result: offer formation and meeting timing remain obstacles.
All disabled F8 controls reproduce exactly. Run
`python3 scripts/run_barter.py NEW_OUTPUT_DIRECTORY` after building `settlement_probe`.
No money, credit, inheritance, trade-search routing or communal arbitration repair.

The [future army-control target](docs/ARMY_PLAYER_CONTROL.md) documents formation
intent, geometry, stable QWER categories and shared civilian logistics. No military
system, final UI or Jev integration is implemented.


### F9B: physical local markets, no money

[Final report](research/market/REPORT.md) · [specification](docs/MARKET_EXPERIMENT.md)
· [validation](research/market/validation-final.json).

Persistent offers physically escrow goods at existing settlement nodes. Settlement
is local and atomic; payment stays at the market, and expiry/cancellation never
teleport property. F9A offer motivation is unchanged. In 360 final paired runs,
controlled co-location produces 30 delayed swaps; the 120 larger-world market runs
produce one offer and zero swaps. Custody/valuation churn also changes behavior
without exchange. No money is justified or implemented.

```sh
cargo run --release --bin settlement_probe -- colocated /tmp/won-local-market 1 0 market
python3 scripts/run_market.py /tmp/won-f9b-reproduction
python3 scripts/analyze_market.py /tmp/won-f9b-reproduction /tmp/won-f9b-analysis.json
```

The final evidence is `research/market/matrix-final`; earlier matrices in that
folder are preserved development failures, not independent confirmation runs.


### F10: authority through people and places

[Report](research/authority/REPORT.md) · [registered study](docs/AUTHORITY_EXPERIMENT.md)
· [validation](research/authority/validation.json).

An optional polity designates an existing mortal sovereign. Local appointments create
steward offices; living presence determines practical administration and derived
territory. A steward may commission an existing communal crossing locally, with no
free materials or forced workers. Seven deployment controls and 180 paired project
runs expose both functioning construction and failure from congestion, death, missing
materials and unknown benefits. No autonomous government planner or coercive power.
F9 is closed: money remains unjustified; offer formation remains open research debt.

```sh
cargo build --release --bins
python3 scripts/run_authority.py /tmp/won-f10-reproduction
python3 scripts/analyze_authority.py /tmp/won-f10-reproduction /tmp/won-f10-analysis.json /tmp/won-f10-traces.md
python3 scripts/verify_authority.py /tmp/won-f10-regressions
```


### B0: common person foundation before military

[Report](research/person/REPORT.md) · [architecture](docs/PERSON_ARCHITECTURE.md)
· [exact regression evidence](research/person/replay/validation.json).

One embodied person now exposes a common decision context, needs, existing
undertakings, next-step action plans, roles and rights. Farming, capital and public
works compose capabilities; local exchange choice is pure. The existing executor,
priorities, physical rules and institutions remain unchanged. All 587 registered
regression executions match their preserved outputs; 192 tests pass.

The roadmap now requires B0 → B1 households → B2 relationships/affect → B3 life cycle
before F11 military. B0 is passed; B1–B3 are not implemented. No implicit household
sharing, reproduction, stronger commitment policy or money was introduced.

```sh
cargo build --release --bins
python3 scripts/run_person.py /tmp/won-b0-replay
python3 scripts/analyze_person.py /tmp/won-b0-replay /tmp/won-b0-coverage.json
```

### B1 household / kinship

[Structural care evidence](research/household/REPORT.md) and [registered assumptions](docs/HOUSEHOLD_EXPERIMENT.md): personal property, separate directed caregiver obligations, local need observations and physical voluntary transfers through the common Person interface. Includes 660 paired-condition runs, negative outcomes, opportunity costs and exact historical regression checks. No inheritance, pooled property, emotions or reproduction.

### B2 relationships / appraisal / affect

[Evidence](research/affect/REPORT.md) · [focal causal chains](research/affect/analysis/TRACE_EXCERPTS.md) · [registered rules](docs/RELATIONSHIP_EXPERIMENT.md).

Four sparse directed relationship dimensions and lazy anger/fear/grief feed the common Person care arbitration. Controlled observed history changes a later autonomous choice with identical non-social inputs. Remote events do not update relationships or grief. All four live B1 comparisons preserve physical behavior; no drama or improved welfare is claimed. Historical configurations retain social disabled. No new property rights, reproduction or military.

```sh
cargo build --release --bins
python3 scripts/run_affect.py /tmp/won-b2-primary
python3 scripts/run_affect_cost.py /tmp/won-b2-cost
python3 scripts/verify_affect.py /tmp/won-b2-regressions
```

### B3 embodied generations

[Report](research/lifecycle/REPORT.md) · [registered rules](docs/LIFECYCLE_EXPERIMENT.md) · [causal traces](research/lifecycle/analysis/TRACE_EXCERPTS.md).

Optional accelerated biology adds birth time, child/adult capabilities, mutual timed reproduction, pregnancy, parentage and persistent offspring through the same Person architecture. No automatic food, property pooling, sex-linked psychology or inheritance. Controlled support reaches Generation3 in27/30 seeds; less-assisted worlds remain at Generation2 and zero-prior B2 care can fail newborns. These failures are preserved.

```sh
cargo build --release --bins
python3 scripts/run_lifecycle.py /tmp/won-b3-matrix
python3 scripts/verify_lifecycle.py /tmp/won-b3-regressions
```

B3 passed at mechanism scope. F11's service institution requires an owner decision before coercive powers or service obligations are introduced.

### F11B embodied unit movement

One locally delivered unit objective drives individual Persons through the common movement/FIFO system. Missing orders, hunger, fatigue, queue interruptions and service expiry remain causal. Unit membership grants no supply, cart access or transport priority. Continuous battlefield formations remain deferred.

[Movement evidence](research/formation/REPORT.md):960 final paired runs,267 tests,137 historical replays and9,298 unchanged prior research artifacts. The congested march increases civilian waiting and starvation. No protective benefit is claimed. [Architecture](docs/UNIT_ARCHITECTURE.md), [registered fixtures](docs/FORMATION_EXPERIMENT.md), [F11C threat opening](docs/THREAT_EXPERIMENT.md).

Reproduce into a **new** output directory:

```sh
cargo test
cargo run --release --bin formation_probe -- short move /tmp/f11b-short-move 30
cargo run --release --bin formation_probe -- short hold /tmp/f11b-short-hold 30
```

`python3 scripts/run_formation.py NEW_DIRECTORY` runs the full paired panel after building release binaries. Output creation refuses to overwrite preserved evidence.
