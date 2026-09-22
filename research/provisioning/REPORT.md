# Local provisioning P1 — finite food, failure preserved

## Finding

The implemented rules support physical self-provisioning and migration, but **do not establish a working communal delivery system**. In the reference, 28 of 50 people remain alive at 72h; all survivors are at F, no communal jobs are accepted, and no food is delivered to C. Disabling communal provisioning gives the same survival outcome. This is a negative result for communal provisioning, not a milestone passed by balancing numbers.

The source implementation is recorded in `provenance.json`. The prior transport source and results remain untouched. No farming, construction, trade, money, safety, combat, graphics, or physics was added.

## Protocol and what the numbers mean

The reference uses the locked 50 agents, initial nutrition 12h, initial fatigue 4h, no carried food, and 3000 food quanta at F. All food is created at zero and expires at 48h. Both C–X and X–F take 2h at unimpaired speed and have four shared-direction slots. Carrying capacity remains 10; unimpaired gathering remains 2 seconds per quantum. Decisions normally run every minute, with local action/critical-state wakeups on the same clock.

The initial biological 600 nutrition-hours plus 3000 physical quanta equal the theoretical 3600 nutrition-hours needed for 50 people over 72h. That equality says nothing about discovery, access, consumption before expiry, or allocation. No source replenishes, and gathering never resets age.

An ideal one-direction edge admits 2 people/h and at most 20 carried quanta/h at full loads. Repeated empty outbound and loaded inbound trips share that capacity, limiting steady-state delivery to at most 10 quanta/h per edge before startup delay, physiology, queues, and other traffic. Consumer migration is an alternative to hauling; it is not prohibited.

The matrix contains **19 scenarios × 30 paired seeds = 570 runs**, with 85,500 individual checkpoint records. Randomness breaks exploration ties; the reference line graph offers little such choice. Identical results across its seeds are not evidence of robustness to untested geography or institutions. Contested requests and the branching near/far map exercise seed-dependent exploration.

[Full matrix](SUMMARY.md) · [raw run metrics](runs.csv) · [per-agent checkpoints](agents.csv.gz) · [parameters](parameters.txt).

## Main results

| Intervention | Survivors at 72h | Interpretation |
|---|---:|---|
| Reference | 28/50 | Consumers reach food; no delivery network forms. |
| Prior-information control | 28/50 | Prior information has no survival effect on this simple reference route. |
| Self-only control | 28/50 | The communal institution adds no demonstrated reference benefit. |
| Edge capacity 2 | 14/50 | Capacity limits access before starvation. |
| Edge capacity 1 | 7/50 | The loss grows as capacity falls. |
| Each edge 6h | 4/50 | Distance and impaired movement leave very few able to reach food. |
| Each edge 12h | 0/50 | No survivor at the endpoint. |
| No transport / no food | 0/50 | Everyone dies at 36h; remote stock does not feed anyone. |
| Half initially at F | 50/50 | Unequal initial access changes outcomes substantially. |
| 12h or 24h shelf life | 0/50 | Earlier expiry eliminates 72h survival in these cases. |
| Stale-depletion fixture | 0/50 | 28 remembered-food trips fail after physical source depletion. |

The generated summary supplies exact means/ranges for every scenario, including contested requests and near/fresh versus far/older food.

In the reference, 1264 quanta are consumed and 1736 become waste. At expiry, 1568 remain at the natural source and 168 are carried. All 28 survivors have zero nutrition at 72h and average approximately 9.54 continuous starvation-hours (range 6.92–12h). Their survival is protected by the explicit starvation grace period. It is not adequate nutrition or long-term sustainability. Before 36h, survival alone cannot demonstrate provisioning at all.

## Measured causal chains

Full seed-0 traces and [readable excerpts](TRACE_EXCERPTS.md) record event IDs, timestamps, actor links, decisions, physiological performance changes, ownership-sensitive consumption, and actual locations.

**Reference access failure:** local exploration → FIFO waiting → nutrition depletion/performance reduction → delayed traversal → death before reaching F. For example, agent 28 dies at exactly 36h on X–F with progress 2,625,000,000,000 out of 7,200,000,000,000 work units. No arrival or cargo relocation is fabricated. The trace records the earlier queue admission and changes in physiological performance.

**Reference temporal failure:** finite food gathered and eaten → immutable 48h expiry of all remaining food → declining biological reserves → zero nutrition and prolonged starvation by 72h, despite survivors still being alive.

**Contested-request failure:** actor 12 accepts a request at X at 30.266667h → its fixed 12h board lease expires at 42.266667h without cancelling the worker → at 44.466667h the worker cancels because the return is predicted survival-infeasible or too late for the cargo's expiry → remaining food spoils at 48h. The actual run records no completed delivery. A recorded claim does not reserve distant food.

**Stale-information failure:** a controlled actor at F physically gathers and consumes six quanta in a recorded 12-second preparation period → absent actors retain their timestamped source memories → actor 0 arrives at F at 4.003334h and records a failed remembered-food trip → empty-source knowledge is refreshed locally → eventual starvation. This fixture differs intentionally from the 3000-quantum reference and is not an estimate of population-scale depletion frequency.

There is no autonomous food spoilage in transit in the saved matrix: the tested policies fail to establish delivery. The physical mechanism is separately exercised by a regression test that claims a job, travels C–X–F, gathers ten communal quanta, returns physically, and verifies conversion to carried waste on an edge at the 8h expiry. Do not infer field frequency from that controlled mechanism test.

## Invariants, failures, and limitations

All 37 tests pass, including the 12 preserved legacy tests, and Clippy passes with warnings denied. All saved runs have zero material-accounting error and zero FIFO overtakes. For food-origin matter, `initial = remaining edible + physical waste + consumed`; spoiled quantities are counters, not an extra term that double-counts waste. Dead owners retain ownership, and stranded cargo remains physically represented.

Reversing policy evaluation order reproduces both final state and event hashes for every paired seed. Full trace auditing independently reconstructs queues and checks each admission against the earliest waiting key. Changing caller advance partitions is tested across 72h, including sleep, spoilage and death. The 30/60/120-second policy variants separately measure decision-cadence sensitivity; they do not alter the simulation clock.

Limitations that remain visible:

- Communal work is ineffective in the reference. In contested cases jobs are accepted but none deliver food. This increment does not validate the institutional model.
- Reference prior knowledge has no measured survival effect. A linear map is weak evidence about the value of information; the stale-depletion fixture demonstrates a distinct information failure.
- Policies can repeatedly cancel queues or alternate exploration and sleep under conflicting needs. Queued attempts greatly exceed actual admissions in the traces. The scheduler remains FIFO; voluntarily leaving and rejoining loses queue position. This is a decision-rule failure to investigate, not a reason to secretly change capacity or urgency priorities.
- Sleep renews in one-minute atomic actions. The low-level `Interrupted` trace label also includes replacement of an expired sleep tick; it must not be interpreted as a separate job failure. Actual job cancellations have their own event and metric.
- Visible queue estimates include only people at the observer's endpoint. In-flight and opposite-end traffic remain unknown. Predictions can therefore be wrong without omniscience.
- Source/job selection is greedy, with disclosed buffer, urgency and safety-margin heuristics. The six-quantum personal buffer and six-hour predicted margin are assumptions, not calibrated economics.
- The board updates from cooperating people physically present. There is no general conversation, broadcast, or automatic sharing of discovered food locations. A deserted board retains stale demand.
- Requests use observed need and claim leases, not guaranteed service. Expired leases can create duplicate attempts; neither claims nor memories create matter.
- Node transfers/eating are instantaneous. Loading labor, collision geometry, edge-cargo recovery and reclamation of dead personal property are not modeled.
- FNV-1a state/event hashes are deterministic regression identifiers, not security hashes. Artifact integrity uses SHA-256. Reproduction is tied to the recorded Rust toolchain; cross-platform floating-point policy-score identity is not claimed.

During development, tests exposed an over-capacity test fixture. Inspection also found remote queue-count leakage, an incomplete multi-leg job estimate, and a job selector that stopped at an ineligible local source. These were corrected without changing the locked quantities or imposing successful deliveries. The full matrix was rerun on the final recorded source.

## Reproduce

```sh
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
cargo run --offline --release --bin provisioning_probe -- reference 0
cargo run --offline --release --bin provisioning_probe -- --matrix /tmp/provisioning-replay 30
python scripts/analyze_provisioning.py /tmp/provisioning-replay
python scripts/capture_provisioning_traces.py /tmp/provisioning-replay
```

The last command uses the already-built release binary. Full traces are gzip-compressed without discarding failures. The provenance manifest records source revision, toolchain, commands, seeds, hash algorithm, and artifact checksums.

The next falsifiable question stays inside this model: how much failure comes from voluntary queue/sleep interruption and stale claims, versus unavoidable travel/capacity/expiry constraints? Compare those rules explicitly before changing them. No later system or settlement-sustainability claim follows from this result.

Instantaneous transfers/eating complete locally, then the agent waits for its next scheduled decision; timed action completions wake decisions early. There is no unbounded same-timestamp decision loop.
