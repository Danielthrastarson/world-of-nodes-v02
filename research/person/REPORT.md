# B0 — common person foundation

**PASSED at behavior-preserving architectural scope.** B0 introduces a common person
interface and capability-based decision path without changing the tested economic
or institutional histories. F11 remains blocked behind B1, B2 and B3.

[Registration](../../docs/PERSON_FOUNDATION.md) ·
[architecture](../../docs/PERSON_ARCHITECTURE.md) ·
[complete replay validation](replay/validation.json) ·
[command/event coverage](coverage.json) · [prior artifact hashes](prior-artifacts.json).

Baseline `ff6fbb1`; registration `bcc57b6`; core refactor `2c640d2`;
replay source **`c35e729`**. Architecture documentation followed in `72ea4c3`.

## What changed

Inspection found that the previous activities already shared physiology, survival
checks and the physical executor. The fragmentation was in runtime capability
dispatch, domain-specific entry functions and barter arbitration embedded in World.
B0 refactors those boundaries; it does not replace the person with a new agent model.

1. `Person` is the existing embodied `Agent` record, not a second entity. Stable IDs,
   storage and historical debug/hash strings remain compatible. One person can farm,
   carry, build, exchange and hold an office over its lifetime.
2. `person::Context` passes local observation, personal timestamped beliefs and
   optional cultivation/capital/public-work capability components to common selection.
   The runtime no longer chooses a separate farming, capital or infrastructure entry
   function. Domain evaluators remain opportunity evaluators, not separate brains.
3. `person::Needs` derives existing hunger, starvation and fatigue from the body.
   Its existing threshold predicates participate in the actual common policy.
   No need score, physiological rate, priority or random draw was intentionally changed.
4. `ActionPlan` makes the selected next command, reason, person, time, existing route
   goal and accepted undertaking inspectable. The command still goes through the
   original kernel. An accepted `Job` remains distinct from atomic action and board lease.
5. `person_state` exposes personal roles, rights, current action and bounded memory
   components without refreshing them. Roles represent actual sovereign/steward
   identities, not permanent worker classes. Crop/cart/goods/office records remain
   in their authoritative ledgers; possession never becomes equivalent to ownership.
6. Barter selection is now a pure opportunity stage using local goods/offers and
   locally inspectable escrow. It returns a chosen command plus observer notes.
   Counter deduplication, log emission and omniscient research-only coincidence
   classification happen outside the choice, in their original order.

No utility optimizer, BDI framework, GOAP search, neural model, new personality stats
or new institutional power was introduced. The useful distinctions are beliefs vs
world truth, longer undertakings vs atomic commands, conditional opportunities vs
guaranteed outcomes, and roles vs physical persons.

## What this deliberately does not claim

The current reactive policy still plans one selected step at a time while retaining
its existing accepted-job undertaking. B0 does not claim every occupation now has a
persistent multi-step planner. Existing conditional candidate forecast tapes remain
conditional, and commitment diagnostics remain research-only. Domain goal selection
and next-action selection may still occur together in the bounded evaluators.

F10's appointments and initial commissions remain controlled institutional commands.
They use the same person executor and law checks; B0 does not invent autonomous
steward recruitment, residence, replacement or political planning.

Relations, kinship, affect, age/fertility and reproduction are not implemented here.
An unimplemented relationship system is not encoded as zero trust or an empty
emotion meter. B1/B2/B3 require their own causal models and experimental assumptions.

## Evidence and gate

- **192 tests pass**, including nine new person-foundation tests. Clippy across all
  targets with warnings denied and formatting checks pass.
- Pure-input tests show the same command and RNG evolution after hidden-world state
  changes while captured personal inputs remain fixed.
- Assigning a steward role leaves the person, needs and selected behavior unchanged.
  The person remains mortal; death removes observation/decisions and practical office use.
- Food/survival retains priority over economic capability opportunities.
- An existing undertaking survives an atomic sleep interruption and disappears only
  through existing job cancellation semantics; no new commitment rule.
- Read-only person inspection leaves subsequent state and event hashes unchanged.
- Typed commands still cannot perform an institutional action during atomic travel
  or obtain rights/resources that kernel validation refuses.

**587 regression run executions match their preserved results exactly:**

| Panel | Runs | Coverage |
|---|---:|---|
| Full F10 | 187 | Seven controlled deployments plus 180 paired autonomous project runs |
| Historical panel | 40 | 19 P1 scenarios, 14 direct/market examples, four F6 and three F7 cases |
| Full F9A/F9B comparison | 360 | Six worlds × direct/market × 30 paired seeds |

Some historical-panel seeds overlap the full matrices. These are regression
executions, not 587 independent experiments or new evidence of economic emergence.
The full exchange panel includes all seeds0–29, including the rare low-cost seed18
posting. F10's main physical outcomes remain seed-invariant as previously reported.

Comparisons include physical metrics, state/event hashes, per-person records,
statistics, and complete recorded trace files—not just survivor totals. The
person refactor changes **zero covered physical outcomes, state hashes or event
hashes**. No expected result was rewritten to accommodate a difference.
The old F9 parameter text differs only by `authority: None`, introduced by F10,
which the validator explicitly normalizes; no outcome, trace or hash is normalized.

All **3,638 prior research files remain byte-identical**. No conservation or FIFO
failure appears. Original study outputs are preserved and new replay outputs are
separate. `coverage.json` inventories selected command and physical event families;
its pooled trace counts include overlapping panels and are not behavioral rate estimates.

## Observer failure caught and repaired

The first coverage reader assumed every `events.tsv.gz` file had a TSV header.
Older probes actually store Rust `Event { ... }` debug records under that filename.
It consequently omitted farming, capital and exchange families from its counts.
`coverage-initial.json` preserves that incorrect auxiliary output. The corrected
reader detects the two formats explicitly, rejects unknown schemas, and checks
that covered activity families are actually found. `coverage.json` is the corrected
inventory. Exact replay comparison had already compared raw/decompressed bytes
independently of this reader, so all 587 equality results remain valid. No simulation
rerun, policy change or expected-outcome revision was required for this repair.

## Negative results preserved

- Large-world exchange still has the rare-offer bottleneck; money remains unjustified.
- Direct barter still misses live settlement in the accepted F9 comparison.
- Physical markets still enable the controlled delayed swap without creating
  widespread trade or new offer motivation.
- Escrow-induced posting/cancellation churn and its effects on work and nourishment
  remain; extracting pure exchange choice did not repair or tune that behavior.
- F10 authority can still fail to arrive through congestion, vanish on departure
  or death, and authorize projects that lack materials or perceived benefit.
- Existing communal-delivery arbitration, ownership lock-up, migration, starvation,
  spoilage, uncertainty and fixed risk margins remain.

These are preserved results, not failures of the B0 refactor. There was no observed
regression requiring a behavior fix in the registered coverage. This does not prove
all possible scenarios equivalent or certify a complete generative human model.

## Roadmap boundary

The dependency order is now:

**F10 → B0 (passed) → B1 → B2 → B3 → F11.**

B1 starts with seeded household/kinship relations between still-independent people.
Care/sharing must be a separately explicit experimental assumption; B0 introduces no
pooling, inheritance or super-agent. B2 requires directed history/appraisal to change
later choice, not decorative affect. B3 requires an explicit biological time-scale
model without psychological or occupation rules assigned by sex. No F11 work began.

## Reproduction

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo build --release --bins
python3 scripts/run_person.py /tmp/won-b0-replay
python3 scripts/analyze_person.py /tmp/won-b0-replay /tmp/won-b0-coverage.json
```

Use a fresh output path. The runner stops on a mismatch before proceeding to the
next panel, preserving any partial run. `SHA256.json` seals this research directory
excluding itself. The architecture document defines where future social components
may enter without granting remote knowledge, free goods or a second authoritative body.
