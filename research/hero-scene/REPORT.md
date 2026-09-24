# W1 — The Ford: isolated art-direction review

**Ready for owner review. Visual direction not yet accepted.**

The normal client, shared W0 renderer and headless simulation remain unchanged.
This is a separate `hero_scene` binary using the existing Geography,
RenderSnapshot, route interpolation, camera and ordered Layer contract. It is not
a new game engine, a gameplay milestone or a rollout of new art across the world.

Starting revision: `42cde88`. Implementation: `0e7e5ae25b928619332c5359af85e07de2f0c073`. Exact hashes are in
`provenance.json`. [Proposed grammar](../../docs/ART_DIRECTION.md).

## Review set — five screenshots and one clip

| View | Scale | Purpose |
|---|---:|---|
| [01 Overview](review/01-overview.png) | 0.90 | Read settlement, river, crossing, detour, working land and resource zone without graph nodes |
| [02 Settlement](review/02-settlement.png) | 1.60 | Compare people/building scale, material palette, road/crossing relationship |
| [03 Close detail](review/03-detail.png) | 2.65 | Inspect simple figures, crop rows, timber/plaster/roof and canopy vocabulary |
| [04 Ruler knowledge](review/04-ruler-knowledge.png) | 1.15 | Check the same treatment on permitted observed/remembered state, not full truth |
| [05 Night](review/05-night.png) | 1.65 | Judge cool ambient darkness versus two warm stationary light-study props |

[Day-to-night video](day-to-night.webm): 16 seconds, 240 frames at 15 FPS,
1680×1050, no audio. It advances eight actual simulated hours, T+30 to T+38,
in an accelerated controlled capture. PNG frames and per-frame hashes remain in
`review/motion/` and [frames.tsv](review/frames.tsv).

The sequence is an art study, not an autonomous decision demonstration or a new
human playtest. The fixed framing makes lighting and state changes comparable.

## Visual choices

- Moss/olive open ground, sparse procedural grass and broad low-contrast texture.
- An asymmetric timber/plaster hall silhouette representing the existing
  institutional location; no invented village population or extra housing stock.
- Tiny individual people, role accents only where known, lower-detail marks at
  distant zoom. No decorative crowd filler.
- Irregular soil plots with crop rows; actual crop presence/stage determines the
  treatment. Actual goods determine the food/wood pile glyphs.
- A canopy mass for an existing finite wood deposit. It is a resource-zone symbol,
  not 28 newly harvestable tree objects.
- Earth paths with shoulders and wheel wear. They follow the same W0 curves as
  travelers. No node circles, IDs, graph labels or generic dotted connections in
  the art view. The explicit graph-debug option remains separate and off.
- River banks, subdued water glints and a timber crossing whose completion comes
  from the real project. The upper route remains a physical detour around the
  authored headwaters; it does not imply a second crossing.
- Simple ground shadows, strong dusk/night palette change and localized amber
  flame cores/light pools. No weather, battle smoke, sound or particle system.
- Minimal title/provenance strips, not a replacement command interface.

## Source-of-state contract

People, movement, crops, goods, paths, crossing progress and carts use the existing
permitted snapshot records. No fake population, crop yield, stock, cart or completed
bridge is inserted. The fixture issues the existing typed `Commission { office: 0,
project: 0 }` at T0 and lets the existing world execute until the capture times.
The simulation retains 18 Persons and ordinary consequences.

Four screenshots and the clip deliberately use **labeled research full-state
observations** so the art direction can be judged. Screenshot 04 uses actual
ruler knowledge; missing resource zones are not filled from research truth. No
new information access is added to normal gameplay.

Two authored exceptions are explicitly presentation state:

1. The river is a scenic interpretation of the existing constrained crossing
   geography. It provides no collision, hydration, fertility or hydraulic behavior.
   A geometry test ensures the detour does not intersect its banks.
2. Two stationary light-study props sit at offsets (-49,26) and (67,19) from the
   known institutional location. They are enabled only when that institution is
   currently present in the supplied observation. Their flame/light animation is
   deterministic from the real clock. They imply no inventory torches, fuel
   economy, heat, burning state or visibility bonus. They are not attached to
   arbitrary Persons. These are prototype props, not a shipped fire mechanic.

Neither exception is silently generalized into the playable world. The art
direction document requires a justified source model before shipping such light
props as ordinary gameplay objects. No physical rules, resource rights or new
systems were needed for this review.

## Validation and preservation

- 369 headless tests pass.
- 40 distinct client tests pass: previous 39 plus one meaningful river/route
  geometry guard. The hero target runs 8 tests, of which 7 repeat existing shared
  coverage; other probe targets also repeat tests and are not double-counted.
- Client Clippy with warnings denied and formatting checks pass.
- Every recorded render sample preserves the simulation state/event hashes.
- The exact commissioned command history replays independently to the same final
  state `0010304667e1a5cf` and event hash `586426cadbb4123f`.
- All previously tracked kernel/client/dependency sources and 22,755 historical
  research files remain unchanged. The only roadmap edit records this bounded
  review stop. Existing human-playtest data was untouched.
- The capture uses an isolated Xvfb/software-OpenGL window; no human window input
  was injected. The normal executable and renderer were not rebuilt or changed
  to integrate the hero treatment.

Review-frame mean CPU draw submission was 1.20 ms, maximum 2.39 ms. These are
single-scene diagnostic timings, not a performance benchmark: they exclude PNG
readback, UI captions, frame presentation and GPU completion, and do not establish
large-population rendering performance. No GPU timing or scaling claim is made.

## Problems caught and remaining tradeoffs

The first draft placed the river headwaters under the detour and let a field
overlap a road. The final authored layout fixes both without altering route
connectivity. The draft is retained under `draft/`, including its longer T30–44
sequence; only `review/` and `day-to-night.webm` form the requested final review set.
Initial compile-quality checks caught four routine Clippy issues; they were fixed.

Remaining visual limitations are explicit review material:

- Road junctions can still reveal the underlying graph's small fixture topology.
- A single institutional building symbol does not yet communicate a large town.
- Roof/forest/field silhouettes remain deliberately simple; ground texture is
  generated geometry, not a finished terrain asset system.
- Night light pools show concentric banding. They are an inexpensive style study,
  not physically correct shadowed illumination.
- The river has authored headwaters and a bounded layout; it is not generated
  geography or a hydrology claim.
- Knowledge fading is strongest for Persons and the institutional silhouette;
  consistent evidence styling for every material is not a completed V3 interface.
- No fresh unit muster or extra population was injected for the picture. Service
  may expire during the existing scenario, so the art does not invent an army to
  improve the composition.
- This scene has no new selection/command interface. Its purpose is visual choice.

## Reproduce

```sh
CARGO_HOME=/tmp/world-of-nodes-cargo cargo build --manifest-path client/Cargo.toml --offline --release --bin hero_scene
env LIBGL_ALWAYS_SOFTWARE=1 xvfb-run -a -s '-screen 0 1680x1050x24' client/target/release/hero_scene /tmp/w1-new-capture
ffmpeg -nostdin -n -framerate 15 -i /tmp/w1-new-capture/motion/frame-%04d.png -c:v libvpx-vp9 -crf 30 -b:v 0 -cpu-used 4 -row-mt 1 -threads 2 -pix_fmt yuv420p /tmp/w1-new-capture/day-to-night.webm
```

Use a fresh output directory. The Cargo cache path is host-specific. No network,
new package, model asset pipeline or external engine is required.

**Stop:** the owner should choose or reject this visual direction before further
renderer work. No advanced features or general art rollout follow automatically.
