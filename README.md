# World of Nodes

**The simulation is the source of the graphics.** A causal kingdom simulation with primitive rendering of real agents, stocks, movement, work, and observed state.

Current scope: a small emergent settlement experiment with about 50 agents, food, wood, stone, needs, gathering, carrying, farming, construction, and local job selection.

- [Current design contract](WORLD_OF_NODES_HANDOFF.md)
- [Settlement experiment and implementation gates](docs/SETTLEMENT_EXPERIMENT.md)
- [Preserved earlier security handoff](docs/archive/WORLD_OF_NODES_HANDOFF_SECURITY.md)

The 2026-09-22 design change defers the earlier V0.2 military/security work until settlement and transport foundations are validated. It does not establish that the security/economy loop passed.

A dependency-free Rust headless kernel now implements identities, stocks, travel, gathering, and local transfers. Run `cargo test --offline` to verify it. See [kernel assumptions](docs/KERNEL.md) and [the first transport experiment](research/TRANSPORT_REPORT.md). Needs, autonomous settlement decisions, farming, construction, and rendering are not implemented yet. V0.1 source, seeds, tests, and raw outputs were never supplied, so its reported negative results remain inherited claims, not reproduced evidence. The archived handoff preserves them unchanged.
