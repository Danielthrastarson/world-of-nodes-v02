# Headless playable evidence

The final matrix contains **48 runs plus 48 exact dated-input replays**: three
seeds, two starting histories and eight fixed transcripts, each followed for 24
hours after the session becomes ready. Every replay matched both the authoritative
state hash and event hash. Every hourly invariant audit passed. The ruler remained
alive at the measured endpoint in all 48 runs; this does not establish indefinite
survival or general gameplay quality.

These are controlled playable initial conditions: an accepted steward, a willing
courier, four already assembled voluntary militia members, seeded households and
a seeded hostile raid. Commands thereafter enter the normal Person/institution
mechanisms. No actor is held in place, no construction completion is scheduled,
and no autonomous result is fabricated. No-input retains the initial militia; it
is not a no-mobilization control.

## Fixed-transcript outcomes

The following physical metrics were identical across all three tested seeds.
This limited agreement is not a population-level statistical claim. Food is in
physical quanta; starvation is summed person-hours. Final counters include the
Remote setup's initial quarter-hour journey. Compare arms within the same setup;
Local versus Remote is not a pure communication-distance intervention.

| Setup / input | Crossing completed | Food stolen | Food consumed | Starvation h | Deaths | Input errors/run |
|---|---:|---:|---:|---:|---:|---:|
| Local / none | 0 | 30 | 176 | 4 | 0 | 0 |
| Local / Commission | 1 | 30 | 224 | 0 | 0 | 0 |
| Local / Hold | 0 | 30 | 176 | 4 | 0 | 0 |
| Local / Move to 2 | 0 | 30 | 192 | 8 | 0 | 0 |
| Local / Withdraw to 0 | 0 | 30 | 172 | 4 | 0 | 0 |
| Local / Attack at 2 | 0 | 0 | 168 | 8 | 1 | 0 |
| Local / Commission + Hold | 1 | 30 | 224 | 0 | 0 | 0 |
| Local / Commission, revoke at 2h | 1 | 30 | 224 | 0 | 0 | 0 |
| Remote / none | 0 | 30 | 226 | 4.5 | 0 | 0 |
| Remote / Commission | 1 | 30 | 262 | 0 | 0 | 0 |
| Remote / Hold, Move, Withdraw or Attack attempted | 0 | 30 | 226 | 4.5 | 0 | 1 |
| Remote / Commission + Hold attempted | 1 | 30 | 262 | 0 | 0 | 1 |
| Remote / Commission, revoke at 2h | 1 | 30 | 262 | 0 | 0 | 0 |

Farming produced 0 in every Local arm and 96 in every Remote arm. Care transferred
24 quanta in every arm. The measured starvation occurred in the initial dependent
cohort. Thus the changed starvation cannot be described merely as a changed total
care transfer. This probe measures the complete command consequence, not a
factorial attribution of every later choice.

## Causal witnesses and limits

**Commission is an objective, not a spawned crossing.** Local issue, receipt and
lawful steward commission occurred at t=0. Construction completed at 46,860,000ms
(13h01m), after consuming 20 wood and 8.65 person-hours of labor. Eleven completed
trips by one Person subsequently used the crossing; their summed food loads were
50 food-edge quanta, not 50 distinct goods or deliveries. The no-input arm created
no crossing. This demonstrates physical realization and changed movement/food
access in this fixture, not that every infrastructure investment is beneficial.

**Remote orders require transport.** The Remote order was issued and handed to
courier 5 at 900,000ms. Steward 1 received it at 1,856,400ms, 956,400ms later
(15m56.4s). The steward commissioned at 1,860,000ms. Construction completed at
49,100,000ms, using 20 wood and 8.80556 person-hours. The issuer's possession of the
original order did not reveal the recipient's ongoing private obligation status.
The CSV's `issuer_known_order_status` is **raw kernel-memory capture**, not the
full client projection. Session separately adds the ruler's witnessed handoff;
for example, the Local UI can show `Delivered locally; recipient decision
unreported` while this raw diagnostic says `Recipient status unknown`. These
rows therefore do not claim that the UI hides a personally witnessed receipt.

**Attack and Move are different objectives.** The Local Attack order was received
by four physically present people at node 1. They reached node 2 and began actual
contact at 14,400,000ms. Eleven strike hits accumulated 11 injury units. Person 11,
a seeded raider, died from contact at 17,100,000ms; two other raiders and one
defender were incapacitated and alive at the endpoint. Theft was 30→0, but food
consumption was 176→168 and starvation 4→8 person-hours. Protection therefore has
a measured cost; this is not a general welfare improvement. Move reached the
location without granting the protective/hostile contact objective and still lost
30 food. Hold at node 1 did not protect the stock at node 2.

**Remote military control is unavailable in this starting history.** At t=0,
ordinary service arbitration ended the travelling ruler's service with
`PRIORITY_OVERRIDE`. At ready time 900,000ms, all attempted military commands
failed with `commander lacks active voluntary service in the agreed unit`.
There were 15 failed inputs across the complete matrix. The four Remote
military-only arms have exactly the no-input state/event hashes for their seed.
They are rejected commands, not executed marches or failed battles. Setup and
service policy were deliberately left unchanged. Likewise all service terms have
ended by the 24-hour endpoint; this interface currently provides no arbitrary
right to restore command over volunteers.

**Revocation is not retroactive demolition.** In Local, the steward's obligation
was completed by authorizing the project at t=0. A revocation reached the steward
at 2h and explicitly recorded `completed=true`. It did not undo the lawful
commission or completed work. The crossing still finished at 13h01m. This preserves
the distinction between revoking an unfinished order and inventing power to cancel
an independent institution's ongoing physical project.

Exact selected event IDs and timestamps are in [TRACE_EXCERPTS.md](TRACE_EXCERPTS.md).
Complete typed transcripts, hourly metrics and losslessly compressed causal traces
are under [scenarios-3seeds](scenarios-3seeds/). See [README.md](README.md) for units,
measurement windows, reproduction, and the preserved pilot instrumentation limits.

## Actual-Person performance measurement

The separate scaling fixture contains **1,000 actual autonomous Persons**, ten
nodes and ordinary finite-capacity edges. It has no added household, combat,
authority or reproduction systems; it does not establish the performance of an
integrated 1,000-person kingdom. No renderer or GPU was measured.

The measurement run after the scenario matrix gave:

| Simulated checkpoint | Play records (including memories) | Play median / p95 ms | Debug records | Debug median / p95 ms |
|---|---:|---:|---:|---:|
| 0 | 100 | 0.0248 / 0.0333 | 1,000 | 14.7345 / 30.9654 |
| 15m | 199 | 0.0570 / 0.1246 | 1,000 | 15.3309 / 20.1192 |
| 1h | 503 | 0.1577 / 0.2520 | 1,000 | 16.2277 / 22.1974 |

Each cell uses 50 captures. One simulated hour took 0.6767 seconds of cumulative
kernel advance time on this host. The Person count remained 1,000. Process peak
RSS reached 14,264 KiB (about 13.9 MiB), including allocator retention and
instrumentation; this is not an isolated World heap measurement. All snapshot
batches preserved state and event hashes. Debug is substantially more expensive
because it includes full private inspection and global state hashing. These timings
do not establish a 60fps rendering guarantee or justify extrapolation to 100,000
people. A separately preserved concurrent-load run showed similar medians and
variable tails; host timings are observations, not universal bounds.

Final data: [performance-1000-after-matrix/performance.csv](performance-1000-after-matrix/performance.csv).
The earlier concurrent run and development pilots remain preserved separately.

## Provenance

Each final output records the base revision, working-tree status, a complete Rust
source-tree fingerprint, and the checksum of the executable actually running.
The final binary was built after the Play/Debug boundary fixes and 29 playable
tests passed. Subsequent Clippy-only cleanups collapse a nested snapshot
`if !debug { if let Some(local) ... }` into an equivalent let-chain and remove
a redundant `.into_iter()` from the probe's source-file checksum iterator. They
change source fingerprints but not semantics. They were not used to regenerate
or overwrite these outputs. The executable checksum identifies the measured build precisely.
