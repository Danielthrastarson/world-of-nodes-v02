# Acceptance gate decomposition — registered design

Frozen control: delivery fixture (50 at C, request backlog 600 at C, all 3000
quanta at F, immutable t=0 creation/48h expiry, two 2h/capacity-four edges,
capacity ten, unchanged FIFO/physiology/execution/migration rules). Preserve
`research/provisioning/` and `research/delivery/` byte-for-byte.

Run seeds 0–29 for current, known-source-at-C with unchanged policy, meal-aware
forecast with 6h margin, and original no-meal forecast with 0h margin. If zero
margin adds at least one accepted job per run on average and increases acceptance
in at least 15 paired seeds, run exactly one 3h margin sensitivity. No combined
variant is initially authorized by this protocol. The optional purposeful-search
hypothesis will be considered only after these results; it is not a default change.

Known-source initial memory is an explicitly stipulated t=-1h belief: F has
3000 available quanta expiring at 48h and its X–F edge was observable with zero
queue. C–X is observed locally at t=0. No remote people/board records are injected.
This is not a simulated prehistory (actual creation remains t=0). The documented
route is necessary to distinguish known food from an unknown path to that food.

Preserve the exact legacy no-meal estimator for current and margin variants.
Zero margin allows positive predicted starvation only while completion is before
24 continuous starvation-hours. No predicted death is permitted.

Meal-aware forecasting uses only the same observations/memory plus own inventory.
It may predict legal meals at nodes, using integer quanta, actual ownership ceilings,
immutable expiry, occupied carrying capacity, individual gathering work, physiological
rate changes and remembered queue evidence. Predicted communal meals reduce cargo;
claims/reservations/acquisition are conditional, not guaranteed future events.
It does not introduce a new execution policy. Because a physical replay of meal
opportunities must also place estimated queue waiting before traversal and execute
one-quantum work, record a *read-only no-meals shadow* of that same replay. Report
any acceptance that this shadow would also permit: such a result would confound
meal attribution with estimator arithmetic. The shadow never controls agents.

Use separate denominators: every local observation of a positive board request;
every decision with a remembered request; and evaluations actually reaching job
priority. Higher-priority deferrals have explicit reasons and are not silently
counted as source/forecast rejections. For evaluated candidates record a primary
failure gate and all independent violations; exclusive primary-gate percentages
sum to 100% with passes. Preserve detailed route/meal forecasts in a compressed
journal, including observation timestamps and conditional quantities.

There is no hard source-memory age cutoff in P1. Do not invent one to populate a
metric; SOURCE_MEMORY_TOO_STALE is reported as an inactive gate. Score confidence
continues to decay with the existing 12h time constant.

Keep starvation reset unchanged. Record individual cumulative time at zero,
checkpoint nutrition distributions and continuous nourishment as well as survival.
The previously demonstrated two strategically timed quanta per stationary resident
can prolong technical survival with severe undernourishment; this physiology
question remains a separate later experiment.
