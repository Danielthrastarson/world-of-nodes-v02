# Headless regression and contact-clock review

Baseline: `983b5b5b522456358aa9c7f3d7adb408df77a89f`. The tested playable
implementation is identified by `build-provenance.json`, including source and
binary hashes and the working-tree status. Historical output was not updated.

## Results

| Check | Result |
|---|---|
| Full headless `cargo test` | PASS: 352 library tests and 13 integration tests; binaries/doc tests also pass |
| Existing pre-combat historical controls | PASS: 169 executions under their existing documented comparison rules |
| Existing 217-control verifier | **FAIL**, retained: first difference is `bottleneck_defended/runs.csv` |
| Complete F13B command-chain replay | **PASS: 600 runs, 3,000 arm artifacts exactly equal**, including hashes, parameters and traces |
| Complete F11C replay against clean `983b5b5` | **FAIL: 60 of 1,440 runs change hashes and occupied activity time**; six arm artifacts differ |
| Historical artifact integrity | PASS: all files in the original playable prior-artifact manifest unchanged |

The 60 changed F11C runs are all 30 seeds of `bottleneck_defended` and
`crossing_defended`. The other 1,380 runs exactly match the clean baseline.
For all 1,440 runs, final `people.csv` and quarter-hour `timeline.csv` match.
Final goods match for every emitted goods trace. **The registered F11C probe
emits event and goods traces only for seed zero**, so this is 48 trace samples,
not 1,440 full goods/event traces. The two affected seed-zero event traces differ.

Only five final CSV fields change in each affected run: `combat_h`,
`prospective_defenders_combat_h`, `prospective_defenders_military_duty_h`,
`state_hash`, and `event_hash`. No theft, casualty, nutrition, consumption,
starvation, production, care, transport or queue outcome field changes.
All changed values are enumerated in `combat-baseline-validation.json`.

## Demonstrated cause

The new Withdraw checkpoint test exposed a pre-existing clock defect: after a
target leaves contact or becomes incapacitated, a pending strike could remain
active until the caller next advances the world. Arbitrary render/checkpoint
cadence could therefore determine strike cancellation and subsequent decisions.

The correction schedules the already-invalid contact at the next authoritative
millisecond. It is a kernel correction, not a military speed, damage, immunity,
or player-only exception. The independent kernel test covers contact loss
without a playable order. The failing transcript remains in
`../development/withdraw-partition-before.txt`.

In the preserved bottleneck fixture, target 8 becomes incapacitated at
29,800,000 ms. Two invalid strikes formerly clear at 29,820,000 ms; they now
clear at 29,800,001 ms. Each defender resumes the existing duty 19,999 ms
earlier: **39,998 person-ms moves from ineffective combat to duty**.

In the crossing fixture, three invalid strikes formerly clear at
29,940,001 ms; they now clear at 29,920,001 ms. **60,000 person-ms moves from
ineffective combat to duty**. Injury/contact outcomes do not change.

The exact seed-zero semantic event differences are six moved events plus two
additional duty decisions in the bottleneck case, and nine moved events plus
three additional duty decisions in the crossing case. Sequential event IDs
then change. `combat-event-causal-differences.json` lists those events, with
the raw strict comparison and compressed raw diffs retained separately.

## State-hash attribution

State hashes include each Person's `last_event` causal reference. Extra decisions
therefore legitimately change final state hashes even when later physical state
is the same. Complete final Person dumps for both seed-zero cases show that
only those references differ; memory, RNG, goals, actions and physiology match.
Bodies, metrics, goods, boards, military memory and unit views also match.

The diagnostic in `all-seed-event-reference-hash-proof.json` repeats **all 60
changed runs** from the clean baseline, then replaces only the final
`Agent.last_event` references with the current values. This reproduces every
complete current state hash. The diagnostic setter exists only in the temporary
baseline copy, after execution; it is not production code or a comparison
normalization. The original state/event comparisons remain **FAIL**.

## Historical schema and retention

Comparing clean `983b5b5` with the original F11C archive differs only in 48
`parameters.txt` files: the already-existing disabled `information: false` and
`commands: false` fields. All other original F11C bytes match that baseline.
This pre-existing schema difference is preserved in a separate strict failure
report; it is not attributed to playable work or silently removed.

The F13B archive's source `ff70527` has no `src` difference from `983b5b5`.
Its complete exact replay therefore validates the baseline command behavior.
Proved duplicate new replay directories were discarded only after comparison;
`compaction.json` retains every removed file's original path and hashes.
All changed current/baseline F11C raw arms and the historical verifier's strict
failure archive remain. The original research is untouched.

Reproduction: build the root release binaries, run the existing
`verify_information.py`, `run_combat.py` with 30 seeds, and
`run_command_chain.py` with 30 seeds and `--compress-traces`, then run the
retained `compare_panels.py`. `run_baseline_combat.py` uses a binary built from
`git archive 983b5b5 Cargo.toml Cargo.lock src`. Full commands, source hashes,
results and logs accompany this report.

Conclusion: exact historical identity fails at the documented contact-clock
correction. The evidence bounds that failure; it does not turn changed hashes
into passes or claim every conceivable scenario is unaffected.
