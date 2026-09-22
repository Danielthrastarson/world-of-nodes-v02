# Temporal consistency — registered research design

Freeze the default policy, all physical rules and all three prior evidence sets.
Run seeds 0–29 for five 72h arms in the unchanged 600-quanta delivery fixture:

* reactive: existing meal-aware policy; observational plan episodes only.
* remembered: identical decisions, plus a retained memory of the chosen board visit.
* resumable: saved board goal is eligible after eating, critical hunger, sleep,
  atomic work and ordinary below-12h self-provisioning, before fresh job valuation
  and optional personal-buffer gathering. Local actions pause the goal. An ordinary
  self-provisioning trip away cancels it explicitly as a preference override.
* committed: saved board goal is eligible after eating, critical hunger, sleep and
  atomic work, before the ordinary below-12h preference. No job/worker is assigned.
* self_priority: no persistent goal; skip only the ordinary below-12h branch while
  executing a previously selected, survival-feasible board route. An ephemeral
  route marker survives traversal/queue waiting and is cleared by eating, gathering,
  sleep or any other interruption. Revaluation and all other priorities remain.

Remembered/resumable/committed goals survive eating, local gathering, sleeping and
waiting. Critical hunger is the existing nutrition <=6h threshold. Eating always
remains possible; critical food-seeking and fatigue >=22h/recovery pause goals.
No sleep recovery rates or thresholds change. Critical food-seeking may take the
agent away; the goal remains remembered unless a separate validity check cancels it.

Continuation checks use personal knowledge only: known route connectivity/capacity,
predicted death before the next node (including remembered local queue evidence),
locally learned request/source invalidation, and food timing. For timing, use a
necessary lower bound: even nominal travel board→remembered source→board plus one
2s gathering unit must fit before remembered expiry, after the estimated board trip.
This is a rejection test, not a forecast guaranteeing acquisition or completion.
It does not reimpose the whole-job six-hour acceptance margin during an already
chosen board visit. Actual local job acceptance still uses the unchanged meal-aware
forecast and six-hour end margin. Request expiry is inactive: this board has no
request deadline (12h claim leases are not request expiry).

Reaching the board completes the visit intention before the next decision; it does
not claim the request. Death cancels the intention. Request identifiers in this
kernel are board-node identifiers, as with existing Job.request. Research intention
records cannot change boards, reservations, goods, FIFO or remote observations.
If an explicit validity cancellation would otherwise leave a resumable/committed
agent waiting in its old queue, cancel that atomic action through the kernel's
existing interruption mechanism. Remembered-only instrumentation never does this.

Reactive/self_priority episodes are observational: a non-serving action ends that
episode. Remembered goals may persist even when ordinary policy does something else;
they confer no priority. Record active, paused, completed and cancelled populations
at the horizon rather than treating unresolved goals as successful or cancelled.

Define reversal before running: at a node, a newly selected traversal back along
the last completed edge counts once per completed leg, even if its queue is later
cancelled. Separately count physically admitted opposite traversals and completed
F→X→F trips. Restrict the primary uninformed-reversal statistic to reversals of
board-directed legs; also report all-route reversals with their denominator.

Material new observations since the original board decision are: new/changed route
topology or capacity; queue changes that alter the locally estimated nominal service
bucket; accessible food crossing the chosen quantity requirement or changing expiry;
and request availability crossing the planned quantity. Ignore timestamps alone,
people counts, stock changes above the needed quantity, known expiry/lease passage,
and predictable hunger/fatigue. Exclude one's own queue membership from the current
queue bucket. Retain explicit changes, raw local observations, physiology and policy
reasons: new evidence being present does not prove it caused a reversal. Record
survival-driven reversals separately from ordinary policy overrides, even when both
occur without new external information.

No combined arm, new threshold, physical tuning or default adoption. Interpret
results as evidence about these specified arbitration semantics in this fixture,
not a universal choice of NPC architecture.
