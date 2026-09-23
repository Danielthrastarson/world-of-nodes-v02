# F13B controlled message-receipt witnesses

These are **controlled mechanism tests, not autonomous experimental results**.
They supplement the registered F13B primary panel; they do not replace, retune or
reclassify a primary run in which a tired carrier never hands over its message.
No receipt means no exposure to the hypothesized post-receipt effect. Preserve
that negative result separately from whether the receipt mechanism works.

The tests use the existing small deterministic commands unit-test fixture, with
ordinary Person physiology, travel, edge capacity, FIFO, spoilage and institutional
endpoints. The harness chooses departures, meeting times and deliberation times
with autonomous world scheduling disabled. Where noted, the **normal Person
selector** chooses the steward's next typed action. There is no teleportation,
forced construction, survival override, changed primary scenario or new policy.
All three witnesses passed the targeted test command below.

## Expired order physically delivered

Test: `controlled_physical_carriage_can_deliver_an_already_expired_order`
in `src/provisioning/commands/tests.rs`.

The sovereign issues a crossing order at time 0 with an explicit half-hour
lifetime. A courier receives it locally. Courier and steward physically traverse
the ordinary one-hour edge to their controlled meeting location. Mid-edge handoff
is rejected. At one hour the courier hands over the original message.

Assertions establish that receipt occurs after the deadline, the retained record
is `Expired`, no active obligation is formed, normal Person selection does not
choose fulfilment, and a direct fulfilment attempt is rejected. The crossing
remains unauthorized. FIFO overtakes remain zero and the world audit passes.

This proves expired-receipt semantics. It does not prove that an autonomous
courier will deliver a late message in a long, tiring primary scenario.

## Valid but outdated order does not refresh food knowledge

Test: `controlled_stale_command_delivery_does_not_refresh_spoiled_food_beliefs`
in `src/provisioning/commands/tests.rs`.

The controlled initial source contains eight food quanta with a half-hour
remaining lifetime. The sovereign and steward genuinely observe it before
issuance/departure. A courier carries the still-valid order during a one-hour
physical trip. The food becomes eight physical waste quanta through the normal
spoilage mechanism before receipt.

Receipt preserves the steward's original timestamped food observation exactly;
it supplies no current source quantities or freshness. A remembered expiry can
already imply that food is inedible—the stale observation is not a new guarantee
of food. The command is an instruction to commission an existing crossing, not a
claim that this stock remains edible or an order conditional on that claim.

The normal Person selector chooses physical return to the office. After that
one-hour return and a new legitimate local observation, the same selector chooses
the local commission action. Authorization completes while the crossing remains
unbuilt; all eight spoiled quanta remain accounted for. No construction benefit,
economic wisdom or autonomous delivery frequency is inferred.

This proves that stale surrounding conditions do not magically refresh an
instruction, and that the narrow valid institutional obligation can still be
attempted. It does not establish that the command includes a represented economic
motive, or that the steward autonomously re-evaluates the sovereign's reasons.

## Revocation follows physical travel and cannot undo earlier action

Test: `controlled_late_revocation_travels_after_the_steward_and_cannot_undo_prior_action`
in `src/provisioning/commands/tests.rs`.

After physical appointment/acceptance and controlled prior journeys, sovereign,
steward and courier are together away from the assigned settlement. The steward
receives the order locally and its normal Person selector chooses the return
journey. Relative to that departure:

| Time | Verified event |
| --- | --- |
| 0 | Steward enters the one-slot ordinary edge. |
| 0.25h | Sovereign issues revocation and hands it to the local courier. The absent steward's memory and embodied state remain unchanged. |
| 0.25h–1h | Courier waits behind the earlier traveler under FIFO. Mid-edge handoff is rejected. |
| 1h | Steward reaches the office; courier enters the released edge slot. Without received revocation, normal Person selection chooses and executes the lawful commission. |
| 2h | Courier arrives and physically hands over revocation, 1.75h after issuance. |

The receiver records the revocation only at contact. The already-completed
commission remains completed and authorized: revocation cannot retroactively
remove an institutional action or its consequences. The audit passes with zero
FIFO overtakes. This test deliberately controls travel and opportunities to
choose; it is not evidence that autonomous messengers naturally achieve these
meetings.

Complementary existing tests isolate other parts of the same contract:

- `remote_revocation_cannot_change_a_receivers_knowledge_or_choice`: identical
  recipient inputs yield the same decision despite unseen revocation elsewhere.
- `physically_received_revocation_releases_only_the_stewards_queued_route`: an
  open role undertaking loses its pending queue application after receipt, while
  the earlier traveler retains its physical traversal.
- `expiration_during_traversal_cannot_teleport_the_person_out_of_the_edge`: ending
  an obligation does not stop or relocate an already atomic traversal.
- `revocation_received_before_order_survives_duplicates_and_delayed_order`:
  delivery reordering cannot revive a revoked command.

An additional deadline regression,
`a_known_order_deadline_cancels_the_old_queue_before_fifo_admission`,
places a known expiry 30 seconds away, the next released edge slot 45 seconds
away, and the next ordinary policy tick 60 seconds away. The old role queue
application is cancelled at the actual deadline, before the slot opens. Direct
and irregular clock partitions agree. With autonomous selection enabled, ordinary
exploration may independently create a **new** request for that same edge after
expiry; that is not continuation of the expired obligation and is not forbidden
by this test. With selection disabled, the former role applicant remains at its
node after the slot opens. Atomic traversal already underway remains physical.

## Reproduction and limits

Run:

```sh
cargo test --lib provisioning::commands::tests::controlled_ -- --nocapture
```

Observed result: **3 passed, 0 failed**. The full commands test group also
passes **29 tests**, including the deadline regression. Broader tests cover the related
contracts; the full suite and primary panel have their own validation artifacts.
No aggregate survival, construction completion, autonomous messenger success,
changed ruler behavior or welfare conclusion follows from these three witnesses.
