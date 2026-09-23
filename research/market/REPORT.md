# F9B — physical local markets, without money

**F9B passes for mechanism correctness and the bounded causal comparison.**
Persistent deposited offers enable one autonomous delayed swap in every controlled
co-location run, versus none with direct barter. They do **not** generate trade in
the 50-person F8 worlds: those still produce only one offer across 120 market runs.
Offer formation remains the first break there. No monetary institution is justified
by this experiment; no money or further economic mechanism was implemented.

The market also exposes strong posting/cancellation churn. A separated fixture
changes farming and survival with **zero swaps**. Improved survival cannot therefore
be attributed to successful exchange alone. The treatment changes custody and
available choices as well as the duration of offers; its full consequences matter.

## Institution and unchanged policy

[Specification and audit corrections](../../docs/MARKET_EXPERIMENT.md).
Markets are initially designated at existing settlement nodes A and B. They add no
building, route, free resources or labor capacity. Posting deposits a physical
personal batch quantity into `MarketEscrow(offer_id)`, retaining owner and original
food age. Escrow is unavailable to eating, pickup, construction and other offers.
An idle, living local buyer may accept an active offer using owned, physically local
payment. Both transfers validate before either occurs. Both outputs stay at that
market as recipient-owned loose goods, with ordinary kernel batch merging.

The fixed lease is 12h; earlier spoilage invalidates settlement. Local cancellation
or expiry releases goods at the market, never into an absent inventory. Departure,
work or remote death does not revoke an already deposited standing authorization;
if it settles, payment still belongs to the original owner, even after death.
This provisional interpretation is explicit and tested. No inheritance or seizure.

No remote notification, cancellation, board or inventory lookup enters policy.
The one-outstanding-offer limit uses the proposer's private record, refreshed only
locally or by its known deadline. A remote settlement cannot free that recorded
commitment telepathically. No market-search journey or automatic retrieval task is
added. Existing local eating/pickup/construction can use released goods and payment.

The F9A formation loop, use-based willingness, food buffer, conditional cart demand,
quantities, priorities, farming, communal jobs, migration and transport are unchanged.
For local reconsideration of one's own offer, escrow counts toward the quantity
already committed for surrender; it does not become spendable stock. The capital
calculation still sees only cargo actually available to haul. That distinction can
cause cancellation/reposting. No anti-churn rule or new economic motivation was added.

## Experiments and preservation

Final source **3168814**. [Provenance](matrix-final/provenance.json),
[validation](validation-final.json), [full analysis](analysis-final.json).
Six fixtures × direct/market × seeds 0–29, **360 final runs**, each 168h.
Four unchanged 50-person F8 worlds use complementary, symmetric, low-cost and
high-cost geography. Two unchanged F9A two-person fixtures start complementary
personal holdings together or separated. The two-person results are mechanism
controls, not population-level estimates for the larger worlds. Their outcomes are
identical across these seeds; thirty repetitions do not establish geographic breadth.

All **180 direct controls** reproduce their F9A state/event hashes, checkpoint
outcomes and full seed-0 traces. The runner also compares their archived output
files, allowing only additional observational counter rows. **2,195 prior research
files** retain their hashes. **173 tests pass**, including 17 market tests; clippy,
formatting, material and FIFO checks pass. Every final run has zero accounting error
and zero overtakes. Two detailed witnesses reproduce their matrix CSV rows exactly.

Two development defects were found and repaired rather than tuned:

1. The inherited global active-offer check exposed remote settlement through posting
   admission. [Failed test](remote-admission-failure.txt). The repair uses the
   proposer's locally maintained record, without changing offer motivation.
2. Research initially retained separate receipt lots, bypassing normal goods merging.
   Because existing capital reasoning is batch-based, that changed later choices.
   [Failed test](receipt-fragmentation-failure.txt). Release/settlement now use normal
   `Goods::move_part`. Receipt attribution is observational; deleting its journal
   between decisions produces identical physical state and events.

The 720 development runs in `matrix` and `matrix-corrected`, their analyses, failed
checks and preliminary traces are preserved, **superseded, and excluded from final
causal conclusions**. The second repair changes outcome/event checkpoints in 61
market runs. This demonstrates why the instrumentation defect mattered. They are
not additional independent samples. Historical F9A/F8 findings remain unchanged.

## Offers and settlement

Counts below are totals across 30 runs in each arm, direct → market. Observations
are unique observer/offer pairs; one person can observe many reposts. Declines are
unique observer/offer/reason combinations, not separate people or distinct offers.
Formation evaluations are repeated eligible decision points, not independent trials.

| Fixture | Formation evaluations | Offers created | Offers observed | Swaps |
|---|---:|---:|---:|---:|
| complementary | 5,773,726 → 5,773,726 | 0 → 0 | 0 → 0 | 0 → 0 |
| symmetric | 7,394,032 → 7,394,032 | 0 → 0 | 0 → 0 | 0 → 0 |
| low_cost | 6,947,823 → 6,947,841 | 1 → 1 | 41 → 42 | 0 → 0 |
| high_cost | 5,006,065 → 5,006,065 | 0 → 0 | 0 → 0 | 0 → 0 |
| colocated | 132,000 → 253,440 | 60 → 960 | 30 → 960 | 0 → 30 |
| separated | 185,640 → 240,300 | 30 → 10,680 | 0 → 0 | 0 → 0 |

All 11,641 market offers physically deposit their promised goods. Final statuses:
30 accepted, 11,610 locally cancelled, one invalidated by spoilage, zero timed out
while active, zero remaining open at 168h. The direct controls create 91 offers,
all closed on departure, with zero swaps. Static tests separately exercise exact
12h expiry and local reclamation; absence of live expiries is an outcome.

Market declines: eight unwanted-food cases and two unavailable-payment cases in
the low-cost world, plus 30 unavailable-payment cases in co-location. There are
also 30 failed settlement attempts because cancellation won the same-time action
ordering; these are distinct from voluntary refusal and create no partial swap.
Work/priority deferrals are separately logged, not relabeled as unwanted offers.

No qualifying live double-coincidence failure is established. The low-cost hungry
observers lack available personal payment rather than demonstrating a willing
seller rejecting their useful alternative goods. In co-location the buyer's wood
is locked in its own offer: aligned wants with unavailable collateral, not mismatched
wants. The diagnostic declines to infer an absent seller's current preferences;
zero established cases is not a proof that such friction can never arise.

## Causal traces

### Controlled co-location: asynchronous settlement works, with churn

[Final trace](colocation-final-trace.json); full replay in `colocated-final-witness`.
All 30 runs follow the same pattern:

- Agent 0 offers six food for ten wood. Deposit reduces locally available cargo;
  after eating, the unchanged conditional cart evaluator says `NO_TIME_PAYBACK`.
  The seller cancels. Normal release restores available stock and `INVEST`, so the
  same rule posts again. There are 31 food posts/cancellations per run. These reuse
  existing units; they are not 186 newly created food quanta.
- At 1h the other agent finishes sowing and chooses to accept a food offer. The
  proposer cancels first in the existing deterministic action order. Acceptance
  fails atomically. No scheduler priority or action ordering was changed.
- At 1h01m agent 1 posts ten wood for two food. At 1h02m it queues and departs;
  the wood stays in escrow. At 1h03m agent 0 accepts. Posting-to-settlement delay:
  **two minutes**. The seller is already travelling; direct barter cannot settle
  that offer after departure.
- Wood becomes agent 0's property at A. Two food become agent 1's property at A.
  The food is not sent to its travelling owner. Agent 1 returns at **19.5h** and
  consumes the two food locally: **18.45h after settlement**. There is no backpack
  pickup of payment; local consumption is recorded separately as actual receipt use.
- Paying two food leaves only ten available food at A. The cart opportunity no
  longer saves a trip under the existing evaluator. No cart is built. The acquired
  ten wood remain unused at A and later become inaccessible dead-owner property.

Thus each run has two unique traders, one swap, no repeat trader, ratio ten wood
for two food, and no inter-settlement transport of the exchanged wood. Across the
30 runs: 300 wood and 60 food change owners. Payment is locally consumed in all
30 runs; no payment is teleported or remotely acknowledged. The exchanged food is
used; the received wood does not realize its hoped-for capital use. This exposes a
limitation of the unchanged bounded valuation rule, not a monetary success case.

### Separated partners: survival changes without trade

[Final trace](separation-final-trace.json). Agent 0 posts/cancels **356 times** per
run, with no observer and no swap. Last posting is at 11.85h. Time spent in this
cycle changes later allocation: cultivation starts at 14.1833h, the first harvest
occurs at 28.5167h, and eventual production is 120 food. Agent 0 survives to 168h;
agent 1 dies. The direct control produces no food and both die.

This is not a successful market-mediated resource flow. It is an institution-induced
custody/valuation loop changing the timing of existing choices. Offer-generation
code and cadence were not increased; the physical state repeatedly makes the same
rule eligible again. Preserve and measure this churn rather than crediting its
incidental survival effect to exchange or adding a rule to hide it.

### Larger low-cost world: longer exposure still lacks useful payment

[Final seed-18 trace](low-cost-final-trace.json). Agent 35 deposits six food for ten
wood at 46.275h. The offer survives departure, attracts 42 distinct local observers
versus 41 in direct barter, and closes at **48h** when its original food spoils.
Eight observers decline unneeded food; two hungry observers lack available personal
payment. The six units become owner-held waste at A. They are not refreshed by
posting and do not return to the seller. No swap occurs. The other 119 larger-world
runs produce no offers.

## Nourishment, production, movement and labor

Means per run at 168h, direct → market. Food units are physical quanta; visits count
inter-settlement arrivals, not inferred permanent migration or planned trade travel.

| Fixture | Alive | Produced | Consumed | Spoiled | Starvation-person-hours | Arrivals |
|---|---:|---:|---:|---:|---:|---:|
| complementary | 15.067 → 15.067 | 2813.600 → 2813.600 | 4879.800 → 4879.800 | 924.600 → 924.600 | 914.331 → 914.331 | 68.000 → 68.000 |
| symmetric | 18.000 → 18.000 | 3696.000 → 3696.000 | 6146.000 → 6146.000 | 550.000 → 550.000 | 856.000 → 856.000 | 40.000 → 40.000 |
| low_cost | 18.533 → 18.533 | 3302.400 → 3302.400 | 5600.400 → 5600.433 | 671.533 → 671.533 | 761.000 → 761.000 | 90.333 → 90.333 |
| high_cost | 13.267 → 13.267 | 2427.200 → 2427.200 | 4310.600 → 4310.600 | 1116.600 → 1116.600 | 944.969 → 944.969 | 30.267 → 30.267 |
| colocated | 0.000 → 1.000 | 0.000 → 144.000 | 95.000 → 218.000 | 2905.000 → 2926.000 | 48.058 → 24.000 | 6.000 → 3.000 |
| separated | 0.000 → 1.000 | 0.000 → 120.000 | 97.000 → 205.000 | 2903.000 → 2915.000 | 48.000 → 35.598 | 2.000 → 3.000 |

The larger-world survivor and starvation results are unchanged. Low-cost seed 18
consumes one extra food quantum (mean +0.0333) with no survival gain. No wood exports
appear. Existing communal deliveries remain unchanged in the larger worlds;
co-location's communal deliveries fall from four to zero. Barter is not automatically
communal provisioning. All worlds retain zero completed carts; low-cost arms each
have 89 unfinished projects and 890 invested wood in total, unaffected by this swap
institution. No crossings, engineered roads or capital upgrades were added.

In co-location, farming labor (sowing plus harvesting) rises from 1.717 to 14.889
person-hours; total travel falls from 45.049 to 14.500. In separation, farming rises
from zero to 18.696 hours, and travel from 8.788 to 13.525. Lifetime labor totals
also change because an agent survives longer. They are not equal-horizon estimates
of opportunity cost. Complete exclusive activity accounts, stock inventories,
resident locations and per-seed checkpoints are in the CSVs and analysis. There is
no trade-search action; it would be incorrect to label all changed travel as
exchange-search labor or all changed farming as the value of a swap.

Both controlled fixtures retain two living agents at 36h, 48h and 72h in both arms,
yet diverge in starvation and later survival. At 72h, co-location starvation exposure
is 25 → 12 person-hours, separation 23 → 11. At 168h these become 48.058 → 24 and
48 → 35.598 respectively. Alive is not equivalent to adequately nourished. The
starvation reset mechanics remain unchanged; no indefinite sustainability claim.

## Escrow and stranded property

Peak concurrent escrow in the co-location witness is six food plus ten wood;
separation peaks at six food; the single low-cost offer locks six food. These are
resource-specific quantities, not a common economic value. Gross deposit/redeposit
flows can exceed the world stock because the same units circulate. Exact material
accounting does not.

No live offer expires while active in the final matrix: offers cancel, settle or
spoil first. Timed expiry semantics still pass deterministic conformance tests.
At 168h each co-location run retains ten wood received by the now-dead buyer and
ten waste from cancelled food offers under that owner's rights. The low-cost
witness retains six waste from spoiled escrow under a living owner's rights. In
separation, reclaimed food is subsequently used; there is no residual receipt stock.

Receipts use a conservative, documented attribution when identical goods merge:
remove pre-existing/untracked units first, then receipt cohorts in creation order.
This never changes batches or actions. Exact exchange quantities, owners, origins,
age and settlement location are recorded independently. Payment's two food are a
distinct owned batch here, so their later local consumption is unambiguous. Receipt
rows record pickup, local use, redeposit and other transfer separately. `other_removed`
in co-location is the two food used as payment, not disappearing matter.

## Gate and limits

**Pass:** explicit local physical escrow, atomic settlement after departure,
conserved ownership/custody, lawful local receipt use, expiry/cancellation/spoilage,
inspectable failure, exact controls, and a measured causal effect. **Not established:**
a self-organizing two-settlement trading economy, efficient allocation, generalized
specialization, double-coincidence pressure, or any need for money.

Persistent offers solve a temporal availability constraint in the controlled
meeting. They do not supply economic motivation in the larger worlds or spatial
contact in the separated fixture. Custody-induced churn and the valuation change
when payment removes a cart's hoped-for cargo remain visible. No policy repair,
merchant behavior, communication channel, priority transport or monetary institution
is adopted after obtaining these results. F9B ends here with evidence.

## Reproduction

From the repository root, using a fresh output directory:

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release --bin settlement_probe
python3 scripts/run_market.py /tmp/won-f9b-reproduction
python3 scripts/analyze_market.py /tmp/won-f9b-reproduction /tmp/won-f9b-analysis.json
python3 scripts/verify_market.py /tmp/won-f9b-archived-validation.json
```

The runner validates historical direct controls and previous research hashes.
`verify_market.py` verifies the archived final matrix and witness CSVs; it does not
silently replace artifacts. Final evidence is `matrix-final`, not either development
matrix. [Machine-readable results](analysis-final.json) contain mean/min/max and
paired differences; raw per-run evidence remains available. `SHA256.json` seals all
F9B artifacts except the manifest itself.
