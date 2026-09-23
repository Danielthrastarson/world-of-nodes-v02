# F1: geography/generalization gate

**PASSED.** One adopted no-meal policy runs every registered topology. No policy
special cases, source priors, new institutions or parameter tuning were introduced.
Each map has30 seeds,50 agents, food3000 created0/expiry48h and request600 at C.
Only registered topology/source placement and the bottleneck capacity differ.

| Map | Survivors72h | Jobs accepted | Delivered | Consumed | Starvation-person-hours |
|---|---:|---:|---:|---:|---:|
| Line |28|0|0|1264|944|
| Branch |40|8|0|1800|940|
| Alternative routes |50|0|0|2366|634|
| Dead end |16|12–37 (mean17.9)|0|697–708|1068–1079|
| Bottleneck |6|0|0|263|1153|
| Unequal sources |32|8|0|1564|1048.457–1071.492|

All180 runs preserve material accounting and FIFO, with0 overtakes. All13,140
hourly checkpoints pass the physical audit. Branching maps have30 distinct event
histories; line and bottleneck have one. Identical aggregate survival is not evidence
that stochastic trajectories are identical. Per-node populations, personal memories,
health checkpoints, full seed0 events and per-run metrics are preserved.

## Gate failure found and repaired

Before the matrix, relabeling the line's board from node0 to node2 changed its
request from600 to0 at t=0. The scalar initial request backlog was accidentally
bound to literal node0 in refresh and delivery. It now follows the configured
primary board. The failed test is preserved in `relabel-failure.txt`.

The repaired test renames nodes and reverses edge IDs, checking request state and
normalized agent physiology/location plus all physical metrics through72h.
Additional tests move hidden food without changing observations and ensure no
initial action changes; every remembered node in all six worlds must have been
physically visited, and food-directed goals require source memory. Nine previous
historical scenarios still reproduce exactly after the repair. Current remains
identical to the previously validated coherent mode on all19 original scenarios.

The full suite passes (77 library +12 integration/binary tests), format and clippy
checks pass. Source revision77a7ddd. See `matrix/provenance.json` for evidence hashes.

## Causal interpretation

Alternative routes distribute access and give everyone a path to finite food;
this is migration, not provisioning or sustainability. The bottleneck adds shared
scarce capacity on the only route and only six agents survive. Dead-end exploration
can consume time without nourishment. Unequal sources permit nearby access but
force later decisions after local depletion. These are possibility-space changes,
not different laws or hand-authored occupation ratios.

In seed0 branch, eight jobs are accepted (four at6h, four at8.033333h),80 communal
quanta are gathered and none is delivered. The jobs eventually cancel for inability
to obtain expected cargo. Traces include communal consumption by those carriers.
This does not prove a universal acceptance blockade: geography has moved failure
into the accepted-job chain. F2 must examine acquisition, carrier use and transport,
with a self-only control, before any production layer is added.

## Integrity and reproduction

`cargo test --all-targets`; `cargo build --release --bins`;
`python3 scripts/run_geography.py NEW_DIRECTORY`.

The initial integrity collector accidentally included its own still-growing
progress log and a same-run regression CSV still being completed. Its failure is
retained in `matrix.log`; analysis explicitly excludes only the progress stream
and verifies the completed regression CSV is byte-identical to the previous
19-scenario proof. All earlier evidence, including F0, is unchanged. This is an
instrumentation snapshot race, not a physical or decision failure. The repaired
runner excludes its progress stream; the complete experiment was not discarded
or tuned. See the exact exclusions in the provenance file.
