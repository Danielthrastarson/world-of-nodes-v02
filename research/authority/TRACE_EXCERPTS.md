# F10 causal excerpts

Seed 0; exact millisecond records from compressed primary traces. Authority transitions are an observer journal, never remote messages.

## bottleneck_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
32.080000000h (115488000ms) Some(9) Admitted edge=0 from=0 to=1
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 3528000000000 }
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## death_after_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(9) Admitted edge=0 from=0 to=1
0.500000000h (1800000ms) Some(9) Arrived node=1 edge=0
0.500000000h (1800000ms) office 0 AuthorityTransition PRESENT active=true
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
36.000000000h (129600000ms) office 0 AuthorityTransition HOLDER_DIED active=false
```

## death_before_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(9) Admitted edge=0 from=0 to=1
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 61128000000000 }
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## departure_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(9) Admitted edge=0 from=0 to=1
0.500000000h (1800000ms) Some(9) Arrived node=1 edge=0
0.500000000h (1800000ms) office 0 AuthorityTransition PRESENT active=true
1.500000000h (5400000ms) Some(9) Queued edge=0 target=0 remembered_food=false
1.500000000h (5400000ms) Some(9) Admitted edge=0 from=1 to=0
1.500000000h (5400000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
2.000000000h (7200000ms) Some(9) Arrived node=0 edge=0
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## discovery_commissioned

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 1, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 1 }
0.000000000h (0ms) Some(1) CrossingCommissioned Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 0 }; no resources or workers assigned
0.000000000h (0ms) Some(1) Decision intent=Gather { batch: 0, communal: false } nutrition_ms=43200000 starvation_ms=0 reason=one quantum from observed deposit; communal=false
0.000000000h (0ms) office 0 AuthorityTransition PRESENT active=true
0.016666667h (60000ms) Some(1) Decision intent=Gather { batch: 0, communal: false } nutrition_ms=46740000 starvation_ms=0 reason=one quantum from observed deposit; communal=false
0.033333333h (120000ms) Some(1) Decision intent=Wait nutrition_ms=46680000 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed
0.050000000h (180000ms) Some(1) Decision intent=Wait nutrition_ms=46620000 starvation_ms=0 reason=no locally justified work; remaining near resources is allowed
1.033333333h (3720000ms) Some(1) Decision intent=Travel { edge: 0, target: 0, food: false } nutrition_ms=61080000 starvation_ms=0 reason=explore unknown location; no resource target; predicted_ms=900000 arrival_nutrition_ms=60180000 arrival_starvation_ms=0
1.033333333h (3720000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
```

## discovery_pending

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 1, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 1 }
0.000000000h (0ms) office 0 AuthorityTransition PRESENT active=true
1.033333333h (3720000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
```

## distant_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(9) Admitted edge=0 from=0 to=1
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.555555556h (30800000ms) Some(9) Arrived node=1 edge=0
8.555555556h (30800000ms) office 0 AuthorityTransition PRESENT active=true
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
36.000000000h (129600000ms) office 0 AuthorityTransition HOLDER_DIED active=false
```

## local_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(9) Admitted edge=0 from=0 to=1
0.500000000h (1800000ms) Some(9) Arrived node=1 edge=0
0.500000000h (1800000ms) office 0 AuthorityTransition PRESENT active=true
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
36.000000000h (129600000ms) office 0 AuthorityTransition HOLDER_DIED active=false
```

## multiple_command

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 9, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(9) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(9) Admitted edge=0 from=0 to=1
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 1, polity: 17, holder: 10, settlement: 2, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 0 }
0.000000000h (0ms) Some(10) Queued edge=0 target=1 remembered_food=false
0.000000000h (0ms) Some(10) Admitted edge=0 from=0 to=1
0.500000000h (1800000ms) Some(9) Arrived node=1 edge=0
0.500000000h (1800000ms) Some(10) Arrived node=1 edge=0
0.500000000h (1800000ms) Some(10) Queued edge=1 target=2 remembered_food=false
0.500000000h (1800000ms) Some(10) Admitted edge=1 from=1 to=2
0.500000000h (1800000ms) office 0 AuthorityTransition PRESENT active=true
1.000000000h (3600000ms) Some(10) Arrived node=2 edge=1
1.000000000h (3600000ms) office 1 AuthorityTransition PRESENT active=true
6.000000000h (21600000ms) Some(9) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
6.000000000h (21600000ms) Some(10) PerformanceChanged old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
8.000000000h (28800000ms) Some(9) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
8.000000000h (28800000ms) Some(10) PerformanceChanged old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
12.000000000h (43200000ms) Some(9) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(9) StarvationStarted continuous_starvation_ms=0
12.000000000h (43200000ms) Some(10) PerformanceChanged old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
12.000000000h (43200000ms) Some(10) StarvationStarted continuous_starvation_ms=0
16.000000000h (57600000ms) Some(9) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
16.000000000h (57600000ms) Some(10) PerformanceChanged old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
36.000000000h (129600000ms) Some(9) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
36.000000000h (129600000ms) Some(10) Died cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
36.000000000h (129600000ms) office 0 AuthorityTransition HOLDER_DIED active=false
36.000000000h (129600000ms) office 1 AuthorityTransition HOLDER_DIED active=false
```

## scarce_commissioned

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 1, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 1 }
0.000000000h (0ms) Some(1) CrossingCommissioned Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 0 }; no resources or workers assigned
0.000000000h (0ms) Some(1) Decision intent=Gather { batch: 2, communal: false } nutrition_ms=43200000 starvation_ms=0 reason=voluntary communal crossing project=0; conditional own food access/remembered transport savings; no guaranteed outcome
0.000000000h (0ms) office 0 AuthorityTransition PRESENT active=true
0.000555556h (2000ms) Some(0) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(1) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(2) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(3) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(4) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.016666667h (60000ms) Some(1) Decision intent=Gather { batch: 0, communal: false } nutrition_ms=43140000 starvation_ms=0 reason=one quantum from observed deposit; communal=false
0.033333333h (120000ms) Some(1) Decision intent=Travel { edge: 0, target: 0, food: false } nutrition_ms=46680000 starvation_ms=0 reason=visit remembered board to claim locally; predicted_ms=900000 arrival_nutrition_ms=45780000 arrival_starvation_ms=0
0.033333333h (120000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
1.033333333h (3720000ms) Some(1) Decision intent=Wait nutrition_ms=43080000 starvation_ms=0 reason=remain FIFO; seek remembered food; source=1 observed_at=120000 believed_quantity=250 expires_at=172800000; predicted_ms=902000 arrival_starvation_ms=0
1.283333333h (4620000ms) office 0 AuthorityTransition PRESENT active=true
37.000000000h (133200000ms) office 0 AuthorityTransition HOLDER_DIED active=false
```

## scarce_pending

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 1, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 1 }
0.000000000h (0ms) office 0 AuthorityTransition PRESENT active=true
0.000000000h (0ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
1.500000000h (5400000ms) office 0 AuthorityTransition PRESENT active=true
36.000000000h (129600000ms) office 0 AuthorityTransition HOLDER_DIED active=false
```

## sufficient_commissioned

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 1, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 1 }
0.000000000h (0ms) Some(1) CrossingCommissioned Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 0 }; no resources or workers assigned
0.000000000h (0ms) Some(1) Decision intent=Gather { batch: 2, communal: false } nutrition_ms=43200000 starvation_ms=0 reason=voluntary communal crossing project=0; conditional own food access/remembered transport savings; no guaranteed outcome
0.000000000h (0ms) office 0 AuthorityTransition PRESENT active=true
0.000555556h (2000ms) Some(0) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(1) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(2) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(3) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(4) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(5) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(6) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(7) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(8) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(9) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(10) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(11) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(12) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(13) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(14) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(15) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(16) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(17) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(18) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.000555556h (2000ms) Some(19) CrossingDonation project=0 q=1 owner=Commons(0) physical_embedded_wood
0.016666667h (60000ms) Some(0) CrossingWorkStarted project=0; labor confers no personal title
0.016666667h (60000ms) Some(1) Decision intent=BuildCrossing { project: 0 } nutrition_ms=43140000 starvation_ms=0 reason=voluntary communal crossing project=0; conditional own food access/remembered transport savings; no guaranteed outcome
0.033333333h (120000ms) Some(1) Decision intent=Gather { batch: 0, communal: false } nutrition_ms=43080000 starvation_ms=0 reason=one quantum from observed deposit; communal=false
0.050000000h (180000ms) Some(1) Decision intent=Travel { edge: 0, target: 0, food: false } nutrition_ms=46620000 starvation_ms=0 reason=visit remembered board to claim locally; predicted_ms=900000 arrival_nutrition_ms=45720000 arrival_starvation_ms=0
0.050000000h (180000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
1.033333333h (3720000ms) Some(1) Decision intent=Wait nutrition_ms=43080000 starvation_ms=0 reason=remain FIFO; seek remembered food; source=1 observed_at=180000 believed_quantity=251 expires_at=172800000; predicted_ms=902000 arrival_starvation_ms=0
1.300000000h (4680000ms) office 0 AuthorityTransition PRESENT active=true
5.033333333h (18120000ms) Some(1) CrossingWorkStarted project=0; labor confers no personal title
6.033333333h (21720000ms) Some(5) CrossingWorkStarted project=0; labor confers no personal title
8.055555556h (29000000ms) Some(5) CrossingCompleted project=0 owner=Commons(0); adjacent observation required; existing traversals unchanged
8.066666667h (29040000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
10.983333333h (39540000ms) office 0 AuthorityTransition PRESENT active=true
10.983333333h (39540000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
11.755555556h (42320000ms) office 0 AuthorityTransition PRESENT active=true
13.696666667h (49308000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
15.030000000h (54108000ms) office 0 AuthorityTransition PRESENT active=true
16.560000000h (59616000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
19.516666667h (70260000ms) office 0 AuthorityTransition PRESENT active=true
20.016666667h (72060000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
22.500000000h (81000000ms) office 0 AuthorityTransition PRESENT active=true
22.500000000h (81000000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
24.000000000h (86400000ms) office 0 AuthorityTransition PRESENT active=true
29.208333333h (105150000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
30.966666667h (111480000ms) office 0 AuthorityTransition PRESENT active=true
30.966666667h (111480000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
31.522222222h (113480000ms) office 0 AuthorityTransition PRESENT active=true
32.388888889h (116600000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
34.000000000h (122400000ms) office 0 AuthorityTransition PRESENT active=true
34.000000000h (122400000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
34.666666667h (124800000ms) office 0 AuthorityTransition PRESENT active=true
35.500000000h (127800000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
36.666666667h (132000000ms) office 0 AuthorityTransition PRESENT active=true
36.666666667h (132000000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
37.600000000h (135360000ms) office 0 AuthorityTransition PRESENT active=true
38.166666667h (137400000ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
47.431448889h (170753216ms) office 0 AuthorityTransition PRESENT active=true
```

## sufficient_pending

```text
0.000000000h (0ms) Some(0) StewardAppointed Office { id: 0, polity: 17, holder: 1, settlement: 1, status: Appointed, appointed_at: 0, appointed_by: 0, appointment_node: 1 }
0.000000000h (0ms) office 0 AuthorityTransition PRESENT active=true
0.000000000h (0ms) office 0 AuthorityTransition HOLDER_DEPARTED active=false
1.500000000h (5400000ms) office 0 AuthorityTransition PRESENT active=true
36.000000000h (129600000ms) office 0 AuthorityTransition HOLDER_DIED active=false
```
