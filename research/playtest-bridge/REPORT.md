# Opt-in playtest logging bridge

Requested during the P4.2 human-playtest freeze. Base: `d3958d5`; accepted gameplay
implementation remains `243d07b`. This instrumentation enables evidence collection;
it does not pass the human-playtest gate or redesign the interface. The commit
containing this report identifies the implementation. Executable fingerprints and
validation commands are in `provenance.json`.

## Result

The recorded client logs delivered game-window inputs, disabled/widget attempts,
selection/composition changes, pause/speed, sampled camera state, accepted/rejected
commands and reasons, visible reports/order stages, current ruler knowledge,
periodic hashes and final state. Native close finalizes without requiring F12.
The local assistant bridge can read evidence or request a snapshot, screenshot
or checkpoint. It cannot issue game commands. No external server, dependency,
policy change, hidden knowledge exposure or new simulation system was added.

Normal launch remains opt-out by default. The recorded launcher creates a fresh
session with provenance and debug off. Previously open frozen sessions stay open;
their missing input histories cannot be reconstructed retroactively.

## Validation

- Full root/headless suite passed; authoritative `src/`, root/client Cargo files
  and lockfiles are byte-identical to accepted base.
- Client: 24 main tests and 5 replay tests passed; render-probe repeats 12 shared
  presentation tests. Clippy all targets with warnings denied passed.
- Native XTest on isolated display: identical state/event hashes with journal
  off/on after rejected Withdraw plus accepted Hold. Disabled inputs, selection,
  speed, camera, help and debug were exercised. Six invalid diagnostic requests
  were rejected. Snapshot requested while Debug was visible still exported Play
  knowledge. Live replay reported a verified prefix; normal native window close
  produced a fully verified replay without F12.
- Four-hour commissioned Local comparison: off/on state `56580759d1a77ea6`, event
  `4085c1f659f2670c`. Remote: state `f88b3c9fc98eea09`, event `8b0dcdf416979400`.
  Both final journals replayed completely, including their actual startup command
  timestamps (0 Local; 900000 Remote).
- Missing `--journal` argument fails explicitly. JSON escaping, fresh-directory
  protection, bounded requests, interrupted prefixes and I/O finalization failures
  have focused tests.

Raw controlled results, logs and screenshot are in `native-smoke/` and
`startup-pairs/`. They are automated checks, **not human playtest evidence**.
Initial harness execution found an X11 window-mapping race before input; the test
now waits for viewability before focusing. No game behavior was changed to make
that test pass. Review also found screenshot-export panic and premature clean
status paths; both were corrected before final validation.

## Limits

No claim of zero wall-clock overhead or perfect mouse/video replay. Journal
writes, periodic snapshots and requested screenshots cost frame time. The existing
clock preserves backlog. Canonical replay validates simulation state/event hashes
at recorded command times; UI state is evidence for diagnosis, not deterministic
pixel replay. Camera is sampled at most four times per second, idle knowledge
publication twice per second. Input ordering within a frame has Macroquad's frame
resolution. Kernel source and dependency files remain unchanged.

A clean exit syncs logs; crashes, forced kill and power loss can leave an incomplete
prefix. A stale current.json alone cannot establish process liveness. No previous
human results, controls, scenario, physical parameters or accepted research were
rewritten. Next work remains observing the owner's session and classifying their
reported problems, rather than adding gameplay or interface features.

See `docs/PLAYTEST_BRIDGE.md` for launch, inspection and replay commands.
