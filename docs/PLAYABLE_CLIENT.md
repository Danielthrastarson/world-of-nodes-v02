# First kingdom client

The optional native Rust client exposes the existing world. It does not replace
the kernel or the historical research programs. The root crate still has no
graphics dependencies. This is a graph-based playable prototype, not a finished
game or a continuous battlefield.

Optional human-session recording and the local diagnostic connection are
documented in [PLAYTEST_BRIDGE.md](PLAYTEST_BRIDGE.md). The ordinary launch below
keeps recording disabled; the recorded launcher preserves a fresh session.

```sh
cargo run --release --manifest-path client/Cargo.toml
```

The first build fetches pinned Macroquad dependencies. Linux requires an available
X11/Wayland display and OpenGL. Linux/X11 is tested; other native platforms and
browser/WASM deployment are not yet validated. The renderer has no network or AI
service dependency. Its bundled Liberation Sans font retains its license in
`client/assets/FONT-LICENSE.txt`.

## Start playing

The default Local scenario starts paused. The ruler is the gold Person at Hearth
settlement. Four existing people, including the ruler, have already voluntarily
assembled. The office acceptance, courier agreement, old map knowledge, household
structure, resources and raiders are explicit initial conditions. They are not
evidence of spontaneous political organization.

1. With Hearth selected, click **Commission crossing** in the right panel (or **W → 1**), click the crossing marker, then **Enter**. This issues
   an office-scoped order; it assigns no workers and creates no infrastructure.
2. Press **Space** to run. Click **4x** or **16x** to advance faster. Inspect the
   crossing and the Orders panel. Wood, labor and voluntary participation still
   determine whether construction finishes.
3. Select **UNIT 0**. **W → 2 → East fields → Enter** asks the local unit members
   to approach and engage the existing hostile raid. They must take a real route.
   Alternatively, right-click a location for Move. Orders only reach members
   beside the embodied commander. The ruler does not stay safe off-map.
4. Inspect Reports. Every report has an author, sender, observation time, receipt
   time and age. A late observation does not reveal the current distant world.

No input is also a valid run. The inherited autonomous policies continue while
the world is unpaused. The scenario contains real failure and starvation; there
is no scripted victory, free supply, automatic defense or tuned optimal opening.

## Controls

| Input | Meaning |
|---|---|
| LMB | Select; choose a command target |
| Enter / Confirm button | Issue the composed typed command |
| RMB | Cancel/back while composing; contextual Move or locally recognized hostile Attack with Unit 0 selected |
| Wheel | Zoom about cursor over map; scroll inspection text over sidebar |
| Arrows / middle drag / right drag | Pan camera |
| Space | Pause/resume |
| 1x / 4x / 16x buttons | Playback speed; 1x is explicitly 2 simulated minutes per real second |
| Q | Observe: 1 Inspect, 2 Reports, 3 Record local observation, 4 Help |
| W with civilian context | 1 Commission crossing; 2 send revocation of selected order |
| W with Unit 0 selected | 1 Move, 2 Attack, 3 Hold current node, 4 Withdraw to another node |
| E / R | Stable Govern / Allocate categories, visibly deferred in this slice |
| F1 / Escape | Help; dismiss help or command composition |
| F2 | Explicit DEBUG truth toggle; grants no command rights |
| F9 | Hide/restore panels and labels; knowledge mode stays unchanged |
| F10 | Save a screenshot; optional `--capture PATH` sets destination |
| F12 | With `--evidence PATH`, save inputs, metrics and independent replay comparison |

Help blocks command input. Rendering, selection, camera movement and entering
debug mode never alter simulation state. The simulation clock advances through
fixed steps; rendering interpolates only between already executed poses. Slow
frames retain backlog. The UI displays that backlog rather than dropping events.

## What the map means

Solid figures are currently observable Persons. Faded outline figures are last-seen
Persons, not live tracking. Gold identifies the embodied ruler. Military cloth
and shields mark observed unit membership; red requires locally recognized
hostility, unless DEBUG is on.
Node packing and the map layout are schematic; they are not continuous physical
positions or invented formation geometry. Progress along an edge is physical.

The roof/place symbol marks a known public institutional location, not new housing.
Arable patches represent actual cultivation units; planted rows require an actual
crop. Stock glyphs vary with known quantity; wood-deposit canopy symbols represent
finite resources, not simulated trees. A gap and partial timbers mark an existing
crossing site/project; only a completed crossing gets a full deck. Curved route
width reflects observed path wear where known.
Carts, if present, are actual objects with their own owner and user. The playable
scenario does not spawn decorative carts or traffic to populate the scene.

F9 gives a world-only view with a persistent knowledge-mode watermark. The V0/V1
[projection contract](SPATIAL_PROJECTION.md) distinguishes display geometry from
physical state. No water, land use or unknown remote contents are inferred for art.

W0 moves coordinates, route sampling and presentation data into a backend-neutral
[world presentation contract](WORLD_PRESENTATION_ENGINE.md). At close zoom people
use figures, at medium zoom compact markers, and at far zoom tiny individual
marks with fewer labels. These are the same known Persons. A restrained ambient
cycle and simple body shadows use simulation time; they do not change knowledge
or gameplay. No torches, particles or water sources are fabricated.

The inspector shows source and observation age. Map-only starting knowledge does
not mean a place was observed empty. A historical cart marker remains dated; it does not claim current presence.
Unit badges count members in the current view, not global strength or current
global orders. A remembered unit target is explicitly labeled with its unavailable receipt date;
it is not a current global order. A separate **Last issued by you** record shows
the exact local command time and recipient count, without asserting later execution.
DEBUG can inspect true inventories, queues, goals, roles and related diagnostics;
it is visibly labeled and is not the normal ruler-information view.

## Orders, reports and limits

An issued order is known to its issuer. A personally performed handoff is known.
The agreed courier's subsequent distant movements and delivery are **not** live
tracking. The existing report vocabulary does not carry an acknowledgment of the
steward's obligation status. Therefore Play says recipient decision/status is
unreported even when DEBUG knows the result. A returned physical NodeView report
can reveal observed project consequences; it does not fabricate a command ACK.

Revocation must travel too. It ends the applicable future role obligation on
receipt. It does not destroy an already commissioned project, refund invested
wood, undo work, or revoke public access. A completed role obligation means that
the commission was attempted, not that construction physically completed.

Use `--remote` for the physical courier variant. Its initial quarter-hour trip is
real movement. Ordinary policy ends the ruler's military service during that
journey; P2 orders consequently reject in this variant. This is preserved, not
silently repaired by restoring a role. Use the default Local setup for P2.

Military Attack means approach a known graph node and engage existing recognized
hostiles. It cannot make an arbitrary citizen an enemy. Withdrawal suppresses the
departing person's strikes but leaves them vulnerable until they actually leave.
There is no remote control of dispersed members, pursuit of unseen opponents,
continuous line geometry, army inventory, supply grant or immunity.

The ruler can become hungry, sleep, lose service, or die. There is no succession
or restart-in-world power. Restarting the executable creates a new scenario.
E/R do not add mobilization recruitment, taxation, requisition or new laws.

## Reproduction and development

```sh
cargo test --offline
cargo test --manifest-path client/Cargo.toml --offline
cargo run --release --bin playable_probe -- scenarios /tmp/won-play-new 3 24
cargo run --release --bin playable_probe -- performance /tmp/won-scale-new 1000

# Actual keyboard/mouse events in a separate X server; no desktop manipulation.
xvfb-run -a -s '-screen 0 1440x900x24' python3 scripts/playable_clarity_smoke.py /tmp/won-ui-new commission
xvfb-run -a -s '-screen 0 1440x900x24' python3 scripts/playable_clarity_smoke.py /tmp/won-army-ui-new military
```

`--seed N`, `--hour H`, `--commission`, `--debug`, `--reports`, `--map-only`, `--zoom Z`, `--run`, `--frames N`,
`--capture PATH` and `--evidence PATH` support reproducible research captures.
`--commission` dispatches the same typed command as the UI; it is not an instant
construction switch. `--hour` advances the ordinary Session to that checkpoint.
Research captures and replay hashes are explicitly outside normal player knowledge.

No save/load format, polished tutorial, audio, binding remap, final accessibility
pass or browser release is claimed. Human play-testing is still needed to evaluate
comprehension and pacing; automated input tests are not evidence of enjoyment.

## P4.1 clarity pass

Start at 1440×900 where possible. Gold outlines connect map selection with the
inspection title; teal identifies observed friendly organization, and recognized
hostility remains red. The Current matters strip lists up to four items derived
from ruler-visible evidence. Click a matter to inspect its subject. An observed
food request is not a diagnosis of every resident's private hunger.

The right panel leads with status and available actions. Disabled controls keep
the reason visible; the authoritative command handler still validates an attempted
command. Q/W/E/R meanings stay fixed. Map-only and remembered objects are muted
and hollow; select/scroll to Evidence / time for exact original timestamps.

Orders show Issued → Handoff → Received → Obligation → Result. Filled steps have
evidence; hollow steps remain unknown. A local handoff never fills the obligation
or physical-result step automatically. Debug has a red banner, and even there
command composition uses the separate Play snapshot. Debug-visible unknown
crossings cannot become Play targets.

The interface pass changes no kernel or policy. [Evidence and limits](../research/playable-clarity/REPORT.md).
