# F4 — dynamic labor without occupations (registered after F3, 2026-09-23)

F3 passed. This increment tests the existing voluntary activity selector; it does
not add a central allocator or extra economic systems. Currently useful labor is
personal cultivation, finite-food gathering and communal carrying. Wood and stone
have no consuming process yet: inventing jobs to collect them would be decorative.
F4 therefore establishes allocation over present meaningful activities, and defers
wood/stone demand until an authorized consumer such as F5 capital exists.

Reuse F3 line/F base as the 3000-quanta control (30 seeds,168h). Run two paired
initial scarcity interventions:300 and6000 quanta at the same F, with all other
parameters unchanged. These are new research worlds, not changes to the locked
reference. Same50agents,50plots,2h/cap4route,capacity10,growth12h,yield24,personal
rights,48hinitial/harvestfoodlife. Seeds0–29. No policy change or outcome calibration.
A third comparison reuses base C versus F geography from F3. Measure action labor
allocation (sow,harvest,gather,travel,queue,sleep,idle) over0–36,36–48,48–72,72–120,
120–168h. Normalize by recorded living agent time; report mortality separately so
work lost to death is not mistaken for a change in preference. Report crops, food,
interruption/resumption and refusal in the same arms. Use causal traces, not only
endpoint totals. Action durations represent allocation, not predefined occupations.

Tests: changing local accessible food changes cultivation vs gathering choice;
changing hidden remote quantity cannot change decisions before arrival/observation;
identical personal history remains deterministic. Record all failures.

Gate: known shortages and geography change voluntary labor with one policy, no
worker quotas or omniscient dispatcher, and observable churn. Does not require
optimal ratios, cooperative work, efficient allocation, or every material being
useful already. Scope is explicit rather than adding artificial wood/stone demand.
