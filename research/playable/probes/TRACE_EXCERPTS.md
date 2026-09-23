# Exact final trace excerpts

Controlled seed-0 initial conditions and typed inputs; all times are absolute
simulation milliseconds. Event details below are copied from the preserved full
trace. A received objective is not proof of physical completion.

## local-commission-0

Source: [events.tsv.gz](scenarios-3seeds/local-commission-0/events.tsv.gz) and
[typed transcript](scenarios-3seeds/local-commission-0/transcript.txt).

```text
id	at_ms	person	kind	detail
18	0	Some(0)	OrderIssued	Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 0, expires_at: 172800000 }; no recipient state changed
19	0	Some(1)	CommandReceived	Transmission { message: 0, sender: 0, recipient: 1, node: 1, issued_at: 0, received_at: 0 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 0, expires_at: 172800000 })
20	0	Some(1)	RoleObligation	command=0; status=Received; reason=VALID_SCOPED_OFFICE_ORDER
22	0	Some(1)	RoleObligation	command=0; status=Active; reason=PERSON_SELECTED_ROLE_UNDERTAKING
32	0	Some(1)	RoleObligation	command=0; status=Active; reason=ATTEMPT_LOCAL_COMMISSION
33	0	Some(1)	CrossingCommissioned	Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 0 }; no resources or workers assigned
34	0	Some(1)	RoleObligation	command=0; status=Completed; reason=LOCAL_COMMISSION_AUTHORIZED_NOT_PHYSICAL_SUCCESS
14251	46860000	Some(0)	CrossingCompleted	project=0 owner=Commons(0); adjacent observation required; existing traversals unchanged
```

## remote-commission-0

Source: [events.tsv.gz](scenarios-3seeds/remote-commission-0/events.tsv.gz) and
[typed transcript](scenarios-3seeds/remote-commission-0/transcript.txt).

```text
id	at_ms	person	kind	detail
302	900000	Some(0)	OrderIssued	Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 900000, expires_at: 173700000 }; no recipient state changed
303	900000	Some(5)	CommandReceived	Transmission { message: 0, sender: 0, recipient: 5, node: 0, issued_at: 900000, received_at: 900000 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 900000, expires_at: 173700000 })
304	900000	Some(5)	CommandDeliveryStarted	message=0; recipient=1; rendezvous=1; controlled voluntarily accepted purpose; no remote tracking
552	1856400	Some(1)	CommandReceived	Transmission { message: 0, sender: 5, recipient: 1, node: 1, issued_at: 900000, received_at: 1856400 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 900000, expires_at: 173700000 })
554	1856400	Some(5)	CommandDeliveryCompleted	message=0; recipient=1
563	1860000	Some(1)	CrossingCommissioned	Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 1860000 }; no resources or workers assigned
14756	49100000	Some(2)	CrossingCompleted	project=0 owner=Commons(0); adjacent observation required; existing traversals unchanged
```

## local-commission_revoke-0

Source: [events.tsv.gz](scenarios-3seeds/local-commission_revoke-0/events.tsv.gz) and
[typed transcript](scenarios-3seeds/local-commission_revoke-0/transcript.txt).

```text
id	at_ms	person	kind	detail
2134	7200000	Some(0)	RevocationIssued	Revocation { id: 1, issuer: 0, recipient: 1, office: 0, command: 0, issued_at: 7200000 }; recipient continues until actual receipt
2136	7200000	Some(1)	RevocationReceived	message=1; command=0; completed=true; no retroactive undo of lawful commission or physical work
```

## local-attack-0

Source: [events.tsv.gz](scenarios-3seeds/local-attack-0/events.tsv.gz) and
[typed transcript](scenarios-3seeds/local-attack-0/transcript.txt).

```text
id	at_ms	person	kind	detail
24	0	Some(0)	PlayerUnitOrder	unit=0; order=Attack; node=1; target=2; local_recipients=[0, 6, 7, 8]; voluntary command agreement
3260	14385600	Some(0)	UnitArrived	serial=0; physical state only
3261	14385600	Some(6)	UnitArrived	serial=0; physical state only
3262	14385600	Some(7)	UnitArrived	serial=0; physical state only
3263	14385600	Some(8)	UnitArrived	serial=0; physical state only
3278	14400000	Some(0)	StrikeStarted	target=9 location=Node(2)
3592	15300000	Some(6)	Injured	before=0 after=1 hits=1; own body; no unit HP
3593	15300000	Some(7)	Injured	before=0 after=1 hits=1; own body; no unit HP
3594	15300000	Some(8)	Injured	before=0 after=1 hits=1; own body; no unit HP
3595	15300000	Some(9)	Injured	before=0 after=2 hits=2; own body; no unit HP
3596	15300000	Some(10)	Injured	before=0 after=1 hits=1; own body; no unit HP
3597	15300000	Some(11)	Injured	before=0 after=1 hits=1; own body; no unit HP
3906	16200000	Some(10)	Injured	before=1 after=2 hits=1; own body; no unit HP
4216	17100000	Some(8)	Injured	before=1 after=2 hits=1; own body; no unit HP
4217	17100000	Some(11)	Injured	before=1 after=3 hits=2; own body; no unit HP
4218	17100000	Some(11)	Died	cause=contact nutrition_ms=69300000 starvation_ms=0 location=Node(2)
```

## remote-none-0

Source: [events.tsv.gz](scenarios-3seeds/remote-none-0/events.tsv.gz) and
[typed transcript](scenarios-3seeds/remote-none-0/transcript.txt).

```text
id	at_ms	person	kind	detail
20	0	Some(0)	ServiceEnded	unit=0; active=true; reason=PRIORITY_OVERRIDE
```

Remote Attack input at 900,000ms is rejected with `commander lacks active
voluntary service in the agreed unit`; see its [typed transcript](scenarios-3seeds/remote-attack-0/transcript.txt).
Its absence of an executed order is not an information transmission failure or
a combat failure.
