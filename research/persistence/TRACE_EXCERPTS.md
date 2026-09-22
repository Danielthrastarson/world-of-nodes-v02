# Temporal-consistency traces

## reactive

- 5.033333h · agent 0 · plan 0 · INTENT_CREATED · nutrition 17.966667h · fatigue 9.033333h: VOLUNTARY_BOARD_DECISION
- 5.033333h · agent 0 · plan  · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 9.033333h: edge=1 from=2 to=1 target=0 food=false previous_plan=None selected_at=7200000 new_information=true changes={"NEW_NODE:2"} decision=Decision { intent: Travel { edge: 1, target: 0, food: false }, reason: "visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=49866666 arrival_starvation_ms=0" }
- 5.050000h · agent 0 · plan 0 · INTENT_CANCELLED · nutrition 17.950000h · fatigue 9.050000h: changes={}
- 5.050556h · agent 0 · plan 1 · INTENT_CREATED · nutrition 17.949444h · fatigue 9.050556h: VOLUNTARY_BOARD_DECISION
- 5.066667h · agent 0 · plan 1 · INTENT_CANCELLED · nutrition 17.933333h · fatigue 9.066667h: changes={}
- 5.067222h · agent 0 · plan 2 · INTENT_CREATED · nutrition 17.932778h · fatigue 9.067222h: VOLUNTARY_BOARD_DECISION
- 5.083333h · agent 0 · plan 2 · INTENT_CANCELLED · nutrition 17.916667h · fatigue 9.083333h: changes={}
- 5.083889h · agent 0 · plan 3 · INTENT_CREATED · nutrition 17.916111h · fatigue 9.083889h: VOLUNTARY_BOARD_DECISION
- 5.100000h · agent 0 · plan 3 · INTENT_CANCELLED · nutrition 17.900000h · fatigue 9.100000h: changes={}
- 5.100556h · agent 0 · plan 4 · INTENT_CREATED · nutrition 17.899444h · fatigue 9.100556h: VOLUNTARY_BOARD_DECISION
- 5.116667h · agent 0 · plan 4 · INTENT_CANCELLED · nutrition 17.883333h · fatigue 9.116667h: changes={}
- 5.117222h · agent 0 · plan 5 · INTENT_CREATED · nutrition 17.882778h · fatigue 9.117222h: VOLUNTARY_BOARD_DECISION
- 5.133333h · agent 0 · plan 5 · INTENT_CANCELLED · nutrition 17.866667h · fatigue 9.133333h: changes={}
- 5.133889h · agent 0 · plan 6 · INTENT_CREATED · nutrition 17.866111h · fatigue 9.133889h: VOLUNTARY_BOARD_DECISION
- 8.000000h · agent 0 · plan 6 · INTENT_REACHED_X · nutrition 15.000000h · fatigue 12.000000h: PHYSICAL_ARRIVAL
- 8.000000h · agent 0 · plan 6 · INTENT_CANCELLED · nutrition 15.000000h · fatigue 12.000000h: changes={"QUEUE_SERVICE:1:0->1"}
- 8.016667h · agent 0 · plan 7 · INTENT_CREATED · nutrition 14.983333h · fatigue 12.016667h: VOLUNTARY_BOARD_DECISION
- 11.016667h · agent 0 · plan 7 · INTENT_CANCELLED · nutrition 11.983333h · fatigue 15.016667h: changes={"QUEUE_SERVICE:1:0->1"}
- 11.033333h · agent 0 · plan 6 · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 15.033333h: edge=1 from=1 to=2 target=2 food=true previous_plan=Some(6) selected_at=18482000 new_information=true changes={"QUEUE_SERVICE:1:0->1"} decision=Decision { intent: Travel { edge: 1, target: 2, food: true }, reason: "seek remembered food; source=2 observed_at=21600000 believed_quantity=2932 expires_at=172800000; predicted_ms=16106667 arrival_nutrition_ms=48573333 arrival_starvation_ms=0" }

## remembered

- 5.033333h · agent 0 · plan 0 · INTENT_CREATED · nutrition 17.966667h · fatigue 9.033333h: VOLUNTARY_BOARD_DECISION
- 5.033333h · agent 0 · plan  · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 9.033333h: edge=1 from=2 to=1 target=0 food=false previous_plan=None selected_at=7200000 new_information=true changes={"NEW_NODE:2"} decision=Decision { intent: Travel { edge: 1, target: 0, food: false }, reason: "visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=49866666 arrival_starvation_ms=0" }
- 5.050000h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 17.950000h · fatigue 9.050000h: PERSONAL_BUFFER_PROVISIONING
- 5.066667h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 17.933333h · fatigue 9.066667h: PERSONAL_BUFFER_PROVISIONING
- 5.083333h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 17.916667h · fatigue 9.083333h: PERSONAL_BUFFER_PROVISIONING
- 5.100000h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 17.900000h · fatigue 9.100000h: PERSONAL_BUFFER_PROVISIONING
- 5.116667h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 17.883333h · fatigue 9.116667h: PERSONAL_BUFFER_PROVISIONING
- 5.133333h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 17.866667h · fatigue 9.133333h: PERSONAL_BUFFER_PROVISIONING
- 8.000000h · agent 0 · plan 0 · INTENT_REACHED_X · nutrition 15.000000h · fatigue 12.000000h: PHYSICAL_ARRIVAL
- 8.000000h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 15.000000h · fatigue 12.000000h: NO_JUSTIFIED_CURRENT_ACTION
- 11.016667h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 11.983333h · fatigue 15.016667h: EATING
- 11.033333h · agent 0 · plan 0 · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 15.033333h: edge=1 from=1 to=2 target=2 food=true previous_plan=Some(0) selected_at=18120000 new_information=true changes={"QUEUE_SERVICE:1:0->1"} decision=Decision { intent: Travel { edge: 1, target: 2, food: true }, reason: "seek remembered food; source=2 observed_at=21600000 believed_quantity=2932 expires_at=172800000; predicted_ms=16106667 arrival_nutrition_ms=48573333 arrival_starvation_ms=0" }
- 33.160000h · agent 0 · plan 0 · INTENT_CANCELLED · nutrition 0.000000h · fatigue 22.160000h: changes={"QUEUE_SERVICE:1:0->1", "QUEUE_SERVICE:1:0->2", "QUEUE_SERVICE:1:0->3"}

## resumable

- 5.033333h · agent 0 · plan 0 · INTENT_CREATED · nutrition 17.966667h · fatigue 9.033333h: VOLUNTARY_BOARD_DECISION
- 5.033333h · agent 0 · plan  · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 9.033333h: edge=1 from=2 to=1 target=0 food=false previous_plan=None selected_at=7200000 new_information=true changes={"NEW_NODE:2"} decision=Decision { intent: Travel { edge: 1, target: 0, food: false }, reason: "visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=49866666 arrival_starvation_ms=0" }
- 8.000000h · agent 0 · plan 0 · INTENT_REACHED_X · nutrition 15.000000h · fatigue 12.000000h: PHYSICAL_ARRIVAL
- 11.016667h · agent 0 · plan 0 · INTENT_CANCELLED · nutrition 11.983333h · fatigue 15.016667h: changes={"QUEUE_SERVICE:1:0->1"}
- 11.016667h · agent 0 · plan 0 · DIRECTION_REVERSAL · nutrition 11.983333h · fatigue 15.016667h: edge=1 from=1 to=2 target=2 food=true previous_plan=Some(0) selected_at=18120000 new_information=true changes={"QUEUE_SERVICE:1:0->1"} decision=Decision { intent: Travel { edge: 1, target: 2, food: true }, reason: "seek remembered food; source=2 observed_at=21600000 believed_quantity=2956 expires_at=172800000; predicted_ms=16094667 arrival_nutrition_ms=27045333 arrival_starvation_ms=0" }

## committed

- 5.033333h · agent 0 · plan 0 · INTENT_CREATED · nutrition 17.966667h · fatigue 9.033333h: VOLUNTARY_BOARD_DECISION
- 5.033333h · agent 0 · plan  · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 9.033333h: edge=1 from=2 to=1 target=0 food=false previous_plan=None selected_at=7200000 new_information=true changes={"NEW_NODE:2"} decision=Decision { intent: Travel { edge: 1, target: 0, food: false }, reason: "visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=49866666 arrival_starvation_ms=0" }
- 8.000000h · agent 0 · plan 0 · INTENT_REACHED_X · nutrition 15.000000h · fatigue 12.000000h: PHYSICAL_ARRIVAL
- 17.000000h · agent 0 · plan 0 · INTENT_PAUSED · nutrition 6.000000h · fatigue 21.000000h: CRITICAL_HUNGER
- 17.000000h · agent 0 · plan 0 · DIRECTION_REVERSAL · nutrition 6.000000h · fatigue 21.000000h: edge=1 from=1 to=2 target=2 food=true previous_plan=Some(0) selected_at=18120000 new_information=true changes={"QUEUE_SERVICE:0:0->1", "QUEUE_SERVICE:1:0->1", "QUEUE_SERVICE:1:0->2"} decision=Decision { intent: Travel { edge: 1, target: 2, food: true }, reason: "seek remembered food; source=2 observed_at=21600000 believed_quantity=2956 expires_at=172800000; predicted_ms=32408000 arrival_nutrition_ms=0 arrival_starvation_ms=10808000" }
- 25.880000h · agent 0 · plan 0 · INTENT_CANCELLED · nutrition 0.000000h · fatigue 24.000000h: changes={"QUEUE_SERVICE:0:0->1", "QUEUE_SERVICE:1:0->1", "QUEUE_SERVICE:1:0->2", "QUEUE_SERVICE:1:0->3", "QUEUE_SERVICE:1:0->4"}

## self_priority

- 5.033333h · agent 0 · plan 0 · INTENT_CREATED · nutrition 17.966667h · fatigue 9.033333h: VOLUNTARY_BOARD_DECISION
- 5.033333h · agent 0 · plan  · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 9.033333h: edge=1 from=2 to=1 target=0 food=false previous_plan=None selected_at=7200000 new_information=true changes={"NEW_NODE:2"} decision=Decision { intent: Travel { edge: 1, target: 0, food: false }, reason: "visit remembered board to claim locally; predicted_ms=14813334 arrival_nutrition_ms=49866666 arrival_starvation_ms=0" }
- 5.050000h · agent 0 · plan 0 · INTENT_CANCELLED · nutrition 17.950000h · fatigue 9.050000h: changes={}
- 5.050556h · agent 0 · plan 1 · INTENT_CREATED · nutrition 17.949444h · fatigue 9.050556h: VOLUNTARY_BOARD_DECISION
- 5.066667h · agent 0 · plan 1 · INTENT_CANCELLED · nutrition 17.933333h · fatigue 9.066667h: changes={}
- 5.067222h · agent 0 · plan 2 · INTENT_CREATED · nutrition 17.932778h · fatigue 9.067222h: VOLUNTARY_BOARD_DECISION
- 5.083333h · agent 0 · plan 2 · INTENT_CANCELLED · nutrition 17.916667h · fatigue 9.083333h: changes={}
- 5.083889h · agent 0 · plan 3 · INTENT_CREATED · nutrition 17.916111h · fatigue 9.083889h: VOLUNTARY_BOARD_DECISION
- 5.100000h · agent 0 · plan 3 · INTENT_CANCELLED · nutrition 17.900000h · fatigue 9.100000h: changes={}
- 5.100556h · agent 0 · plan 4 · INTENT_CREATED · nutrition 17.899444h · fatigue 9.100556h: VOLUNTARY_BOARD_DECISION
- 5.116667h · agent 0 · plan 4 · INTENT_CANCELLED · nutrition 17.883333h · fatigue 9.116667h: changes={}
- 5.117222h · agent 0 · plan 5 · INTENT_CREATED · nutrition 17.882778h · fatigue 9.117222h: VOLUNTARY_BOARD_DECISION
- 5.133333h · agent 0 · plan 5 · INTENT_CANCELLED · nutrition 17.866667h · fatigue 9.133333h: changes={}
- 5.133889h · agent 0 · plan 6 · INTENT_CREATED · nutrition 17.866111h · fatigue 9.133889h: VOLUNTARY_BOARD_DECISION
- 8.000000h · agent 0 · plan 6 · INTENT_REACHED_X · nutrition 15.000000h · fatigue 12.000000h: PHYSICAL_ARRIVAL
- 8.000000h · agent 0 · plan 6 · INTENT_CANCELLED · nutrition 15.000000h · fatigue 12.000000h: changes={"QUEUE_SERVICE:1:0->1"}
- 8.016667h · agent 0 · plan 7 · INTENT_CREATED · nutrition 14.983333h · fatigue 12.016667h: VOLUNTARY_BOARD_DECISION
- 11.016667h · agent 0 · plan 7 · INTENT_CANCELLED · nutrition 11.983333h · fatigue 15.016667h: changes={"QUEUE_SERVICE:1:0->1"}
- 11.033333h · agent 0 · plan 6 · DIRECTION_REVERSAL · nutrition 17.966667h · fatigue 15.033333h: edge=1 from=1 to=2 target=2 food=true previous_plan=Some(6) selected_at=18482000 new_information=true changes={"QUEUE_SERVICE:1:0->1"} decision=Decision { intent: Travel { edge: 1, target: 2, food: true }, reason: "seek remembered food; source=2 observed_at=21600000 believed_quantity=2932 expires_at=172800000; predicted_ms=16106667 arrival_nutrition_ms=48573333 arrival_starvation_ms=0" }

