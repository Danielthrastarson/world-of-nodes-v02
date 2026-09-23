# F7 registered civilian path-feedback experiment

Scope: use-created treaded terrain on explicitly eligible existing ground edges.
No armies, military movement layer, ruler commands, new access rights, traffic
priority, stone paving, road workers or maintenance institution. Existing open
access is unchanged; a surface condition does not convey land ownership.

## Minimal causal law

Each eligible edge has stable edge identity, cumulative completed passages and
integer compaction0..100. One living person's completed traversal adds one
compaction unit (saturating at100). Queuing, mere admission, idle agents and goods
alone do not create paths. A cart's pulling person counts once; no separate wheel
physics is assumed. Partial travel ending in death is not credited in this coarse
whole-edge model. This limitation is explicit, not a continuous footprint model.

After six simulated hours without a passage, compaction relaxes by one; repeat
every six hours until bare or used again. Passage refreshes that timer. This is a
provisional passive surface-recovery law, not a weather/ecology subsystem. Continued
use physically maintains the path; staffed maintenance and durable engineered roads
are deferred. No commodity is created or deleted by a surface-state transition.

Effective nominal traversal work = ceil(base work × (500-compaction)/500).
At maximum compaction the reduction is20%. Stage labels are only descriptive:
0 bare;1..49 trail;50..100 worn path. They do not add bonuses. Capacity never changes.
Eligible edges cannot be blocked crossings or F6 project edges; traffic cannot
build a bridge. Public crossings remain separate constructed assets.

At admission, snapshot the actual movement work for that traversal. Subsequent
use/recovery changes later admissions, not in-flight progress. Adjacent observations
see current effective edge duration; remote memories remain stale. Policies and
risk thresholds remain unchanged. Forecasts are conditional on remembered edge
costs, not future traffic or surface recovery.

## Registered paired tests

30 identical seeds0..29 per enabled/frozen-surface arm;72h;50 agents;3000 finite
food created0/expiry48h;nutrition12/fatigue4;backpack10;request600C. Frozen control
records actual traffic but never changes surface/cost. Except specified below,
source knowledge initially absent and all startC. No tuning after outcomes.

- line: C-X-F, each edge.5h/capacity4;foodF; both eligible.
- diamond: C-A-F and C-B-F, each edge.5h/capacity4;foodF; all eligible.
- detour: same diamond, route viaA .5h/capacity1 each, viaB .625h/capacity4;
  all eligible. Tests feedback against congestion, not a desired route optimum.
- dead_end: C-X-F plus C-D empty, all.5h/capacity4;ground edges eligible.
- local_food: line with allfoodC and explicit prior observations of empty remote
  nodes; tests no use/no path.
- crossing: preserved F6 closed fixture: C-X.25h/cap4, X-F blocked until20wood/8h
  project opens.25h/cap4;wood100C,food300X/2700F, prior memories as F6. Only C-X
  is eligible ground. Tests common transport/clock with construction.
- carts: preserved F5 medium-high fixture (.5h edges,600request,500woodC,
  prior source memories, personal capital enabled), both ground edges eligible.

Measure state transitions/recovery, traffic by edge and direction, route shares,
queue/occupancy/realized travel, conditional same-trip work reduction, food flow,
deliveries, consumption/spoilage, migration, starvation and deaths. Separate actual
paired outcomes from a matched-trip physical capability comparison. Preserve seeds,
state/event hashes, full seed-zero traces, parameters and historical checksums.

Gate: actual movement changes geography and subsequent movement/route decisions;
unused surfaces do not improve; disuse recovery obeys one clock; same bounded policy
works across topology; congestion and negative effects remain explainable; no
conservation, locality, FIFO, ownership or historical regressions. This establishes
treaded paths/use maintenance, not full engineered roads or military gameplay.
