# F0: minimal decision vocabulary

No BDI runtime, GOAP search or new arbitration is installed. The existing kernel
already stores the distinctions that F0 needs; `Agent::reasoning_state()` exposes
them as a read-only typed projection rather than creating a second authority.

- Belief: `PolicyMemory`, timestamped personal observations. Prediction receives
  `Observation` and memory, never a `World` reference.
- Goal: desired reachable node or food-access location (`Goal`). This route marker
  is not an undertaking, board claim, source guarantee or home obligation.
- Intention/undertaking: accepted `Job`, exposed as `Undertaking::Provision` with
  request, destination, target and acquired quantity. It spans claim, outward
  travel, acquisition, return and deposit, and can remain while the agent eats or
  sleeps. Board leases remain separate records with different lifetimes.
- Action: authoritative `Action` (idle, queued, traversal, atomic gather, sleep,
  dead). `Intent` in the existing command API means proposed atomic command; it is
  not a cognitive commitment. Commands are validated by the kernel.

Candidates record primitive action tapes. A conditional executable tape is not a
promise that the reactive policy will follow it. Pre-claim board travel stays
reactive: the previous remembered/resumable/committed diagnostics remain research
only. This preserves the evidence that temporal memory alone did not cure failure.

Cancellation/interruption remains in `Decision.reason`, queue cancellation,
reservation release and job cancellation events. `decomposition::JobRecord` tracks
accepted, sought, reached source, acquired, consumed, return, delivery, abandonment
reason and churn. Death releases actual jobs while remote board records stay stale.
Nothing in this vocabulary updates knowledge or grants priority.

The foundation gate covers known action/forecast contracts and representability,
not optimality, plan completion or general intelligence. Identical observations,
memory and RNG must produce identical decisions even if hidden world truth changes.
Evidence: the original interruption/lease/death tests, new reasoning tests, static
forecast matrix and unchanged-history regression hashes in `research/meals`.
