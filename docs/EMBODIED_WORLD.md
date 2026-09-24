# E0 — same Person, two views

Status: bounded prototype; **human presence/preference gate OPEN**. No first person,
combat controls, riding, succession or broader 3D production follows without review.

## Boundary and engine choice

`World → Session → ruler Snapshot → W0 RenderSnapshot → E0 3D Scene`

The separate `embodied_client` binary uses pinned Macroquad. Its
[3D primitives and cameras](https://macroquad.rs/examples/3d/) suffice for this
40-Person experiment. [Bevy ECS](https://bevy.org/learn/quick-start/getting-started/ecs/)
adds an entity lifecycle to synchronize without a demonstrated need here.
[wgpu](https://wgpu.rs/) remains an option if measured requirements justify it.
No dependency, kernel migration or graphics dependency in the headless crate was
added. The original strategic client remains unchanged. E0's elevated camera is
another view of one in-process Session, not a second synchronized simulation.

`EmbodiedSession` is a headless input/detail adapter. It owns a bounded local
`Pose(node,x_mm,z_mm)` for sovereign Person0, not another avatar. A node is refined
into an 18m-radius walkable cell. Local walking uses integer millimetres and elapsed
authoritative time; it cannot change nodes. Building/grove footprints block walking.
At the road meeting point, known adjacent destinations issue ordinary Travel:
capacity, FIFO, duration, hunger, fatigue and mortality remain kernel-owned.
Closed/unconnected routes are rejected. Steering cannot cancel an active traversal.
Camera coordinates never determine Person location.

This is the explicitly bounded local-detail option, not continuous kernel physics.
Local walking adds no separate distance-based fatigue cost; time-based physiology
continues. A node remains the observation/contact cell: local steps do not introduce
finer line-of-sight, combat reach or institutional proximity. No person collision;
camera-terrain clearance is basic and buildings can occlude the camera. These are
production questions, not silently claimed solved mechanics.

Both cameras use Play snapshots. Remote remembered Persons remain frozen/darkened.
No remote stocks, private households/goals or roles are read from World to render.
Inspection identifies missing household knowledge. No debug truth toggle is exposed.
Replay/performance logs are developer evidence, not ruler knowledge.

## Controlled scene, not an emergence claim

Seed0 extends the local fixture to 40 real Persons, retaining its sovereign,
steward, courier, two dependents, four-person militia, six cultivation units,
communal food, wood, graph, crossing and seeded threat. Added adults own starting
food. People start rested in E0, and consenting militia accepts an 18h service
term through existing eligibility/muster. Historical 18-Person defaults are intact.

Person18 initially owns10 loose wood and starts ordinary 4h cart construction.
Five willing donors19–23 each carry4 wood and6 food (capacity10). A sovereign
Commission is issued atT0. AtT+2m, after institutional processing, donors contribute
real goods and Person19 attempts construction. Policies continue normally: a start
is not forced completion. Play begins T+14h, after the bridge actually completes.
If the cart owner remains present, controlled hitch/travel attempts use ordinary
rights/actions. No completed object is spawned for the handoff. Courier5 then captures a real
local F12 report and receives a controlled delivery purpose to meet Person12 at
node2. This is a stated rendezvous, not a query of that person's current location;
ordinary delivery policy can succeed or fail.

These are controlled prehistory actions, not player power to compel private donors
or proof of autonomous cooperation. Rest, service term and starting endowments are
disclosed fixture choices, not demographic/economic calibration. Service expires,
people depart, and hunger can disrupt the scene. It never resets to preserve crowds.

## Presentation

An authored valley embeds stable W0 coordinates into `(x,height(x,z),z)`. Connections
are curved roads; completed known crossings have decks. Fields, stock glyphs,
forest zone, cart and figures come from known records. One hall represents the
actual communal institution, not invented housing/capacity/interiors. This is a
village-scale schematic, not a complete architectural village.

River/terrain are embedding metadata, not hydrology. Near/far low-poly figures and
culling affect presentation only: no full controller/physics/AI per Person.
Directional/contact shading and warm lamps are explicitly presentation studies,
not simulated fuel/fire/visibility. Daylight uses the world clock, sunrise T+6h.

## Build/play

```sh
cargo build --release --manifest-path client/Cargo.toml --bin embodied_client
python3 scripts/start_embodied_playtest.py
```

Prepared offline host: prefix Cargo with `CARGO_HOME=/tmp/world-of-nodes-cargo`
and add `--offline`.

- Starts paused, strategic, ruler knowledge only.
- V / top-right button: strategic ↔ third person, no pause/reset/reload.
- Space / button: pause. +/-:1×,4×,16×; 1×=10 simulated seconds per second.
- Third person: WASD local walk, right drag orbit, wheel distance.
- Strategic: middle drag pan, wheel scale, right-click known location for real
  ruler travel. Return near local(0,0) before leaving the detail cell.
- LMB inspect visible Person. Unknown private state stays unavailable.
- W / Orders: existing crossing commission and locally received militia move/hold.
  In third person W walks; use Orders button. This provisional E0 conflict does
  not replace the original client's stable strategic QWER vocabulary.
- Q / Local report: existing report command.
- Escape / window close: finalize logs and independently replay inputs.

Commands can reject, delay or fail. The prepared crossing is already built;
inspecting its order is useful, reissuing it is not a new construction opportunity.
Military command authority remains local.

## Verification/review

Record timestamped accepted/rejected commands, UI/camera/time state, pose, hashes
and CPU/RSS. On exit reconstruct prehistory and replay inputs, verify hashes/pose.
Rendering asserts World hashes remain unchanged. Fixed one-second simulation steps
are frame-rate independent; clock backlog is retained. Capture time skips are
labelled. Human sessions receive no scripted input.

Owner playtest must judge presence, strategy, pacing and desire to continue. Neither
a screenshot nor mechanical parity proves those experiential gates.
