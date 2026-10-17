# World of Nodes — Generalship G0

G0 is an independent, authoritative, two-player battlefield experiment. Its question is whether delayed information, physical orders, and a small command vocabulary reward positioning and anticipation between people. It is not the finished kingdom simulation.

The research kernel, provisioning policies, experiments, saved results, and baseline tests remain unchanged. This crate reuses the kernel's integer `Stock` type, not its time, movement, personnel, or starvation policies. Baseline: `8072a4152f1557c4fef24f74b260c003438aec7d`. Implementation branch: `codex/generalship-g0`.

## Start a private game

Open the deployed service, enter the private host code, and choose **Create private match**. Send the generated one-use invitation to one opponent. Both players press **I’m ready** on the start screen. The battlefield controls unlock when the server starts the round. Command desk → People → Reported personnel allows individual selection and shows dated health/food information. Each browser holds its own reconnect credential in session storage; keep that tab available. The host code is for creating rooms, not joining an opponent's seat.

The current temporary Internet URL and verification record are in `generalship/PLAYTEST_RESULTS.md`. The game server runs on this computer, reached through a Cloudflare Quick Tunnel; it is not an always-on cloud deployment. The host code must remain outside Git. A service process restart aborts its rooms without awarding a winner. A disconnected browser has sixty seconds to return while standing orders continue. Both players can request a rematch after the result.

## Solo developer practice

Choose **Play vs bot · dev mode** using the same private host code, then **Start practice**. A multiplayer lobby with an unclaimed invitation also offers **Play vs bot**; choosing it starts practice and invalidates that invitation. A joined human cannot be replaced by a bot. You command West; the bot commands East. **Surrender → Play again vs bot → Start practice** resets the same room without a second person. Reconnection uses the same sixty-second grace as multiplayer.

The practice opponent uses only its filtered player snapshot and submits ordinary validated inputs. Its accepted decisions enter the replay journal, so replay does not depend on running the AI again. It pushes line/ranged infantry toward the bridge, sends light infantry by the ford, accompanies columns with physical generals, reacts to recent reports, withdraws depleted formations, assigns finite food routes, and attempts costly general replacement. It receives no map reveal, resource bonus, immediate orders, or casualty immunity. Its simple fixed opening is for testing controls and tactics, not a substitute for multiplayer balance testing. It does not build or upgrade outposts.

## Temporary Internet playtest

The current public link is **https://officially-yearly-assist-definitely.trycloudflare.com**. This build runs on local port 8085 so the prior build can finish its existing matches. It works while this computer, the Rust server, and the tunnel remain running. It exposes the game HTTP/WebSocket routes, not the repository filesystem. Match creation still needs the private host code in `generalship/runtime/host-code.txt`. Share the generated opponent invitation instead of that code.

From the repository root:

```bash
python3 generalship/playtest.py status 8085
python3 generalship/playtest.py stop 8085
python3 generalship/playtest.py start 8085
```

Omitting the optional port selects 8080; each port has its own supervisor and log files. Starting prints a fresh public URL. `start` builds the Rust executable, starts it on localhost, and opens the tunnel. Ctrl+C stops both processes. `status` and `stop` should run in a normal host terminal; an isolated sandbox cannot see the host's process table. Python 3 and the official `cloudflared` binary at `generalship/runtime/cloudflared` are required. The locally installed binary is Cloudflare release 2026.10.0, checked against its published SHA-256 digest; the binary and host code are ignored by Git. On another computer, obtain the appropriate binary from [Cloudflare's downloads](https://developers.cloudflare.com/tunnel/downloads/).

A reusable external test uses Node.js 22 or later:

```bash
node generalship/internet-smoke.mjs https://YOUR-CURRENT-URL.trycloudflare.com
```

It creates two test seats and verifies HTTPS, secure WebSockets, ownership isolation, a one-use invitation, movement, reconnect, surrender, rematch, and post-match replay download. It reads the local host-code file or `G0_HOST_CODE`; it does not print credentials. Run the replay verifier below against `generalship/runtime/internet-replay.json` afterward.

Cloudflare Quick Tunnels are temporary development links, with no uptime guarantee; the hostname changes on restart. [Provider documentation](https://developers.cloudflare.com/tunnel/get-started/quick-tunnels/). Render service creation was rejected because Render could not fetch the private repository. No Render service was created, and no hosting was purchased. The Render configuration remains available if permanent hosting is chosen later.

## Exact local launch

Requires Rust 1.98 and Cargo. Run from the repository root:

```bash
read -rsp 'Private host code (12+ characters): ' G0_HOST_CODE
export G0_HOST_CODE
cargo run --locked --release --manifest-path generalship/Cargo.toml --bin generalship-server
```

Open `http://127.0.0.1:8080` in two separate tabs or browser profiles; each invitation reserves a different seat. For LAN use set `G0_BIND=0.0.0.0`; normal local use binds only to localhost. `PORT` defaults to 8080. `G0_DATA_DIR` defaults to `generalship/runtime`. Local replay files are ignored by Git.

Use the committed lockfile. The browser assets are embedded in the executable, so rebuild and restart after changing HTML, CSS, or JavaScript.

## Gameplay feel pass — G0.2

The October 17 browser playtest found silent general MOVE orders, support commands that did not change duties, overlapping labels and arrival positions, buried delivery feedback, and carriers marching into danger before assignment. G0.2 fixes these specific problems. It does not establish that the game is balanced or fun for two humans.

Formation buttons now stay above the battlefield, and double-clicking a troop selects its formation. Right-click issues MOVE. The regular MOVE slot directly moves generals and sets the duty rendezvous for scouts, couriers, and carriers. Unsupported combinations give a clear explanation. Auto command chooses among known nearby command sources, falling back to a source with a recently observed courier; it cannot know hidden availability.

Delivery status stays beneath the field and distinguishes pending, failed, and partial delivery. Estimates use the last reported recipient position and fastest configured ground speed; they are a lower bound, not a promise. Group labels and counts use reported people, health bars use observed health, and death notices appear only when received. Scroll over the field to zoom; Fit map resets the view.

Scenario g0.2 adds seven-metre formation arrival spacing and makes carriers wait at HQ until assigned. This preserves people, resource totals, courier delay, fog of war, combat values, the bridge capacity, and the twenty-minute match clock. The earlier g0-v1.json remains as a balance record; replays require their matching engine revision.

## Smooth movement — G0.3

The previous renderer restarted a 100 ms animation on every snapshot arrival. Uneven delivery could interrupt the animation or make it pause; labels used unsmoothed positions. Outposts also sampled only once per second and bypassed interpolation entirely.

The renderer now buffers 200 ms of authorized snapshots, advances a monotonic display clock, and moves dots, labels, range rings, and selection targets together. Duplicate snapshots do not restart motion. It never extrapolates beyond received positions; old courier reports, lost contacts, casualties, and changed signal delays remain explicit report updates. A connection stall exhausts the buffer and holds the last authorized position. This small display delay trades immediacy for steadier movement; orders still send immediately.

Scenario g0.3 samples connected outposts every tick (10 Hz), retaining the one-second base delay per link and 300 ms upgraded delay. The new `relay_report_ticks` configuration records that cadence; older configurations default to their former ten-tick interval. Terrain is drawn once into a cached layer instead of redrawing 5,400 cells every animation frame. See the dated playtest record for tests and deployment checks.

## Compact command interface

The battlefield now spans the window. General and formation shortcuts sit along its upper edge. Selecting living people opens a compact five-command bar in the lower left, with a short selection summary and delivery status. Buttons show the custom key, verb, and an icon; hover text retains execution modifiers and follow-up details. Unsupported commands for generals or support roles are dimmed with an explanation.

Choosing a command tucks the bar away and shows a targeting prompt with Cancel. Clicking a destination restores the bar. This also keeps the ford accessible in smaller windows. Right-click MOVE and the existing 1–5, Alt+1–5, and Ctrl+Alt+1–5 bindings remain available.

The collapsible Command desk has Orders, People, Works, and Reports. Details beside a selection opens People; the delivery arrow opens Orders; the contact indicator opens Reports; clicking a post or hill site opens Works. Starting a map assignment closes the desk. Esc cancels targeting first, then closes the desk, then clears selection; none of those actions cancels a dispatched order. Match surrender is under Reports, and rematch remains in the top bar. Before the round starts, a central start screen explains that the field is paused and presents Start practice or I’m ready. Battlefield controls stay unavailable until the server confirms the round is running. A lost connection displays a reconnect screen while standing orders continue; unsent orders are not presented as successful dispatches.

This is a client interface change on scenario g0.3. Simulation and network rules are unchanged.

## Visual presentation — field-map-1

The renderer remains an inspection surface for the simulation. `web/field.mjs` draws only the authorized player view and positions supplied by `motion.mjs`; it cannot access the server world, issue orders, or change physical positions. The existing 200 ms observation buffer remains intact. This pass changes no scenario values, simulation rules, or navigation cells.

The field uses a quiet survey-map palette, exact cell boundaries, static woods/water hatching, bridge decking, a 100-metre grid, coordinates, and a zoom-aware distance scale. Terrain is common prior knowledge. These patterns are map symbols, not simulated individual trees, currents, or elevation contours. Static terrain is cached at 1800×1200 and reused between frames.

People remain dots. Generals are diamonds, ranged infantry has a dark inner ring, and supply personnel have small duty marks. Filled dots denote recent observations; hollow, faded dots denote reports at least three seconds old or a contact known to have left the observed area. Hover shows identity, recorded health, and report age. No observed person acquires invented motion. Selection brackets surround the reported bounds of the selected people. Plan lines are explicitly labelled PLAN; they do not claim confirmed pathfinding or completion. Broad circular vision washes were removed because they falsely implied unobstructed sight through woods.

Screen-size labels remain readable at every zoom. A narrow force rail, compact resource strip, selection-only command dock, on-demand map key, and collapsible Command desk keep the battlefield central. Tablet/phone layouts reflow the force buttons across the top. The map key and command dock tuck away during targeting; keyboard and pointer commands retain their previous meanings.

## Command structure and physical movement — G0.4

Ordinary commands do not require construction. A general or headquarters hands an order to locally present addressed people. For infantry orders, those recipients pass the original instruction to other addressed formation members within 30 metres and line of sight, taking 300 ms per link. A broken link or a newer received sequence stops the older handoff. Detached people retain their standing orders until a message actually reaches them. The original addressed group supplies stable formation slots and declared strength; local execution and follow-ups do not mutate the instruction being passed.

Couriers still travel at 8 m/s on open ground (10 m/s on roads), compared with 4.5 m/s for line infantry. Their dispatch-time destination stays fixed. They can hand over to addressed recipients they physically encounter on the route, and return acknowledgements when they locally reach headquarters, a living general, or a connected post. They do not chase hidden coordinates. Receipt counts combine returned acknowledgements with received observations of friendly order sequences; they never read unseen current state.

Build and staff outposts to extend a command network into positions that generals cannot cover. Posts require their existing two staff and functioning signal chain. A forward post also needs a courier nearby to dispatch beyond local exchange distance. Upgrades improve signal range and transmission, while isolated detachments still depend on physical messengers. These costs and the existing construction and supply rules remain in force.

All roles now share one physical movement resolver. Starting deployment reserves room for each body; policies submit destinations, then deterministic routing and swept separation checks resolve movement. Nearby visible people influence route costs. Short clear sections of the navigation corridor are smoothed so cell centres do not become compulsory pile-up points. Rotating tie priority, local passing, terrain clearance, and shared FIFO bridge admission preserve physical capacity. Occupied rendezvous are approached with space around them. The six-metre minimum centre spacing is recorded by `person_radius: 300` (centimetres) in the versioned scenario, alongside local relay range and time. The legacy scenarios remain as records; replays require the matching engine revision.

## Five-minute introduction

**0:00–1:00 — Read your information.** The green people belong to you; orange people are observed enemies. Diamonds are generals, squares are outposts, dashed squares are potential building sites. Faded dots are timestamped reports. A hollow contact means a later observation found its old location empty; it does not mean death. Scouts begin on the approaches and return reports. Terrain is common knowledge, enemy deployment is not.

**1:00–2:00 — Give one deliberate order.** Recall Group 3 (light infantry), press 1, and click near the bridge. Watch the order acknowledgement. “Accepted by server” is only a network receipt. An in-world acknowledgement arrives after delivery. Headquarters and generals exchange orders within 30 metres; addressed troops pass the instruction through nearby formation members. Distant or detached units need a courier to visit their last reported position or encounter them locally; repeated clicking sends more work.

**2:00–3:00 — Put a general where a decision matters.** Select a general, then right-click a destination or use MOVE. Both generals and headquarters provide live local information. An escort follows only while it can locally see its general. To move couriers forward, select them, open **Details**, choose **Courier at rendezvous**, press **Assign on map**, and click the rendezvous. Keep some couriers at headquarters for remote dispatches.

**3:00–4:00 — Make your opponent interpret a choice.** Show a line formation near the bridge while sending light infantry toward the longer ford. A MOVE can finish in HOLD or OBSERVE; ENGAGE fights only locally observed opponents. WITHDRAW uses an explicit fallback. An OBSERVE formation reports through nearby command equipment or an available courier; it has no telepathy.

**4:00–5:00 — Maintain the position.** Food is physical. Carriers shuttle headquarters food to their assigned rendezvous; troops and outpost staff must meet them. Reserve a carrier for a forward post. To construct, select available workers at headquarters, select a hill site, then Build. They haul materials, build locally, and the first two become staff. To upgrade an existing outpost, select available HQ workers, select the post, and Install relay upgrade. Losing a general costs position and knowledge; select an available HQ resident and Train replacement general to recover.

## Controls

| Action | Control |
| --- | --- |
| Select existing people | Click or drag; Shift adds; double-click a troop for its formation |
| Quick movement | Right-click; MOVE also directly moves generals and sets support rendezvous |
| Zoom / reset | Scroll over the field / Fit map |
| Choose command source | Auto by default, or choose a specific general, HQ, or reported outpost |
| Recall initial formations | Alt+1 line, Alt+2 ranged, Alt+3 light |
| Recall support groups | Alt+4 couriers, Alt+5 carriers |
| Assign a group | Ctrl+Alt+1–5, or Ctrl-click a group button |
| Activate a private preset | 1–5, or its visible button |
| Default slots | 1 MOVE, 2 HOLD, 3 ENGAGE, 4 OBSERVE, 5 WITHDRAW |
| Route | MOVE, then Shift-click up to eight waypoints, then click destination |
| Target area | HOLD/OBSERVE/ENGAGE, then Ctrl-drag from centre to radius |
| Cancel targeting / close desk / clear selection | Escape (in that order), or the visible Cancel / close buttons |
| Customize slots / command keys | Help or Command desk → Orders → Customize commands / keys |
| Set a fallback | Preset editor → Set fallback on map |
| Move a general | Select general → Move selected general → click |
| Reassign support duty | Select people → Details → standing duty → Assign on map → click |
| Select infrastructure | Click its square; selected workers remain selected |
| End / repeat | Command desk → Reports → Surrender; then Request rematch, or Play again vs bot |
| Download full replay | Available after the result only |

Changing a preset affects future dispatches only. Choose either a follow-up or a fallback, not both. MOVE can end in HOLD or OBSERVE. An optional fallback triggers below 60% of strength at acceptance using locally confirmed casualties. Fallback defaults to headquarters until a custom point is selected. Command and troop-group single-character keys are editable; group recall adds Alt, and assignment adds Ctrl+Alt.

Scouts, couriers, and carriers accept MOVE as a duty-rendezvous assignment. Other support duties take priority over infantry orders: use the duty controls to route carriers/couriers or staff an outpost. Reassignment changes the existing person's role and work; it creates nobody. Available roles are carrier, courier, scout, staff, escort, and guard. Original infantry specialisms are not regenerated by reassignment.

## Rules and scenario

`generalship/scenarios/g0-v4.json` contains the versioned balance settings and local steering values; `Config` loads that committed file. Every replay includes the full configuration. Map topology, fixed roster layout, and bounded input limits are code-versioned. G0 currently uses one fixed mirrored map; the recorded seed is reserved for future deterministic scenario variants and does not randomize this map.

- 900 × 600 metre map, 10 metre navigation grid, continuous integer centimetre positions. Roads speed movement; woods slow movement and block long sightlines. Both bridge directions share a four-person admission queue. The ford is slower and further away.
- Each side has 48 identifiable people: two generals, three ten-person infantry formations, four carriers, four couriers, two scouts, two post staff, two escorts, and two HQ guards. Scouts start forward; infantry and support begin around HQ; post staff start at their post.
- Food: 600 at HQ plus two units on each person. One meal per person every two minutes. Capacity twelve units. Missing meals gradually reduce movement and damage; there is no accelerated starvation death policy or invisible replenishment. Staff stop operating after three missed meals. Bodies retain cargo.
- Materials: 80 at HQ. Outpost: twenty delivered units and sixty worker-seconds, then two actual staff. Relay upgrade: ten delivered units and forty-five worker-seconds.
- Signals: 250 metre base range, one second per link; upgraded 375 metres and 300 ms. Each link uses the weaker endpoint, requires clear terrain sight, and needs staffed posts. HQ/general equipment supports upgraded links. Signals improve transmission, not sight.
- Initial observation radii: ordinary person 110 metres, general/HQ 150 metres, outpost 190 metres. Local exchange radius thirty metres. Courier arrival radius eight metres. Missed recipients cause a fifteen-second wait, then a physical return with failure. Unseen courier death does not produce a notification.
- Local combat uses health, cooldowns, weapon range, line protection, woodland cover, and simultaneous damage. Current coarse ranges are 22 metres melee and 75 metres ranged; these are explicit experimental balance choices, not realistic weapon modelling. No general aura exists.
- A replacement general costs one available HQ person, ten food, and sixty seconds. Two generals active or training maximum.
- Capture requires five combatants within thirty-five metres of enemy HQ for thirty uninterrupted seconds, with no defending combatant present. Any interruption resets capture. Simultaneous capture draws. General loss alone does not end play. Surrender loses; unresolved play draws at twenty minutes.

## Architecture and information boundary

`model.rs` defines integer quantities, configuration, plans, inputs, reports, personnel, missions, and replay records. `terrain.rs` owns deterministic pathfinding and line of sight. `sim.rs` owns the sole authoritative `Game`, accepted inputs, accounting, and tick sequence. `policies.rs` requests movement and implements duties and logistics; `movement.rs` resolves physical space and travel for every living person; `communications.rs` delivers physical orders and reports; `observation.rs` filters knowledge; `combat.rs` resolves damage and outcomes. `bot.rs` contains the deterministic practice policy, isolated behind the serialized player-view boundary. `server.rs` owns Axum HTTP/WebSocket transport, seats, timing, reconnect, and room lifecycle. `web/` renders received views and submits intentions; it runs no simulation.

All gameplay advances on one 100 ms clock. Pathfinding uses deterministic ties. Damage is accumulated before applying casualties. Every accepted input records its execution tick; replays store scenario, seed, ordered inputs, a SHA-256 state checkpoint every ten seconds, and the final state hash. Network sequence numbers reject duplicates; rematch generations reject delayed commands from previous games. In-world order sequence numbers prevent an older delivered order replacing a newer one.

`Game::view` is the only snapshot path. It includes common terrain, public match status, the player's observations and dated memories, private dispatched plans, acknowledgements, and HQ stock. It excludes enemy intentions, unobserved movement, distant inventory changes, mission internals, credentials, and omniscient state hashes. Source and receipt times travel with observations. Preallocated site identities avoid revealing hidden construction counts. A construction blueprint is not visible until physical materials or work exist.

A client's camera and selection have no authority. A seat is derived from its random reconnect token; input cannot specify another player. Credentials are sent in the initial secure WebSocket message, not URL query strings. Invitations are single use. Snapshot responses contain only that seat's view. A replay contains both sides' state history and therefore is available only after the match.

## Verification commands

```bash
cargo test --locked
cargo test --locked --release --manifest-path generalship/Cargo.toml
node --check generalship/web/app.js
node --check generalship/web/field.mjs
node --test generalship/tests/controls.mjs generalship/tests/motion.mjs generalship/tests/field.mjs
cargo test --locked --release --manifest-path generalship/Cargo.toml --test network twenty_minute_two_socket_clock_soak -- --ignored --nocapture
cargo run --locked --release --manifest-path generalship/Cargo.toml -- --replay /absolute/path/to/downloaded-replay.json
```

The socket tests bind to localhost. In a restricted execution sandbox they require permission to bind local ports. The soak deliberately takes twenty real minutes and is ignored by the normal test command. Tests cover personnel and stock conservation, casualties and dropped cargo, replay tampering, secret-state isolation, delivery ordering, missing recipients, hidden courier loss, staffing/relay disconnection, upgrades, construction, replacement, capture, clock duration, invalid credentials, simulated application RTT, reconnects, and rematches. See the results file for what has actually run.

## Render deployment

`render.yaml` describes a single free Rust service in Frankfurt. Build from `codex/generalship-g0` with:

```text
cargo build --locked --release --manifest-path generalship/Cargo.toml --bin generalship-server
```

Start `generalship/target/release/generalship-server`. Set `G0_BIND=0.0.0.0`, `G0_HOST_CODE` to a private generated secret, `G0_DATA_DIR=/tmp/generalship-replays`, and `RUST_VERSION=1.98.0`. Render supplies `PORT`. Health path: `/health`. Use one service instance: memory is authoritative and is not shared between instances. HTTPS and WSS use the same public port. An optional Dockerfile is included for other environments.

The service allows four private rooms, with one-hour retention for disconnected inactive rooms, no public lobby, no accounts, no chat, no spectators, and no matchmaking. Download finished replays before the process restarts. Free-service filesystem contents and in-memory matches are ephemeral. Free idle spin-down can delay the first page load; service interruption produces an aborted match, never a fabricated victory. Do not deploy changes during a human match.

Provider references: [WebSockets](https://render.com/docs/websocket), [free-service limits](https://render.com/docs/free), [Axum WebSockets](https://docs.rs/axum/latest/axum/extract/ws/index.html).

## Deliberate simplifications and remaining acceptance

This is a primitive tactical-map client with dots, terrain hatching, and information ages. It has no illustrated units, 3D camera, production economy, farming, settlement autonomy, ranked play, persistent campaign, or technology tree beyond one relay upgrade. Living people have a shared three-metre-radius movement footprint. This is deterministic crowd navigation and space reservation, without rigid-body forces, pushing, or mass-based formation physics. Congestion can delay travel; this is not a globally optimal traffic solver. Casualties leave cargo in place but do not block movement. Dated observation markers and larger general symbols are still abstractions and can visually overlap even though living people occupy separate physical positions. Fixed construction sites replace arbitrary building placement. Destroyed-post and body cargo remains conserved in place; scavenging is not implemented. There is no loading a live match from a replay or recovering a match across server restart.

Orders normally target their recipients' dispatch-time reported centroid. Use **Set next delivery rendezvous** to specify a fixed alternative for one order. These checks do not establish fun or production readiness.

Two human players must still test a visible threat, a concealed flank, relay disruption, and general loss/replacement. After each match, record what each thought the other was doing, the evidence used, one misread they can explain, whether a feint worked, and whether repeated corrective clicking felt necessary. Save the replay and do not call automated success a positive fun test. Use those observations to decide the next balance and interface iteration.
