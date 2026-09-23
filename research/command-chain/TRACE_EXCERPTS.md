# F13B selected causal traces — paired seed 0

These excerpts are from the final registered matrix, source revision
`ff7052773c97be2d202dbb0b1e7a9bb3a03d2f6e` as recorded in
[`matrix/provenance.json`](matrix/provenance.json). All ten paired seed-0 traces
were inspected. They illustrate mechanisms and limitations; they are not averages
or a claim that every seed has identical timing. Each run's `parameters.txt`,
`results.csv`, `timeline.csv` and `institutions.txt` sit beside its compressed event
file. Milliseconds below are exact; hour conversions are rounded.

Appointment/acceptance, initial courier purpose, ruler issuance and the refusal
intervention are controlled setup/actions. Person arbitration, ordinary physical
travel, survival, gathering, donations and construction execute through existing
mechanics. The paired arms carry matched dated factual reports; only the treatment
adds the scoped order. No control trace in the ten seed-0 pairs contains an
`OrderIssued`, `RoleObligation` or `CrossingCommissioned` event.

## Local: authority creates an obligation, then a commission

[`matrix/local_order_000/events.tsv.gz`](matrix/local_order_000/events.tsv.gz); paired [`matrix/local_control_000/events.tsv.gz`](matrix/local_control_000/events.tsv.gz).

The office is explicitly accepted locally at 0ms. Issuance at 16 minutes
changes the issuer's held message, not the absent steward. Courier 2 arrives and
hands over at 32 minutes. The next policy opportunity at 33 minutes selects the
role action. The obligation completes by authorizing a local project, not by
building it.

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 13 | 960000; 0.266667h | Some(0) | `OrderIssued` — `Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 960000, expires_at: 86400000 }; no recipient state changed` |
| 19 | 1920000; 0.533333h | Some(2) | `Arrived` — `node=1 edge=0` |
| 21 | 1920000; 0.533333h | Some(1) | `CommandReceived` — `Transmission { message: 0, sender: 2, recipient: 1, node: 1, issued_at: 960000, received_at: 1920000 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 960000, expires_at: 86400000 })` |
| 22 | 1920000; 0.533333h | Some(1) | `RoleObligation` — `command=0; status=Received; reason=VALID_SCOPED_OFFICE_ORDER` |
| 26 | 1980000; 0.550000h | Some(1) | `RoleObligation` — `command=0; status=Active; reason=PERSON_SELECTED_ROLE_UNDERTAKING` |
| 30 | 1980000; 0.550000h | Some(1) | `CrossingCommissioned` — `Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 1980000 }; no resources or workers assigned` |
| 31 | 1980000; 0.550000h | Some(1) | `RoleObligation` — `command=0; status=Completed; reason=LOCAL_COMMISSION_AUTHORIZED_NOT_PHYSICAL_SUCCESS` |

The paired control's first steward decision is event 23 at 1,980,000ms:
`intent=Wait ... reason=no locally justified work; remaining near resources is allowed`.
The treatment has no crossing completed, no wood committed and no construction
labor by 24h. Its 100 local wood quanta are not equivalent to committed project
materials. This trace demonstrates authorization without automatic participation;
it does not establish a physical shortage of wood in the local case.

## Resource failure is not refusal

[`matrix/resource_order_000/events.tsv.gz`](matrix/resource_order_000/events.tsv.gz); paired [`matrix/resource_control_000/events.tsv.gz`](matrix/resource_control_000/events.tsv.gz).

Events 13, 21, 30 and 31 reproduce issue at 960,000ms, receipt at 1,920,000ms,
and successful commission at 1,980,000ms. The resource fixture provides only five
wood quanta against a twenty-wood recipe. End state records `wood_committed=0`,
`construction_labor_h=0`, `constructed=0` and
`project_outcome=INSUFFICIENT_COMMITTED_MATERIAL`; the **commission duty** is
`Completed`, not `Refused` or a claim of completed infrastructure.

The abundant-wood local fixture also has zero committed material. Therefore this
pair of cases does not isolate insufficient total wood as the sole reason people
decline investment. Separate the bounded recipe insufficiency in `resource` from
the broader absence of voluntary commitment present in both small fixtures.

## Explicit refusal and invalid scope are different

[`matrix/refusal_order_000/events.tsv.gz`](matrix/refusal_order_000/events.tsv.gz); paired [`matrix/refusal_control_000/events.tsv.gz`](matrix/refusal_control_000/events.tsv.gz).

The received valid order creates an obligation at 1,920,000ms. At that same
physical receipt checkpoint the registered controlled Person choice declines it:

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 22 | 1920000; 0.533333h | Some(1) | `RoleObligation` — `command=0; status=Received; reason=VALID_SCOPED_OFFICE_ORDER` |
| 26 | 1920000; 0.533333h | Some(1) | `RoleObligation` — `command=0; status=Refused; reason=CONTROLLED_EXPLICIT_CHOICE: CONTROLLED_PERSON_CHOICE_DECLINES_ACCEPTED_DUTY` |

No commission follows. This tests expressible informed refusal without punishment;
it does not establish an autonomously emerging refusal motive or its frequency.

[`matrix/scope_order_000/events.tsv.gz`](matrix/scope_order_000/events.tsv.gz); paired [`matrix/scope_control_000/events.tsv.gz`](matrix/scope_control_000/events.tsv.gz).

The sovereign's authenticated message asks for unsupported
`seize_private_food`. Carrier travel still occurs, but physical receipt produces
`OrderInvalid` rather than a role obligation:

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 13 | 960000; 0.266667h | Some(0) | `OrderIssued` — `Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: Unsupported { label: "seize_private_food" }, issued_at: 960000, expires_at: 86400000 }; no recipient state changed` |
| 21 | 1920000; 0.533333h | Some(1) | `CommandReceived` — `Transmission { message: 0, sender: 2, recipient: 1, node: 1, issued_at: 960000, received_at: 1920000 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: Unsupported { label: "seize_private_food" }, issued_at: 960000, expires_at: 86400000 })` |
| 22 | 1920000; 0.533333h | Some(1) | `OrderInvalid` — `message=0; reason=OUT_OF_SCOPE` |

No ownership seizure, commission or refusal classification follows. The message's
existence is not an authority grant.

## Distant, late and stale: arrival is not receipt

Sources:

- `distant`: [`matrix/distant_order_000/events.tsv.gz`](matrix/distant_order_000/events.tsv.gz); paired [`matrix/distant_control_000/events.tsv.gz`](matrix/distant_control_000/events.tsv.gz).
- `late`: [`matrix/late_order_000/events.tsv.gz`](matrix/late_order_000/events.tsv.gz); paired [`matrix/late_control_000/events.tsv.gz`](matrix/late_control_000/events.tsv.gz).
- `stale`: [`matrix/stale_order_000/events.tsv.gz`](matrix/stale_order_000/events.tsv.gz); paired [`matrix/stale_control_000/events.tsv.gz`](matrix/stale_control_000/events.tsv.gz).

In each treatment, issue is at 28,860,000ms (8.016667h). Courier 2 holds the
message and physically reaches node 1 at 68,670,000ms (19.075h). It does **not**
hand the order to the steward within the 24h run. The first post-arrival decisions
are personal eating and sleep. For `distant`, the exact sequence is:

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 17 | 28860000; 8.016667h | Some(0) | `OrderIssued` — `Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 28860000, expires_at: 86400000 }; no recipient state changed` |
| 35 | 68670000; 19.075000h | Some(2) | `Arrived` — `node=1 edge=0` |
| 37 | 68670000; 19.075000h | Some(2) | `Decision` — `intent=Eat { batch: 5, quantity: 6 } nutrition_ms=17730000 starvation_ms=0 reason=locally accessible food; exact ownership/ration ceiling` |
| 38 | 68670000; 19.075000h | Some(2) | `Consumed` — `batch=5 q=6 owner=Agent(2) nutrition_ms=39330000` |
| 40 | 68700000; 19.083333h | Some(2) | `Decision` — `intent=Sleep nutrition_ms=39300000 starvation_ms=0 reason=critical fatigue or ongoing recovery` |
| 41 | 68700000; 19.083333h | Some(2) | `SleepStarted` — `fatigue_ms=83100000` |
| 3077 | 86400000; 24.000000h | Some(2) | `Decision` — `intent=Gather { batch: 0, communal: false } nutrition_ms=21600000 starvation_ms=0 reason=one quantum from observed deposit; communal=false` |

The courier arrives with 4.925h nutrition, consumes six personally owned food
quanta, and begins sleep with 23.083333h fatigue. At the horizon it chooses local
food gathering. These are physical arrival followed by survival/recovery actions,
not failed route finding and not an institutional rejection by the steward.

`late` carries an explicit deadline of 32,460,000ms (9.016667h), before arrival.
However, no steward receipt occurs, so the primary run does **not** expose the
expired-at-receipt validation branch. `orders_expired_at_receipt=0` must not be
read as proof that expired orders would be accepted.

In `stale`, the changed source is concrete:

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 23 | 36000000; 10.000000h | None | `Spoiled` — `batch=1 q=120 holder=Deposit(2) created=0 deadline=36000000` |
| 36 | 68670000; 19.075000h | Some(2) | `Arrived` — `node=1 edge=0` |
| 38 | 68670000; 19.075000h | Some(2) | `Decision` — `intent=Eat { batch: 5, quantity: 6 } nutrition_ms=17730000 starvation_ms=0 reason=locally accessible food; exact ownership/ration ceiling` |
| 41 | 68700000; 19.083333h | Some(2) | `Decision` — `intent=Sleep nutrition_ms=39300000 starvation_ms=0 reason=critical fatigue or ongoing recovery` |

The 120 source-food quanta spoil at 10h while the order is in transit. Again,
there is no steward receipt, so this is not evidence of obedience to a received
stale instruction. The source change and physical delivery failure are both
preserved. Separate controlled receipt witnesses are documented in
[`MECHANISM_CONTROLS.md`](MECHANISM_CONTROLS.md): they exercise expired and stale
receipt without converting these primary non-exposures into successes.

## Delayed revocation: received duty expires before withdrawal is heard

[`matrix/revocation_order_000/events.tsv.gz`](matrix/revocation_order_000/events.tsv.gz); paired [`matrix/revocation_control_000/events.tsv.gz`](matrix/revocation_control_000/events.tsv.gz).

Unlike the previous cases, the original order is delivered directly to the
steward at the initial local meeting. The steward selects physical return to its
settlement; the sovereign sends a second message one hour after issue.

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 23 | 28860000; 8.016667h | Some(0) | `OrderIssued` — `Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 28860000, expires_at: 86400000 }; no recipient state changed` |
| 24 | 28860000; 8.016667h | Some(1) | `CommandReceived` — `Transmission { message: 0, sender: 0, recipient: 1, node: 0, issued_at: 28860000, received_at: 28860000 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 28860000, expires_at: 86400000 })` |
| 26 | 28920000; 8.033333h | Some(1) | `RoleObligation` — `command=0; status=Active; reason=PERSON_SELECTED_ROLE_UNDERTAKING` |
| 29 | 28920000; 8.033333h | Some(1) | `Admitted` — `edge=0 from=0 to=1` |
| 33 | 32460000; 9.016667h | Some(0) | `RevocationIssued` — `Revocation { id: 1, issuer: 0, recipient: 1, office: 0, command: 0, issued_at: 32460000 }; recipient continues until actual receipt` |
| 38 | 32520000; 9.033333h | Some(3) | `Admitted` — `edge=0 from=0 to=1` |
| 51 | 68670000; 19.075000h | Some(1) | `Arrived` — `node=1 edge=0` |
| 53 | 68670000; 19.075000h | Some(1) | `RoleObligation` — `command=0; status=Paused; reason=CRITICAL_HUNGER` |
| 57 | 68700000; 19.083333h | Some(1) | `RoleObligation` — `command=0; status=Paused; reason=CRITICAL_FATIGUE` |
| 933 | 76770000; 21.325000h | Some(3) | `Arrived` — `node=1 edge=0` |
| 935 | 76770000; 21.325000h | Some(3) | `Decision` — `intent=Eat { batch: 6, quantity: 6 } nutrition_ms=9630000 starvation_ms=0 reason=locally accessible food; exact ownership/ration ceiling` |
| 947 | 76800000; 21.333333h | Some(3) | `Decision` — `intent=Sleep nutrition_ms=31200000 starvation_ms=0 reason=critical fatigue or ongoing recovery` |
| 2859 | 86400000; 24.000000h | Some(1) | `RoleObligation` — `command=0; status=Expired; reason=KNOWN_DEADLINE_REACHED` |

Carrier **3**, carrying the revocation, physically arrives at 76,770,000ms
(21.325h), eats and sleeps. It never hands over the revocation by 24h.
`courier_arrival_h` in the summary describes the designated primary courier,
not every carrier; a blank field here must not be interpreted as no revocation
carrier arrival. The actual event trace supplies that distinction.

The steward's role record becomes `Received` again at 70,680,000ms after the
critical threshold is passed, but ordinary ongoing sleep recovery still wins
selection. There is no commission. At 86,400,000ms its **known order deadline**
expires. Terminal reason is `KNOWN_DEADLINE_REACHED`, not remotely received
revocation. The run establishes post-issuance travel and survival interruptions;
it does not expose revocation-receipt behavior. Controlled late-revocation
receipt and queue-cancellation witnesses remain separately labeled in
[`MECHANISM_CONTROLS.md`](MECHANISM_CONTROLS.md).

## Dead recipient: stale office knowledge is not telepathy

[`matrix/death_order_000/events.tsv.gz`](matrix/death_order_000/events.tsv.gz); paired [`matrix/death_control_000/events.tsv.gz`](matrix/death_control_000/events.tsv.gz).

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 8 | 3600000; 1.000000h | Some(1) | `Died` — `cause=starvation nutrition_ms=0 starvation_ms=86400000 location=Node(1)` |
| 17 | 28860000; 8.016667h | Some(0) | `OrderIssued` — `Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 28860000, expires_at: 86400000 }; no recipient state changed` |
| 32 | 68670000; 19.075000h | Some(2) | `Arrived` — `node=1 edge=0` |

The steward dies through ordinary starvation at 1h, before issuance at 8.016667h.
The remote sovereign still has the previously accepted office certificate and
issues using that stale personal information. Courier arrival at 19.075h does not
revive the steward or transfer the office/duty to another Person. There is no
receipt or commission. The holder's fatal starting condition is a labeled
controlled fixture; this is not a finding that office work caused the death.

## Integrated: one order enables a longer physical cascade

[`matrix/integrated_order_000/events.tsv.gz`](matrix/integrated_order_000/events.tsv.gz); paired [`matrix/integrated_control_000/events.tsv.gz`](matrix/integrated_control_000/events.tsv.gz).

The high-level order is issued at 1h and carried by Person 5. Receipt occurs
at 1.265667h. Steward deliberation remains under the matched six-hour research
hold; the 4.751h receipt-to-undertaking delay is therefore not an inferred
reluctance or communication delay. The first post-release policy opportunity
selects the role action at 6.016667h.

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 912 | 3600000; 1.000000h | Some(0) | `OrderIssued` — `Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 3600000, expires_at: 259200000 }; no recipient state changed` |
| 1122 | 4556400; 1.265667h | Some(1) | `CommandReceived` — `Transmission { message: 0, sender: 5, recipient: 1, node: 1, issued_at: 3600000, received_at: 4556400 }; payload=Order(Order { id: 0, issuer: 0, polity: 17, recipient: 1, office: 0, objective: CommissionCrossing { project: 0, settlement: 1 }, issued_at: 3600000, expires_at: 259200000 })` |
| 4011 | 21660000; 6.016667h | Some(1) | `RoleObligation` — `command=0; status=Active; reason=PERSON_SELECTED_ROLE_UNDERTAKING` |
| 4013 | 21660000; 6.016667h | Some(1) | `Decision` — `intent=FulfilOrder { command: 0 } nutrition_ms=64740000 starvation_ms=0 reason=role obligation 0: attempt supported local institutional act` |
| 4015 | 21660000; 6.016667h | Some(1) | `CrossingCommissioned` — `Commission { project: 0, office: 0, polity: 17, issuer: 1, node: 1, issued_at: 21660000 }; no resources or workers assigned` |
| 4016 | 21660000; 6.016667h | Some(1) | `RoleObligation` — `command=0; status=Completed; reason=LOCAL_COMMISSION_AUTHORIZED_NOT_PHYSICAL_SUCCESS` |

The existing source report is a separate information object. Report 0 was
observed by Person 2 at time 0 and delivered to steward 1 at 18,302,400ms
(5.084h), event 3319. That is **after** order issue and order receipt. It did not
cause the sovereign to issue this research-injected command. It is available to
the steward's later bounded private investment reasoning. Both paired arms retain
the report mechanism. This experiment does not separately ablate the report's
contribution to construction motivation.

The command duty is already completed when the same Person later chooses to
supply wood and build through the pre-existing voluntary crossing policy:

| Event ID | Time (ms; hours) | Person | Event / exact detail |
| --- | --- | --- | --- |
| 4030 | 21720000; 6.033333h | Some(1) | `Decision` — `intent=Gather { batch: 1, communal: false } nutrition_ms=64680000 starvation_ms=0 reason=voluntary communal crossing project=0; conditional own food access/remembered transport savings; no guaranteed outcome` |
| 4046 | 21722000; 6.033889h | Some(1) | `CrossingDonation` — `project=0 q=1 owner=Commons(0) physical_embedded_wood` |
| 4403 | 22862000; 6.350556h | Some(1) | `CrossingDonation` — `project=0 q=1 owner=Commons(0) physical_embedded_wood` |
| 4406 | 22920000; 6.366667h | Some(1) | `CrossingWorkStarted` — `project=0; labor confers no personal title` |
| 10265 | 43260000; 12.016667h | Some(1) | `Interrupted` — `eat accessible food; was=BuildCrossing { project: 0 }` |
| 10299 | 43320000; 12.033333h | Some(1) | `CrossingWorkStarted` — `project=0; labor confers no personal title` |
| 13772 | 56540000; 15.705556h | Some(1) | `CrossingCompleted` — `project=0 owner=Commons(0); adjacent observation required; existing traversals unchanged` |
| 13796 | 56580000; 15.716667h | Some(3) | `Admitted` — `edge=1 from=2 to=1` |

Twenty one-quantum donations occur from 21,722,000ms through 22,862,000ms.
The final project record attributes all twenty wood and 33,560,000ms
(9.322222 person-hours) of construction labor to Person 1. The recorded reasons
for gathering, contribution and building are the existing conditional personal
food-access/transport-savings motive; the order did not confer labor entitlement.
Eating interrupts construction at 12.016667h, then voluntary building resumes at
12.033333h. Completion at 15.705556h changes edge 1 physically; Person 3 enters
that crossing at 15.716667h under ordinary admission.

In the paired control, steward event 4008 at 21,660,000ms selects a voluntary
communal job claim, rather than `FulfilOrder`. The first Person-1 cultivation
start is 54,660,000ms (15.183333h; event 12637). In the order treatment it starts
at 56,540,000ms (15.705556h; event 13775), immediately after building. These
are different activity histories, not an abstract economy penalty.

### Integrated seed-0 end-state context

These figures come from the two adjacent `results.csv` files at 72h. They are
one paired seed, not the thirty-seed means and not a welfare score.

| Measure | Control | Order |
| --- | ---: | ---: |
| Crossings completed | 0 | 1 |
| Wood committed | 0 | 20 |
| Construction person-hours | 0 | 9.322222222222223 |
| Farm output | 360 | 360 |
| Food consumed | 656 | 680 |
| Delivered food quanta | 8 | 73 |
| Deaths | 8 | 6 |
| Starvation-person-hours | 197.98333333333332 | 244.24666666666667 |
| Initial dependent cohort starvation-hours | 53.983333333333334 | 66.28333333333333 |
| Care food quanta | 0 | 11 |
| Caregiver starvation-hours | 0 | 12.783333333333333 |
| Queue-person-hours | 1.1 | 1.775 |
| Improved-edge trips | 0 | 42 |
| Food stolen | 30 | 30 |

Fewer deaths and more consumption coexist with more starvation exposure, including
the initial dependent cohort. Do not call this unqualified welfare improvement.
The order changes authorization, voluntary work, connectivity and subsequent
allocation through real state transitions. It neither guarantees nourishment nor
improves raid protection in this paired trace: thirty food quanta are stolen in
both arms. A separate intervention would be needed to attribute every downstream
change to one intermediate path, report or worker decision.
