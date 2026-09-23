# Independent review: one preserved F13A trace changes after the clock correction

Conclusion: accept the narrowly demonstrated mechanical correction, with the
historical difference disclosed. **Strict historical replay remains failed for
30 `long_commissioned` runs.** Do not relabel the strict result as exact replay,
rewrite the old evidence, or add a generic state/event-hash exception.

The other 150 F13A runs retain exact outcomes, hashes and traces under the strict
verifier's existing wall-runtime and explicitly disabled-Config-field rules.
This review concerns the F13A discrepancy only; it does not adjudicate separate
parameter comparisons or replace the older/F12 regression validations.

## Demonstrated defect

`military::volunteer` previously inserted a pending policy wake after selecting
voluntary enlistment. The next arbitrary caller checkpoint could consume that
wake and assemble someone earlier than the normal policy opportunity. The
retained `partition-before-order.txt` reproduces different muster timing when
the same no-order world is advanced directly to one minute versus through a
one-millisecond observation checkpoint. `partition-before-order-fixed.txt`
records the passing counterpart after removal of that insertion.

The change in `src/provisioning/military/adapter.rs` removes that unscheduled
wake. It does not change volunteer motivation, travel, physiology, service
duration or command priority. The reproducer occurs before any ruler order
exists, separating the clock defect from the new institution.

## Exact historical effect

In preserved F13A `long_commissioned`, Person 1 volunteers at 108,180,000ms.
Another Person's sleep event formerly let the pending wake trigger muster at
108,232,800ms. The corrected run assembles Person 1 at the ordinary policy
boundary, 108,240,000ms: **7,200ms later**. There is no travel displacement;
the existing journal labels elapsed volunteer-to-muster time `travel_ms`, which
changes from 52,800 to 60,000ms here.

The same twelve-hour service ends at 151,440,000ms instead of 151,432,800ms.
Thus the change shifts a fixed-duration duty interval rather than extending it.
Between the quarter-hour samples 30.25h and 42h, the corrected activity ledger
has 0.002 more idle person-hours and 0.002 fewer military-duty person-hours.
After the shifted service ends, these totals converge exactly. This occurs in
48 sampled times × 30 seeds = 1,440 timeline rows.

For all 30 affected runs:

* Every final `runs.csv` value except wall runtime and the two hashes is exact.
* All 540 final Person rows and all 2,070 final activity rows are exact.
* Goods, initial beliefs, received reports and infrastructure trip traces are
  exact, as are the other institutional records.
* The final institutional roster changes only Person 1's assembly and service
  end times, by the same 7,200ms in each seed.
* Timeline differences are limited to those two transient activity columns and
  state/event hashes from the first affected sample onward.

The available per-Person final fields, physical outcome metrics, institutional
records and seed-zero causal traces support unchanged nourishment, mortality,
production, construction, care, transport and security outcomes. This is not a
claim that the complete serialized state is byte-identical: its service history
and event identities intentionally differ.

## Exact causal trace, including changed hashes

The seed-zero trace changes from 100,566 to 100,564 events. Three assembly events
move to the normal policy boundary, replacing a redundant military-duty choice.
Three service-end events move by 7,200ms, and the redundant early post-service
Wait choice disappears. Subsequent event IDs consequently shift. Five later
`SocialAppraisal.source_event` pointers refer to the corresponding renumbered
events; their appraisal contents and times remain unchanged.

`scripts/audit_command_clock_regression.py` reconstructs the **entire** corrected
trace from these exact edits and checks every remaining field. It also pins the
full old/new semantic CSV contents, retaining all state/event hash values, and
the full uncompressed changed traces to independently inspected SHA-256 values.
Any other changed hash, event, numeric value or byte fails this review. This is
not an instruction to ignore hashes generally.

Reproduce the bounded review without rerunning or changing either panel:

```sh
python3 scripts/audit_command_clock_regression.py \
  research/kingdom-readiness/matrix \
  research/command-chain/regression/kingdom-readiness \
  research/command-chain/NEW_CLOCK_REVIEW.json
```

Report the result as **reviewed technical correction**, alongside the preserved
strict failure. The F13B conclusions must not depend on pretending that a proven
clock inconsistency remains correct for experimental purity.
