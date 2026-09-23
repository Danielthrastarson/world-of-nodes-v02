# F9A — local barter rights, without money

**F9A passes for the implemented institution and its explanatory experiment. No
autonomous barter settles in the registered live worlds.** Static conformance
demonstrates voluntary policy-generated settlement and subsequent use of receipts;
that is not evidence that a trading economy emerged. Preserve this distinction.

Alienability removes the previous legal prohibition. It does not create a reason
to acquire goods for sale, a meeting, simultaneous availability, willingness to
wait, or a useful payment bundle. The first live failure is predominantly **offer
formation**, followed in the controlled meeting case by **temporal coordination**.
No monetary institution is proposed or implemented. F9B needs further evidence,
not an automatic currency choice.

## Scope, assumptions and implementation

The [preregistered specification](../../docs/BARTER_EXPERIMENT.md) was committed
before measurement. The owner authorized voluntary alienation of personally owned
goods and atomic local spot exchange. No forced sale, inheritance, credit, wages,
rental, communal seizure, remote market, price list or settlement super-agent.
Land/crop/cart-title transfer is outside this batch-goods implementation. Dead
owners still cannot consent; their property is not redistributed.

Typed `OfferBarter`, `AcceptBarter` and `RevokeBarter` commands operate on explicit
identified offers. One standing offer per person names a particular owned batch,
quantity, requested resource/quantity, node, creation and expiry. It lasts at most
one hour, with no goods reservation. It closes on departure, death, loss of inputs,
expiry or revocation. An offer is consent to its exact terms, not a future contract.
Concurrent acceptances cannot settle it twice. Both owners and both inputs must be
at the node and both people idle; another agent cannot interrupt someone's work
to complete a bargain. Both sides validate before either transfer occurs.

Outputs become recipient-owned **loose goods at the meeting node**, preserving
original age and ancestry. Pickup, carrying, eating and construction still require
their existing actions. Barter cannot overfill a backpack, teleport goods, refresh
food or create a cart. Immediate atomic exchange uses a decision opportunity;
there is no invented bargaining labor or communication delay.

The voluntary policy is a deliberately small **new behavioral assumption**, separate
from the owner's new legal right:

- Food is wanted when current nutrition plus locally owned edible food is below12h.
  Selling food protects that shortfall plus the existing six-quantum personal buffer.
- Wood is wanted only when the existing capital evaluator, conditionally supplied
  with owned recipe inputs, judges a remembered cart opportunity worthwhile and
  survivable. No construction, demand or knowledge is created by that calculation.
- An offer exchanges up to six surplus food for missing wood, or up to ten surplus
  wood for the food shortfall. These quantities derive from the disclosed buffer,
  holdings and recipe; they are not common-unit prices or a worker quota.
- Acceptance requires useful incoming goods and surrender outside protected uses.
  This is a bounded use-category rule, not a complete portfolio optimizer. It does
  not value speculative resale or gather resources deliberately for later sale.
- Only idle agents without communal jobs may consider exchange. Existing immediate
  eating/sleep priorities remain; no offer-waiting commitment or market-search route
  is added. Communal delivery arbitration remains unchanged.

Policy sees its own local holdings, personal memories and currently local offers.
It never inspects another agent's private needs/inventory or remote offers. Research
diagnostics may inspect those states only to classify coincidence failures; their
results never enter actions. A standing offer can become unattractive after it was
posted; voluntary revocation is possible, but no optimizer guarantees a good deal.

These assumptions limit all economic conclusions. In particular, instantaneous
food demand, cart-only wood use, single-batch payment and no speculative acquisition
can suppress trade. We did not repair them after seeing results.

## Experiments and preservation

360 runs, six fixtures × enabled/disabled × seeds0–29, each168h:

- Four unchanged50-person F8 worlds: complementary2h edges, symmetric2h,
  low-cost0.5h and high-cost4h. Same3000 food,50 plots,500 wood, carrying limits,
  FIFO, physiology, crop/cart rights, production and spoilage. Paths frozen as in
  these F8 controls; F7 laws unchanged.
- Two controlled two-person opportunity fixtures. Agent0 owns24 food at A;
  agent1 owns10 wood. Remaining2976 food/490 wood are at B. Two cultivation units
  at A. Agent0 has documented t=−1h route/request memories, including an earlier
  100-quantum request at B; current boards start without preloaded demand. Initial
  t=0 food is not revealed by those earlier memories. Agent1 has only local knowledge.
  In `colocated` both people and their goods start at A. In `separated`, agent1 and
  its wood start at B. Ownership/endowment interventions are initial conditions in
  both arms, not runtime transfers or forced trades.

These two-person fixtures are mechanism controls, not welfare comparisons with
the50-person worlds. No parameters were tuned after results. Two replay witnesses
are duplicates for reproducibility, not additional independent samples.

**156 tests pass**, including15 barter tests. Full clippy and formatting checks
pass. Every run retains zero material-accounting error and zero FIFO overtakes.
All **1,889 previous research files** retain their checksums. The120 disabled F8
runs reproduce all840 checkpoint rows exactly, including state/event hashes; their
four full seed0 traces are byte-identical to F8. Thus the F8 arrival-without-deposit
case, F7 outcomes and previous negative research remain intact. Complementary,
symmetric and high-cost enabled seed0 traces also match their disabled traces.
See [final-validation.json](final-validation.json).

## Offer and exchange funnel

Counts below are totals across30 enabled runs per fixture. Observation counts are
distinct observer/offer pairs. Declines are distinct observer/offer/reason records;
declining does not cancel the offer for other people. They are not interchangeable
denominators for a single acceptance percentage.

| Fixture | Offers created | Observations | Explicit declines | Closed on departure | Expired | Settled |
|---|---:|---:|---:|---:|---:|---:|
| Complementary |0|0|0|0|0|0|
| Symmetric |0|0|0|0|0|0|
| Low cost |1|41|8|1|0|0|
| High cost |0|0|0|0|0|0|
| Co-located opportunity |60|30|0|60|0|0|
| Separated opportunity |30|0|0|30|0|0|

All disabled arms have zero offers and swaps. Across enabled arms there are91
offers,71 observer/offer pairs, zero accepted offers, zero exchanged quantities,
zero unique or repeat live traders, and **no realized barter ratios**. All91 offers
close on departure rather than lease expiry. No live settlement fails a kernel
ownership/spoilage/atomicity check because no live acceptance intent reaches it.
Those rejection paths are covered by static tests, not claimed as observed markets.

The low-cost fixture records seven `COUNTERPARTY_DOES_NOT_WANT_GOOD` declines and
one `INSUFFICIENT_OWNED_QUANTITY`. Co-location creates an opportunity, not a forced
agreement. Sixty offers are posted with no other person locally present (30 in each
controlled fixture); no remote observer is invented. `TOO_COSTLY_TO_REACH` is not
fabricated: this policy has no trade-search journey. Missing remote offer information
is documented through zero local observations, not an omniscient NPC refusal.

There are3,167 eligible missing-wood-plan evaluations across low-cost runs, but only
one offer. The complementary, symmetric and high-cost arms have zero such eligible
evaluations. These counts describe repeated decision points, not independent people
or unique projects. Food pressure alone does not generate owned wood: baseline B's
wood remains a natural deposit, and existing acquisition rules gather wood for
personal investment, not speculation. Legal transferability did not create a supply
of owned payment goods outside protected uses.

## Witnesses explain the failures

### The single50-person offer

Low-cost seed18 agent35 finishes work at A and at46.275h offers six personal food
quanta for ten wood. It owns30 food, protects6 and has a conditionally worthwhile
cart opportunity. At46.2833h it chooses to visit a remembered request board and
joins the existing FIFO queue. The standing offer remains locally visible while
the proposer queues, but the proposer is no longer idle for settlement.

Seven evaluated observers have no immediate food demand. Agent23 later wants food
but owns neither food nor wood; it has no alternative owned bundle to present.
Agent35 departs and the offer closes at47.175h. Forty-one people observed the
offer, but observations do not imply eligibility or willingness to accept.

[Full selected trace](low-cost-offer-trace.json) and [independent replay](low-cost-seed18).
The actual mismatch is lack of a useful/owned payment among evaluated recipients,
plus the proposer's continued movement. It is **not a demonstrated double-coincidence
failure**: the hungry recipient had no alternative goods a willing seller refused.
No private inventory was exposed to the proposer to improve its offer.

### Co-location without simultaneous readiness

In co-located seed0 at t=0, agent0 posts six food for ten wood. Agent1 simultaneously
chooses to sow its personal crop. The new offer becomes observable at the next
decision, when agent1 is already working. At1min agent0 eats12 of its own food;
at2min it leaves to visit the remembered board. The offer closes at3min.

At1h agent1 finishes sowing and offers ten wood for one food quantum. Agent0 is
already absent. At1h1min agent1 explores; the second offer closes at1h2min. Both
people had complementary holdings at A, but their actions did not align into a
transaction. No waiting/meeting obligation is added to make this succeed.

[Timing trace](colocation-timing-trace.json) and [replay](colocated-witness).
All30 seeds follow the same offer outcome. In separation, the food-for-wood offer
has no observer before departure. This contrast demonstrates a local information
and timing constraint, not an estimated benefit from currency or a global market.

### Positive conformance is not emergence

The test `two_willing_local_policies_can_settle_and_use_received_goods` deliberately
starts both owners idle, with the recipient at8h nutrition. The normal use-based
functions propose/accept six food for ten wood. The kernel swaps owners atomically.
Nutrition does not change until the recipient performs Eat; it then reaches14h.
The other owner can commit the wood to a cart project, but no cart appears without
labor/time. This frozen decision-point witness proves that the API and willingness
rule can execute a consensual exchange. Its8h setup is a disclosed static fixture,
**not a retuned live arm or proof of autonomous market formation**.

A separate static diagnostic demonstrates a genuine coincidence case: a willing
wood seller wants food, while a wood-seeking agent owns only stone, which the seller
does not want. Adding a small quantity of the desired payment instead changes the
classification to a quantity problem. The live matrix contains **zero qualifying
double-coincidence cases**. The test is classifier validation, not empirical support
for money.

## Physical consequences and opportunity cost

Means at168h for enabled50-person worlds:

| Fixture | Alive | Produced food | Consumed | Spoiled | Starvation person-h | Communal deposits | Visits |
|---|---:|---:|---:|---:|---:|---:|---:|
| Complementary |15.07|2813.6|4879.8|924.6|914.33|0|68.00|
| Symmetric |18.00|3696.0|6146.0|550.0|856.00|0|40.00|
| Low cost |18.53|3302.4|5600.4|671.53|761.00|2.7|90.33|
| High cost |13.27|2427.2|4310.6|1116.6|944.97|0|30.27|

Every listed outcome equals its paired disabled arm. The low-cost offer moves30
seconds from queue time to idle time in seed18 (mean1 second per run), without
changing nourishment, production, deposits or migration counts. Those2.7 communal
quanta are the preserved F8 transfers, **not barter**. Wood exports and completed
carts remain zero. Low-cost89 projects across30 runs remain incomplete, with890
wood committed; no barter-financed capital forms.

In the two-person co-located worlds, both die by168h in either arm:95 food consumed,
zero production,4 communal quanta deposited and48.0582 starvation-person-hours.
Offers change A→B carried food from11 to10 quanta, while total consumption stays
equal. Four hours shift from loaded to empty travel; this is mainly a load change,
not four additional hours of journey. Total travel rises only0.00519h. In separated
worlds both also die; enabling offers changes consumption98→97 and spoilage2902→2903,
with total starvation48h and visits2 unchanged. No outcome is tuned away.

Exact exclusive labor, stocks, local need, crop lock-up, unused productive capacity,
capital projects and owner/holder batches are in the CSVs and [analysis.json](analysis.json).
The graph has no geometric distance unit; actual goods movement is reported as
**quantity-edge passages**, not invented kilometres. Mean edible quantity-edges in
the50-person worlds are60.5/0/96.8/0 respectively; waste is separately measured.
No traded goods exist in live runs, so subsequent movement of traded goods is zero.
The instrument also exposes an explicitly labeled ancestry upper bound for future
exchanged goods that merge with fungible unexchanged siblings; it is not exact tagged
quantum distance. No such bound is needed to interpret this zero-swap result.

Exchange-directed search travel is structurally zero: no new search command exists.
Paired activity differences measure displaced realized time, not a counterfactual
claim that every idle/transport hour would otherwise have been farming. Posting and
observing can change subsequent physical action sequences even without a swap.

## F9B — what the evidence supports

| Proposed monetary motivation | Finding |
|---|---|
| Double coincidence | Zero qualifying live cases. The single hungry evaluated recipient lacks payment goods rather than offering an unwanted alternative. Static classifier witness is not live evidence. |
| Comparing many goods | Not established. Policy values food and cart inputs only; it does not run a many-good comparison experiment. |
| Search/matching costs | Local discovery and simultaneous availability matter. Meetings can be missed while people work or depart. No trade-search policy or monetary treatment identifies a remedy. |
| Intertemporal exchange pressure | Time matters, but no future-payment proposals exist. Missed spot timing is not automatically a demand for debt/credit. |
| High variance in ratios | Offered ratios include6 food:10 wood and1 food:10 wood. There are no realized ratios, convergence or volatility estimates. Offers alone are not market prices. |
| Holding goods for resale | Structurally absent from this use-based rule. Its absence is not evidence that such behavior would never arise under another policy. |

The strongest supported bottlenecks remain formation of offers backed by owned
goods and temporal coordination of local participants. Alienable rights are now
implemented, but useful decentralized barter has **not** emerged in these tests.
Do not convert this into either “barter works” or “money is needed.” No monetary
institution has evidence sufficient for selection here. Any subsequent monetary,
credit, meeting, resale or access institution must be an explicitly scoped decision;
none is silently adopted in this increment.

## Reproduction and gate

Preregistration `dd50cb1`; live implementation `10b6915`; runner/additional policy
witness `a92f0c5`. Historical configurations retain `barter=false`; explicit F9A
arms select the new institution. Later conformance tests did not change live code.

```sh
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --bin settlement_probe
python3 scripts/run_barter.py /tmp/f9a-new-matrix
python3 scripts/analyze_barter.py /tmp/f9a-new-matrix /tmp/f9a-analysis.json
target/release/settlement_probe low_cost /tmp/f9a-seed18 1 18 on
python3 scripts/verify_barter.py /tmp/f9a-verification.json
```

Use new output paths. The verifier checks preserved artifacts, both witness replays,
all disabled regression histories and physical invariants. Main outputs retain the
F8 data dictionary plus `offers.csv`, `exchanges.csv` and `barter_counts.csv`.
`exchanges` preserves both original batches' ancestry/age, participants, quantities
and resulting batch IDs. `runs.barter_attempts` means offers posted, not negotiations.
Gate counts are repeated eligible evaluations; refusal counters are deduplicated by
observer/offer/reason. No percentage is inferred from unlike denominators.

Gate passed: explicit voluntary local transfer, atomic conservation, no hidden
knowledge/transport, inspectable successful static execution and failed live
coordination, and a bounded money-decision assessment. This gate does not certify
live trade, welfare gains, optimal preferences or currency necessity. F8 communal
delivery arbitration and every earlier negative result remain preserved.
