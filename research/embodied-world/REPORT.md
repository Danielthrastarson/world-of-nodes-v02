# E0 — embodied-world prototype review

**Implementation:** `ec11ad4` (initial implementation `cb69203`).
**Status:** playable prototype ready for owner testing; **E0's immersion and owner-preference gates remain OPEN**.
No further renderer, first-person, combat or simulation expansion is authorized by this report.

## Run

From the repository root:

```sh
CARGO_HOME=/tmp/world-of-nodes-cargo cargo build --offline --release --manifest-path client/Cargo.toml --bin embodied_client
python3 scripts/start_embodied_playtest.py
```

The cache path is specific to this prepared host. Other machines can use ordinary
Cargo without `--offline`. The built native executable is
`client/target/release/embodied_client`. Original strategic client remains available.
The launcher creates a fresh local human-session directory; it sends no inputs and
captures no screen/audio. Escape or normal close finalizes command replay.

Starts paused. **V** switches cameras; **Space** runs/pauses; **+/-** adjusts time.
Third person: **WASD**, right-drag orbit, wheel distance. Strategic: middle-drag pan,
wheel zoom, right-click a known adjacent location for real ruler travel. Return
near local `(0,0)` first. **LMB** inspects a Person. **W / Orders** opens existing
institutional/militia actions; in third person use the button because W walks.
**Q** captures a legitimate local report. No debug omniscience is exposed.

## What was built

One separate Macroquad 3D client, one World/Session, two cameras, the same mortal
sovereign Person0. Forty actual Persons are alive at the prepared handoff, including
four voluntarily assembled militia members. A real privately constructed cart,
communal crossing, six cultivation units, resource zone, households and an F12
courier purpose remain in the existing simulation. No dummy crowd or avatar.

One authored valley/river, a communal hall silhouette, crop rows, forest-zone glyph,
organic graph-route projection, low-poly figures, cart and bridge form the scene.
The hall is a representation of the existing institution, not fabricated housing. Grey military panels are role glyphs, not newly simulated shields or armor.
River, sunlight/contact shadows and warm study lamps are presentation metadata;
there is no new hydrology, fire/fuel, lighting-based knowledge or building system.

The [architecture contract](../../docs/EMBODIED_WORLD.md) documents engine comparison,
source boundaries, local cells, knowledge limits, exact controls and prehistory.
Macroquad was sufficient; no dependency, kernel/ECS migration or alternate Person AI.
Both cameras consume the same ruler observation snapshot through W0 projection.

## Controlled setup and negative findings

This is an authored experiment, **not an autonomous settlement success claim**.
The starting fixture extends the historical local world to40 people, initially
rested, with an explicitly consented18h militia term. Existing eligibility checks
are unchanged. Extra food and private construction holdings are initial conditions.

- Five willing initial donors each carry4 wood. A typed sovereign Commission atT0
  propagates through existing local authority. AtT+2m they contribute their actual
  property. Person19 attempts construction, then ordinary policy redirects them.
- Person2 later finishes the remaining work. The crossing completes atT+12h,
  consuming20 wood and8 person-hours: one minute by19, the remainder by2.
- Person18's cart consumes10 wood and4h, completing atT+4h. At handoff T+14h the
  owner hitches it and begins an actual edge traversal. Cart custody, private
  title and capacity20 are preserved. This first trip is not a throughput claim.
- Courier5 captures a local report atT+14h and receives an existing F12 delivery
  purpose: meet Person12 atnode2. It does not query that recipient's live location.
- The captured unit order is locally received by four existing members. A later
  return order is rejected because the ruler is mid-edge. Receipt is not arrival.
- By the night segment, service/survival and congestion have continued normally;
  the ruler is still travelling. No new nighttime muster or protective benefit is
  claimed. Camera mode does not create remote command authority.

Rejected setup attempts are retained in validation logs: extending the original
service term failed six-hour survival/fatigue checks; nutrition48h violated the
kernel's24h bound; earlier handoffs found unfinished construction. None of those
checks was relaxed. The final controlled rest/endowment/time choices are disclosed,
not hidden rebalancing or evidence that people naturally cooperate this way.

## Evidence to review

Use **release/** and the labelled **E0-review.webm** as the final review set:

- `release/view-020.png`: elevated strategic valley view.
- `release/view-065.png`: following the real ruler among local people.
- `release/view-200.png`: strategic view after the military movement order.
- `release/view-300.png`: the same continuing world at night.
- `E0-review.webm`:24 seconds,1440×900,15fps,360 frames, no audio.

The video adds a permanent acceleration caption; its unannotated source is retained as release-motion.webm. The clip accelerates ten simulated seconds per captured frame and explicitly skips
from the afternoon toT+20h at frame240; it is not24 seconds of unaccelerated play.
Camera switches occur at35/180/215, local walking45–52, local report90, militia
move110, attempted return210. All commands, outcomes, frame hashes and replay
results are retained. Four screenshots plus the encoded clip are kept; disposable
per-frame PNGs have content hashes and can be regenerated. Earlier drafts remain
labelled separately and are not the final visual judgement.

## Mechanical verification

- Full headless suite: **374 tests pass** (361 library +13 integration).
- Five new E0 boundary tests cover local time/collision limits, mortality,
  forbidden/closed travel, actual Person travel, observation read purity,
  replay, and one-second versus five-minute time partitioning.
- Final river-headwater audit adds coverage within the boundary tests. Adapter
  and renderer share five integer scene centres; local cells cannot enter water.
- All seven client-target test suites pass (shared tests repeat across binaries;
  this is not119 distinct new tests). E0 Clippy with warnings denied and formatting pass.
- **48 historical scenarios +48 input replays,193 artifacts match** all ordered
  semantic fields and uncompressed trace bytes. Only pre-existing wall-time fields
  are excluded. The first run stopped on `/tmp` quota; its failed output is retained.
  The successful rerun used workspace scratch storage, not changed assertions.
- Actual isolated keyboard/mouse testing confirms paused camera switches leave
  both World hashes unchanged, walking changes the same ruler's local pose,
  LMB selects Person0, the commission button creates a typed delivered-local order,
  speed control works, and the whole session replays exactly.
- Each captured draw checks World state/event hashes before/after rendering.
  Independent final replay verifies state, events and local pose. Source/binary
  hashes are in provenance.json. Existing tracked research remains unchanged.

Initial capture exposed Macroquad's default index-buffer clamp; E0 now allocates
enough capacity for its small terrain mesh. Draft images/logs retain the fault.
The input harness initially lacked `xdotool`; it now reuses the repository's XTest
helper. All graphical tests use their own Xvfb display, never a human game window.

## Performance and limits

See release/performance.json and input-release/performance.json for raw samples.
Measurements use **software OpenGL in Xvfb**, not a hardware GPU benchmark.
The isolated interactive sample averaged about41fps with roughly1.93ms mean CPU
world draw submission (p95≈2.85ms),36 maximum on-screen Person centres, and≈318MiB
peak process RSS. Most frames were paused; its near-zero simulation average is
not a simulation throughput claim. The final capture averaged1.02ms CPU per ten-step simulation batch (p95≈2.14ms) and1.71ms CPU draw submission. The separate capture advances10 one-second
steps per frame and measures that batch, including snapshot updates; the explicit
long night skip is outside that batch timer. PNG readback affects capture FPS.
GPU execution timing is unavailable and reported null. No large-population claim.

The renderer uses primitive batching, distance culling and near/far Person detail.
It adds no physics controller per Person and can later consume an instanced backend.
That is an architectural path, not proof of performance at thousands of people.

## Deliberately unresolved compromises

- Local walking refines a node inside18m. Between places, use real graph travel;
  this is **not unrestricted terrain roaming**. Busy, queued, sleeping, wounded or
  dead rulers do not get an overriding action-game controller.
- Local walking consumes authoritative time, but has no additional distance-based
  fatigue model. Nodes remain the interaction/observation scale; no fine sightlines.
- No person-person collision, general local pathfinding, free bridge approach,
  robust camera-wall avoidance, animation rig or detailed character art.
- Ordinary policies are not forced to keep farmers busy or repeat a nighttime muster for the picture; not every requested activity is guaranteed to co-occur.
- One institutional hall is a schematic village centre, not a full street/building
  layout. Figures face generically; crop/work locations retain graph abstractions.
- Knowledge is shared and remote memories freeze/fade, but E0 does not reproduce
  the original client's entire reporting/WHY interface. Unknown household detail
  stays unavailable. Courier purpose exists; no invented omniscient courier labels.
- Crossing is already completed at handoff. Recommissioning is a valid command-chain
  inspection exercise, not another construction opportunity. Military movement is
  the immediate live consequence in this prepared scene.
- The night uses labelled study lights, not a new authoritative torch system.
- Command controls are a temporary subset. W means walking in the following view;
  Orders remains available by button. Original strategic controls were not replaced.

## Production cost — planning estimate, not a commitment

If the owner prefers embodiment, expect roughly **8–16 engineering weeks** to
harden one small slice: robust camera/navigation, clear node/local transitions,
knowledge-aware interaction, basic animation/content tooling, input logs and
profiling on real GPUs. A production-scale embodied kingdom is more plausibly
**6–12+ person-months**, excluding extensive content, multiplayer and a full action
combat game. Confidence is low; local navigation/authority semantics and the art
pipeline dominate uncertainty. Nothing measured here yet justifies engine migration.

## Owner playtest decision

Mechanical continuity is supported. Stronger emotional connection, coherent
switching, desire to visit places, meaningful army presence, acceptable pacing,
convincing scale and preference over strategy-only play are **unanswered**.

Please play before choosing production work. In particular: did standing among
these people change a decision; did waiting/travel fight the strategy pacing;
did switching feel like one place; and would you choose to return to this view?
First person is not justified merely by completing this prototype.

**Stop here. E0 is ready for review, not declared passed at experiential scope.**
