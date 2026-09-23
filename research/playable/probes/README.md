# Headless playable probes

`src/bin/playable_probe.rs` uses the public `Session` input/replay interface. It
never mutates the session's World. The scenario evidence is a bounded interface
exercise, not an optimized strategy or a new population-level experiment.

Reproduce into new directories (existing directories are rejected):

```sh
cargo build --release --bin playable_probe
./target/release/playable_probe scenarios /tmp/playable-scenarios-new 3 24
./target/release/playable_probe performance /tmp/playable-1000-new 1000
```

The scenario matrix compares eight fixed input transcripts at each seed, separately
within Local and Remote starting histories: no input; commission; Hold; Move to
node 2; Withdraw to the neighboring rear node; Attack node 2; commission + Hold;
and commission + revoke after two hours. Hold uses the ruler's current physical
node. Inputs that fail validation remain in the transcript and are not retried by
a hidden controller. Remote starts after the setup's real quarter-hour journey,
so the equal 24-hour measurement duration ends at different absolute times.
Cumulative final physical counters include Remote's quarter-hour prelude; the first
timeline row records that initial contribution. These are cumulative-from-world-zero
counters, not final-minus-ready deltas. Compare paired arms within a starting
history; do not attribute every Local/Remote difference to communication distance.

No-input is **not** a no-military control: every Session starts with the explicitly
consenting, physically assembled four-Person militia and a seeded hostile raid.
The ruler, steward and couriers remain ordinary embodied Persons throughout play.
The scenario gives no actor a research hold and schedules no protection order or
construction completion. Replay repeats exactly the dated successful and failed
inputs, then compares both state and event hashes. Hourly audit failures stop the
probe. Raw timestamps use absolute simulation milliseconds. Starvation, queue exposure,
combat action, care travel, farm labor and construction labor are summed
person-milliseconds, not wall-clock durations. Household separation is summed
unordered household-pair-milliseconds. `initial_dependents_starvation_ms` tracks
the seeded recipients 16 and 17 even if they later mature; it is not a dynamic
child count. `military_service_roles` counts assembled service holders and can
include incapacitated people; it is not fighting strength.
`observed_person_node_changes` is an all-person node-transition count from the
existing household journal, not a claim of migration or change of permanent home.
`run_wall_ns` includes hourly metrics, audits and timeline serialization, whereas
`replay_wall_ns` includes replay setup but omits those output checkpoints; they
measure different work and are not a speed comparison.

The scenario CSV column `issuer_known_order_status` comes from raw
`capture(world, ruler, Play)`: it describes the ruler's kernel-held command memory,
not the client's complete `Session::snapshot` projection. Session adds explicit
initial-map provenance and the ruler's own witnessed handoff history. In particular,
a Local order can display `Delivered locally; recipient decision unreported` in
the UI while this raw column reads `Recipient status unknown`. Receipt knowledge
is distinct from knowledge of the recipient's subsequent private decision. These
probe rows do not test the Session overlay; its boundary tests and GUI validation
are separate. `play_visible_person_records` and `play_received_reports` are also
raw-capture counts. The metric probe did not mutate NPC knowledge.

The performance fixture is separate: actual autonomous Persons on a ten-node line,
15-minute edges of capacity four, two initial food quanta per Person at node 9,
24-hour initial nutrition and zero initial fatigue, no added household/combat/
authority institutions. It runs one simulated hour and measures 50 owned snapshot
captures per mode at three checkpoints. These are not fake render dots. The probe
measures headless simulation and snapshot capture only, **not frame rendering**.
Debug includes privileged private diagnostics and hashes, so its cost is not the
cost of normal Play. RSS and process high-water RSS include allocator retention
and instrumentation; they are not an isolated World heap measurement.

Output directories contain parameters/provenance, final metrics and hourly rows,
exact typed transcripts, complete causal event streams, and end-state diagnostics.
Large trace/state files are stored losslessly as `.gz` in the preserved evidence.
Reproduction initially writes their uncompressed equivalents. Source fingerprints
are FNV-1a checksums, not cryptographic attestations. Final runs also fingerprint
the executable actually running.

## Preserved pilot limitation

The `pilot-*` directories were produced during concurrent development and are
preserved as exploratory evidence. Their source-tree fingerprint was captured at
run start and is not a guarantee that concurrent edits were in the already built
binary. Final evidence must use the final directories and executable fingerprint.
The pilot scenario CSV accidentally requested nonexistent journal keys
`orders_completed`, `orders_failed`, `orders_refused`, `revocations_issued`; their
zero values are **not measurements of those outcomes**. The final probe corrects
these to the actual `obligations_*` and `revocations_sent` counters. All pilot
physical metrics, typed input records and causal traces are retained unchanged.
