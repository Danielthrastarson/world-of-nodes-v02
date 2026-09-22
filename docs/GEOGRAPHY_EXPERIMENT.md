# F1 registration: one policy, multiple possibility spaces

Proceed only after F0 is marked passed. No new economic, behavioral or information
system. All runs use adopted Current (corrected no-meal), 50 agents at C, n12h,
f4h, cap10, food total3000 created0/expiry48h, request600 at C, no source prior,
FIFO and the same 72h clock/physiology. All edges nominal2h/capacity4 except the
explicit bottleneck crossing (capacity1). No parameter search or per-map policy.

| Fixture | Connectivity | Food |
|---|---|---|
| line | C-X-F | F3000 |
| branch | C-X-F1, C-F2 | F1=1500, F2=1500 |
| alternative_routes | C-A-F, C-B-F | F3000 |
| dead_end | C-X-empty, C-F | F3000 |
| bottleneck | C-X-crossing-F | F3000; X-crossing cap1 |
| unequal_sources | C-X-far, C-near | far2700, near300 |

Seeds0–29 per map. These are counterfactual geographies, not changes to the locked
reference. Store parameters and per-node populations at36/48/72h, nourishment,
consumption, spoilage, gathering, queue times/overtakes, stale trips, jobs/deliveries,
and seed0 traces. Always audit materials/capacity/exclusivity at checkpoints.

Tests before matrix: unknown destination food relocation cannot change decisions
on unchanged personal observations; unknown exploration targets correspond to
remembered edges, not hidden food; memory changes only after local presence;
relabel nodes and reorder input edge IDs in unique-route maps; normalized physical
outcomes must match. In equal-cost branching maps, stable numeric tie-breaking may
choose another equally justified branch after renaming: exact realization symmetry
is not required, semantic food/commons IDs are forbidden.

Trace policy selected food-targets only after memory contains a resource observation.
Exploration may select an unknown node but cannot name its hidden resource as motive.
The same policy code must run every map, including depletion, empty branches and
congestion. Negative outcomes pass when physically/local-causally explained.
Do not adopt conditional meals or persistent pre-claim intentions in F1.
