# World of Nodes

**The simulation is the source of the graphics.** A causal kingdom simulation with primitive rendering of real agents, stocks, movement, work, and observed state.

Current scope: **local provisioning under scarcity**—hunger, sleep, starvation, physical spoilage, local memory, communal requests, autonomous decisions, and FIFO transport. No farming, construction, trade, safety, graphics, or physics in this increment.

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

Run the new model with `cargo run --offline --release --bin provisioning_probe -- reference 0`. Run checks with `cargo test --offline`. The original `transport_probe` and its evidence remain preserved as a legacy baseline. The new provisioning model is versioned separately in the same crate.

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
