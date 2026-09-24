# V0 / V1 — make existing meaning visible

Implementation revision: **`610d3b34a6477d77479505fd441175ac9d253ecb`**. Baseline: `37652d1a5933dd980d22fd4b2bb8ba67df3f9534`. Frozen visual/physical
comparison: P4.1 implementation `243d07b`; its accepted evidence remains unchanged.

**Implemented and mechanically validated; owner readability review pending.**
Work stops here before V2–V4. No simulation milestone is added or reopened.

The owner’s human finding was: “The simulation is technically playable but the
world is still too visually unreadable to evaluate the deeper gameplay fairly.”
The resulting task is presentation, not optimization of simulation outcomes.

## Evidence first

- [Normal Play, initial view](final/local-initial-play/after/world.png)
- [Normal Play with panels and labels hidden](final/map-only-local-initial-play/after/world.png)
- [Normal Play after actual crossing construction, 24h](final/local-commissioned-24h-play/after/world.png)
- [Explicit DEBUG world view at 24h](final/map-only-local-commissioned-24h-debug/after/world.png)
- [Frozen P4.1 comparison](final/local-initial-play/before/world.png)
- [Actual mouse selection and two accepted commands](input-final/result.json)
- [Eight paired captures and replay/hash comparisons](final/summary.json)

The full-world Debug capture deliberately reveals more than normal Play. It is
not a proposed player-information change. Initially only Hearth’s contents are
observed; remote chart marks must remain sparse. At 24h the ruler has physically
moved and its dated observations differ. The opened crossing comes from actual
materials/work after a typed commission, not a screenshot-only object.

## Changes and rationale

| Previously | This projection |
|---|---|
| Nodes, edges and labels did most of the explanatory work | Known civic place, arable patches, actual crop rows, food stock, finite wood zones and a physical-project marker provide distinct silhouettes. |
| Small dots carried most role distinctions through labels | Every supplied Person has the same small body figure; observed military role, ruler identity, steward office and recognized hostility add narrow visual marks. |
| Straight abstract links looked alike | Curved routes follow the same graph, with visible path wear where known. An unopened crossing remains a gap; material investment adds timbers and actual completion adds the deck. |
| Stock numbers competed with place labels | Stocks vary in fill with quantity. Exact quantities remain in inspection/selection; there is no invented sack inventory. |
| Unit badge could imply one assembled army across a whole route | Group standards count observed members at a node or within a direction/progress segment. Individual figures remain physically separated. |
| Looking at the world always included dense panels | F9/`--map-only` removes panels and labels for visual review; the Play/Debug watermark remains. |
| Projected labels and old click geometry could diverge | Unit labels have matching hit rectangles; curved routes and the crossing use their displayed hit positions. |

The default camera is closer. Selection remains gold, military organization teal,
recognized hostility red, memory faded/hollow and report provenance marked.
Stable Q/W/E/R and the existing command handler remain unchanged. This is not a
new mouse-control vocabulary or a claim that V2/V3 are complete.

## Causal honesty

The [projection specification](../../docs/SPATIAL_PROJECTION.md) maps every glyph
to existing permitted state. The rendering modules receive Snapshot values and
have no World/Session access. The root crate, client dependencies, scenario and
physical parameters did not change.

- The civic roof is a **place symbol**, not a house or new housing capacity.
- Green ground and clearing shapes imply no terrain cost, land title or extra plots.
- Canopy symbols denote finite wood deposits, not modeled trees/renewable forest.
- The crossing cleft is a schematic obstacle, not a new river/hydrology model.
- Available cultivation units are not drawn as planted fields. Crop state must
  actually exist; memory does not advance crops into maturity.
- Crossing timbers show committed material, not an invented labor percentage.
- Every Person figure comes from an existing supplied Person record. No crowd is
  generated from settlement counts, and unit standards do not replace members.
- Actual cart glyphs are supported, but none are spawned for the default fixture.
- A courier badge is deferred: current public Person data does not reliably expose
  that role. Private carried messages are not inspected to make an icon.
- No recipient completion acknowledgment is invented. A completed visible crossing
  and an unreported order decision can coexist under the existing information model.

## Validation

- **369 headless tests pass** (356 library and 13 integration tests).
- **35 distinct client tests pass**: 30 client tests and 5 replay-parser tests.
  The render probe repeats 18 shared tests; these are not additional mechanisms.
- Client formatting, Clippy with warnings denied, and release build pass.
- **Eight frozen/current pairs** match state hashes, event hashes, typed commands,
  scenario/seed/time, knowledge mode, known orders, known matters and visible or
  remembered Person counts. All 16 native executions independently replay exactly.
  These are controlled views of overlapping fixtures, not 16 independent experiments.
- The actual-window test selects Unit 0 by its rendered label, issues Hold,
  selects Hearth and the new crossing position, and issues a commission. Both
  commands are accepted and the journal replays exactly after a native close.
- F9 twice plus zoom/pan changes no paused world/event hash, time or command count.
- **48 historical playable scenarios plus 48 dated-input replays** match across
  193 artifacts, excluding only the two registered wall-runtime columns.
- **600 F13B histories match exactly across 3,001 artifacts**, including native
  arm outputs and the aggregate results CSV. See
  `validation-final/commands-comparison.json`.
- All **113 protected root source/dependency files** and **21,659 prior tracked
  research files** remain unchanged; client dependency manifests/lock are unchanged.
  See [preservation check](validation-final/preservation.json).

The older contact-clock exceptions documented in P4/F13 research remain preserved;
this increment neither reclassifies them nor claims every historical revision has
always been identical. No physical regression was repaired as part of this work.

## Performance and limits

The final paused render probe contains 1,000 actual Persons and draws 997 visible
records for 240 frames. Snapshot capture: **5.583ms**. Mean CPU draw:
**1.922ms**, maximum **51.677ms**. Measured frame rate including screenshot capture:
**65.3fps** in isolated Xvfb/software OpenGL. State remains unchanged.

These are host measurements, not a hardware-independent bound, simulation scaling
claim or comparison against the earlier renderer on equivalent timing conditions.
The five-frame screenshot startup FPS values are not steady-state performance.

Accepted limitations:

- This is a schematic landscape, not continuous terrain. The crossing orientation
  is tailored to the existing near-horizontal fixture edge without changing it.
- High-density node packing can overlap neighboring place symbols. The load probe
  establishes draw cost and identity preservation, not 1,000-person layout quality.
- Stock glyph fill uses a logarithmic scale and saturates at 256; exact quantities
  require inspection. Shape alone does not communicate all ownership detail.
- There is no general label collision system, wide-screen-size playtest, new
  accessibility pass or proof that a first-time player understands the symbols.
- Unknown locations are intentionally visually sparse; a full landscape cannot be
  filled with undisclosed gameplay contents for presentation convenience.

## Failed attempts retained

`development-1/` preserves the first capture matrix. Review found objects too
small, overlapping low-priority stock labels, a pond-like crossing gap despite no
modeled water, and a unit label outside its click target. The closer camera,
selection-only quantity labels, cleft banks and exact label hit rectangle address
those presentation defects. `candidate-2/` and `input-candidate-2/` retain the
next comparison and passing input check. No physical outcome was retuned.

The first broader historical replay attempt exhausted the **/tmp disk quota**.
The failed/partial logs and archives remain under `validation/`; they are not
marked successful. The same tests rerun under workspace scratch storage are in
`validation-final/`. No simulation mismatch justified a kernel change.

## Reproduction

```sh
cargo test --offline
cargo test --manifest-path client/Cargo.toml --offline
cargo clippy --manifest-path client/Cargo.toml --all-targets --offline -- -D warnings
cargo build --release --manifest-path client/Cargo.toml --bins --offline
cargo build --release --offline --bin playable_probe --bin command_chain_probe

python3 scripts/spatial_projection_probe.py /path/to/new-captures \
  --before /path/to/frozen-243d07b-client \
  --render-probe client/target/release/render_probe
python3 scripts/spatial_input_probe.py /path/to/new-input-evidence

mkdir -p target/spatial-scratch
TMPDIR="$PWD/target/spatial-scratch" python3 research/spatial-projection/validation/replay.py \
  scenarios /path/to/new-validation
TMPDIR="$PWD/target/spatial-scratch" python3 research/spatial-projection/validation/replay.py \
  commands /path/to/new-validation
```

Offline builds require the cached pinned dependencies. The capture script requires
Xvfb and starts its own displays; it never targets a human window. Build the before
binary from a separate `243d07b` checkout if the local frozen executable is absent.
The summary records SHA256 values for all compared executables and screenshots.
Historical exact duplicate replay output is discarded only after artifact comparison,
with hashes retained; original raw reference data remains unchanged.

For manual review, launch the existing native client normally. It starts paused
with Debug OFF. F9 exposes the world-only view; Space, speed controls, selection,
pan/zoom, Q/W/E/R and existing typed orders continue to operate. The CLI commission
in the controlled screenshots is explicitly injected ruler input, not autonomous
political behavior or guaranteed player success.
