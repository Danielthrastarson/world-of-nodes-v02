# F11B — physical unit formation and movement

**PASSED at organizational/physical movement scope.** One locally issued unit order can cause multiple existing Persons to move through the common physical scheduler. It does not guarantee assembly, receipt, departure, simultaneous arrival or completion before service expires. **F11A's protective/economic benefit gate remains OPEN. No combat is implemented.**

## Reproducibility and controls

Implementation revision `78ac3c1e39b24432b0714b9fee1a9b2e39824882`. Registration `aae06e0`, with documented pre-panel fixture/measurement corrections. [Specification](../../docs/FORMATION_EXPERIMENT.md), [architecture](../../docs/UNIT_ARCHITECTURE.md), [binary/source provenance](matrix/provenance.json).

16 cases × Move/hold-at-muster ×30 paired seeds0–29 = **960 final runs**. Each pair has identical authoritative pre-command state hashes. Observe24h after command; late/partial commands occur at2h15m, constructed infrastructure/cart at5h10m, others1h. The hold arm retains voluntary service and receives no movement order; this estimates the additional effect of movement, not the entire mobilization cost.

Four original prospective recruits have personal physical food. Initial nutrition20h (24h late/partial/cart/crossing) and12h voluntary terms are explicit movement-fixture parameters, not changes to F11A or default physiology. The rejected18h fixture failed the existing six-hour acceptance margin after one policy minute and is documented, not tuned into a default. Stress cases initialize one member's nutrition/fatigue or one child's observed need at command time identically in both arms; they do not claim these conditions arose from recruitment. Subsequent execution is autonomous. No per-soldier march is injected. Known two-edge routes use disclosed timestamped prior edge observations. Unknown-route controls omit them.

Cart/crossing preparation uses real material and construction actions/time, with labor injected only for this controlled preparation. It is not evidence of autonomous investment induced by military demand. Paths use explicit prior civilian traffic state. No new speed, supply, capacity or access rights belong to a military unit.

## Results

Means per run; times measured from order issuance. Full [table](analysis/TABLE.md), [paired metrics and ranges](analysis/paired.json), [event excerpts](analysis/TRACE_EXCERPTS.md). Most small deterministic cases have identical physical results across seeds;30 repetitions are not30 distinct environments. Congestion and shared-route civilian behavior have seed variation.

| Case | Order recipients | Departed | Arrived during service | First–last arrival (h) | Causal result |
|---|---:|---:|---:|---|---|
| Short | 4 | 4 | 4 | 2.017–2.017 | One order, four actual traversals |
| Two-edge route | 4 | 4 | 4 | 4.017–4.017 | Remembered route, two physical legs |
| Unknown route | 4 | 0 | 0 | — | A destination does not create route knowledge |
| Opposing civilian congestion | 4 | 3 | 3 | 3.500–7.556 | Fourth member runs out of service time while queued |
| Long march | 4 | 4 | 0 | — | Term expires mid-edge; all later reach endpoint as released people |
| Food stress | 4 | 3 | 3 | 2.017–2.017 | Critically hungry member pauses; no suicidal obedience |
| Fatigue stress | 4 | 4 | 4 | 2.017–9.033 | One member recovers7h, retains order and resumes |
| Capacity-one bottleneck | 4 | 4 | 4 | 2.017–8.130 | Common FIFO spreads arrivals by6.113h |
| Path on | 4 | 4 | 4 | 1.817–1.817 |12 minutes faster than path-off through existing F7 physics |
| Partial muster | 2 | 2 | 2 | 2.017–2.017 | Four enlist; only two physically assemble/receive |
| Late arrivals | 2 | 2 | 2 | 2.017–2.017 | Two later assemble but never receive departed unit's order |
| Care conflict | 4 | 4 | 4 | 2.017–2.033 | Gift delays caregiver one minute; obligation/property remain real |
| Shared civilian route | 4 | 4 | 4 | 6.000–6.000 | Prior civilians delay unit; unit also imposes external queue cost |
| Constructed crossing | 4 | 4 | 4 | 2.000–8.922 | Public crossing remains capacity-one; no army exception |
| Owned cart | 4 | 4 | 4 | 2.000–2.000 | Owner's constructed cart moves physically with its lawful user |

Path-off matches the short-move timing. Crossing has5 enlisted roster entries but4 assembled: the builder also volunteers and existing local admission refuses the extra person. This is preserved rather than calling all five soldiers. In the long case release occurs11.017h after command and the old endpoint is reached12.907h after command. Those four endpoint arrivals are **not active-service completions**. Late arrivals may independently visit that node after service; they are not retroactively counted as commanded soldiers.

## Congestion, interruption and external cost

In the congestion fixture, original civilian queue exposure rises **57.173→81.984 person-hours**, a mean paired increase **24.811h**. Civilian starvation exposure rises **90.313→123.004 person-hours**, **+32.691h**. Total food consumption falls **217.533→116 quanta**. Active soldiers consume24quanta in both arms and accumulate no starvation during service. There are no deaths within this24h measurement window; absence of deaths does not imply adequate nourishment.

The [fourth member's seed0 trace](trace-audit/FOCAL_QUEUE_TRACE.md) is informative. It queues at absolute1h01m. At8h01m the existing eating priority consumes six personally owned quanta and interrupts the queue action. The movement undertaking and membership persist. At8h02m it rejoins the same shared FIFO behind other real applicants. Service ends12h01m before admission. This is preserved queue/interruption behavior, not forgetting membership and not a new priority lane. The report does not attribute all missed movement to road capacity alone: eating, queue reentry and the finite service term interact.

The shared-route case adds **1.200 original civilian queue-hours** and reduces original civilian farming by **0.0220h** (6.1031→6.0810h). Both arms produce zero harvested food within the observation window; production is not inferred from work. The public crossing adds **2.939 civilian queue-hours** and spreads soldiers' arrivals by6.922h. Geography and civilian movement remain common constraints.

The food-stress member remains in the roster with the remembered order but does not depart. Its total19starvation-hours match hold;6.017 occur while still serving in Move, versus none classified as active service in hold because the prior F11A policy withdraws earlier. This is a **role-classification change, not new total starvation**. The caregiver transfers six owned food quanta in both arms. It retains service during the care pause only in Move, then arrives one minute behind peers. Total caregiver/dependent starvation remains5/18hours respectively in both arms. No automatic substitute caregiver, sharing or guaranteed rescue is added.

## Cohesion and information

The roster, organizational objective and research snapshot are never global NPC knowledge. Each member reads only its own received order and observations/memories. An unknown route causes an explicit pause. A later locally issued order does not update an absent recovering member (unit test).

[Timelines](matrix/fatigue_stress_move/timeline.csv) separate active, moving, queued, recovering, no-order, target arrivals and off-cluster members. Node-only concentration is zero during shared edge travel. Exact coincident along-edge progress is therefore also measured: four synchronized travelers have coincident fraction1 and separated count0. A resting straggler yields a different physical cluster. Coarse endpoint-hop separation is only a graph lower bound; no continuous battlefield spacing is claimed.15-minute snapshot peaks can miss shorter events; event timestamps and queue integrals remain exact. Ratios are undefined with no active members. Fatigue summaries use sampled kernel state; raw position/physiology traces are preserved.

[Seed0 per-edge admissions/completions/occupied-slot hours](trace-audit/edge-use-seed0.csv) reconstruct infrastructure use from physical admission/arrival events, including travel that crosses the command boundary. These selected traces supplement, rather than replace, the30-run queue/cohort metrics.

There is no arbitrary cohesion scalar, combat bonus, group body/inventory, army food, speed override, requisition or remote commander. Graph gathering is the only formation representation. Line/column/front geometry remains deferred.

## Gate evidence and preservation

1. Organizational Units reference persistent Persons; identity, B3 biology, B1 obligations, B2 histories and property remain on those Persons.
2. Enlisted/assembled/active/released/dead are separate; partial and late muster validate the distinction.
3. One local order produces many individual actions; no per-member movement commands are injected.
4. All travel uses existing edges, admission, impairment, paths and crossings. Zero FIFO overtakes.
5. Fatigue, care, congestion and missed orders create observable separation without snapping/catch-up speed.
6. Hunger can stop departure; recovery can delay and then resume it.
7. Paired civilian costs establish shared transport competition.
8. Path/crossing/cart tests use existing physical laws and rights.
9. Death uses ordinary Person death and preserves ownership/roster history. Birth creates an ordinary Person with no inherited unit role/order. Observer sampling does not alter execution.
10. No combat strength, safety/protection bonus or alternate military movement layer exists.

**267 tests pass**, formatting and warning-clean Clippy pass. [Historical validation](regression/validation.json): **137 exact historical replay executions**, including24 F11A arms, and **9,298 prior research files unchanged** by SHA256. Authoritative historical physical outcomes, state/event hashes and raw traces match. Optional `formation:false` changes Config debug schema; older B2 derived debug signatures are excluded by the existing documented verifier, not authoritative hashes. Runtime measurements are nondeterministic. Prior artifacts are never overwritten.

Failed fixture/test-development logs and the32-run smoke panel remain preserved separately; they are not pooled into960 final runs. The smoke separation label was replaced by the physically correct coincident-position metric before the final panel. No post-panel behavioral or physical tuning was performed. The authorized organizational commitment is an explicit new role behavior, documented separately from unchanged civilian priorities.

## Next bounded question

F11C begins with the [concrete threat/control specification](../../docs/THREAT_EXPERIMENT.md). It must first establish actual undefended harm, then test whether physically present defenders prevent it at a measurable cost. Combat is not implemented in this increment. F11A's full protective-benefit gate remains open; orderly movement and army existence do not close it.
