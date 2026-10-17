# G0 verification record

Date: 2026-10-05. Branch: `codex/generalship-g0`. Baseline: `8072a4152f1557c4fef24f74b260c003438aec7d`.

## Internet playtest

Temporary URL: **https://ben-cutting-explorer-son.trycloudflare.com**. The actual Rust server runs on the user's computer through Cloudflare Quick Tunnel. It is usable over the public Internet while both processes and this computer remain running. This is not permanent cloud hosting.

Render creation failed with `repository URL is invalid or unfetchable` for the private repository. The user requested an alternative. No Render service or paid hosting was created.

## Completed

- Original research crate: 66 tests passed; original tracked files preserved.
- Simulation: **37 tests passed**. Deterministic replay, resource/personnel conservation, hidden-state filtering, physical orders, missing couriers, relay breaks, simultaneous damage, bridge capacity, initial scouting within one minute, construction, replacement, and capture covered by automated checks.
- Multiplayer integration: **passed on the final modular server**. One-use invitations, isolated seats, invalid credentials, ownership, duplicate inputs, simulated 150 ms application round-trip delay, brief disconnect/reconnect, surrender, replay access, rematch, and stale process identity passed.
- Accelerated twenty-minute simulation completes with no stock/personnel accounting error.
- JavaScript syntax and Rust Clippy checks passed without warnings.
- A real twenty-minute two-WebSocket clock soak passed: 12,000 simulation ticks in 1,200.100 seconds, 12,001 snapshots received by each seat, no reported slow simulation ticks. This was the initial networking milestone; subsequent rule/interface/module changes passed the final simulation and integration suites. The explicit soak command is available for subsequent builds.
- Two browser tabs verified invitation, readiness, group selection, dispatch/acknowledgement, general movement, combat, general death, stale relay information, custom command/group keys, and reconnect. The final interface also verified individual personnel selection, two HQ guards becoming builders and then staff after construction, surrender, and rematch to fresh forces. No browser warnings/errors were recorded in the final walkthrough.
- Actual public HTTPS/WSS test passed with two independent protocol clients: host-code rejection, no repository-file route, one-use invite, seat isolation, command, reconnect, surrender, rematch, replay. Observed order acknowledgement RTT: 21 ms on this route. Largest snapshot: 27,368 bytes. These are a single test's observations, not global latency guarantees.
- Public-test replay reproduced tick 19 and SHA-256 `c002637f042b03c45a7e9a3f400fb3e160d55a0eb80c96dc59ce93ad2011acfe` with the release replay verifier.
- Public page opened successfully in the browser.

## Remaining

- Permanent cloud deployment, if required instead of the temporary computer-hosted link.
- Real two-human playtests and balance/readability evaluation. The browser sessions above were operated by the implementation agent.
- A real shaped-network packet-loss test has not been performed. The local test adds 75 ms outbound and 75 ms inbound application delay and separately closes/reconnects sockets; the public test uses the actual network path.

## Human playtest record

No two-human fun test has occurred. No claims about balance, readability under pressure, or enjoyment have been validated.

| Scenario | Evidence to record | Result |
| --- | --- | --- |
| Visible threat at bridge | What response did it provoke? | Pending humans |
| Concealed ford flank | When was it suspected and detected? | Pending humans |
| Relay disruption | Was stale information recognizable? | Pending humans |
| General loss and replacement | Was recovery costly but understandable? | Pending humans |
| Feint / misread | Could players explain their inference afterward? | Pending humans |
| Command friction | How often did players correct the same order? | Pending humans |

## Solo bot extension — 2026-10-05

Added private dev practice with a bot in the East seat and one-player rematches. The bot receives only `Game::view(1)` and uses the same validated, recorded inputs and physical communications as a human. No economy, combat, observation, or movement balance was changed.

- 37 existing simulation tests, two new bot tests, and three network tests pass (42 total). The optional real-time twenty-minute soak was not repeated for this extension.
- The bot test runs up to ten simulated minutes and verifies an actual bridge crossing, a ford flank, damage to a passive player, resource conservation, and identical replay hashes.
- Changing hidden enemy health, positions, or plans leaves the bot input unchanged.
- Network tests verify bot readiness, real orders, reconnect, solo rematch, protection of the bot seat, invitation invalidation, and refusal to replace a reserved human seat.
- Clippy with warnings denied and JavaScript syntax validation pass.
- The opponent is a simple practice policy; human difficulty and fun remain untested.

Final bot build URL: **https://campaign-part-mlb-kelkoo.trycloudflare.com** (replaces the earlier temporary links). The browser verified creating a bot room, starting its clock, surrender, one-player rematch, and reconnect to the solo lobby. A fresh bot room is left at 00:00 for the user. The HTTPS/WSS two-human smoke test also passed on this extension before the final CSS-only change: 80 ms measured order RTT, 27,469-byte maximum sampled snapshot.

## Gameplay feel pass — 2026-10-17

This was an agent-operated browser playtest, not a human fun/balance study. Separate bot rounds were used for before/after inspection.

Observed before changes:

- Selecting a general and using the ordinary MOVE slot created a courier order eventually acknowledged by zero recipients. The general stayed still.
- The user-visible order history contained repeated undelivered single-person moves. Formation selection was difficult, selection labels overlapped, and delivery status was below the visible sidebar.
- Groups converged on a single destination point. Enemy formations could appear to be only a few dots.
- A forward general's death only removed its selection button. Dead selected people remained in the selection summary.
- All four unassigned supply carriers marched toward the front and were reported killed during the revised opening.

Changes:

- Regular MOVE and right-click now route generals to direct movement, infantry to bounded orders, and scouts/couriers/carriers to physical duty assignments. Unsupported orders are explained, and the server rejects silent no-op orders using reported duties.
- Named formation buttons sit above the field; double-click selects a formation. Auto source chooses from authorized observations.
- Pending/partial/failed delivery is visible beneath the battlefield; remembered positions are explicitly dated. Map labels have a readable minimum size, troop arrival positions are spaced, overlapping enemy contacts show counts, observed damage has a brief flash, and observed health has bars.
- Zoom and Fit map work without changing observation permissions. A received general loss creates a clear notice. Confirmed casualties are removed from the active selection.
- Carriers now remain at HQ until an actual route assignment arrives. Food and combat numbers, signal rules, and clock rate were not changed. Scenario version is g0.2.

Verification: 40 simulation tests, two bot tests, three network tests, and six client-control tests (51 total). Added regression coverage for separate formation arrival positions with replay equality, support/general no-op rejection, carrier standby and physical assignment, direct general movement routing, support duties, water rejection, ownership, auto source, and partial delivery feedback. The twenty-minute wall-clock soak was not repeated.

The browser walkthrough verified starting a bot round, regular MOVE on a general, right-click line/ranged moves, double-click selection of ten light infantry, zoom, Fit map, and delivered-to-10/10 feedback. A coordinated line/ranged opening showed separate formation positions and fresh enemy reports.

Published G0.2 at **https://evaluated-weighted-lottery-specialists.trycloudflare.com**, replacing previous temporary links. The public HTTPS/WSS check passed seat isolation, one-use invitations, reconnect, surrender, and rematch: 102 ms measured order RTT and 27,435-byte maximum sampled snapshot. Its downloaded replay verified at tick 22 with hash `a79dd6499e6f4e0c8e63f022e5457d979a15fde68293b49732eacc4cc43fda81`. The public browser loaded the revised controls and created a fresh bot lobby at 00:00, with 600 HQ food and all four supply carriers. Start practice is ready for the user. The computer and supervisor must remain running for this temporary link to work.
