# B2 focal causal chains

Seed 0. Raw event/appraisal files preserve all 30 seeds of appraisal data and seed-0 physical traces. Histories are controlled kernel actions; matched later choices are autonomous. Times are authoritative milliseconds. Source-event IDs index the corresponding raw stage event stream.

## help_safe

```text
0	history	0	Some(0)	CareTransferred	dependent=1 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
0	history	0	Some(0)	CareCompleted	dependent=1; ownership transferred, nourishment not guaranteed
0	history	0	Some(1)	SocialAppraisal	Appraisal { evidence: Evidence { at: 0, observer: 1, actor: 0, node: 0, source_event: 1, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 0 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	history	0	Some(1)	Consumed	batch=1 q=6 owner=Agent(1) nutrition_ms=21600000
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 1, attachment: 2, obligation: 1, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=10 cost=6 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: GiveCareFood { dependent: 0, batch: 0, quantity: 6 }, reason: "directed care; dependent=0 observed_at=3600000 observed_nutrition=0 target=0 predicted_travel=0 own_arrival_nutrition=64800000; social=Assessment { dependent: 0, need: 6, trust: 1, attachment: 2, obligation: 1, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 }" }
0	execution	3600000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3600000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3600000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 0, actor: 1, node: 0, source_event: 7, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3660000	Some(0)	Consumed	batch=1 q=6 owner=Agent(0) nutrition_ms=21600000
```

## help_costly

```text
0	history	0	Some(0)	CareTransferred	dependent=1 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
0	history	0	Some(0)	CareCompleted	dependent=1; ownership transferred, nourishment not guaranteed
0	history	0	Some(1)	SocialAppraisal	Appraisal { evidence: Evidence { at: 0, observer: 1, actor: 0, node: 0, source_event: 1, occurrence: Support { quantity: 6, giver_nutrition: 25200000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 12, attachment: 26, grievance: 0, obligation: 12, last_update: 0 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	history	0	Some(1)	Consumed	batch=1 q=6 owner=Agent(1) nutrition_ms=21600000
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 2, attachment: 2, obligation: 2, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=12 cost=6 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: GiveCareFood { dependent: 0, batch: 0, quantity: 6 }, reason: "directed care; dependent=0 observed_at=3600000 observed_nutrition=0 target=0 predicted_travel=0 own_arrival_nutrition=64800000; social=Assessment { dependent: 0, need: 6, trust: 2, attachment: 2, obligation: 2, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 }" }
0	execution	3600000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3600000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3600000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 0, actor: 1, node: 0, source_event: 7, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3660000	Some(0)	Consumed	batch=1 q=6 owner=Agent(0) nutrition_ms=21600000
```

## failed_attempt

```text
0	history	0	Some(0)	CareAttemptUnavailable	recipient=1 physical_rejection=NOT_PERSONAL_CARRIED_FOOD
0	history	0	Some(1)	SocialAppraisal	Appraisal { evidence: Evidence { at: 0, observer: 1, actor: 0, node: 0, source_event: 1, occurrence: UnavailableAttempt }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: -1, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=8 cost=6 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: GiveCareFood { dependent: 0, batch: 0, quantity: 6 }, reason: "directed care; dependent=0 observed_at=3600000 observed_nutrition=0 target=0 predicted_travel=0 own_arrival_nutrition=64800000; social=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 }" }
0	execution	3600000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3600000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3600000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 0, actor: 1, node: 0, source_event: 7, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3660000	Some(0)	Consumed	batch=1 q=6 owner=Agent(0) nutrition_ms=21600000
```

## refusal

```text
0	history	0	Some(0)	Consumed	batch=0 q=6 owner=Agent(0) nutrition_ms=86400000
0	history	0	Some(1)	SocialAppraisal	Appraisal { evidence: Evidence { at: 0, observer: 1, actor: 0, node: 0, source_event: 0, occurrence: UnmetExpectation { quantity: 6, alternative_believed: true } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: -6, attachment: 20, grievance: 6, obligation: 0, last_update: 0 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 43200000, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: -1, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 1, anger: 1, fear: 0, grief: 0 } pressure=7 cost=8 permits=false selected=Wait
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: Wait, reason: "no locally justified work; remaining near resources is allowed" }
0	execution	3660000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: -1, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 1, anger: 1, fear: 0, grief: 0 } pressure=7 cost=8 permits=false selected=Wait
```

## remote_refusal

```text
0	history	0	Some(0)	Consumed	batch=0 q=6 owner=Agent(0) nutrition_ms=86400000
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=8 cost=6 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: GiveCareFood { dependent: 0, batch: 0, quantity: 6 }, reason: "directed care; dependent=0 observed_at=3600000 observed_nutrition=0 target=0 predicted_travel=0 own_arrival_nutrition=64800000; social=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 }" }
0	execution	3600000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3600000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3600000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 0, actor: 1, node: 0, source_event: 7, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3660000	Some(0)	Consumed	batch=1 q=6 owner=Agent(0) nutrition_ms=21600000
```

## fear

```text
0	history	3600000	Some(1)	OwnDangerObserved	nutrition=0 starvation_ms=64800000
0	history	3600000	Some(1)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 1, actor: 1, node: 0, source_event: 0, occurrence: OwnStarvationDanger }, before: Relationship { trust: 0, attachment: 0, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 0, attachment: 0, grievance: 0, obligation: 0, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 43200000, at: 3600000 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 2, grief: 0 } pressure=8 cost=8 permits=false selected=Wait
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: Wait, reason: "no locally justified work; remaining near resources is allowed" }
0	execution	3660000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 1, grief: 0 } pressure=8 cost=7 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3660000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3660000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3660000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3660000, observer: 0, actor: 1, node: 0, source_event: 11, occurrence: Support { quantity: 6, giver_nutrition: 64740000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3660000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
```

## death_known

```text
0	history	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	history	3600000	Some(1)	DeathObserved	person=2 node=0
0	history	3600000	Some(1)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 1, actor: 2, node: 0, source_event: 1, occurrence: DeathLearned }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 36000000, at: 3600000 } } }
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 1 } pressure=8 cost=7 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: GiveCareFood { dependent: 0, batch: 0, quantity: 6 }, reason: "directed care; dependent=0 observed_at=3600000 observed_nutrition=0 target=0 predicted_travel=0 own_arrival_nutrition=64800000; social=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 1 }" }
0	execution	3600000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3600000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3600000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 0, actor: 1, node: 0, source_event: 7, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3660000	Some(0)	Consumed	batch=1 q=6 owner=Agent(0) nutrition_ms=21600000
```

## death_unknown

```text
0	history	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=8 cost=6 permits=true selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: GiveCareFood { dependent: 0, batch: 0, quantity: 6 }, reason: "directed care; dependent=0 observed_at=3600000 observed_nutrition=0 target=0 predicted_travel=0 own_arrival_nutrition=64800000; social=Assessment { dependent: 0, need: 6, trust: 0, attachment: 2, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 }" }
0	execution	3600000	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	execution	3600000	Some(1)	CareCompleted	dependent=0; ownership transferred, nourishment not guaranteed
0	execution	3600000	Some(0)	SocialAppraisal	Appraisal { evidence: Evidence { at: 3600000, observer: 0, actor: 1, node: 0, source_event: 7, occurrence: Support { quantity: 6, giver_nutrition: 64800000 } }, before: Relationship { trust: 0, attachment: 20, grievance: 0, obligation: 0, last_update: 0 }, after: Relationship { trust: 6, attachment: 23, grievance: 0, obligation: 6, last_update: 3600000 }, affect_before: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } }, affect_after: Affect { anger: Pulse { height: 0, at: 0 }, fear: Pulse { height: 0, at: 0 }, grief: Pulse { height: 0, at: 0 } } }
0	execution	3660000	Some(0)	Consumed	batch=1 q=6 owner=Agent(0) nutrition_ms=21600000
```

## stranger

```text
0	execution	3600000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
0	execution	3600000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 0, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=6 cost=6 permits=false selected=Wait
0	execution	3600000	Some(1)	MatchedPersonChoice	Decision { intent: Wait, reason: "no locally justified work; remaining near resources is allowed" }
0	execution	3660000	Some(1)	SocialChoice	assessment=Assessment { dependent: 0, need: 6, trust: 0, attachment: 0, obligation: 0, food: 6, travel: 0, grievance: 0, anger: 0, fear: 0, grief: 0 } pressure=6 cost=6 permits=false selected=Wait
```
