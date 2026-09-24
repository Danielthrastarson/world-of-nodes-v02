# Local playtest bridge

The owner requested recording and later diagnosis during the P4.2 freeze. This
is an opt-in client instrumentation change over accepted P4.1; it does not change
the kernel, scenario, autonomous policy, command authority or controls. It is a
local filesystem bridge, not a Replit service or a network endpoint.

## Start

Build the optional client and replay tool, then launch a fresh session:

```sh
cargo build --release --manifest-path client/Cargo.toml --bins
python3 scripts/start_recorded_playtest.py
```

On this workstation the dependency cache is `/tmp/world-of-nodes-cargo`; prefix
Cargo commands with `CARGO_HOME=/tmp/world-of-nodes-cargo` and use `--offline`.
A graphical desktop/display is required. The launcher prints the session and
journal paths. The window is titled **World of Nodes · Recorded playtest**.
It starts paused, with debug truth off. Existing windows and sessions are left
intact. The previously running frozen client cannot acquire this instrumentation
in place; earlier unlogged inputs cannot be recovered.

The launcher preserves seed, scenario, executable and source SHA-256 fingerprints,
base revision, PID and process output. `--seed N` and `--remote` are available for
explicit research use; default play conditions remain Local/0. A directory is
always fresh. No earlier journal is replaced.

## Recorded evidence

`journal/events.jsonl` is append-only. It records sequence, wall elapsed time,
simulation time and frame, followed by delivered game-window key/button events,
clicked target/context, widget attempts (including disabled/help-blocked), UI
state changes, selection, command composition, pause/speed, sampled camera state,
accepted/rejected typed commands and reasons, known reports/order stages, periodic
ruler-visible snapshots, checkpoints and normal close. `process.log` captures
client errors and panics. This is not screen video, raw text entry, clipboard,
system-wide keyboard capture or desktop recording.

`current.json` is atomically replaced at most twice per second while idle and
on relevant UI/command changes. It contains ruler-visible information even when
the user enters Debug. `view_mode` records that deliberate debug exposure;
`knowledge_mode` remains Play. Exact DTO text supplements structured report,
action and order summaries. Camera changes are sampled at up to four per second.
Input logs describe delivered input and before/after UI context, not inferred
player intent. Multiple inputs within one render frame retain Macroquad's own
frame-level granularity; this is not an OS event video or an exact mouse replay.

`replay.tsv` preserves typed command order, simulation timestamps and state/event
hash checkpoints. It includes rejected commands. The independent replay checks
simulation outcomes, not pixel-identical rendering or wall-clock input timing.
Final physical state is reconstructible from seed + commands + final time; it
is not a save/load feature. A clean close records final visible state, hashes and
an explicit `E` marker. A crash/forced kill remains an incomplete prefix. Normal
writes use OS file writes without per-event fsync; sudden power loss can lose a
tail. Final normal close syncs the journal. Do not infer a clean ending just from
process disappearance or an old `current.json` file.

## Diagnose

Replace SESSION below with the printed **journal** directory:

```sh
python3 scripts/playtest_bridge.py status SESSION
python3 scripts/playtest_bridge.py summary SESSION
python3 scripts/playtest_bridge.py tail SESSION --lines 20
client/target/release/playtest_replay SESSION
```

Replay exits 0 for a complete verified session, 2 for a verified live/unclean
prefix, 1 for malformed or mismatched evidence. Python summaries do not claim
hash verification. Status snapshots may become stale after a crash; check event
times, process output and replay end state together.

While the client is open, the assistant can request a bounded diagnostic:

```sh
python3 scripts/playtest_bridge.py request SESSION snapshot
python3 scripts/playtest_bridge.py request SESSION screenshot
python3 scripts/playtest_bridge.py request SESSION checkpoint
```

Requests are local files with a validated unique ID and one of these three
operations. They cannot pause, issue orders, press keys, evaluate code, expose
hidden state through a Play snapshot or otherwise control the simulation.
Screenshots capture only the actual game framebuffer, including Debug if the
player has already selected it. They are explicit requests, not continuous video.
The response event identifies the output path. Screenshot/readback can cause a
brief frame hitch; its duration is recorded. No external service receives data.

The client writes small synchronous local files; diagnostics add some wall-clock
cost. The existing fixed-step accumulator retains simulation backlog rather than
dropping time. Do not claim zero performance overhead. Logging failure is shown
in the game and process log; play can continue, but evidence is incomplete.

## Frozen evidence and human interpretation

Accepted source/evidence remain at `243d07b` / `d3958d5`; the pinned frozen
executable and session-001/session-002 files are preserved. Recording is an
explicitly authorized exception to the client freeze, not another clarity pass.
No control tutorial or scenario optimization is injected into the recorded game.
Human observations remain separate from automated evidence. A command rejection
is not proof of misunderstanding; ask what the player intended. Requesting help
already made session-002 an assisted discovery session. Recording starts a fresh
session rather than retroactively making that session fully observable.
