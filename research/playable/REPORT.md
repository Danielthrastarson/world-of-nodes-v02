# P0–P4: first playable kingdom interface

Implementation checkpoint: **`6c1159e`**, based on clean `983b5b5`.
The optional native client now runs the authoritative world, accepts typed ruler
and unit inputs, and displays local observations and dated reports. P0–P3 pass
at the **Local native graph/technical scope**. P4 is an integrated playable
candidate; human comprehension, pacing and sustained play remain unvalidated.
No new large simulation system, economic institution or coercive power is added.

## Run and controls

```sh
cargo run --release --manifest-path client/Cargo.toml
```

Start paused. **W → 1 → click crossing → Enter**, then **Space**. Select **UNIT 0**
for **W → Move / Attack / Hold / Withdraw**. RMB a location gives Move; RMB a
locally recognized hostile gives Attack. **Q** observes, **1–4** compose options,
wheel zooms, arrows/middle/right drag pan. **F2** explicitly toggles debug truth.
E/Govern and R/Allocate retain stable meanings but their new powers are deferred.
[Full control contract and limits](../../docs/PLAYABLE_CLIENT.md).

## Milestone results

| Gate | Technical result | Evidence and limits |
|---|---|---|
| P0 | PASSED | Separate Macroquad crate; real Persons, goods, fields, paths, crossing sites, carts if present, institutional nodes and unit membership. Selection, camera, pause/speed and owned snapshots. Renderer/headless parity; no fake traffic. Linux native tested, WASM deferred. |
| P1 | PASSED at local command-loop scope | Actual keyboard/mouse commission creates a typed F13 order; physical construction follows only through accepted office duty and voluntary material/labor use. Local handoff is known; distant obligation completion has no transported ACK and remains unknown. |
| P2 | PASSED at local graph scope | Actual window inputs exercise Hold, contextual Move, Attack and Withdraw. Only local active members receive orders; injury, service expiry, survival and FIFO remain binding. Remote setup loses ruler service and does not pass P2. |
| P3 | PASSED at dated-observation scope | Source, author/sender, observation time, receipt time and age are inspectable. Current observations, last-seen people and map-only knowledge are distinct. Matched Play/Debug views can disagree without changing hashes. No remote live tracking or generic truth-revealing fog. |
| P4 | ACTIVE — playable candidate | One 18-Person scenario joins households, production, voluntary militia, raid, office, physical reports and crossing commission. All tested inputs execute through the causal world. Human playtest is still needed; no enjoyment or complete long-session loop is claimed. |

The roadmap order is now F13 → P0 → P1 → P2 → P3 validation → P4 playtest.
P3's information boundary is required from P0. F14's political coordination target
is not skipped or claimed; broad F15/F16 visual depth remains deferred. The original
roadmap history remains intact.

## Evidence from the integrated world

There are **48 fixed-transcript runs and 48 exact dated-input replays**, covering
three seeds, Local/Remote starting histories and eight input transcripts. All
hourly invariant audits and both replay hashes pass. The following outcomes were
identical across the three tested seeds; this is a bounded interface test, not a
large population study or optimized strategy.

| Local input, 24h | Crossings built | Food stolen | Food consumed | Starvation person-hours | Deaths |
|---|---:|---:|---:|---:|---:|
| No input | 0 | 30 | 176 | 4 | 0 |
| Commission | 1 | 30 | 224 | 0 | 0 |
| Hold at home | 0 | 30 | 176 | 4 | 0 |
| Move to food settlement | 0 | 30 | 192 | 8 | 0 |
| Attack at food settlement | 0 | 0 | 168 | 8 | 1 |

No-input already contains the four initially consenting militia members. It is
not a no-mobilization control. The setup seeds offices, courier consent, kin,
resources, map knowledge and a hostile raid; none is relabeled autonomous emergence.
After setup there are no research actor holds, scheduled protection, forced project
completion or parameter tuning. Persons retain ordinary behavior and mortality.

The Local commission consumes **20 wood and 8.65 person-hours**, completing at
**13h01m**. It enables eleven later trips by one Person, carrying 50 food-edge
quanta in total; that is not 50 unique goods. The same world without the input
builds no crossing. Revoking at 2h cannot undo the commission already performed at
t=0 or demolish the subsequent independent construction.

The Remote order is issued at 15m and received **15m56.4s later** after courier
movement. Construction completes at 13h38m20s. Remote consumption changes
226→262 and starvation 4.5→0 person-hours. These counters include the setup's
quarter-hour journey; Local versus Remote is not a clean distance-only experiment.

Local Attack causes actual contact at 4h. Eleven strikes produce eleven injury
units; one raider dies and two raiders plus one defender remain incapacitated.
It prevents **30 stolen food**, while consumption falls **176→168** and starvation
rises **4→8 person-hours**. Protection is not a general welfare improvement.
Move alone does not grant the protective contact objective and still loses food.

All attempted Remote military inputs reject after ordinary arbitration ends the
ruler's service during the initial journey. There are fifteen rejected inputs in
the matrix, not fifteen failed marches. No role is restored to improve usability.
All rulers remain alive at the measured endpoint; all initial service terms have
ended by 24h. Indefinite command availability is not established.

[Detailed physical results](probes/REPORT.md), [exact causal witnesses](probes/TRACE_EXCERPTS.md),
[complete traces/transcripts](probes/scenarios-3seeds/), [probe specification](probes/README.md).
The raw probe's `issuer_known_order_status` column measures kernel memory, before
the client Session adds the ruler's personally witnessed handoff metadata.

## Rendering and knowledge boundary

Root Cargo remains dependency-free. `client/` owns Macroquad, graphics assets,
camera, input composition and presentation. The Session owns World and accepts
typed commands. Its read-only snapshot capture changes no clock, RNG, observation,
memory, event or physical state. A command rejected by authority/locality checks
does not become legal merely because DEBUG reveals a target.

Play has the ruler's own physiology, current local public observations and dated
memories/reports. Other people's private hunger, plans, beliefs and social state
are not exposed. Remote dots are last-seen ghosts; map-only seeded knowledge does
not assert zero people, zero food or zero queues. Hostility is recognized from
legitimate local evidence, not a remote raid configuration lookup. Debug is
visibly separate and may inspect true private/global state.

The ruler remembers an actual local handoff or handoff to the agreed courier.
The client does **not** query a distant steward's receipt/obligation to improve
feedback. F12 NodeView reports can carry observed consequences, but no new private
command-status acknowledgment institution was added. “Unreported” is an intended
knowledge outcome, not a claim that the order failed.

The graph layout and stable crowd packing are display geometry. Edge progress
comes from actual traversal state. There are no invented continuous formations,
formation bonuses, map coordinates in physical law or decorative people. Visual
interpolation is bounded by two executed poses. Slow frames retain time backlog;
pause, selection, camera and snapshots do not drive Person deliberation.

## Tests and actual window checks

- **369 headless tests pass:** 356 library and 13 integration tests.
- **29 playable boundary/mechanism tests** are included in that total, covering
  locality, hidden-state noninterference, pure captures, partial muster, invalid
  commands, withdrawal vulnerability, map-only knowledge and witnessed handoffs.
- Client has three distinct camera/clock tests; the camera test also runs in the
  render-probe binary. Both crates pass Clippy with warnings denied and formatting.
- Actual X11 keyboard/mouse tests issue commissions and all four military verbs,
  verify help blocks commands, pan/zoom, pause/speed, and switch debug views.
  Recorded dated inputs independently replay to identical state and event hashes.
  These are automated interaction tests, not a substitute for human playtest.
- Native fixed-frame runs exit successfully and preserve replay hashes.

Window evidence: [commission](client/commission-release/),
[military](client/military-release/), [test driver](../../scripts/playable_window_smoke.py).
Earlier development/window passes remain separate; they are not extra independent
simulation experiments. Screenshots were inspected for legibility and layout.

## Performance

These are observations on this host, not scale guarantees. The graphics runs use
isolated Xvfb displays. Screenshot encoding and software-display scheduling affect
frame timings. Root simulation and rendering are measured separately.

| Measurement | Observed result | Scope |
|---|---|---|
| Autonomous kernel, 1,000 real Persons | 0.6767s for one simulated hour | Ten-node finite-capacity line, no added household/combat/authority systems |
| Play snapshot at 1h | 0.1577ms median, 0.2520ms p95 | 503 currently known/remembered records; 50 samples |
| Debug snapshot | 14.7–16.2ms median; worst sampled p95 30.97ms | 1,000 private diagnostic records and full hashing |
| Headless process high-water RSS | 14,264 KiB | Includes allocator/instrumentation; not isolated World heap |
| Native render, 1,000 actual visible Persons | 0.622ms mean CPU draw; 13.06ms maximum; 123fps over 240 frames including capture | Paused kernel; real World/snapshot, not duplicated fake dots |
| Native 18-Person remote capture at 16h | About 114 MiB process RSS | Includes graphics context/font/driver/capture; not comparable to headless-only RSS |

The render probe has **1,000 actual Persons and 1,000 visible Person records**;
state/event hashes remain unchanged. Its 6.43ms one-off dense local snapshot is
not interchangeable with the sparse-node repeated-capture benchmark.
[Render measurements](client/render-release/1000-persons.txt),
[simulation/snapshot measurements](probes/performance-1000-after-matrix/performance.csv).
Nothing here establishes a fully integrated thousand-person kingdom at 60fps,
long-term memory bounds, 100,000-person scaling or browser performance.

## Failures found and preserved

Withdrawal testing exposed an existing invalid-strike cancellation clock defect.
If a target left contact, cancellation could wait for the next external checkpoint.
A separate test reproduces it without any playable order. The kernel now schedules
cancellation at the next authoritative millisecond. This is a global coherence
repair, not special military movement, damage, immunity or a UI-only timing rule.

Historical comparisons are **not all exact**:

- 169 pre-combat controls pass under their existing comparison rules.
- All **600 F13B runs / 3,000 artifacts** reproduce exactly.
- **60 of 1,440 F11C runs retain strict failures**: every seed of two defended
  bottleneck/crossing arms. Ineffective pending-strike time becomes existing duty
  time; event references and hashes change. The strict 217-control verifier's
  failure archive remains preserved.
- Final people/timeline results match in all 1,440 runs. All 48 emitted seed-zero
  goods samples match; two event samples differ. There are not 1,440 full goods
  traces. No measured theft, casualty, nourishment, production, care or queue
  outcome changes. For all sixty changed runs, replacing only final `last_event`
  references in a post-run diagnostic accounts for the entire state-hash change.

The [regression report](regression/REPORT.md) preserves raw differences, exact
commands, provenance and the bounded attribution. It does not rename FAIL as PASS.
All **20,266 prior research files are unchanged**, verified against the pre-work
SHA-256 manifest in [historical-integrity-final.json](historical-integrity-final.json).

The first large render test saved its results then crashed during shutdown because
a thread-local font's GPU destructor outlived the graphics context. The single
process-lifetime font asset now avoids that teardown ordering. Its failed run is
preserved in `development/render-probe-before-font-lifetime-fix.txt`; the committed
render probe and finite-window captures exit normally. Dense crowd packing was
also changed after the initial rendering produced unreadable radial spokes. Both
versions are retained; neither altered physical positions or state.

## Remaining UX and scope limits

The remote order lifecycle has no returned private-status ACK. Reports can be
stale, and the ruler can be elsewhere or dead. The Remote starting history is a
transport scenario, not an available military-command scenario. The interface
cannot renew expired service, recruit arbitrary civilians, pool carts or seize
goods. Remembered unit objectives need their own age treatment before display;
the current badge shows observed membership only.

There is no save/load UI, polished tutorial, human-tested binding/pacing scheme,
audio, continuous tactics, final accessibility pass, WASM distribution or long-run
population-scale validation. E/R show deferred powers. Carts and broader inherited
mechanisms remain renderable but are not cosmetically spawned in this scenario.

Next: human playtest of this candidate, with attention to unknown-versus-failed
orders, ruler embodiment, service expiry, readable causal feedback and time scale.
That feedback should guide interface iteration. No new institution or simulation
system is justified simply to make the prototype easier to win.

## Reproducibility

[Source/provenance manifest](provenance.json) identifies `6c1159e` and executable
checksums. Each research subpanel records its actual measured binary and source
fingerprint. Equivalent lint-only source changes after the 48-run panel are
disclosed in its report; outputs were not overwritten to hide them. Window
release transcripts include the measured executable fingerprint and headless
replay result. Reproduction commands are in the client guide and subpanel READMEs.
No previous research artifact was replaced. No push or deployment is required
to run the local client.
