# World of Nodes

**The simulation is the source of the graphics.** A causal kingdom simulation with primitive rendering of real agents, stocks, movement, work, and observed state.

Current work follows the [gated foundation roadmap](docs/ROADMAP.md): F0–F6 have passed; F7 traffic-driven infrastructure is next. The six tested mechanical forecast corrections are adopted; conditional meal planning remains research-only. Earlier reports below describe their original historical policies. No later systems are implemented before their gates pass.

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

Foundation gates F0–F7 passed at their documented scopes. [F1 geography](research/geography/REPORT.md) and
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

The [future army-control target](docs/ARMY_PLAYER_CONTROL.md) documents formation
intent, geometry, stable QWER categories and shared civilian logistics. No military
system, final UI or Jev integration is implemented.
