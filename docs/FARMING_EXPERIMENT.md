# F3 personal renewable production — registered 2026-09-23

Owner chose cultivator ownership: claim begins when sowing starts; harvest inherits
it; active process is exclusive, underlying land never becomes owned. Death never
reassigns a crop. No communal crop jobs or appropriation. Existing voluntary food
transfer mechanisms are unchanged. Rights are provisional for F3.

## Mechanics registered before implementation/results

One suitable unit holds one crop. Sow: 1 nominal labor-hour, interruptible with
physical progress retained. Once sowing completes, grow on the authoritative clock.
Harvest: 1 nominal labor-hour, interruptible with progress retained. Work uses the
existing hunger/fatigue performance function. Finished harvest produces the whole
crop as personal loose food at the plot (not in a magically enlarged backpack).
The freed unit has no owner. Food created at harvest expires 48 hours later;
splitting/handling never changes that time. Mature unharvested crops persist in
this minimal model; no crop rot/weather is modeled. This is a disclosed abstraction,
not preservation of already harvested food. Death can therefore lock a unit forever.
No crop material is edible before harvest. The production ledger explicitly counts
new food from unmodeled environmental inputs; this is an open agricultural system,
not a claim of conservation of unmodeled water/carbon/soil. Exact food accounting is
initial + produced = physical food + waste + consumed. Growth alone produces no food.

Registered parameter sets (no tuning after results):

| Model | Sow | Grow | Harvest | Yield quanta |
|---|---:|---:|---:|---:|
| base | 1h | 12h | 1h | 24 |
| slow | 1h | 36h | 1h | 24 |
| low_yield | 1h | 12h | 1h | 12 |

## Bounded decision extension

Only locally observed plots/crops enter timestamped personal farming memory.
Existing immediate eating, critical food seeking, fatigue and accepted delivery
jobs retain precedence. A personally owned unfinished/mature crop can motivate
work/return through remembered routes. No generic committed-intention arm is adopted.
An agent considers new cultivation when known personal nourishment covers less
than one production lead time plus its existing six-hour margin, it has no known
active crop, and frozen-belief work/growth/harvest predicts reaching harvest alive.
This is a disclosed prospective investment heuristic, not learning or optimality.
Available equivalent plots use the private seeded RNG. No prescribed worker count.
No exact future queues or remote crop updates. Ordinary food seeking may still
interrupt investment and migration may strand it. Missing crop knowledge is valid.

## Paired experiments

Seeds 0–29, 50 agents, original regional line, unchanged 3000 initial quanta at F,
48h initial shelf life, capacity 10, 2h edges / capacity 4, initial nutrition 12h,
fatigue 4h, communal rules and no inherited source memory. No preloaded request.
Run to 168h; checkpoints 36/48/72/120/168h. Compare no farming vs three models,
with 50 suitable units either at C or at F. Plot count is land access, not assigned
workers. Extra branch topology repeats base C/F placements with same policy.
Register no-work and no-land kernel controls, a physical execution witness, and
ownership/death/locality/exclusivity/accounting/time partition tests. No parameter
search. Optional discovered technical defects get separate preserved failure logs.

Measure attempts, starts, incomplete/stranded/dead-owner crops (do not conflate
interruption with abandonment), maturity, harvest, output, output consumption and
spoilage, labor hours, interruptions/resumptions, starvation, deaths, migration,
queue time, inaccessible mature crops, material error and FIFO overtakes. Preserve
sample causal traces, per-agent/checkpoint data, source revision and hashes.
F3 gate is causal production and explainable nourishment beyond original stock,
not indefinite sustainability or universal survival.
