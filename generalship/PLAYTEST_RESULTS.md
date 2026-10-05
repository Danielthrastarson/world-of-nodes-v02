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
