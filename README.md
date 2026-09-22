# World of Nodes

**The simulation is the source of the graphics.** A causal kingdom simulation with primitive rendering of real agents, stocks, movement, work, and observed state.

Current scope: **local provisioning under scarcity**—hunger, sleep, starvation, physical spoilage, local memory, communal requests, autonomous decisions, and FIFO transport. No farming, construction, trade, safety, graphics, or physics in this increment.

- [Current design contract](WORLD_OF_NODES_HANDOFF.md)
- [Locked provisioning specification](docs/PROVISIONING_SPEC.md)
- [Preserved provisioning results](research/provisioning/REPORT.md)
- [Delivery mechanism decomposition](research/delivery/REPORT.md)
- [Physical feasibility bounds](docs/DELIVERY_BOUNDS.md)
- [Deferred settlement experiment](docs/SETTLEMENT_EXPERIMENT.md)
- [Preserved earlier security handoff](docs/archive/WORLD_OF_NODES_HANDOFF_SECURITY.md)

The 2026-09-22 design change defers the earlier V0.2 military/security work until settlement and transport foundations are validated. It does not establish that the security/economy loop passed.

Run the new model with `cargo run --offline --release --bin provisioning_probe -- reference 0`. Run checks with `cargo test --offline`. The original `transport_probe` and its evidence remain preserved as a legacy baseline. The new provisioning model is versioned separately in the same crate.

The reference produces migration toward food, 28 survivors at 72h, and zero communal deliveries. All those survivors have zero nutrition at 72h. This is a measured failure of communal provisioning, not a validated sustainable settlement.

The delivery follow-up demonstrates 280 physically transported quanta against zero autonomous delivery, with a 430-quantum upper bound (exact bulk optimum unresolved). A request visible at C still produces no accepted jobs across 30 paired seeds per isolated policy variant. The default policy and original negative evidence remain unchanged. Run `delivery_probe --matrix OUTPUT_DIRECTORY 30` and `delivery_probe --envelope OUTPUT_DIRECTORY` after building the release binaries; see the linked report for reproduction and limits.

V0.1 source, seeds, tests, and raw outputs were never supplied, so its reported negative results remain inherited claims, not reproduced evidence. The archived handoff preserves them unchanged.
