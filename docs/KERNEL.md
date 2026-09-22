# Headless kernel — preserved legacy implementation

This document describes the original transport kernel, retained for reproduction. The active provisioning increment is specified in [PROVISIONING_SPEC.md](PROVISIONING_SPEC.md); its modules do not change this legacy experiment.

Implemented: 50 stable agent identities, a seeded three-node terrain graph, finite food/wood/stone stocks, shared carrying limits, individual edge occupancy, time-based travel/gathering, local pickup/drop, interruptible gathering reservations, and causal completion events.

No external dependencies. Run `cargo test --offline` and `cargo clippy --offline --all-targets -- -D warnings`.

## Explicit assumptions and limitations

- Integer material units have equal carrying cost; no claim of calibrated physical mass.
- Time is integer milliseconds. Travel duration is ceiling(distance / speed) at this resolution, with less than 1 ms upward error. Completions are processed at their own time within a caller's advance interval. Policies will still need a specified decision cadence on this same clock.
- Edges are traversable terrain connections, not existing roads. Their lengths may exceed straight-line endpoint distance. No collision geometry or intermediate route shape is modeled yet; do not render a detailed path as authoritative.
- Edge capacity counts simultaneous individual travelers, shared in both directions. Admission is immediate or rejected; callers retry. There is no party-size gate, queue fairness, congestion speed law, or transport dispatch policy yet.
- Gathering reserves finite source stock and carrying space, then transfers it at completion. Cancellation releases the reservation and loses unfinished work. Pickup/drop are instantaneous local transfers; loading labor is not yet modeled.
- Stable agent-ID order resolves simultaneous completions; submission order resolves contention. This can favor earlier callers. Randomized or fair dispatch must be tested before crowd-level claims.
- All public read-only views are research/debug views, not player observations. No observer filtering, autonomous decisions, hunger, sleep, farming, construction, renderer, or physics exists yet.
- Completion events refer to their initiating command; transfer commands record actor, place, quantity, and time. Cross-action causal attribution is not yet automatic. Invalid unknown-agent commands return an error without an event because no actor exists.
- Current material accounting is closed: there is no production or consumption. Those systems must extend the ledger with explicit sources/sinks before inclusion.

## Validation

Ten tests cover conservation, locality, shared carrying limits, action exclusivity, edge contention, interrupted reservations, exact arrival timing, time overflow/reversal, invalid scenarios, and partition-independent completion/event histories. Randomized command testing covers 30 seeds × 2,000 attempted commands, auditing invariants after each step. This validates a transport kernel, not a functioning settlement.
