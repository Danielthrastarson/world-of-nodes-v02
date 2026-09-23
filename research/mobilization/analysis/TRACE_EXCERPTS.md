
## small enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	7	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	9	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	11	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	29	3660000	Some(0)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	31	3660000	Some(1)	ServiceEnded	unit=0; active=false; reason=LOCAL_ADMISSION_CLOSED
0	33	3660000	Some(2)	ServiceEnded	unit=0; active=false; reason=LOCAL_ADMISSION_CLOSED
0	35	3660000	Some(3)	ServiceEnded	unit=0; active=false; reason=LOCAL_ADMISSION_CLOSED
0	513	10860000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
```

## large enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	7	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	9	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	11	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	29	3660000	Some(0)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	31	3660000	Some(1)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	33	3660000	Some(2)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	35	3660000	Some(3)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	513	10860000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	515	10860000	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	517	10860000	Some(2)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	519	10860000	Some(3)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
```

## food_constrained enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Travel { edge: 0, target: 1, food: false }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Travel { edge: 0, target: 1, food: false }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Travel { edge: 0, target: 1, food: false }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Travel { edge: 0, target: 1, food: false }
0	24	3600000	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	25	3600000	Some(1)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	26	3600000	Some(2)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	27	3600000	Some(3)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	32	3660000	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	33	3660000	Some(1)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	34	3660000	Some(2)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	35	3660000	Some(3)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	40	3720000	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	41	3720000	Some(1)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	42	3720000	Some(2)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	43	3720000	Some(3)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	2212	36060000	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2213	36060000	Some(1)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2214	36060000	Some(2)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2215	36060000	Some(3)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2220	36120000	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2221	36120000	Some(1)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2222	36120000	Some(2)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2223	36120000	Some(3)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2228	36180000	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2229	36180000	Some(1)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2230	36180000	Some(2)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2231	36180000	Some(3)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2236	36240000	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2237	36240000	Some(1)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2238	36240000	Some(2)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	2239	36240000	Some(3)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
```

## caregiver enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=HIGHER_PRIORITY; selected=GiveCareFood { dependent: 4, batch: 0, quantity: 6 }
0	2	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	4	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	7	0	Some(0)	CareTransferred	dependent=4 quantity=6 from_batch=0 to_batch=8 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	10	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	12	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	14	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	16	60000	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	17	60000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Eat { batch: 8, quantity: 6 }
0	19	60000	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	31	120000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	36	180000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	38	240000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	40	300000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	42	360000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	44	420000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	46	480000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	48	540000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	50	600000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	52	660000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	54	720000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	56	780000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	58	840000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	60	900000	Some(4)	ServiceEvaluation	request=0; gate=CHILD; selected=Wait
0	111	3660000	Some(1)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	113	3660000	Some(2)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	115	3660000	Some(3)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	119	3720000	Some(0)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10920000
0	715	10860000	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	717	10860000	Some(2)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	719	10860000	Some(3)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	726	10920000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	979	14580000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=9 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	994	14700000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=9 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	1009	14820000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=9 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	1024	14940000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=9 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	1039	15060000	Some(0)	CareTransferred	dependent=4 quantity=1 from_batch=9 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
0	1300	18060000	Some(0)	CareTransferred	dependent=4 quantity=6 from_batch=9 to_batch=10 node=0 owner=Agent(4) custody=Loose; recipient must eat normally
```

## farmer enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	7	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	9	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	11	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	29	3660000	Some(0)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	31	3660000	Some(1)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	33	3660000	Some(2)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	35	3660000	Some(3)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	513	10860000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	515	10860000	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	517	10860000	Some(2)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	519	10860000	Some(3)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
```

## working_farmer enabled, seed0

```text
0	120	1800000	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	121	1800000	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	122	1800000	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	123	1800000	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	125	1800000	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	127	1800000	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	129	1800000	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	131	1800000	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	149	5460000	Some(0)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=12660000
0	151	5460000	Some(1)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=12660000
0	153	5460000	Some(2)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=12660000
0	155	5460000	Some(3)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=12660000
0	633	12660000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	635	12660000	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	637	12660000	Some(2)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	639	12660000	Some(3)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
```

## distant enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	7	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	9	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	11	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	37	21720000	Some(0)	MusterArrived	unit=0; travel_ms=21720000; same Person; until=28920000
0	39	21720000	Some(1)	MusterArrived	unit=0; travel_ms=21720000; same Person; until=28920000
0	41	21720000	Some(2)	MusterArrived	unit=0; travel_ms=21720000; same Person; until=28920000
0	43	21720000	Some(3)	MusterArrived	unit=0; travel_ms=21720000; same Person; until=28920000
0	525	28920000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	527	28920000	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	529	28920000	Some(2)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	531	28920000	Some(3)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
```

## bottleneck enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	7	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	9	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	11	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	200	3660000	Some(0)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=10860000
0	385	7260000	Some(1)	MusterArrived	unit=0; travel_ms=7260000; same Person; until=14460000
0	568	10860000	Some(0)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	572	10860000	Some(2)	MusterArrived	unit=0; travel_ms=10860000; same Person; until=18060000
0	754	14460000	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	759	14460000	Some(3)	MusterArrived	unit=0; travel_ms=14460000; same Person; until=21660000
0	997	18060000	Some(2)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	1239	21660000	Some(3)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
```

## unknown enabled, seed0

```text

```

## history_help enabled, seed0

```text
0	4	3600001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Travel { edge: 0, target: 1, food: false }
0	5	3600001	Some(1)	ServiceEvaluation	request=0; gate=HIGHER_PRIORITY; selected=GiveCareFood { dependent: 0, batch: 0, quantity: 6 }
0	11	3600001	Some(1)	CareTransferred	dependent=0 quantity=6 from_batch=0 to_batch=1 node=0 owner=Agent(0) custody=Loose; recipient must eat normally
0	15	3660001	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	17	3660001	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	25	7320001	Some(1)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=14520001
0	84	10800001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	87	10860001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	90	10920001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	93	10980001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	96	11040001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	99	11100001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	102	11160001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	105	11220001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	108	11280001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	111	11340001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	114	11400001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	117	11460001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	120	11520001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	123	11580001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	126	11640001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	224	14520001	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	943	36000001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	946	36060001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	949	36120001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	952	36180001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	955	36240001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	958	36300001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	961	36360001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	964	36420001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	967	36480001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	970	36540001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	973	36600001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	976	36660001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	979	36720001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	982	36780001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	985	36840001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	988	36900001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
```

## history_refusal enabled, seed0

```text
0	4	3600001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Travel { edge: 0, target: 1, food: false }
0	5	3600001	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	10	3600001	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	19	7260001	Some(1)	MusterArrived	unit=0; travel_ms=3660000; same Person; until=14460001
0	79	10800001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	83	10860001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	87	10920001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	91	10980001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	95	11040001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	99	11100001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	103	11160001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	107	11220001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	111	11280001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	115	11340001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	119	11400001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	123	11460001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	127	11520001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	131	11580001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	135	11640001	Some(0)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Wait
0	278	14460001	Some(1)	ServiceEnded	unit=0; active=true; reason=TERM_COMPLETED
0	1180	36000001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1183	36060001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1186	36120001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1189	36180001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1192	36240001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1195	36300001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1198	36360001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1201	36420001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1204	36480001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1207	36540001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1210	36600001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1213	36660001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1216	36720001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1219	36780001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1222	36840001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
0	1225	36900001	Some(0)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Wait
```

## civilian_traffic enabled, seed0

```text
0	0	0	Some(0)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1	0	Some(1)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2	0	Some(2)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3	0	Some(3)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5	0	Some(0)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	7	0	Some(1)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	9	0	Some(2)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	11	0	Some(3)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	1152	4500000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1172	4502500	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1175	4560000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1193	4562500	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1196	4620000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1214	4622500	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1217	4680000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1235	4682000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1238	4740000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1256	4742000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1259	4800000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1277	4802000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1280	4860000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1298	4862000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1301	4920000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Gather { batch: 0, communal: false }
0	1319	4922000	Some(4)	ServiceEvaluation	request=0; gate=SIX_HOUR_MARGIN; selected=Eat { batch: 9, quantity: 1 }
0	1930	7320000	Some(4)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	1936	7320000	Some(4)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	2895	10920000	Some(5)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	2902	10920000	Some(5)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	3544	13500000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3564	13502500	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3567	13560000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3585	13562500	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3588	13620000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3606	13622500	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3609	13680000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3627	13682500	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3630	13740000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3648	13742500	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3651	13800000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3669	13802000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3672	13860000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3690	13862000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3693	13920000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Gather { batch: 0, communal: false }
0	3711	13922000	Some(6)	ServiceEvaluation	request=0; gate=INSUFFICIENT_TIME; selected=Eat { batch: 11, quantity: 1 }
0	3735	14040000	Some(0)	ServiceEnded	unit=0; active=false; reason=VOLUNTARY_WITHDRAWAL
0	3738	14040000	Some(1)	ServiceEnded	unit=0; active=false; reason=VOLUNTARY_WITHDRAWAL
0	3741	14040000	Some(2)	ServiceEnded	unit=0; active=false; reason=VOLUNTARY_WITHDRAWAL
0	3744	14040000	Some(3)	ServiceEnded	unit=0; active=false; reason=VOLUNTARY_WITHDRAWAL
0	3747	14040000	Some(4)	ServiceEnded	unit=0; active=false; reason=VOLUNTARY_WITHDRAWAL
0	3750	14040000	Some(5)	ServiceEnded	unit=0; active=false; reason=VOLUNTARY_WITHDRAWAL
0	3762	14042000	Some(6)	ServiceEvaluation	request=0; gate=HIGHER_PRIORITY; selected=Eat { batch: 11, quantity: 1 }
0	3807	14100000	Some(6)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	3815	14100000	Some(6)	Volunteered	request=0; previous=Idle; no role or reservation yet
0	5015	18660000	Some(7)	ServiceEvaluation	request=0; gate=PASS; selected=Volunteer { request: 0 }
0	5024	18660000	Some(7)	Volunteered	request=0; previous=Idle; no role or reservation yet
```
