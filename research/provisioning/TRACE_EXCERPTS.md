# Causal trace excerpts

Seed 0; timestamps are simulated hours. Full traces include every decision and event.

## reference

- 0.000000h · event 100 · agent 0 · cause Some(1) · **Admitted**: edge=0 from=0 to=1
- 2.000000h · event 5578 · agent 0 · cause Some(100) · **Arrived**: node=1 edge=0
- 2.000000h · event 5640 · agent 0 · cause Some(5583) · **Admitted**: edge=1 from=1 to=2
- 4.000000h · event 10642 · agent 0 · cause Some(5640) · **Arrived**: node=2 edge=1
- 4.000556h · event 10721 · agent 0 · cause Some(10720) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=32398000
- 4.017222h · event 10780 · agent 0 · cause Some(10779) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=35938000
- 4.033889h · event 10838 · agent 0 · cause Some(10837) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=39478000
- 4.050556h · event 10896 · agent 0 · cause Some(10895) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=43018000
- 4.067222h · event 10954 · agent 0 · cause Some(10953) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=46558000
- 5.016667h · event 13387 · agent 0 · cause Some(13386) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 6.000000h · event 15964 · agent 28 · cause Some(15918) · **PerformanceChanged**: old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
- 8.000000h · event 21316 · agent 0 · cause Some(21274) · **PerformanceChanged**: old_ppm=1000000 new_ppm=900000 nutrition_ms=54000000 fatigue_ms=43200000
- 8.000000h · event 21344 · agent 28 · cause Some(21294) · **PerformanceChanged**: old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
- 11.016667h · event 29294 · agent 0 · cause Some(29293) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 12.000000h · event 32140 · agent 0 · cause Some(32098) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=61200000 fatigue_ms=57600000
- 12.000000h · event 32180 · agent 28 · cause Some(32118) · **PerformanceChanged**: old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
- 12.000000h · event 32181 · agent 28 · cause Some(32180) · **StarvationStarted**: continuous_starvation_ms=0
- 16.000000h · event 42532 · agent 0 · cause Some(42490) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=46800000 fatigue_ms=72000000
- 16.000000h · event 42560 · agent 28 · cause Some(42510) · **PerformanceChanged**: old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
- 17.016667h · event 45454 · agent 0 · cause Some(45453) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 23.016667h · event 79879 · agent 0 · cause Some(79878) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 23.050000h · event 80133 · agent 0 · cause Some(80129) · **PerformanceChanged**: old_ppm=1000000 new_ppm=900000 nutrition_ms=64620000 fatigue_ms=43200000
- 24.080000h · event 85753 · agent 28 · cause Some(85665) · **Admitted**: edge=0 from=0 to=1
- 27.050000h · event 100294 · agent 0 · cause Some(100180) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=50220000 fatigue_ms=57600000
- 29.016667h · event 109979 · agent 0 · cause Some(109978) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 31.050000h · event 120770 · agent 0 · cause Some(120636) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57420000 fatigue_ms=72000000
- 32.080000h · event 126134 · agent 28 · cause Some(85753) · **Arrived**: node=1 edge=0
- 33.083333h · event 131917 · agent 28 · cause Some(131893) · **Admitted**: edge=1 from=1 to=2
- 35.016667h · event 142673 · agent 0 · cause Some(142672) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 36.000000h · event 147157 · agent 28 · cause Some(131917) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 1, from: 1, to: 2, progress: 2625000000000 }
- 36.000000h · event 147158 · agent 29 · cause Some(131918) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 1, from: 1, to: 2, progress: 2625000000000 }
- 36.000000h · event 147159 · agent 30 · cause Some(131919) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 1, from: 1, to: 2, progress: 2625000000000 }
- 36.000000h · event 147160 · agent 31 · cause Some(131920) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 1, from: 1, to: 2, progress: 2625000000000 }
- 36.000000h · event 147161 · agent 32 · cause Some(126146) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 3528000000000 }
- 36.000000h · event 147162 · agent 33 · cause Some(126147) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 3528000000000 }
- 36.000000h · event 147163 · agent 34 · cause Some(126148) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 3528000000000 }
- 36.000000h · event 147164 · agent 35 · cause Some(126149) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 3528000000000 }
- 36.950000h · event 149883 · agent 0 · cause Some(149827) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57780000 fatigue_ms=72000000
- 41.016667h · event 163949 · agent 0 · cause Some(163948) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 43.150000h · event 169223 · agent 0 · cause Some(169195) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57060000 fatigue_ms=72000000
- 47.016667h · event 180289 · agent 0 · cause Some(180288) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 48.000000h · event 183083 · agent None · cause None · **Spoiled**: batch=0 q=1568 holder=Deposit(2) created=0 deadline=172800000
- 48.000000h · event 183084 · agent None · cause None · **Spoiled**: batch=1 q=6 holder=Agent(0) created=0 deadline=172800000
- 48.000000h · event 183085 · agent None · cause None · **Spoiled**: batch=2 q=6 holder=Agent(1) created=0 deadline=172800000
- 48.750000h · event 185592 · agent 0 · cause Some(185536) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=58500000 fatigue_ms=72000000
- 65.000000h · event 236348 · agent 0 · cause Some(236320) · **PerformanceChanged**: old_ppm=800000 new_ppm=500000 nutrition_ms=0 fatigue_ms=41400000
- 65.000000h · event 236349 · agent 0 · cause Some(236348) · **StarvationStarted**: continuous_starvation_ms=0
- 65.500000h · event 237372 · agent 0 · cause Some(237344) · **PerformanceChanged**: old_ppm=500000 new_ppm=450000 nutrition_ms=0 fatigue_ms=43200000
- 69.500000h · event 245392 · agent 0 · cause Some(245364) · **PerformanceChanged**: old_ppm=450000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000

Event counts: Admitted=68, Arrived=60, Consumed=464, Decision=161788, Died=22, GatherStarted=1432, Gathered=1432, Interrupted=40960, PerformanceChanged=432, Queued=6738, RequestObserved=13, SleepStarted=36940, Spoiled=29, StarvationStarted=62

## contested_requests

- 0.000000h · event 100 · agent 0 · cause Some(1) · **Admitted**: edge=0 from=0 to=1
- 2.000000h · event 10030 · agent 0 · cause Some(100) · **Arrived**: node=1 edge=0
- 2.000000h · event 10142 · agent 0 · cause Some(10039) · **Admitted**: edge=1 from=1 to=2
- 4.000000h · event 20334 · agent 0 · cause Some(10142) · **Arrived**: node=2 edge=1
- 4.000556h · event 20455 · agent 0 · cause Some(20454) · **Consumed**: batch=5 q=1 owner=Agent(0) nutrition_ms=32398000
- 4.017222h · event 20557 · agent 0 · cause Some(20556) · **Consumed**: batch=5 q=1 owner=Agent(0) nutrition_ms=35938000
- 4.033889h · event 20657 · agent 0 · cause Some(20656) · **Consumed**: batch=5 q=1 owner=Agent(0) nutrition_ms=39478000
- 4.050556h · event 20757 · agent 0 · cause Some(20756) · **Consumed**: batch=5 q=1 owner=Agent(0) nutrition_ms=43018000
- 4.067222h · event 20857 · agent 0 · cause Some(20856) · **Consumed**: batch=5 q=1 owner=Agent(0) nutrition_ms=46558000
- 5.016667h · event 25643 · agent 0 · cause Some(25642) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 8.000000h · event 41089 · agent 0 · cause Some(41005) · **PerformanceChanged**: old_ppm=1000000 new_ppm=900000 nutrition_ms=54000000 fatigue_ms=43200000
- 11.016667h · event 55627 · agent 0 · cause Some(55626) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 12.000000h · event 60481 · agent 0 · cause Some(60405) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=61200000 fatigue_ms=57600000
- 16.000000h · event 79161 · agent 0 · cause Some(79085) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=46800000 fatigue_ms=72000000
- 17.016667h · event 83822 · agent 0 · cause Some(83821) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 23.016667h · event 121501 · agent 0 · cause Some(121500) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 23.050000h · event 121755 · agent 0 · cause Some(121751) · **PerformanceChanged**: old_ppm=1000000 new_ppm=900000 nutrition_ms=64620000 fatigue_ms=43200000
- 27.050000h · event 142878 · agent 0 · cause Some(142744) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=50220000 fatigue_ms=57600000
- 29.016667h · event 154595 · agent 0 · cause Some(154594) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 30.266667h · event 162707 · agent 12 · cause Some(162706) · **JobAccepted**: board=1 q=10 lease_expires=152160000
- 30.266667h · event 162709 · agent 13 · cause Some(162708) · **JobAccepted**: board=1 q=10 lease_expires=152160000
- 31.050000h · event 167746 · agent 0 · cause Some(167586) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57420000 fatigue_ms=72000000
- 31.250000h · event 169014 · agent 12 · cause Some(168946) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=56996000 fatigue_ms=72000000
- 31.250000h · event 169015 · agent 13 · cause Some(168947) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=56996000 fatigue_ms=72000000
- 32.928333h · event 179836 · agent 12 · cause Some(179746) · **Admitted**: edge=1 from=1 to=2
- 32.928333h · event 179837 · agent 13 · cause Some(179747) · **Admitted**: edge=1 from=1 to=2
- 35.016667h · event 193410 · agent 0 · cause Some(193409) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 36.000000h · event 199300 · agent 20 · cause Some(199224) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
- 36.000000h · event 199301 · agent 21 · cause Some(199225) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
- 36.000000h · event 199302 · agent 22 · cause Some(199226) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
- 36.000000h · event 199303 · agent 23 · cause Some(199227) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
- 36.000000h · event 199304 · agent 24 · cause Some(199278) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)
- 36.000000h · event 199305 · agent 31 · cause Some(199281) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 199306 · agent 34 · cause Some(199284) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 199307 · agent 35 · cause Some(199287) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.928333h · event 201943 · agent 12 · cause Some(179836) · **Arrived**: node=2 edge=1
- 36.928333h · event 201944 · agent 13 · cause Some(179837) · **Arrived**: node=2 edge=1
- 36.950000h · event 202011 · agent 0 · cause Some(201959) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57780000 fatigue_ms=72000000
- 41.016667h · event 213969 · agent 0 · cause Some(213968) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 41.082994h · event 214240 · agent 12 · cause Some(214239) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=25197222
- 41.082994h · event 214242 · agent 13 · cause Some(214241) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=25197222
- 41.083951h · event 214288 · agent 12 · cause Some(214287) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=28793777
- 41.083951h · event 214290 · agent 13 · cause Some(214289) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=28793777
- 41.100617h · event 214336 · agent 12 · cause Some(214335) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=32333777
- 41.100617h · event 214338 · agent 13 · cause Some(214337) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=32333777
- 41.117284h · event 214384 · agent 12 · cause Some(214383) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=35873777
- 41.117284h · event 214386 · agent 13 · cause Some(214385) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=35873777
- 41.133951h · event 214432 · agent 12 · cause Some(214431) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=39413777
- 41.133951h · event 214434 · agent 13 · cause Some(214433) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=39413777
- 41.150618h · event 214480 · agent 12 · cause Some(214479) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=42953777
- 41.150618h · event 214482 · agent 13 · cause Some(214481) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=42953777
- 41.167284h · event 214528 · agent 12 · cause Some(214527) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=46493777
- 41.167284h · event 214530 · agent 13 · cause Some(214529) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=46493777
- 41.390000h · event 215075 · agent 12 · cause Some(215051) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=45692000 fatigue_ms=57600000
- 41.390000h · event 215076 · agent 13 · cause Some(215052) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=45692000 fatigue_ms=57600000
- 42.266667h · event 216995 · agent None · cause None · **LeaseExpired**: board=1 count=4; workers not notified
- 43.083333h · event 218734 · agent 12 · cause Some(218733) · **Consumed**: batch=29 q=1 owner=Commons(0) nutrition_ms=43196000
- 43.083333h · event 218737 · agent 13 · cause Some(218736) · **Consumed**: batch=30 q=1 owner=Commons(0) nutrition_ms=43196000
- 43.100741h · event 218801 · agent 12 · cause Some(218800) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=46733333
- 43.100741h · event 218803 · agent 13 · cause Some(218802) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=46733333
- 43.150000h · event 218876 · agent 0 · cause Some(218844) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57060000 fatigue_ms=72000000
- 44.084074h · event 220721 · agent 12 · cause Some(220720) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=46793333
- 44.084074h · event 220723 · agent 13 · cause Some(220722) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=46793333
- 44.466667h · event 221450 · agent 12 · cause Some(221449) · **JobCancelled**: request=1; reason=delivery survival-infeasible or cargo predicted spoiled; remote board unchanged
- 44.466667h · event 221453 · agent 13 · cause Some(221452) · **JobCancelled**: request=1; reason=delivery survival-infeasible or cargo predicted spoiled; remote board unchanged
- 44.966667h · event 222424 · agent None · cause None · **LeaseExpired**: board=1 count=4; workers not notified
- 45.083333h · event 222766 · agent 12 · cause Some(222765) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=46796000
- 45.083333h · event 222768 · agent 13 · cause Some(222767) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=46796000
- 45.390000h · event 223613 · agent 12 · cause Some(223593) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=45692000 fatigue_ms=72000000
- 45.390000h · event 223614 · agent 13 · cause Some(223594) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=45692000 fatigue_ms=72000000
- 46.083333h · event 225714 · agent 12 · cause Some(225713) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=46796000
- 46.083333h · event 225716 · agent 13 · cause Some(225715) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=46796000
- 47.016667h · event 229063 · agent 0 · cause Some(229062) · **Consumed**: batch=5 q=6 owner=Agent(0) nutrition_ms=64740000
- 47.083333h · event 229370 · agent 12 · cause Some(229369) · **Consumed**: batch=21 q=1 owner=Agent(12) nutrition_ms=46796000
- 47.083333h · event 229372 · agent 13 · cause Some(229371) · **Consumed**: batch=22 q=1 owner=Agent(13) nutrition_ms=46796000
- 48.000000h · event 232729 · agent None · cause None · **Spoiled**: batch=0 q=1568 holder=Deposit(2) created=0 deadline=172800000
- 48.000000h · event 232730 · agent None · cause None · **Spoiled**: batch=1 q=6 holder=Agent(29) created=0 deadline=172800000
- 48.000000h · event 232731 · agent None · cause None · **Spoiled**: batch=2 q=6 holder=Agent(30) created=0 deadline=172800000
- 48.750000h · event 235654 · agent 0 · cause Some(235590) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=58500000 fatigue_ms=72000000
- 57.490000h · event 263008 · agent 12 · cause Some(262996) · **PerformanceChanged**: old_ppm=800000 new_ppm=720000 nutrition_ms=9332000 fatigue_ms=43200000
- 57.490000h · event 263009 · agent 13 · cause Some(262997) · **PerformanceChanged**: old_ppm=800000 new_ppm=720000 nutrition_ms=9332000 fatigue_ms=43200000
- 60.082222h · event 268243 · agent 12 · cause Some(268223) · **PerformanceChanged**: old_ppm=720000 new_ppm=450000 nutrition_ms=0 fatigue_ms=52532000
- 60.082222h · event 268244 · agent 12 · cause Some(268243) · **StarvationStarted**: continuous_starvation_ms=0
- 60.082222h · event 268245 · agent 13 · cause Some(268224) · **PerformanceChanged**: old_ppm=720000 new_ppm=450000 nutrition_ms=0 fatigue_ms=52532000
- 60.082222h · event 268246 · agent 13 · cause Some(268245) · **StarvationStarted**: continuous_starvation_ms=0
- 61.490000h · event 271104 · agent 12 · cause Some(271088) · **PerformanceChanged**: old_ppm=450000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
- 61.490000h · event 271105 · agent 13 · cause Some(271089) · **PerformanceChanged**: old_ppm=450000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
- 65.000000h · event 278273 · agent 0 · cause Some(278245) · **PerformanceChanged**: old_ppm=800000 new_ppm=500000 nutrition_ms=0 fatigue_ms=41400000
- 65.000000h · event 278274 · agent 0 · cause Some(278273) · **StarvationStarted**: continuous_starvation_ms=0
- 65.490000h · event 279321 · agent 12 · cause Some(279305) · **PerformanceChanged**: old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
- 65.490000h · event 279322 · agent 13 · cause Some(279306) · **PerformanceChanged**: old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
- 65.500000h · event 279325 · agent 0 · cause Some(279293) · **PerformanceChanged**: old_ppm=500000 new_ppm=450000 nutrition_ms=0 fatigue_ms=43200000
- 69.500000h · event 288145 · agent 0 · cause Some(288109) · **PerformanceChanged**: old_ppm=450000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000

Event counts: Admitted=112, Arrived=108, Consumed=516, Decision=158432, Died=22, GatherStarted=1432, Gathered=1432, Interrupted=63887, JobAccepted=8, JobCancelled=8, LeaseExpired=2, PerformanceChanged=432, Queued=29489, RequestObserved=46, SleepStarted=37984, Spoiled=25, StarvationStarted=58

## stale_depletion

- 0.003334h · event 113 · agent 0 · cause Some(15) · **Admitted**: edge=0 from=0 to=1
- 2.003334h · event 5591 · agent 0 · cause Some(113) · **Arrived**: node=1 edge=0
- 2.003334h · event 5653 · agent 0 · cause Some(5596) · **Admitted**: edge=1 from=1 to=2
- 4.003334h · event 10655 · agent 0 · cause Some(5653) · **Arrived**: node=2 edge=1
- 4.003334h · event 10664 · agent 0 · cause Some(10655) · **StaleFoodTrip**: node=2 remembered food absent/inaccessible
- 6.000000h · event 15730 · agent 0 · cause Some(15688) · **PerformanceChanged**: old_ppm=1000000 new_ppm=800000 nutrition_ms=21600000 fatigue_ms=36000000
- 8.000000h · event 20895 · agent 0 · cause Some(20853) · **PerformanceChanged**: old_ppm=800000 new_ppm=720000 nutrition_ms=14400000 fatigue_ms=43200000
- 12.000000h · event 31095 · agent 0 · cause Some(31053) · **PerformanceChanged**: old_ppm=720000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000
- 12.000000h · event 31096 · agent 0 · cause Some(31095) · **StarvationStarted**: continuous_starvation_ms=0
- 16.000000h · event 41316 · agent 0 · cause Some(41274) · **PerformanceChanged**: old_ppm=375000 new_ppm=250000 nutrition_ms=0 fatigue_ms=72000000
- 36.000000h · event 177991 · agent 0 · cause Some(177895) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177992 · agent 1 · cause Some(177896) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177993 · agent 2 · cause Some(177897) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177994 · agent 3 · cause Some(177898) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177995 · agent 4 · cause Some(177899) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177996 · agent 5 · cause Some(177900) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177997 · agent 6 · cause Some(177901) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)
- 36.000000h · event 177998 · agent 7 · cause Some(177902) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(2)

Event counts: Admitted=64, Arrived=60, Consumed=1, Decision=124446, Died=50, FixtureComplete=1, GatherStarted=6, Gathered=6, Interrupted=21801, PerformanceChanged=200, Queued=10611, RequestObserved=14, SleepStarted=21423, StaleFoodTrip=28, StarvationStarted=50

## capacity_1

- 0.000000h · event 100 · agent 0 · cause Some(1) · **Admitted**: edge=0 from=0 to=1
- 2.000000h · event 5932 · agent 0 · cause Some(100) · **Arrived**: node=1 edge=0
- 2.000000h · event 5985 · agent 0 · cause Some(5934) · **Admitted**: edge=1 from=1 to=2
- 4.000000h · event 11698 · agent 0 · cause Some(5985) · **Arrived**: node=2 edge=1
- 4.000556h · event 11756 · agent 0 · cause Some(11755) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=32398000
- 4.017222h · event 11809 · agent 0 · cause Some(11808) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=35938000
- 4.033889h · event 11861 · agent 0 · cause Some(11860) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=39478000
- 4.050556h · event 11913 · agent 0 · cause Some(11912) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=43018000
- 4.067222h · event 11965 · agent 0 · cause Some(11964) · **Consumed**: batch=1 q=1 owner=Agent(0) nutrition_ms=46558000
- 5.016667h · event 14674 · agent 0 · cause Some(14673) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 8.000000h · event 23407 · agent 0 · cause Some(23359) · **PerformanceChanged**: old_ppm=1000000 new_ppm=900000 nutrition_ms=54000000 fatigue_ms=43200000
- 11.016667h · event 32231 · agent 0 · cause Some(32230) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 12.000000h · event 35155 · agent 0 · cause Some(35107) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=61200000 fatigue_ms=57600000
- 16.000000h · event 46828 · agent 0 · cause Some(46780) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=46800000 fatigue_ms=72000000
- 17.016667h · event 49885 · agent 0 · cause Some(49884) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 23.016667h · event 87067 · agent 0 · cause Some(87066) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 23.050000h · event 87279 · agent 0 · cause Some(87278) · **PerformanceChanged**: old_ppm=1000000 new_ppm=900000 nutrition_ms=64620000 fatigue_ms=43200000
- 27.050000h · event 113320 · agent 0 · cause Some(113254) · **PerformanceChanged**: old_ppm=900000 new_ppm=750000 nutrition_ms=50220000 fatigue_ms=57600000
- 29.016667h · event 126092 · agent 0 · cause Some(126091) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 31.050000h · event 139439 · agent 0 · cause Some(139368) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57420000 fatigue_ms=72000000
- 35.016667h · event 165767 · agent 0 · cause Some(165766) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 36.000000h · event 172024 · agent 7 · cause Some(152989) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 1, from: 1, to: 2, progress: 2625000000000 }
- 36.000000h · event 172025 · agent 8 · cause Some(146258) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Edge { edge: 0, from: 0, to: 1, progress: 3528000000000 }
- 36.000000h · event 172026 · agent 9 · cause Some(171983) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 172027 · agent 10 · cause Some(171984) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 172028 · agent 11 · cause Some(171985) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 172029 · agent 12 · cause Some(171986) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 172030 · agent 13 · cause Some(171987) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.000000h · event 172031 · agent 14 · cause Some(171988) · **Died**: cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
- 36.950000h · event 172743 · agent 0 · cause Some(172729) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57780000 fatigue_ms=72000000
- 41.016667h · event 176261 · agent 0 · cause Some(176260) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 43.150000h · event 177578 · agent 0 · cause Some(177571) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=57060000 fatigue_ms=72000000
- 47.016667h · event 180346 · agent 0 · cause Some(180345) · **Consumed**: batch=1 q=6 owner=Agent(0) nutrition_ms=64740000
- 48.000000h · event 181043 · agent None · cause None · **Spoiled**: batch=0 q=2642 holder=Deposit(2) created=0 deadline=172800000
- 48.000000h · event 181044 · agent None · cause None · **Spoiled**: batch=1 q=6 holder=Agent(0) created=0 deadline=172800000
- 48.000000h · event 181045 · agent None · cause None · **Spoiled**: batch=2 q=6 holder=Agent(1) created=0 deadline=172800000
- 48.750000h · event 181671 · agent 0 · cause Some(181657) · **PerformanceChanged**: old_ppm=750000 new_ppm=500000 nutrition_ms=58500000 fatigue_ms=72000000
- 65.000000h · event 194360 · agent 0 · cause Some(194353) · **PerformanceChanged**: old_ppm=800000 new_ppm=500000 nutrition_ms=0 fatigue_ms=41400000
- 65.000000h · event 194361 · agent 0 · cause Some(194360) · **StarvationStarted**: continuous_starvation_ms=0
- 65.500000h · event 194616 · agent 0 · cause Some(194609) · **PerformanceChanged**: old_ppm=500000 new_ppm=450000 nutrition_ms=0 fatigue_ms=43200000
- 69.500000h · event 196621 · agent 0 · cause Some(196614) · **PerformanceChanged**: old_ppm=450000 new_ppm=375000 nutrition_ms=0 fatigue_ms=57600000

Event counts: Admitted=17, Arrived=15, Consumed=116, Decision=121447, Died=43, GatherStarted=358, Gathered=358, Interrupted=37240, PerformanceChanged=258, Queued=15222, RequestObserved=13, SleepStarted=22735, Spoiled=8, StarvationStarted=53

