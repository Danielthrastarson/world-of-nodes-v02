# W0 — world presentation engine

Status: technical implementation complete; **owner visual/motion review pending**.
Macroquad remains the backend. No kernel, policy, institution, observation rule,
command semantics, dependency or live population was changed.

Implementation revision: `7e3bcc6d2496a6548f73b7946428b05582afa866`. Detailed source/binary hashes are in
`provenance.json`. Starting revision:
`645e6191cf9ad68cb91ac165cc62cb4e593234c4` (V0/V1 evidence; implementation
`610d3b3`). This increment deliberately preserves the earlier visual grammar
while making its architecture and scaling behavior explicit.

## Implementation and rationale

| Before | W0 | Purpose |
|---|---|---|
| Coordinates and packing embedded in drawing helpers | Backend-neutral `Geography`, continuous `Point`, canonical `RouteGeometry`, source-linked regions | Replaceable graphics backend without replacing graph physics |
| World drawing order implicit in one routine | Ten named passes with separate CPU timing | Explicit responsibility and ordering |
| Travel interpolation cut chords across curved route art | Interpolate observed traversal fraction on the same route curve; blend endpoint packing | Continuous motion that follows the displayed route |
| Detailed figures at every useful zoom | Close figure, medium compact marker, far tiny marker | Preserve identity while reducing primitive cost |
| Limited whole-frame metrics | Projection/pass timing, visible records, effect counts, RSS, bounded batch probes | Measure bottlenecks without inventing GPU timings |
| No backend-neutral light/effect contract | Time-derived ambient/sun, simple body shadows, source-referenced radial-light/effect descriptions | Prepare later work without fabricating torches, fires or particles |

Architecture and precise constraints: [contract](../../docs/WORLD_PRESENTATION_ENGINE.md).
The renderer consumes permitted observations and cannot access World/Session or
dispatch commands. Play/Debug boundaries stay upstream. Existing input and order
feedback remain intact. Q/W/E/R controls were not redesigned. The optional
`--zoom` flag is capture/view configuration only.

Open ground, institutional settlement clearings, actual cultivation patches,
known finite wood deposits, path wear, crossing construction, goods, Persons,
carts and unit markers retain source references. Water has an empty pass/type:
this scenario has no actual water geography. No invented river, building,
resource, person, flame or light source is supplied to make the scene richer.

The visual solar clock uses authoritative elapsed time, a provisional 24-hour
cycle and T=0 dawn. Ambient stays between 0.86 and 1.0. It changes no physical
visibility, travel or work. Effects are only eligible cues; rendered particles
remain zero. An edge location alone cannot emit a movement cue: the observed
action must be travelling. A body can remain on an edge after death.

## Gates and evidence

| Gate | Result and limit |
|---|---|
| Stable continuous coordinates | Covered by canonical/reverse-route and custom-geography tests |
| Smooth Person motion | Actual-Person controlled Move clip; curved interpolation, endpoints, memory freeze and no-extrapolation tests |
| Locations and routes recognizable without IDs | Screenshots provided; human readability remains pending, not certified by tests |
| Clear layers | Terrain → water → resources → routes → infrastructure → actors → effects → lighting → overlays → UI |
| Basic visual LOD | Three representations, same source identity and selection; normal scenario and synthetic load captures |
| Kernel alone authoritative | Projection purity tests, render-sample hash checks, frozen-build parity, root-source integrity |
| Headless determinism | Full suite and historical scenario/input replays pass |
| Instrumentation | CPU components, counts, memory and sampled batches available; GPU elapsed explicitly unavailable |

Validation:

- **369 headless tests passed** (356 library + 13 binary tests).
- **39 distinct client tests passed** (34 main + 5 replay); three probe targets
  repeat 22 shared tests each and are not counted as additional distinct coverage.
- Formatting and client Clippy with warnings denied passed; release binaries built.
- **48 preserved scenarios and 48 dated input replays:** 193 output artifacts
  match historical uncompressed bytes exactly. [Comparison](validation/scenarios-comparison.json).
- **Eight graphical before/after pairs:** frozen V0 build versus W0, matching
  simulation/event hashes, commands, visible/remembered people and known orders/
  matters. Each independent input replay agrees. [Matrix](release-captures/summary.json).
- **Actual mouse/input smoke:** select unit, issue Hold, select settlement,
  commission crossing, pan/zoom/map-only. Typed commands and independent replay
  pass; camera/UI changes preserve world hashes. [Result](release-input/result.json).
- **240 motion frames:** actual Persons following a controlled, legal typed
  Move; 60 executed simulated minutes, four render fractions per minute. Each
  sample leaves World hashes intact and the final command replay is exact.
  [Result](release-motion/result.json), [position/evidence trace](release-motion/motion.tsv).
- Root source and all previously tracked research remain byte-identical to the
  starting revision. [Integrity](validation/integrity.json).

No new 600-run F13B sweep is claimed here. Its previous preserved validation is
historical evidence. The current pass reran the full headless suite and the
48-scenario client/replay regression scope appropriate to this presentation change.

## Performance experiment

Primary results: [release matrix](release-bench/summary.json).
1,000, 10,000 and 50,000 **synthetic presentation records**, each at zooms 1.05,
0.7 and 0.3, are laid out so every record is visible. They exist only in a
benchmark observation buffer. Every case asserts the real simulation still has
18 Persons and unchanged state/event hashes. This is not a 50,000-Person
simulation experiment or evidence of 50,000 autonomous people running well.

Each case uses a fresh process and isolated 1440×900 Xvfb/software-OpenGL display.
There are five warm-up frames followed by 60 CPU draw samples, a 500 ms frame
guard and a 180-second external limit. Scene CPU excludes presentation projection,
text header, screenshot/readback, frame wait and backend execution at frame end.
Projection is measured once per case. Wall FPS includes warmup/capture and is not
an uncapped hardware-gameplay benchmark. RSS is process memory, not GPU memory.
On-demand batch telemetry runs only after timed frames, and is skipped if mean
draw CPU is >=100 ms because its preview textures are expensive.

Pinned miniquad 0.4.11 does not implement useful elapsed GPU queries. GPU frame
time is **unavailable**, never zero. Actual captured batch counts are given where
sampled, otherwise null. Per-pass CPU and raw memory values remain in each result.

| Render records | LOD | Projection ms | Draw CPU mean ms | Captured batches | Process RSS MiB |
|---:|---|---:|---:|---:|---:|
| 1,000 | Close | 0.065 | 2.186 | 61 | 189.3 |
| 1,000 | Medium | 0.068 | 0.336 | 2 | 137.6 |
| 1,000 | Far | 0.066 | 0.300 | 2 | 137.5 |
| 10,000 | Close | 0.743 | 21.839 | 603 | 514.8 |
| 10,000 | Medium | 0.678 | 2.892 | 13 | 171.0 |
| 10,000 | Far | 0.706 | 2.772 | 13 | 194.8 |
| 50,000 | Close | 3.931 | 102.477 | not sampled | 1811.5 |
| 50,000 | Medium | 3.749 | 14.303 | 61 | 319.8 |
| 50,000 | Far | 4.830 | 14.580 | 61 | 319.6 |

The negative result matters: many detailed figures are expensive in this backend
and software-rendered environment. Compact LOD greatly reduces draw work and
batches, but this does not certify a 60 FPS target or solve large-population
simulation. Projection, per-frame sorting/geometry construction and memory remain
future profiling targets. These results do **not** yet justify switching to wgpu;
hardware measurements and targeted batching work should precede that decision.

## Visual review

- [Initial normal-play UI](release-captures/local-initial-play/after/world.png).
- [Known commissioned world at 4 hours](release-captures/local-commissioned-4h-play/after/world.png).
- [Map-only completed infrastructure, Debug truth explicitly labeled](release-captures/map-only-local-commissioned-24h-debug/after/world.png).
- [Medium LOD](lod-medium.png), [far LOD](lod-far.png).
- [16-second motion clip](motion.webm), backed by all 240 PNG frames and the
  position trace. Video is accelerated presentation of one simulated hour,
  not a real-time playtest. The order was research-injected; motion is kernel
  execution. Remembered people remain frozen as the ruler travels out of sight.

Visual checks are developer inspection, not new-player evidence. The scene
remains deliberately schematic. Small far-view markers lose individual role
detail; exact identity is retained for selection. The crossing obstacle is a
gap rather than an invented river. Units are known member groupings, not invented
continuous battlefield formations. Normal play cannot track remote members merely
because they belong to the ruler's unit. Region metadata does not add off-graph
movement, physical terrain or land ownership.

One existing knowledge/presentation mismatch is visible in the motion clip:
when the observation payload retains only a crossing's settlement association
and omits its edge, its marker uses the approximate near-settlement fallback.
This can look like relocation when the ruler leaves. The renderer does not query
hidden truth to restore that edge. A future clarity pass should distinguish this
approximate remembered site from an exactly located crossing; W0 does not claim
that all remembered infrastructure remains visually unambiguous.

## Preserved development evidence and failures

`captures/`, `input/`, `render-bench/` retain the first development pass. Its
benchmark overlapped other capture work, so it is not the primary performance
table. `final-captures/`, `final-input/`, `final-bench/`, `motion/` retain a second
sequential pass before the final movement-cue guard; they are not silently replaced.
`release-*` contains the exact final-source validation used above. The cue guard
changed no particles or physics; it prevents a later effect adapter from treating
an edge-bound dead body as travel.

The first video encoding attempt failed because the installed FFmpeg has no
libx264 encoder. VP9/WebM was used instead; this is a tooling limitation, not a
simulation failure. All screenshots/position traces survive independently of video.
The legacy evidence writer initially placed new metrics inside the multiline
command-log range used by the paired comparator. New metrics now follow the
existing UI fields, preserving the parser contract; final paired checks pass.

## Reproduction

Build from the implementation revision with the existing Cargo lockfiles:

```sh
CARGO_HOME=/tmp/world-of-nodes-cargo cargo test --offline
CARGO_HOME=/tmp/world-of-nodes-cargo cargo test --manifest-path client/Cargo.toml --offline
CARGO_HOME=/tmp/world-of-nodes-cargo cargo clippy --manifest-path client/Cargo.toml --offline --all-targets -- -D warnings
CARGO_HOME=/tmp/world-of-nodes-cargo cargo build --manifest-path client/Cargo.toml --offline --release --bins
python3 scripts/world_render_bench.py /tmp/w0-new-render-matrix
env LIBGL_ALWAYS_SOFTWARE=1 xvfb-run -a -s '-screen 0 1440x900x24' client/target/release/world_motion_probe /tmp/w0-new-motion
python3 scripts/spatial_input_probe.py /tmp/w0-new-input
```

Use fresh output directories. Existing graphical replay/capture scripts record
their full arguments and binary hashes. The historical replay driver is preserved
under `research/spatial-projection/validation/replay.py`; run its `scenarios` mode
with a fresh evidence directory. Offline Cargo cache location is host-specific.
No human window was automated or closed; the existing playtest directory was
left untouched and is not part of this commit.

Stop here for owner screenshots/motion review. No advanced lighting, particles,
physics, new simulation system, renderer migration or next milestone begins.
