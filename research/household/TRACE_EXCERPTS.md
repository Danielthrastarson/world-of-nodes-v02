# B1 focal traces

All excerpts are seed 0 from the final matrix. Times use the authoritative millisecond clock. Complete traces remain in each fixture directory.

## colocated_on

```text
t=0.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=0.000000000h Some(0) CareTransferred dependent=1 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=0.000000000h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=30.016666667h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=42.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## self_conflict_on

```text
t=0.000000000h Some(0) Decision intent=Eat { batch: 0, quantity: 6 } nutrition_ms=14400000 starvation_ms=0 reason=locally accessible food; exact ownership/ration ceiling
t=0.000000000h Some(0) Consumed batch=0 q=6 owner=Agent(0) nutrition_ms=36000000
t=0.000000000h Some(1) Decision intent=Wait nutrition_ms=0 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed
t=0.016666667h Some(0) Decision intent=Wait nutrition_ms=35940000 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed
t=0.016666667h Some(1) Decision intent=Wait nutrition_ms=0 starvation_ms=60000 reason=no locally justified work; remaining near resources is allowed
t=24.000000000h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=34.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## multiple_on

```text
t=0.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=0.000000000h Some(0) CareTransferred dependent=1 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=0.000000000h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=24.000000000h Some(2) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=30.016666667h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=42.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## near_known_on

```text
t=0.000000000h Some(0) Decision intent=TravelForCare { dependent: 1, edge: 0, target: 1 } nutrition_ms=64800000 starvation_ms=0 reason=directed care; dependent=1 observed_at=0 observed_nutrition=0 target=1 predicted_travel=3600000 own_arrival_nutrition=61200000
t=0.000000000h Some(1) Decision intent=Travel { edge: 0, target: 0, food: false } nutrition_ms=0 starvation_ms=0 reason=explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=0 arrival_starvation_ms=7200000
t=0.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=1.000000000h Some(0) CareCancelled dependent=1 reason=LOCAL_ABSENCE_OR_DEATH
t=24.000000000h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=48.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
```

## far_known_on

```text
t=0.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=10.222222222h Some(0) CareCancelled dependent=1 reason=LOCAL_ABSENCE_OR_DEATH
t=24.000000000h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 1, to: 0, progress: 33480000000000 }
t=48.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
```

## unknown_on

```text
t=0.000000000h Some(0) Decision intent=Wait nutrition_ms=64800000 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed
t=0.000000000h Some(1) Decision intent=Travel { edge: 0, target: 0, food: false } nutrition_ms=0 starvation_ms=0 reason=explore unknown location; no resource target; predicted_ms=7200000 arrival_nutrition_ms=0 arrival_starvation_ms=7200000
t=2.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=2.000000000h Some(0) CareTransferred dependent=1 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=2.000000000h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=32.016666667h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=42.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
```

## caregiver_death_on

```text
t=1.000000000h Some(0) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
t=36.000000000h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## farming_on

```text
t=0.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=0.000000000h Some(0) CareTransferred dependent=1 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=0.000000000h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=14.716666667h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=14.716666667h Some(0) CareTransferred dependent=1 quantity=1 from_batch=3 to_batch=4 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=14.716666667h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=14.750000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=14.750000000h Some(0) CareTransferred dependent=1 quantity=1 from_batch=3 to_batch=4 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=14.750000000h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=14.783333333h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=14.783333333h Some(0) CareTransferred dependent=1 quantity=1 from_batch=3 to_batch=4 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=14.783333333h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=14.816666667h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=14.816666667h Some(0) CareTransferred dependent=1 quantity=1 from_batch=3 to_batch=4 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=14.816666667h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=42.733333333h Some(1) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## construction_on

```text
t=0.000000000h Some(0) CareCreated selected=Some(1) reason=OBSERVED_NEED
t=0.000000000h Some(0) CareTransferred dependent=1 quantity=6 from_batch=3 to_batch=4 node=0 owner=Agent(1) custody=Loose; recipient must eat normally
t=0.000000000h Some(0) CareCompleted dependent=1; ownership transferred, nourishment not guaranteed
t=37.794444444h Some(1) Consumed batch=4 q=6 owner=Agent(1) nutrition_ms=63744000
```
