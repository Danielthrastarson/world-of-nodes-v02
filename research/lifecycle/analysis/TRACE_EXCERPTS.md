# B3 focal causal chains

Seed0; authoritative milliseconds. Full raw traces remain in each fixture. Controlled support and Partner seeding are research interventions, not NPC omniscience or autonomous mate search.

## basic

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	3	0	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	4	0	Some(0)	ReproductionStarted	partner=1 until=3600000
0	123	3600000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	124	3600000	Some(0)	PregnancyStarted	other_parent=1 due=32400000
0	1101	32400000	Some(2)	Born	parents=[0,1] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	1105	32400000	Some(0)	CareTransferred	dependent=2 quantity=6 from_batch=4 to_batch=6 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	1108	32400000	Some(1)	CareTransferred	dependent=2 quantity=6 from_batch=5 to_batch=7 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2213	54060000	Some(0)	CareTransferred	dependent=2 quantity=6 from_batch=4 to_batch=6 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2216	54060000	Some(1)	CareTransferred	dependent=2 quantity=6 from_batch=5 to_batch=7 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	5846	97260000	Some(0)	CareTransferred	dependent=2 quantity=6 from_batch=4 to_batch=6 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	5849	97260000	Some(1)	CareTransferred	dependent=2 quantity=6 from_batch=5 to_batch=7 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	7182	118800000	Some(2)	Matured	same Person identity, possessions, beliefs and history
0	12243	162180000	Some(0)	CareTransferred	dependent=2 quantity=6 from_batch=4 to_batch=6 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	12247	162180000	Some(1)	CareTransferred	dependent=2 quantity=6 from_batch=5 to_batch=7 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	12251	162240000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	12253	162240000	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	12256	162240000	Some(0)	ReproductionStarted	partner=1 until=165840000
0	12435	165840000	Some(0)	ConceptionTrial	other_parent=1 roll=545679 threshold=1000000
0	12436	165840000	Some(0)	PregnancyStarted	other_parent=1 due=194640000
0	13900	194640000	Some(3)	Born	parents=[0,1] sex=Male generation=2 birth_time=194640000 due=194640000 node=0; nutrition=0 goods=0
```

## decline

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	4	60000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	7	120000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	10	180000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	13	240000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	16	300000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	19	360000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	22	420000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	25	480000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	28	540000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	31	600000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	34	660000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	37	720000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	40	780000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	43	840000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	46	900000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	49	960000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	52	1020000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	55	1080000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	58	1140000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	61	1200000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	64	1260000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	67	1320000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	70	1380000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	73	1440000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
```

## dependency_social

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	3	0	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	4	0	Some(0)	ReproductionStarted	partner=1 until=3600000
0	123	3600000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	124	3600000	Some(0)	PregnancyStarted	other_parent=1 due=32400000
0	1101	32400000	Some(2)	Born	parents=[0,1] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	9276	118800000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	12892	162240000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	12894	162240000	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	12895	162240000	Some(0)	ReproductionStarted	partner=1 until=165840000
0	13014	165840000	Some(0)	ConceptionTrial	other_parent=1 roll=545679 threshold=1000000
0	13015	165840000	Some(0)	PregnancyStarted	other_parent=1 due=194640000
0	13984	194640000	Some(3)	Born	parents=[0,1] sex=Male generation=2 birth_time=194640000 due=194640000 node=0; nutrition=0 goods=0
```

## parent_death

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	3	0	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	4	0	Some(0)	ReproductionStarted	partner=1 until=3600000
0	123	3600000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	124	3600000	Some(0)	PregnancyStarted	other_parent=1 due=32400000
0	1101	32400000	Some(2)	Born	parents=[0,1] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	1105	32400000	Some(0)	CareTransferred	dependent=2 quantity=6 from_batch=4 to_batch=6 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	1108	32400000	Some(1)	CareTransferred	dependent=2 quantity=6 from_batch=5 to_batch=7 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	1305	36000000	None	ResearchFatalExposure	both initial caregivers set to lethal starvation as controlled death fixture
0	1306	36000001	Some(0)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400001 location=Node(0)
0	1307	36000001	Some(1)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400001 location=Node(0)
0	2932	118800000	Some(2)	Matured	same Person identity, possessions, beliefs and history
0	3816	162060000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## multigeneration

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	3	0	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	5	0	Some(2)	ReproductionConsent	partner=3; no unilateral conception
0	7	0	Some(3)	ReproductionConsent	partner=2; no unilateral conception
0	9	0	Some(4)	ReproductionConsent	partner=5; no unilateral conception
0	11	0	Some(5)	ReproductionConsent	partner=4; no unilateral conception
0	13	0	Some(6)	ReproductionConsent	partner=7; no unilateral conception
0	15	0	Some(7)	ReproductionConsent	partner=6; no unilateral conception
0	16	0	Some(0)	ReproductionStarted	partner=1 until=3600000
0	17	0	Some(2)	ReproductionStarted	partner=3 until=3600000
0	18	0	Some(4)	ReproductionStarted	partner=5 until=3600000
0	19	0	Some(6)	ReproductionStarted	partner=7 until=3600000
0	492	3600000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	493	3600000	Some(0)	PregnancyStarted	other_parent=1 due=32400000
0	494	3600000	Some(2)	ConceptionTrial	other_parent=3 roll=146531 threshold=1000000
0	495	3600000	Some(2)	PregnancyStarted	other_parent=3 due=32400000
0	496	3600000	Some(4)	ConceptionTrial	other_parent=5 roll=920533 threshold=1000000
0	497	3600000	Some(4)	PregnancyStarted	other_parent=5 due=32400000
0	498	3600000	Some(6)	ConceptionTrial	other_parent=7 roll=99594 threshold=1000000
0	499	3600000	Some(6)	PregnancyStarted	other_parent=7 due=32400000
0	4404	32400000	Some(8)	Born	parents=[0,1] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4405	32400000	Some(9)	Born	parents=[2,3] sex=Female generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4406	32400000	Some(10)	Born	parents=[4,5] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4407	32400000	Some(11)	Born	parents=[6,7] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4417	32400000	Some(0)	CareTransferred	dependent=8 quantity=6 from_batch=16 to_batch=24 node=0 owner=Agent(8) custody=Loose; recipient must eat normally
0	28728	118800000	Some(8)	Matured	same Person identity, possessions, beliefs and history
0	28729	118800000	Some(9)	Matured	same Person identity, possessions, beliefs and history
0	28730	118800000	Some(10)	Matured	same Person identity, possessions, beliefs and history
0	28731	118800000	Some(11)	Matured	same Person identity, possessions, beliefs and history
0	28752	118800000	None	ResearchPartnerSeed	adults=8,9; controlled relationship initial condition, not mate search
0	49025	162240000	Some(0)	ReproductionStarted	partner=1 until=165840000
0	49026	162240000	Some(2)	ReproductionStarted	partner=3 until=165840000
0	49027	162240000	Some(4)	ReproductionStarted	partner=5 until=165840000
0	49028	162240000	Some(6)	ReproductionStarted	partner=7 until=165840000
0	49741	165840000	Some(0)	ConceptionTrial	other_parent=1 roll=545679 threshold=1000000
0	49742	165840000	Some(0)	PregnancyStarted	other_parent=1 due=194640000
0	49743	165840000	Some(2)	ConceptionTrial	other_parent=3 roll=116581 threshold=1000000
0	49744	165840000	Some(2)	PregnancyStarted	other_parent=3 due=194640000
0	49745	165840000	Some(4)	ConceptionTrial	other_parent=5 roll=215947 threshold=1000000
0	49746	165840000	Some(4)	PregnancyStarted	other_parent=5 due=194640000
0	49747	165840000	Some(6)	ConceptionTrial	other_parent=7 roll=152576 threshold=1000000
0	49748	165840000	Some(6)	PregnancyStarted	other_parent=7 due=194640000
0	55601	194640000	Some(12)	Born	parents=[0,1] sex=Male generation=2 birth_time=194640000 due=194640000 node=0; nutrition=0 goods=0
0	55602	194640000	Some(13)	Born	parents=[2,3] sex=Male generation=2 birth_time=194640000 due=194640000 node=0; nutrition=0 goods=0
0	55603	194640000	Some(14)	Born	parents=[4,5] sex=Female generation=2 birth_time=194640000 due=194640000 node=0; nutrition=0 goods=0
0	55604	194640000	Some(15)	Born	parents=[6,7] sex=Female generation=2 birth_time=194640000 due=194640000 node=0; nutrition=0 goods=0
```

## generational_control

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	3	0	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	5	0	Some(2)	ReproductionConsent	partner=3; no unilateral conception
0	7	0	Some(3)	ReproductionConsent	partner=2; no unilateral conception
0	9	0	Some(4)	ReproductionConsent	partner=5; no unilateral conception
0	11	0	Some(5)	ReproductionConsent	partner=4; no unilateral conception
0	13	0	Some(6)	ReproductionConsent	partner=7; no unilateral conception
0	15	0	Some(7)	ReproductionConsent	partner=6; no unilateral conception
0	16	0	Some(0)	ReproductionStarted	partner=1 until=3600000
0	17	0	Some(2)	ReproductionStarted	partner=3 until=3600000
0	18	0	Some(4)	ReproductionStarted	partner=5 until=3600000
0	19	0	Some(6)	ReproductionStarted	partner=7 until=3600000
0	492	3600000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	493	3600000	Some(0)	PregnancyStarted	other_parent=1 due=32400000
0	494	3600000	Some(2)	ConceptionTrial	other_parent=3 roll=146531 threshold=1000000
0	495	3600000	Some(2)	PregnancyStarted	other_parent=3 due=32400000
0	496	3600000	Some(4)	ConceptionTrial	other_parent=5 roll=920533 threshold=1000000
0	497	3600000	Some(4)	PregnancyStarted	other_parent=5 due=32400000
0	498	3600000	Some(6)	ConceptionTrial	other_parent=7 roll=99594 threshold=1000000
0	499	3600000	Some(6)	PregnancyStarted	other_parent=7 due=32400000
0	4404	32400000	Some(8)	Born	parents=[0,1] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4405	32400000	Some(9)	Born	parents=[2,3] sex=Female generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4406	32400000	Some(10)	Born	parents=[4,5] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4407	32400000	Some(11)	Born	parents=[6,7] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	4417	32400000	Some(0)	CareTransferred	dependent=8 quantity=6 from_batch=16 to_batch=24 node=0 owner=Agent(8) custody=Loose; recipient must eat normally
0	29496	118800000	Some(8)	Matured	same Person identity, possessions, beliefs and history
0	29497	118800000	Some(9)	Matured	same Person identity, possessions, beliefs and history
0	29498	118800000	Some(10)	Matured	same Person identity, possessions, beliefs and history
0	29499	118800000	Some(11)	Matured	same Person identity, possessions, beliefs and history
0	29528	118800000	None	ResearchPartnerSeed	adults=8,9; controlled relationship initial condition, not mate search
0	39377	140580000	Some(8)	ReproductionStarted	partner=9 until=144180000
0	40584	144180000	Some(9)	ConceptionTrial	other_parent=8 roll=793564 threshold=1000000
0	40585	144180000	Some(9)	PregnancyStarted	other_parent=8 due=172980000
0	49486	165660000	Some(0)	ReproductionStarted	partner=1 until=169260000
0	49487	165660000	Some(2)	ReproductionStarted	partner=3 until=169260000
0	49488	165660000	Some(4)	ReproductionStarted	partner=5 until=169260000
0	49489	165660000	Some(6)	ReproductionStarted	partner=7 until=169260000
0	51018	172980000	Some(12)	Born	parents=[9,8] sex=Female generation=3 birth_time=172980000 due=172980000 node=0; nutrition=0 goods=0
```

## working

```text
0	1	0	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	3	0	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	4	0	Some(0)	ReproductionStarted	partner=1 until=3600000
0	123	3600000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	124	3600000	Some(0)	PregnancyStarted	other_parent=1 due=32400000
0	1095	32400000	Some(2)	Born	parents=[0,1] sex=Male generation=2 birth_time=32400000 due=32400000 node=0; nutrition=0 goods=0
0	2654	63180000	Some(0)	CareTransferred	dependent=2 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2657	63180000	Some(1)	CareTransferred	dependent=2 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2669	63300000	Some(0)	CareTransferred	dependent=2 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2672	63300000	Some(1)	CareTransferred	dependent=2 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2685	63420000	Some(0)	CareTransferred	dependent=2 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2688	63420000	Some(1)	CareTransferred	dependent=2 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2701	63540000	Some(0)	CareTransferred	dependent=2 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	2704	63540000	Some(1)	CareTransferred	dependent=2 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	6563	97200000	Some(0)	CareTransferred	dependent=2 quantity=3 from_batch=6 to_batch=8 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	6567	97200000	Some(1)	CareTransferred	dependent=2 quantity=3 from_batch=7 to_batch=9 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	7895	118800000	Some(2)	Matured	same Person identity, possessions, beliefs and history
0	14813	205260000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```

## seeded_three

```text
0	3	0	Some(0)	CareTransferred	dependent=2 quantity=6 from_batch=0 to_batch=4 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	6	0	Some(1)	CareTransferred	dependent=2 quantity=6 from_batch=2 to_batch=5 node=0 owner=Agent(2) custody=Loose; recipient must eat normally
0	12	60000	Some(0)	ReproductionConsent	partner=1; no unilateral conception
0	14	60000	Some(1)	ReproductionConsent	partner=0; no unilateral conception
0	19	60000	Some(0)	ReproductionStarted	partner=1 until=3660000
0	316	3660000	Some(0)	ConceptionTrial	other_parent=1 roll=607535 threshold=1000000
0	317	3660000	Some(0)	PregnancyStarted	other_parent=1 due=32460000
0	328	3720000	Some(0)	CareTransferred	dependent=3 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	331	3720000	Some(1)	CareTransferred	dependent=3 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	347	3840000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=6 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	350	3840000	Some(1)	CareTransferred	dependent=4 quantity=1 from_batch=7 to_batch=11 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	367	3960000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=6 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	370	3960000	Some(1)	CareTransferred	dependent=4 quantity=1 from_batch=7 to_batch=11 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	387	4080000	Some(0)	CareTransferred	dependent=3 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	390	4080000	Some(1)	CareTransferred	dependent=3 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	407	4200000	Some(0)	CareTransferred	dependent=3 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	410	4200000	Some(1)	CareTransferred	dependent=3 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	427	4320000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=6 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	430	4320000	Some(1)	CareTransferred	dependent=4 quantity=1 from_batch=7 to_batch=11 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	447	4440000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=6 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	450	4440000	Some(1)	CareTransferred	dependent=4 quantity=1 from_batch=7 to_batch=11 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	467	4560000	Some(0)	CareTransferred	dependent=3 quantity=1 from_batch=6 to_batch=8 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	470	4560000	Some(1)	CareTransferred	dependent=3 quantity=1 from_batch=7 to_batch=9 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	1024	10980000	Some(0)	CareTransferred	dependent=3 quantity=6 from_batch=6 to_batch=8 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	1027	10980000	Some(1)	CareTransferred	dependent=3 quantity=6 from_batch=7 to_batch=9 node=0 owner=Agent(3) custody=Loose; recipient must eat normally
0	2865	32460000	Some(5)	Born	parents=[0,1] sex=Male generation=2 birth_time=32460000 due=32460000 node=0; nutrition=0 goods=0
0	10680	86400000	Some(2)	Matured	same Person identity, possessions, beliefs and history
0	10681	86400000	Some(3)	Matured	same Person identity, possessions, beliefs and history
0	10682	86400000	Some(4)	Matured	same Person identity, possessions, beliefs and history
0	14538	118860000	Some(5)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	15565	129660000	Some(2)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	18438	162000000	Some(0)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	18439	162000000	Some(1)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	18448	162180000	Some(3)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
0	18612	169500000	Some(4)	Died	cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(0)
```
