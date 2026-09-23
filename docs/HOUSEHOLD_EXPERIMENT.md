# B1 preregistration: household and directed care

Registered before implementation/results. Baseline df3ed3b, B0 accepted. This tests a provisional care institution, not human psychology. Optional household configuration is absent in historical scenarios. Existing physical laws and eating/survival/sleep priorities remain unchanged.

## Institution and representation

Households have stable IDs, member IDs and optional home association, without inventory or controller. Parent is directed; Child is its inverse query. Partner and Sibling are symmetric relations. Caregiver is separately directed, explicitly seeded, never inferred from membership or kinship. Dead members/owners remain recorded. Kinship need not imply coresidence. No inheritance, grief, reproduction, marriage law or shared property.

Each person receives only their own relations/obligations and personal timestamped observations. At a node they can observe an obligated person's presence, nutrition, starvation and living/dead state. This exact local physiology observation is a provisional information assumption. At a remembered destination, absence is observed without revealing the person's new location. No remote updates. A household has no beliefs.

## Bounded care rule

Use the common Person capability stage, after existing eating, critical hunger, critical/ongoing sleep and one-quantum gathering protection, before optional capital/construction/farming and ordinary self-provision. An accepted communal job is not replaced by care. No role-specific brain.

Only personally owned edible food physically carried by the caregiver (including an accessible personal cart) can be offered. The caregiver does not search for new food specifically for care in B1. This scope restriction is explicit and negative results must retain it.

Observed dependent nutrition at or below 6h can motivate care. Observation must be at most 12h old. Severity uses the observation, not a remotely updated need; forecasts may conditionally extrapolate elapsed metabolism, but never assert continued presence or survival. Choose lowest observed nutrition, then oldest observation, then stable person ID. This deterministic tie rule is not optimal multi-dependent allocation.

Use the existing coherent belief-only route estimator. Reject a trip if the caregiver is predicted to arrive with less than 6h nutrition, if remembered dependent physiology predicts death before arrival, or if offered food expires before arrival. These are conservative experimental preferences, not changes to metabolism. No future queue information. Locally available food already owned by the dependent reduces the apparent unmet need; their hidden backpack is not inspected.

Give at most 6 integer food quanta per atomic action, up to observed deficit toward 12h, from one edible batch. Transfer is immediate like existing pickup/barter: actual caregiver food moves to personally owned loose goods at the meeting node. Recipient ownership changes explicitly; age is unchanged. The recipient decides whether/when to eat using the existing policy. The transfer does not force an incompatible recipient action or bypass their carrying limit. No direct nutrition mutation. The donor's next decision still follows the authoritative scheduler; do not invent an extra action duration.

Care is an inspectable, resumable candidate undertaking, not compulsory commitment. Record creation, pause, resumption, completion and cancellation with reasons. Ordinary survival/economic arbitration can pause it. New local absence/death, old evidence, loss of cargo or infeasibility can cancel it. Edge traversal remains atomic; FIFO is unchanged. No barter stage may overwrite an already selected care action.

## Experiments

Paired seeds 0–29, care enabled versus disabled with identical kinship/household state, and a separate no-obligation/non-kin control. All people use the normal autonomous policy; dependents use existing physiology (B3 childhood is absent). Report deterministic repetition honestly rather than pretending seed count supplies independent variability.

Small controlled fixtures use actual initial personal endowments, not live gifts from a harness. Default two persons, donor nutrition 18h, dependent 0h, donor 6 quanta created t=0 with 48h shelf life, fatigue 4h, capacity 10. No external food. Duration 48h. Worlds have one node unless testing distance; no money or new resources. Dependent's initial zero reserve is an experimental scarcity condition, not child biology.

- Colocated: basic successful/failed allocation and non-kin control.
- Self conflict: donor nutrition 4h, six food quanta; preserve existing self-eating precedence.
- Multiple dependents: three persons, same six food quanta, two hungry dependents.
- Known distant need: explicitly documented t=0 prior observation at dependent's node, route known as an experimental initial condition; donor begins at other endpoint. Compare nominal 1h and 10h edges, capacity 4. This is seeded past information, not a runtime remote query.
- Unknown distant need: same physical state, no prior person observation. Local discovery may later change behavior; do not claim absence of all future care.
- Caregiver death: donor starts zero nutrition, 23h continuous starvation, no food; observe loss of support with no replacement/inheritance.
- Separation: known distant fixture; measure persistence of relations as autonomous travel changes residence, including stale/absent-target failures.
- Farming opportunity: colocated and distant fixtures with existing cultivation units/rules; compare actual paired activity time and production.
- Construction opportunity: existing crossing fixture with identical physical project/endowments in both arms plus a dependent; report if construction is not displaced. No extra construction motivation.

Record exact parameters in run outputs. Do not tune these to improve welfare. If a fixture exposes an implementation defect, preserve the failed output before repair.

## Measurements and gate

Per person: starvation, consumption, death, retained personal food, actual activity hours. Per obligation: observed care opportunities, goals, pauses/resumptions/cancellations, transfer/travel attempts, success/failure reasons, quantity transferred, care-associated travel/queue time, separation hours and caregiver loss. Research-only loss/separation measurements never feed personal knowledge. Activity deltas between paired runs are opportunity-cost evidence; do not assert the precise forgone counterfactual activity of a single action. Also report production, construction, migration (node residence changes, not a new home preference), spoilage and accounting/FIFO errors.

Tests cover directionality, ownership/exclusivity, locality/non-telepathy, age, failed physical transfer, death, care tradeoffs, deterministic choice, interruption and disabled compatibility. Run full Rust suite, fmt, clippy, historical scenario panel, F10 mechanism/project regression panel, and prior artifact SHA256 check. B1 passes only with causal care success/failure and measurable cost, while physical invariants and the B0 common Person interface remain intact. It does not require welfare improvement. Return evidence before B2.

Construction fixture clarification (before outcomes): use the original F6 closed graph, 50 persons initially at C, C–X nominal .25h/capacity4, closed X–F opened to .25h/capacity4, wood100 at C, recipe20wood/8h, food300 at X/2700 at F, initial request600, explicit prior spatial knowledge. Only persons0/1 are a seeded household; person0 has18h nutrition and6 additional personally owned quanta, person1 has0h nutrition. All others retain12h. Both arms share these exact initial conditions. The construction fixture is not claimed identical to historical F6; historical F6 is separately replayed unchanged.

Reporting clarification: the known-distant information controls seed an explicit t=0 initial memory rather than replaying pre-experiment movement. They therefore test conditional response to a supplied belief, not successful endogenous acquisition of that belief. The unknown-need arm and locality tests are the evidence for runtime information boundaries.
