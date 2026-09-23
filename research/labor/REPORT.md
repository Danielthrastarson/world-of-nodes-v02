# F4 — pressure-dependent voluntary labor

Gate: PASSED over currently meaningful activities. No allocator, fixed occupations,
worker quotas or new institution was introduced. Wood/stone collection is deferred
until capital/construction gives it a causal use. This is not evidence of a complete
multi-industry economy. [Registered scope](../../docs/LABOR_EXPERIMENT.md).

## Controlled evidence

60 new runs, seeds0–29,168h, with all F3 mechanics/policy fixed. Initial F stock is
300 or6000; compare existing3000 control and C/F plot geography. These are research
scarcity interventions, never changes to the locked reference. Full parameters,
per-agent/checkpoint values, crop/work counters, traces and normalized activity in
`matrix`. Normalization sums actual living action-person-hours; mortality is reported
separately. Work hours count actual impaired/interruptible labor, not nominal recipes.

| Initial F food | First cultivation, seed0 | Sow+harvest person-hours 0–36h (mean) | Share of living time 0–36h | Crop quanta by168h | Alive168h | Starvation hours168h (mean) |
|---|---:|---:|---:|---:|---:|---:|
| 300 | 11.4833h | 58.641 | 3.258% | 2304 | 16 | 879.598 |
| 3000 | 30.3167h | 39.677 | 2.204% | 3360 | 28 | 694.504 |
| 6000 | 30.3167h | 39.677 | 2.204% | 3360 | 28 | 694.504 |

All30 seeds share the listed survival/production totals; tiny labor/physiology
variation and event identities are preserved, not presented as independent broad
world coverage. Action shares and absolute hours are both in `periods.csv`.

Scarcity reduces early gathering (0.184 vs0.871 person-hours) and prompts earlier
cultivation. Later total farm work falls partly because fewer people survive; that
is not evidence of weaker investment preference. In120–168h, surviving-agent time
in cultivation is11.354% in scarce vs10.930% in reference. Scarce work interruptions
average495.83 versus248.30; resumptions491.83 versus248.30. Churn remains visible.

Doubling stock from3000 to6000 has **zero measured effect** on labor allocation,
food consumption, crop production, survival or starvation across paired seeds.
Extra initial food does not fix travel/access or change the48h expiry deadline.
This negative dose-response finding is preserved. More food is not equivalent to
more accessible future nutrition.

Geography still matters: moving the same plots from F to C changes early farm
labor from39.677 to163.896 person-hours and early queue time from809.582 to891.548.
It ultimately leaves only4 survivors and38 inaccessible mature dead-owner crops.
The same selector can invest locally, explore, gather, return, cultivate, sleep or
fail; there are no map-specific occupation assignments.

## Causal checks and limits

Two added tests show: (1) the same agent/plot with scarce local food chooses sowing,
while accessible local stock prompts gathering; (2) changing hidden F quantity
cannot change any agent/event history before physical arrival. All13 farming/labor
tests pass, on top of the previously completed100-test full suite. Clippy passes.
No policy code was changed for F4; only tests and the registered stock knob/harness.
Accounting errors0; FIFO overtakes0;1038 prior research files unchanged.

Primitive actions rather than job titles define allocation. Instantaneous pickup,
eating, claiming and dropping consume no labor in the inherited kernel and therefore
are represented in causal events, not fake positive-duration work buckets. Communal
jobs/deliveries remain0 in these line arms. F3 branch arms retained some communal
work and poor outcomes; F4 does not claim generalized efficient task coordination.

Reproduce into a new directory:
```
cargo build --release --bin farming_probe
python3 scripts/run_labor.py research/labor/reproduction-NEW
```
Source `771afd6`; baseline source/report remain under `research/farming`.
Proceeding now reaches F5's [capital-rights choice](../../docs/CAPITAL_RIGHTS_DECISION.md).
No cart ownership/use assumption is adopted silently.
