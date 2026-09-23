# F13B development record

Baseline clean source: `f1abb74`. Prior artifact inventory:16,215 files.
No earlier research output is rewritten. Smoke evidence is developmental, not the
final primary panel. Parameters are registered in docs/COMMAND_CHAIN_EXPERIMENT.md.

Adversarial review before the primary panel found and resolved:

- A potential control confound: a control-only source report could refresh queue
  beliefs absent from the command arm. Both arms now physically carry the identical
  factual report; the ordered packet adds only the scoped instruction. A test
  compares recipient physical/belief state at receipt before the role choice.
- A role queue could remain live when duty paused for survival. Only the attributed
  pending queue is now cancelled, with duty retained and any underway edge atomic.
  Subsequent independent exploration is still legal; tests must not prohibit it.
- Expiration between policy ticks needs an explicit known-deadline boundary.
  Queued duty expires before later admission; unknown orders create no timer at
  the recipient. Controlled expiry/queue tests isolate this from later exploration.
- New role attribution and optional-exchange protection originally used descriptive
  reason prefixes. Typed candidate/action matching now carries these semantics.
- Research receipt metrics originally used polling time. Metrics now use actual
  stored receipt times; research release/refusal checks run on fixed minute
  boundaries, so caller checkpoints cannot advance the intervention.

A full integrated partition test then exposed an **existing F11 defect** before
any new command was issued. Volunteering left a pending decision wake. Inserting a
caller checkpoint at1ms caused JoinMuster at1ms rather than the ordinary60000ms,
changing service expiration. Diagnostic event diffs and the small failing witness
are retained in partition-diagnosis.txt and partition-before-order.txt. The narrow
fix removes the post-choice volunteer wake; it retains the actual volunteering
choice and all physical rules. The small and full integrated partition tests pass.
Historical replay validation determines whether preserved protocols remain exact.
No general military rebalancing or priority change is inferred.

Early new tests also made incorrect assertions: event log provenance (`last_event`)
was compared as if communication should leave all body-record bytes untouched;
a pure-policy test omitted the ordinary preceding local memory observation;
and a queue test prohibited independent exploration after the old duty queue was
correctly withdrawn. Assertions were corrected only to match their stated physical
scope. Failed logs remain, including development-tests.txt and
queue-hunger-adversarial-test.txt. No old test expectation was changed except adding
the new optional capability field to a struct initializer.

The first mechanism smoke expected a distant recipient receipt. The trace instead
shows arrival at19.075h, immediate eating, then critical-fatigue sleep and no
handoff by24h. The test now requires equality of paired receipt outcomes, not a
successful receipt. No scenario quantity, time, capacity or physiology was tuned
to remove that negative. The primary panel preserves the unexercised late/stale/
revocation-receipt branches; separate explicit-contact mechanism witnesses test
those semantics without claiming autonomous messenger success.
