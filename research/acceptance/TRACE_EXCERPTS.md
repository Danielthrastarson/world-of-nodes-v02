# Acceptance traces

Times are simulated hours. Nodes: C=0, X=1, F=2.
Full journals retain every evaluated and deferred decision, not just these excerpts.

## current

- 0.000000h, agent 0, Node(0), nutrition 12.000000h: **NO_KNOWN_SOURCE**; explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=36000000 arrival_starvation_ms=0

## known_source

- 0.000000h, agent 0, Node(0), nutrition 12.000000h: **POST_PLAN_NUTRITION_MARGIN**; seek remembered food; source=2 observed_at=-3600000 believed_quantity=3000 expires_at=172800000; predicted_ms=14402000 arrival_nutrition_ms=28798000 arrival_starvation_ms=0

## meal_aware

- 0.000000h, agent 0, Node(0), nutrition 12.000000h: **NO_KNOWN_SOURCE**; explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=36000000 arrival_starvation_ms=0
- 5.033333h, agent 0, Node(2), nutrition 17.966667h: **PASS**; visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=49866666 arrival_starvation_ms=0
- 5.050556h, agent 0, Node(2), nutrition 17.949444h: **PASS**; visit remembered board to claim locally; predicted_ms=14820223 arrival_nutrition_ms=49797777 arrival_starvation_ms=0
- 5.067222h, agent 0, Node(2), nutrition 17.932778h: **PASS**; visit remembered board to claim locally; predicted_ms=14826889 arrival_nutrition_ms=49731111 arrival_starvation_ms=0
- 5.083889h, agent 0, Node(2), nutrition 17.916111h: **PASS**; visit remembered board to claim locally; predicted_ms=14833556 arrival_nutrition_ms=49664444 arrival_starvation_ms=0
- 5.100556h, agent 0, Node(2), nutrition 17.899444h: **PASS**; visit remembered board to claim locally; predicted_ms=14840223 arrival_nutrition_ms=49597777 arrival_starvation_ms=0
- 5.117222h, agent 0, Node(2), nutrition 17.882778h: **PASS**; visit remembered board to claim locally; predicted_ms=14846889 arrival_nutrition_ms=49531111 arrival_starvation_ms=0
- 5.133889h, agent 0, Node(2), nutrition 17.866111h: **PASS**; visit remembered board to claim locally; predicted_ms=14853556 arrival_nutrition_ms=49464444 arrival_starvation_ms=0
- 8.000000h, agent 0, Node(1), nutrition 15.000000h: **POST_PLAN_NUTRITION_MARGIN**; no locally justified work; remaining near resources is allowed
- 8.016667h, agent 0, Node(1), nutrition 14.983333h: **PASS**; visit remembered board to claim locally; predicted_ms=8000000 arrival_nutrition_ms=45940000 arrival_starvation_ms=0
- 8.033333h, agent 0, Node(1), nutrition 14.966667h: **POST_PLAN_NUTRITION_MARGIN**; no locally justified work; remaining near resources is allowed
- 11.016667h, agent 0, Node(1), nutrition 11.983333h: **OTHER_EXPLICIT_REASON**; locally accessible food; exact ownership/ration ceiling
- 11.033333h, agent 0, Node(1), nutrition 17.966667h: **INSUFFICIENT_EXPECTED_QUANTITY**; seek remembered food; source=2 observed_at=21600000 believed_quantity=2932 expires_at=172800000; predicted_ms=16106667 arrival_nutrition_ms=48573333 arrival_starvation_ms=0
- 11.050000h, agent 0, Node(1), nutrition 17.950000h: **POST_PLAN_NUTRITION_MARGIN**; remain FIFO; seek remembered food; source=2 observed_at=21600000 believed_quantity=2932 expires_at=172800000; predicted_ms=16118667 arrival_starvation_ms=0

## zero_margin

- 0.000000h, agent 0, Node(0), nutrition 12.000000h: **NO_KNOWN_SOURCE**; explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=36000000 arrival_starvation_ms=0
- 4.083333h, agent 0, Node(2), nutrition 12.916667h: **PASS**; visit remembered board to claim locally; predicted_ms=14433334 arrival_nutrition_ms=32066666 arrival_starvation_ms=0
- 4.100000h, agent 0, Node(2), nutrition 12.900000h: **PASS**; remain FIFO; visit remembered board to claim locally; predicted_ms=14440000 arrival_starvation_ms=0
- 5.033333h, agent 0, Node(2), nutrition 12.966667h: **PASS**; visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=31866666 arrival_starvation_ms=0
- 5.050000h, agent 0, Node(2), nutrition 12.950000h: **PASS**; remain FIFO; visit remembered board to claim locally; predicted_ms=14820000 arrival_starvation_ms=0
- 8.000000h, agent 0, Node(1), nutrition 10.000000h: **OTHER_EXPLICIT_REASON**; seek remembered food; source=2 observed_at=21600000 believed_quantity=2976 expires_at=172800000; predicted_ms=15203334 arrival_nutrition_ms=20796666 arrival_starvation_ms=0
- 8.016667h, agent 0, Node(1), nutrition 9.983333h: **OTHER_EXPLICIT_REASON**; remain FIFO; seek remembered food; source=2 observed_at=21600000 believed_quantity=2976 expires_at=172800000; predicted_ms=8002223 arrival_starvation_ms=0

