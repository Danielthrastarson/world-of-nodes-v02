# F6 — shared crossings: gate passed

A communal crossing can now be built from physically transported, voluntarily
donated materials and local labor. It changes the actual movement graph and is
used by multiple autonomous agents. The same policy also leaves projects unfunded,
unfinished or unattempted. Public use does not eliminate congestion or starvation.
No previous negative result was replaced or tuned away.

The future [player action language](../../docs/PLAYER_ACTION_LANGUAGE.md) is
recorded separately: OBSERVE, DECLARE, COMMISSION, ALLOCATE, APPOINT, NEGOTIATE,
MOBILIZE and REVOKE. It is conceptual documentation, not a ruler interface or
permission to directly manipulate agents/resources. F6 project identities and
material ledgers can later be targets of a lawful commission.

## Physical and institutional model

The provisional recipe is **20 wood + 8 nominal work-hours**. A stable project at
an edge endpoint records donated wood, remaining work, donors, laborers, finishing
builder, completion time and owner. On completion that physical project becomes
the fixed crossing asset. Its owner is **Commons(0)**; it is not the builder.
All experiment agents are members. Outsiders, tolls, land title, enforcement,
inheritance and private concessions remain unmodeled.

A local contribution removes personally owned carried wood and explicitly embeds
it as communal project material. It cannot take another person's property, exceed
the recipe, or occur remotely. Inputs remain accounted for before and after
completion. A donor's death does not reclaim already donated communal materials;
undonated personal wood retains its original owner. This is the newly authorized
pooling mechanism, not a retrospective change to F5 carts or dead-owner rights.

There is one physical work position per project. Workers can interrupt or replace
one another while preserving progress. Physiology controls the actual work rate.
No worker assignments or occupation quotas exist. Completed crossings either open
a previously blocked edge at its registered capacity, or reduce its nominal
traversal work by 25%. Existing FIFO and shared directional capacity still apply.
No economic multiplier, free goods, transport teleportation or private priority.

An in-flight traveler retains the edge work requirement recorded at admission.
Completion affects subsequent entrants; it cannot instantly finish an existing
trip. Adjacent observations expose effective edge state; remote memories receive
no completion broadcast. All movement, work and observation share one clock.

The new investment rule is a disclosed bounded opportunity heuristic: compare
remembered food access and remembered hauling opportunities with/without a known
crossing improvement. Opening access to remembered edible food can motivate work;
otherwise estimated repeated-trip savings must exceed remaining construction work.
No unknown resource or benefit from unseen travelers is credited. Existing critical
hunger/sleep override. Construction is reconsidered against a one-hour work budget
using the existing mechanical forecast and six-hour nutrition margin.

This heuristic is not a complete supply/coordination optimizer. It does not reserve
future wood, guarantee other contributions, predict future queues, or include an
exact multi-person material-acquisition schedule. It can commit some inputs even
when the project ultimately lacks enough material. Registered static tests check
work mechanics; they do not certify the lifetime investment valuation as optimal.

## Paired design and provenance

Original six cases: revision `2df5a5b`, kernel `24788f7`, **360 runs**.
A separately documented absent-demand control, revision `a84b790`, adds **60 runs**.
Total **420 runs**, seeds 0–29 per arm, 72h. The coverage addition followed the
observation that the short improvement was actually used; no original case,
parameter or policy was changed to obtain a negative result.

Common conditions: 50 agents at C, nutrition12h/fatigue4h, backpack10, no carts or
farming, 3000 finite food quanta created0 and expiring48h, communal request600 at C.
Normally X holds300 food and F holds2700; wood100 at C. C-X is .25h/capacity4.
The worksite is X, so construction wood must actually cross C-X. X-F is a blocked
.25h crossing except in the already-open short case. Alternate adds X-B-F, each
2h/capacity4. Bottleneck opens X-F at capacity1 rather than4. Scarce has5 wood.

All but discovery have explicit initial remembered observations of food, graph,
wood and project opportunities at t=0, identically in both arms. These are research
information controls. Discovery has no inherited remote source/site knowledge.
Local-food moves all3000 food to X and leaves nothing useful beyond the crossing.
See [registered parameters](../../docs/INFRASTRUCTURE_EXPERIMENT.md).

All reported endpoint totals are identical across the 30 seeds within each case.
Pairing verifies reproducibility here; it does not establish general stochastic
robustness. Per-run parameters, hashes and checkpoints are retained.

## Outcomes

Values are per run: construction unavailable → available. Food is in quanta;
starvation means person-hours at zero nutrition, not just deaths.

| World | Completed | Food consumed | Starvation hours | Alive72h | Distinct agents reaching F |
|---|---:|---:|---:|---:|---:|
| Closed crossing | 0 → 1 | 300 → 2417 | 1200 → 583 | 0 → 50 | 0 → 48 |
| Already-open short edge | 0 → 1 | 2329 → 2450 | 671 → 550 | 50 → 50 | 50 → 50 |
| Capacity-one opening | 0 → 1 | 300 → 1873 | 1200 → 819 | 0 → 41 | 0 → 41 |
| Alternative long route | 0 → 1 | 1196 → 2413 | 1100 → 587 | 24 → 50 | 24 → 50 |
| Insufficient wood | 0 → 0 | 300 → 300 | 1200 → 1200 | 0 → 0 | 0 → 0 |
| No prior source knowledge | 0 → 0 | 300 → 300 | 1200 → 1200 | 0 → 0 | 0 → 0 |
| All food already at X | 0 → 0 | 2376 → 2376 | 624 → 624 | 50 → 50 | 0 → 0 |

The new crossing is beneficial in the closed-access world. The no-demand world
needs no crossing and attracts no investment. A separate deterministic forced
construction test demonstrates the unused asset's 20-wood/at-least-8h cost with
zero traffic benefit. It is explicitly a physical control, not autonomous behavior.
No autonomous completed crossing in these tested cases reduced aggregate food
consumption; no such harm is claimed. Congestion and unfinished investment costs
remain observable harms. F5's worse-nourishment and dead-capital findings stand.

All unconsumed food spoils at48h. Waste in the completed worlds is583,550,1127,587
quanta respectively. Alive72h does not establish continuous nourishment or renewable
sustainability. The unchanged starvation-reset/grace mechanics still matter.

## Construction, use and causal traces

| Available case | First donation | Completion | Labor hours | Work starts | Improved completed trips / users | Food departure quanta on completed improved trips |
|---|---:|---:|---:|---:|---:|---:|
| Closed | .250556h | 9.425926h | 8.142593 | 2 | 363 / 48 | 1113 |
| Short | 16.433333h | 36.806944h | 12.004167 | 14 | 146 / 49 | 712 |
| Bottleneck | .250556h | 9.425926h | 8.142593 | 2 | 89 / 41 | 192 |
| Alternate | .250556h | 9.425926h | 8.142593 | 2 | 398 / 50 | 1144 |

Each completed crossing embeds20 wood. `attempts` counts all work-intent submissions,
including continuing/rejected attempts; `work_starts` counts new accepted starts.
The short case uses ten distinct laborers across fourteen starts. The other
completed cases use two laborers. Full donor/labor maps are in `projects.tsv.gz`.
Trips count edge traversals, not unique goods; food columns measure departure loads
on completed crossings, not edible arrival/consumption. Unique consumption is
reported separately. Material flow includes repeat carriage of undonated personal
wood; only20 becomes embedded in the asset.

Closed seed0 chain, exact event IDs in the retained tape:

1. Agents gather personally owned wood at C and carry it across the .25h edge.
2. Agent0 donates at t=.250556h. Twenty agents ultimately donate one unit each.
3. Event3802: agent0 begins labor at1.266667h.
4. Event16278: agent1 takes over at5.033333h after agent0's interruption.
5. Event29574: agent1 completes at9.425926h. Owner remains Commons(0).
6. Local observation exposes the open edge;48 independently acting members cross.
   By72h41 living agents are at F,7 at C,2 at X. Migration was not forbidden.

The short case's later, impaired/interrupted work demonstrates that nominal labor
and elapsed completion time are different. Its improvement was not unnecessary:
known traffic conditions and repeated demand eventually justified investment under
the heuristic. That unexpected result was preserved, not renamed or rebalanced.

Scarce case: five wood are physically carried and donated; fifteen are missing.
No labor or completion occurs; all50 eventually die. The five communal inputs
remain represented. Discovery: agents learn the project but never observe F's food,
so the conditional benefit never exists in their memories; no investment occurs.
The invisible food does not secretly motivate construction. Local-food: the
crossing offers no known food-access advantage; both arms produce identical
physical outcomes with zero materials or labor committed.

## Geography and public-capacity externalities

Capacity-four and capacity-one worlds construct at the same time with the same
cost. The subsequent constraint produces materially different outcomes:

| Opened crossing | Completed trips | Mean admitted queue spell | Maximum admitted queue spell | Deaths while queued there |
|---|---:|---:|---:|---:|
| Capacity4 | 363 | 1.287h | 4.675h | 0 |
| Capacity1 | 89 | 5.761h | 23.750h | 7 |

There are nine total deaths in the capacity-one world; seven occur in the crossing
queue. Waiting statistics above condition on admission and exclude abandoned/fatal
queue spells; total queue exposure includes those spells. Peak reconstructed queue
length is47 in both cases. No urgent-agent or loaded-agent priority was introduced.

Total all-edge queue exposure increases from150.724 to1147.872 person-hours when
the closed world opens. This does not mean the closed world offered good transport:
it had no remote access and all agents died. The short improvement also increases
queue exposure from1128.818 to1220.551h while increasing consumption. More attractive
movement changes demand for shared capacity; lower nominal time does not guarantee
lower aggregate waiting.

Alternate-route world: without construction, each long detour edge records36
admissions. With construction, each records16, while the shortcut records398.
Four final detour admissions finish the route after the shortcut opens; agents on
remote edges do not teleport or receive a universal route update. Route reversal
counts and per-edge traffic phases are retained in `matrix/analysis.json`, not
interpreted as proof of optimal route selection.

No broad settlement layout or persistent political authority is claimed. This is
a fixed crossing chosen at a physically eligible site; same policy operates on
line, bottleneck and alternate-route geography. Future multiple-site settlement
formation remains a separate extension.

## Validation and preservation

Final suite: **124 tests passed** (111 library,2 transport-binary,1 research-controller,
10 integration), with clippy warnings denied. Eleven F6 tests cover pooled materials,
separate builder/owner/users, local owned donations, no overfill, work exclusivity,
interruption/death, physical wood transport, admission-time movement contracts,
shared FIFO, stale remote memory, hidden remote food, renamed node identifiers,
partition-independent clock and unused-capital cost. Nine static physiology
combinations compare construction timing/nutrition/fatigue against the existing
mechanical estimator exactly.

Every experimental run audits conservation, capacity, exclusivity and FIFO; zero
accounting error and zero overtakes. Nine historical endpoint regressions and
19 adoption-equivalence scenarios remain identical. Seed-zero F3 farming and F5
capital tables, including state/event hashes, are unchanged. SHA-256 checks confirm
**1219 pre-existing research artifacts unchanged**. F5 cart property rules, stranded
assets, starvation rules and all prior negative results are preserved.

The initial syntax-check failure is retained in `initial-check.txt`; it was fixed
before any experiment. No live outcome-triggered policy change or physical tuning
was made. Only additional tests and the separately registered no-demand control
followed the original matrix.

Reproduce into new directories:

```sh
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release --bin infrastructure_probe
python3 scripts/run_infrastructure.py /tmp/won-f6-matrix
python3 scripts/run_infrastructure.py /tmp/won-f6-no-demand local_food
python3 scripts/analyze_infrastructure.py /tmp/won-f6-matrix
python3 scripts/analyze_infrastructure.py /tmp/won-f6-no-demand
```

`matrix` and `no-demand` each retain provenance, parameters, per-agent endpoints,
36/48/72h checkpoints, project ledgers, traffic, summary/pair tables and seed-zero
causal events. Traces and project records are compressed losslessly. The analysis
script is research-only and cannot supply information to agents.

## Gate conclusion

F6 passes all eight requested criteria: real shared construction, real inputs/work,
changed connectivity/duration, multiple autonomous users, movement-mediated effects,
geography-sensitive behavior, shared-capacity externalities, and intact invariants.
The test establishes a causal shared-infrastructure capability under an explicit
institution. It does not establish universally wise investment or that unknown
resources will motivate construction. No ruler interface, tolls, new communication,
F7 traffic-generated roads or higher systems were implemented during F6.
