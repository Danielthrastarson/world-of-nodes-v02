# F13A selected causal traces and attribution limits

Source revision: `47533e2`. All event references below are **seed 0** of the final
matrix. Event IDs are local to each run, not line numbers. Times are simulated
hours from the same authoritative clock, rounded for readability. These are
representative causal traces, not substitutes for the 30 paired seeds per case
in [SUMMARY.md](SUMMARY.md) and [paired.csv](paired.csv).

The assigned difference is a lawful local steward commission at time zero.
Report delivery, the initial appointment, the seeded hostile objective, the
voluntary muster request and the one locally issued protect order exist in both
arms. Consequently this panel identifies the commission's total effect in these
fixtures. It does not isolate an information treatment or the benefit of defense.

## Alternate route: one commission, real investment, later crossing use

Source: [alternate commissioned events](../matrix/alternate_commissioned/events.tsv.gz),
[final institutions](../matrix/alternate_commissioned/institutions.tsv.gz),
and [physical trips](../matrix/alternate_commissioned/trips.tsv.gz).

| Time (h) | Event ID(s) | Physical / institutional fact |
|---:|---|---|
| 0 | 4 | Steward 1 commissions project 0 under office 0, polity 17, at settlement 1. The event explicitly says no resources or workers were assigned. |
| 1.083333 | 1110–1113 | Persons 0, 3, 4 and 5 locally receive the unit order targeting node 2. It supplies no route or stock information. |
| 4.083333–4.333333 | 3038–3040 through 3236–3238 | Three raiders take four food quanta each on their first visit: 12 total. Food moves into attacker custody while its communal ownership and age remain recorded. |
| 5.084 | 3667 | Courier 2 hands report 0 to steward 1 at node 1. Its source observation remains at time 0; receipt age is 5.084h. No authority is transferred. |
| 5.096 | 3670, 3672, 3674, 3676 | All four ordered defenders reach node 2 after the raiders' first visit. |
| 5.100556–5.417222 | 3712 through 4067, twenty `CrossingDonation` events | Courier 2 voluntarily contributes 20 wood, one physical quantum per event, to Commons(0). The report recipient is not the builder in this case. |
| 5.433333 | 4070 | Person 2 begins construction. Labor does not create a personal title. |
| 12.016667 | 10713, 10716, 10719, 10722 | The four initial defenders' service terms end. |
| 12.033333 | 10761 | Person 2 resumes construction. Two work-start events represent the same worker, not two workers. |
| 14.461111 | 12348 | The crossing completes under Commons(0). Completion does not alter travelers already on other edges or broadcast remote observations. |
| 14.779111–15.334667 | 12587–12589 through 13097–13099 | The raiders return and take their remaining six quanta each: another 18, making 30 total. This second loss occurs **after crossing completion and after the original service terms ended**. |

The final project records donor `{2: 20}` and worker `{2: 32440000ms}`:
9.011111 actual person-hours for eight nominal hours of work. Physiological
impairment explains why nominal work and elapsed labor differ. The trip ledger
then records Person 2 traversing the new edge from 1 to 2 at 15.8–16.2h, followed
by a return carrying ten food quanta at 16.366667–16.866667h. Connectivity changes
through an actual completed object; cargo travels through the normal edge.
Across the 30 commissioned runs, the completed edge records 19 improved trips,
six unique users and 72 food-quanta passages per run. Passages count movement,
not final delivery or nourishment.

There are no strikes, wounds, incapacitations or lethal injuries in either arm
of this panel. All recorded deaths are ordinary starvation deaths. The initial
food theft was not prevented, and later service did not coincide with the
raiders' return. A crossing existing eventually is not evidence of timely
protection. This does not reverse the earlier F11C protection proof; it is a
separate integration configuration with zero defensive contact.

Paired means in this case: farm output 432 → 312, total food consumed 742 → 626,
starvation exposure 173.977 → 262.717 person-hours, deaths 6 → 7. These are total
commission effects under the registered controls. The trace demonstrates a
specific investment and movement sequence, but does not identify every lost
harvest or death as caused solely by the builder's 9.011h of labor. Other people
change routes, tasks and subsequent observations too.

## Scarce materials: authority cannot supply the missing wood

Source: [scarce commissioned events](../matrix/scarce_commissioned/events.tsv.gz)
and [final institutions](../matrix/scarce_commissioned/institutions.tsv.gz).

The same commission is event 4. Report 0 arrives at 5.084h (event 3667). Courier 2
contributes the five available wood quanta at 5.100556–5.167222h (events 3712,
3732, 3752, 3772, 3792). No construction work starts. At 72h the project still
holds five wood against a requirement of 20; all eight nominal work-hours remain,
and its worker ledger is empty.

The first construction break is material availability, not absent authority or
a missing report. `crossing_started=1` means the project received its first
material contribution; it must not be reported as completed construction or
labor having begun. The five contributed quanta remain physically embedded in
the unfinished communal project. No hidden pooling or replacement material is
created. The commissioned arm nevertheless changes other behavior: mean farm
output falls 432 → 312 and starvation exposure rises 173.977 → 230.228
person-hours, despite zero completed crossings. The experiment does not equate
an uncompleted project with an absence of consequences.

## Long route: funded construction stops at the existing nutrition margin

Source: [long commissioned events](../matrix/long_commissioned/events.tsv.gz)
and [final institutions](../matrix/long_commissioned/institutions.tsv.gz).

| Time (h) | Event ID(s) | Fact |
|---:|---|---|
| 12.016667 | 8870, 8872, 8874, 8876 | Initial defenders' service expires before reaching the target. The final unit snapshot distinguishes four late endpoint arrivals from zero arrivals during active service. |
| 24 | 25756 | Courier 2 physically hands report 0 to steward 1 at node 1, with its original time-zero observation unchanged. The pending arm receives the same report at 24h (event 25755). |
| 27.050556–27.367222 | 34220 through 34948, twenty donation events | Steward 1, acting as the same ordinary Person, contributes 20 wood. Unlike the short-route case, the courier is not this project's donor or worker. |
| 27.383333 | 34953 | Person 1 begins crossing construction. |
| 29.016667 | 38080–38081 | The Person selects `CancelAction`, with nutrition 25,140,000ms = 6.983333h, and reason `crossing investment paused: current local opportunity/physiology gate`. Construction stops after 1.633333 actual person-hours. |
| 29.033333 | 38120 | The same Person starts harvesting its previously established crop 4 at plot 1. |
| 30.041667 | 40285–40287 | Harvest produces 24 personally owned food quanta. Person 1 eats 18 through the existing exact food action and restores nutrition. |
| 30.05 | 40297, 40300 | The existing military-service evaluation passes; Person 1 volunteers. |
| 30.064667 | 40327–40328 | Person 1 physically assembles at muster for a new service term. The event explicitly says no remote orders were received. |
| 42.064667 | 56726 | That service term ends. |

The first interruption is inspectable in
[`infrastructure::consider`](../../../src/provisioning/infrastructure.rs): once
materials are present, it forecasts the next `min(remaining work, 1 nominal hour)`
using the coherent physical estimator and requires at least six hours of
nutrition afterward. From 6.983333h of nutrition even an unimpaired hour would
fall below that margin; the forecast also accounts for impairment during work.
Thus the existing cautious continuation gate necessarily fails at this state;
the trace does not establish it as the only failing predicate because the same
evaluator also checks remembered opportunity and profitability. The already
committed wood is sufficient, and this is not evidence of a new model defect. The generic
`Interrupted` text says `cancel invalid research route`, but the preceding
`Decision` records the actual crossing-physiology reason; the generic phrase is
not evidence of a missing route.

Later eating does not automatically resume the abandoned activity. The common
Person policy then selects voluntary service, and the late member is not given
the earlier protect order by telepathy. By 72h no crossing is complete: 20 wood
remain in the project and 6.366667 nominal work-hours remain. The trace explains
the first interruption and subsequent competing activity; it does not establish
that the crossing was globally impossible or that compulsory labor would be
justified.

## Authority and information remain separate

The factual report arrives at the same time in each pair: 5.084h for `alternate`
and `scarce`, 24h for `long`. Since the commission occurs at time zero, before
these receipts, it cannot be attributed to the report. Report 0 retains author 2,
observation time 0 and the local handoff provenance. None of these fields grants
command rights.

In alternate commissioned seed 0, steward 1 leaves settlement 1 at 6.033333h
(event 4592), returns at 15.201333h (12967), later uses the completed crossing
(events 14101 and 14513), and finally returns at 37.516667h (54657). The office
persists as an institutional record, while active authority ceases during absence.
In the pending arm the steward leaves at the same 6.033333h (4447), returns only
briefly at 15.179333h (13073), and departs again at 15.2h (13116). These location
histories account for active administration 6.054 → 43.165 person-hours, with no
new authority radius or remote power. The six-hour initial deliberation hold is
an explicit control shared by both arms and is not an endogenous administrative
cost. True authority measurements are research observations, not a globally
available ruler map.

## Household history and learned death

Persons 16 and 17 begin as dependents but mature at 24h in all runs. They retain
their identities and kinship. Metrics named `initial_dependents_*` track that
initial cohort for the whole run, including their adulthood; they are not a
claim that everyone in the cohort remains biologically a child at death.

In alternate commissioned seed 0, Person 16 dies of starvation at 52.216667h
(event 78584). Parent/caregiver 3 observes the death locally (78585), and the B2
appraisal (78586) generates grief using the existing attachment value 20. In
scarce commissioned seed 0, Person 17 dies at 32h (44557), but caregiver 4's death
observation occurs only at 54.703704h (80943); grief begins in the ensuing
appraisal (80944), 22.703704h after the death. This is ordinary knowledge-gated
B2 behavior inside the integrated world, not a military-casualty or child-death
script. The excerpt does not claim that this grief caused a particular later
choice without a separate controlled comparison.

Both arms retain actual military-duty and travel time. Zero starvation in the
`military_starvation_h` role-time ledger does not mean mobilization had no cost:
initial service ends at 12.016667h, while much later starvation is attributed to
civilian status. Initial-cohort ledgers and total exposure preserve those later
outcomes. This panel contains no no-mobilization arm, so neither the marginal
cost nor net physical benefit of security can be inferred by subtracting these
commission arms.
