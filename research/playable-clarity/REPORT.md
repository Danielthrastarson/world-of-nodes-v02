# P4.1 — interface clarity

**Implementation: `243d07b40deddfddf94c95842086412e0cbc7a6d`.**
Baseline: clean `0f8c18bc9b16f47dbf75276f577e20f850518f36`.
The bounded clarity pass is complete at native technical/visual-review scope.
P4's first-time human comprehension and sustained play remain unvalidated. Work
stops here; no simulation milestone or economic institution is added.

## What changed and why

| Before | P4.1 |
|---|---|
| Similar weight for grid, routes, labels and selected objects | Quieter background, gold selection geometry, distinct settlement/crossing symbols, friendly unit badges, and muted secondary labels. |
| Selection and inspector were weakly connected | The same gold accent marks the selected map object and panel title; selected units highlight their observed members. |
| The command tree had to be remembered | Contextual actions appear in the selected-object panel and persistent Q/W/E/R strip, with disabled reasons. Hearth's crossing commission can be composed entirely with mouse clicks. |
| No concise indication of what deserves attention | Up to four Current matters cards show only known requests, projects, personally issued office orders, reports, recognized hostiles or the ruler's own survival pressure. |
| Raw descriptive inspection came first | Status and the relevant decision come first. Unit inspection separates observed presence, the ruler's last issued instruction, observable condition and local command reach. |
| Age and resource numbers competed with the map | Solid/hollow and bright/faded marks carry the first age distinction. Named goods quantities appear beside their symbols when relevant; exact evidence times remain in inspection. |
| A flat status string could be mistaken for success | Orders show Issued / Handoff / Received / Obligation / Result, retaining hollow unknown stages. Physical construction is inspected separately. |
| Debug was easier to confuse with ordinary play | A red DEBUG / ALL WORLD TRUTH banner contrasts with PLAY / RULER KNOWLEDGE. Action composition still reads a separate Play snapshot. |

Q = Observe, W = Order, E = Govern, R = Allocate. Meanings do not change with
context. 1–4 select the visible subactions. E/R remain unavailable with reasons;
no new authority is implied. Enter confirms, RMB cancels/context-orders, Space
pauses, wheel zooms, and arrows/middle/right drag pan. The scene starts paused.

The scene draws existing Persons, public goods, cultivation units, infrastructure
and institutional locations. The settlement symbol denotes an institutional
location, not a newly created house. Unit layout remains graph-based display
packing rather than a physical continuous formation.

## Boundary fixes and deliberate limits

The review found a **UI-only target composition defect**: a debug-only crossing
could enter confirmation even though the authoritative Session then rejected it.
Crossing and node target composition now require presence in the supplied Play
snapshot. Tests preserve this boundary. No kernel fix was needed.

A critical hunger/fatigue warning is not a new command prohibition: legal orders
still enter the existing command chain, and Persons arbitrate physical execution.
Unavailable service, locality and knowledge are surfaced without restoring roles.
A remembered unit target has no separate receipt date in the existing DTO; the
panel explicitly says that date is unavailable. It does not turn that target into
a current global unit order. The UI's own last-issued record has its actual issue
time and number of local recipients.

There is still no transported private-status acknowledgment. A courier handoff
therefore remains **receipt unknown** even if DEBUG knows receipt occurred. A
completed office obligation is a local commission, not a constructed crossing.
The physical crossing's separately dated state can show it is open without
inventing an acknowledgment or attributing all causation from the map alone.

## Verification

- **369 headless tests pass**: 356 library plus 13 integration tests.
- **48 preserved playable scenarios and 48 exact dated-input replays match**;
  193 artifact comparisons exclude only the two registered wall-time columns.
- **600 F13B runs match exactly**, including all 3,000 native arm artifacts and
  the aggregate results CSV.
- **15 distinct client tests pass**, including 10 presentation tests, both target
  knowledge boundaries and the existing camera/clock checks. The render-probe
  binary repeats 12 shared tests; these are not extra distinct mechanisms.
- Client formatting and Clippy with warnings denied pass.
- Actual X11 mouse/keyboard input tests pass for mouse-only commission/confirmation,
  Hold, contextual Move, Attack and Withdraw. Help blocks commands; unavailable
  Govern creates no input; pause/camera/debug preserve authoritative hashes.
- **24 recorded client/headless parity observations match**. These include copies
  of final evidence files, not 24 independent experiments. Paired Remote Play and
  Debug captures have identical state and event hashes.
- All **113 root source/dependency files** and **21,439 previously existing research
  files** remain byte-identical. No physical parameter, policy, institution,
  information transport, queue rule or research result changed.

The existing **60/1,440 strict F11C differences from the prior contact-clock fix**
remain historical failures, not new P4.1 failures and not relabeled PASS. See the
[regression boundary report](validation/REPORT.md) for exact scope and retention.

## Actual-window observations and screenshots

The final captures were read visually at 1440×900. They establish layout and
input operation, not first-time human understanding or enjoyment.

- [Preserved before](../playable/client/commission-release/initial.png)
- [After: settlement, known matters, available actions](final/commission/initial.png)
- [Crossing target and explicit confirmation](final/commission/confirmation.png)
- [Received order; decision and physical outcome still unknown](final/commission/orders.png)
- [Selected unit, local recipients, condition and actions](final/military/unit-selected.png)
- [Stale report in normal play at 16h](final/remote-play.png)
- [Same physical state in explicit debug view](final/remote-debug.png)
- [Physically built crossing at 24h, with dated local knowledge](final/local-built.png)

The final mouse-only commission uses the visible inspector action, clicks the
crossing, then clicks Confirm. No direct World mutation or per-Person instruction
is substituted. Both typed command transcripts replay independently.

## Performance and tradeoffs

The final paused render probe draws **1,000 actual Persons / 1,000 visible records**
for 240 frames without changing state or event hashes. CPU draw mean is **0.940ms**,
maximum **76.927ms**, with **124.3fps including capture** in Xvfb. The maximum is
preserved; this is not a worst-case latency guarantee or an integrated kingdom
scaling claim. Snapshot capture took 5.266ms in that dense initial view.
The earlier concurrent development probe is retained separately. Frame startup,
font/driver behavior and screenshot work make short client FPS samples unsuitable
for comparing gameplay speed. No simulation step is dropped to improve rendering.

Deliberately accepted remaining limits:

- This is still a schematic graph. Dense or widely separated unit badges can cover
  nearby symbols. Pan, zoom and selection help; general label collision layout is
  not implemented.
- Compact cards and badges may ellipsize details. Full descriptions, exact dates
  and disabled reasons remain in inspection; long panels require scrolling.
- A remembered crossing whose edge geometry is absent uses a node-associated
  schematic marker. That display fallback can shift its icon; it is not physical
  movement or a claim of updated geography.
- Military receipt remains distinct from continued service or successful movement.
  Ruler mortality and ordinary service expiry can leave actions unavailable.
- No new remote ACK, hover encyclopedia, remappable bindings, full accessibility
  pass or broad screen-size playtest is claimed. 1440×900 is the reviewed size.

These are usability/research debts, not reasons to add a new simulation system.

## Reproduction and provenance

```sh
cargo run --release --manifest-path client/Cargo.toml
cargo test --manifest-path client/Cargo.toml --offline
cargo clippy --manifest-path client/Cargo.toml --all-targets --offline -- -D warnings
xvfb-run -a -s '-screen 0 1440x900x24' python3 scripts/playable_clarity_smoke.py /tmp/p41-new-commission commission
xvfb-run -a -s '-screen 0 1440x900x24' python3 scripts/playable_clarity_smoke.py /tmp/p41-new-military military
```

This host used `CARGO_HOME=/tmp/world-of-nodes-cargo` for its already cached client
dependencies. `final/summary.json` records hash parity; each capture transcript
records the compiled executable fingerprint. `final/render/1000-persons.txt`
contains raw render measurements. `validation/integrity-243d07b.json` rechecks the
committed implementation against the original root/research manifests.

`screenshots/`, `window/` and `development-2/` preserve intermediate layout/input
passes. They are superseded UI evidence, not additional independent experiments.
Previous `research/playable/` artifacts were never overwritten.
