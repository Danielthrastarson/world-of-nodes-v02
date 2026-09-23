# P0–P4: first playable interface

Owner authorization after `983b5b5` moves interface work ahead of F14's political
coordination target. This is a dependency change, not a claim that F14 happened.
The new order is P0 snapshot/client → P1 scoped crossing command → P2 local unit
control → P3 readable dated information → P4 integrated playable scenario. P3's
information boundary is required from P0; its interaction validation comes later.
Existing F0–F13 research and all negative findings remain immutable.

## Architecture and technology

The root Rust crate stays dependency-free and headless. A separate `client` Cargo
crate depends on it and Macroquad, with default audio disabled. Macroquad supplies
primitive 2D geometry, text, mouse/keyboard and a native window, with a future WASM
route. No current measured need justifies direct wgpu or an engine migration.
Official references: https://macroquad.rs/ and https://docs.rs/macroquad/0.4.16/.

A pure Rust playable adapter owns a World and an explicit command log. Rendering
receives owned read-only snapshots. The client cannot obtain mutable World state.
Commands are typed, validated session requests; neither clicking nor rendering
sets inventories, roles, locations or construction completion. Fixed simulation
steps and dated input preserve one clock independent of drawing cadence. Visual
interpolation changes positions on screen only. Slow frames retain simulation
backlog or slow wall-clock playback; they cannot discard physical time/events.

Graph positions are a schematic display layout, not new metric geography or
continuous battle formation. Edge progress is authoritative. Person offsets at a
node are display packing, not physical positions. Every dot has a real Person ID.

## Scope and controls

Stable verbs: Q Observe, W Commission/Order, E Govern, R Allocate/Mobilize.
Unsupported verbs remain visibly unavailable instead of acquiring new powers.
1–4 choose options; LMB selects/targets; RMB contextual military order or cancels
composition; explicit confirmation issues consequential commands. Space pauses;
dedicated time buttons choose 1x/faster; wheel zoom and camera drag/arrows pan.

P0: actual Persons, goods, fields, paths, crossings, institutions and unit members;
hover/selection, camera, time controls, clear PLAY versus DEBUG truth.
P1: accepted-office crossing order, physical local/carried receipt, role duty,
existing commission/construction, known-status versus unknown-status distinction.
P2: graph-level Move/Hold/Attack/Withdraw over existing voluntary unit Persons.
No remote admission of new members or automatic update of absent soldiers.
P3: observation/report author, sender, timestamp, age, current versus last known;
no global observer journals in normal play.
P4: one small scenario joining lawful command, economy, households, infrastructure,
information and a controlled raid. No success tuning or scripted causal outcome.

Scenario initial consent/appointments, known maps, voluntary service and any willing
courier purpose must be labeled. This is a playable initial world, not evidence
of spontaneous government or a new citizen labor entitlement. A scenario courier
agreement can use the existing physical-delivery undertaking; the player cannot
command any arbitrary civilian to deliver. Issuance/selection must not fabricate
acknowledgments or update knowledge remotely.

## Gates and evidence

At each gate: specification, revision, controls, screenshots when executable,
performance observations, repeatable input transcript, limitations and regressions.
Test snapshot purity, hidden-truth noninterference, debug/play separation, invalid
commands, stale observations, locality and same dated commands under different
render/time partitions. Compare headless and client-driven state/event hashes.
Measure real simulation cost, snapshot/draw time, frame rate, entity/visible count
and memory where available. Benchmark actual Persons, never fake decorative traffic.

Keep all old fixtures headless. Full tests and appropriate historical controls
must pass; isolate any divergence before expanding. Reuse the documented F13B
clock correction as the baseline, not a new excuse for changed hashes.

Owner decisions remain necessary for coercive powers, arbitrary civilian control,
hidden NPC state in normal play or an irreversible kernel/architecture change.
No money, taxation, conscription, requisition, conquest, Jev, sound pipeline,
continuous combat geometry or cosmetic simulation system is introduced here.
