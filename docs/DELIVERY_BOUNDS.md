# Physical feasibility bounds (locked P1 kernel)

These concern validated physical schedules, not autonomous behavior. No agent
policy calls this analysis. The auxiliary 600-quantum request ledger permits
communal ownership/Deliver intents in the diagnostic harness; it creates no food.
All 50 agents, initial nutrition/fatigue, actual gathering, clocks, queues,
capacity, meals, immutable food age, spoilage and death remain the locked ones.

## Earliest first quantum: 8h 8.334s

A first delivery needs four 2h nominal traversals and at least one 2s harvest.
Before this time no carrier can die (unfed death is at 36h), and the current
intent API has no live handoff at X or F: Deliver goes to the accepted job's
board at C. Consequently the first delivered quantum's carrier must make the
whole trip and harvest its own meals. This certificate is specific to this
kernel, not to a future relay/drop-anywhere model.

Let E be integer quanta eaten by this carrier before the first delivery.
Give the carrier the optimistic nutrition bound 12+E-t and ignore congestion.
Total required nominal work is at least 8h + 2(E+1)s. A candidate witness
arrives at 8h 8.334s. E>=4 already needs more nominal work than that wall time.
With E<=1 the unavoidable hunger slowdown over at least the last hour loses
hundreds of seconds, also excluding improvement on the witness.

For E=2, maximum rate is one through 8h, then 0.8*0.9=0.72: hunger reaches 6h
and fatigue reaches 12h at 8h. Six seconds of work remain, requiring
ceil(6000/0.72)=8334ms. For E=3, eight seconds remain at rate 0.9, requiring
8889ms. These bounds allow meals optimistically early, never later than the
actual schedule. The E=2 schedule attains its bound: reach F at 4h, harvest
and consume two personal quanta, harvest one communal quantum, return through X.

Sleeping cannot improve these very short candidate times. Relative to staying
awake, S seconds asleep can delay the fatigue boundary by at most 3S. For E=2
this can recover at most 0.08*3S nominal seconds after 8h, while losing at least
0.72S work opportunity. For E=3 the corresponding gain is at most 0.1*3S and
loss at least 0.9S. Waiting also cannot improve the bound. All other fatigue
bands are unreachable before the candidate time. Discrete millisecond rounding
is included. A kernel replay tests attainment, not just arithmetic.

## Bulk delivery: a bound, not a certified optimum

Count distinct quanta transported from F to C, not recycled deliveries of the
same goods. The research schedules never reload goods delivered at C.

The X–F edge cannot first be entered before 2h. A loaded traversal back to X
must finish before 46h to permit even an unimpaired final traversal to C before
48h. Each completed return requires a preceding outward traversal: all people
start at C. Shared capacity is four; each direction costs at least two hours.
Therefore N qualifying returns require 4N edge-person-hours inside a strictly
shorter-than-44h window of capacity four: N<44, hence N<=43. At capacity ten,
**at most 430 distinct food quanta** can reach C before expiry. Extra traversals,
physiological penalties, gathering and queues can only lower this relaxation.

The finite scheduler sweep produces constructive lower bounds. Its best row
is not an exact maximum; the interval from the best witness to 430 remains
unresolved. No mixed-integer/exhaustive global scheduling optimality certificate
is claimed. This uncertainty does not affect the existence of physical delivery
or the observed zero autonomous acceptance.

## What does “supportable at C” mean?

We distinguish three quantities. A resident is someone who never leaves C.

* **Continuously nourished through 48h:** each resident needs at least 36 physical
  quanta beyond the initial 12h. The transport bound implies at most floor(430/36)
  = 11 such residents. Witnesses supply a lower bound, not an exact optimum.
* **Continuously nourished through 72h:** impossible for a communal C resident.
  Unconsumed food expires at 48h and normal communal nutrition is capped at 12h.
  Even a last possible meal cannot bridge the following 24h. Bound: zero.
* **Alive at 72h:** this is much weaker. One quantum near 34h resets starvation;
  a second just after 47h resets it again. At 72h the resident is alive with zero
  nutrition and almost 24h of current starvation. The research-only rescue
  schedule deliberately demonstrates this pathology, not adequate provisioning.

The maximum number of stationary residents alive at 72h is **47**, attained by
three carriers delivering 120 quanta and feeding each resident just two quanta.
Why not 48? Every such survivor needs at least two quanta; one quantum consumed
before the initial 36h death cannot extend survival to 72h. Forty-eight residents
would need at least 96 quanta transported by at most two travelling agents.

For any carrier, nominal movement/gather work rate R obeys
R <= 2/3 + (1/3) df/dt, where f is fatigue in hours. Awake below the fatigue cap,
the RHS is one; awake at the cap it is 2/3, exceeding the actual maximum 1/2;
asleep above zero it is zero. The inequality also holds at the sleeping lower
clamp and after death. Integrating to 48h from fatigue 4h to at most 24h gives
at most 38h40m nominal work per carrier. Two carriers have at most 77h20m.
Transporting 96 quanta requires at least ten loaded crossings on EACH edge and
at least ten outward crossings on EACH edge: 80 nominal person-hours. Impossible.

The three carriers can also remain at C after their final delivery and survive
to 72h from their source meals. Thus **all 50 agents can be alive at C at 72h**
under a centrally scheduled witness (population size is the matching upper bound),
while 47 of them have suffered extreme starvation. This is not a claim that 50
people were fed adequately. Exposure is recorded separately.

## Binding constraints and limitations

The first restriction encountered by a simultaneous carrier dispatch is edge
capacity at t=0. Fatigue first reduces fully fed uninterrupted movement at 8h;
hunger can bind earlier when meals are omitted. Source depletion does not bind
the demonstrated schedules. Food expiry supplies the absolute 48h deadline.
The transport inequality is the binding constraint in the analytical bulk upper
bound; it is not proof that transport alone determines the exact physical optimum.

The scheduler searches a declared finite family of convoy/meal/rest schedules.
It rejects new outward trips from C at or after 40h because even an ideal complete
8h trip plus gathering would miss expiry. This is a research scheduling choice,
not a new autonomous obligation or a world rule. All successful intents still
use the original runtime, FIFO and conservation audits.
