# F13A cross-system readiness control

Registered before execution against the F12 implementation at `4c84081`.
This is research-only integration preparation for F13, not the full kingdom
gameplay gate and not a new autonomous government policy.

## Question and intervention

Can existing authority, voluntary construction, production, households, physical
reports and security execute together, and can their interactions be traced
without changing their mechanics? Compare one lawful **local steward commission**
against leaving the same crossing project pending. This is the only assigned
difference between paired runs. A commission authorizes an objective; it does not
allocate workers, transfer property or guarantee construction.

The sovereign and steward begin together at settlement 1. Both arms execute the
existing physical appointment at time zero. Only the commissioned arm then
executes `CommissionCrossing` as that active local office holder. The report
travels later in both arms. It is factual evidence, never a commission, an order,
or a grant of authority. This control does not claim that the sovereign currently
has a remote command channel to the steward or political command rights over a
unit. Those remain explicit F13 owner-decision boundaries.

## Fixed cases and parameters

Three cases, two arms (`pending`, `commissioned`), seeds 0–29: 180 runs of 72h.
Do not change parameters after seeing outcomes. A smaller one-seed smoke check
validates instrumentation, not the research conclusions.

* `alternate`: old detour is 1–3–2, 2h per edge; 100 wood at settlement 1.
* `scarce`: same geography; only 5 wood, below the 20-wood project requirement.
* `long`: same 100 wood; detour is 8h per edge.

There are five nodes. Settlements are 1 and 2; node 0 is an empty branch off 1
(0.25h); node 4 is the seeded raiders' origin, connected to 2 (2h). All open
edges have capacity 4. The proposed crossing is a closed 1–2 edge, opening only
after 20 wood and 8h of actual construction work, then taking 0.25h with capacity
4. Existing paths start bare on open ground edges and evolve by ordinary use.
These parameters reuse F6's alternate route and 20-wood/8h recipe, F11C's raid
mechanics and F12's long-route contrast. They are physical fixture conditions,
not calibration for a desired result.

Eighteen existing Persons: sovereign 0, steward 1 and courier 2; civilians 3–8
start at settlement 1, residents 12–15 at settlement 2; raiders 9–11 start at 4;
dependent Persons 16 and 17 start with caregivers 3 and 4 at settlement 1. The
courier starts at 2. Sovereign and steward remain ordinary Persons and may later
volunteer, migrate or die. Military eligibility is not restricted to a designated
worker class. Two seeded households have individual property and directed care.
B2 is enabled with its existing kinship initialization; the harness adds no
extra relationship scores. B3 uses its unchanged 24h
adult threshold, with reproduction drive disabled in these initial Persons to
bound this integration control. Children may therefore mature during the run.

All adults begin with 24h nutrition, 4h fatigue and six personally owned carried
food quanta, following the F11C control. Dependents have 8h nutrition and no food.
There are 24 unowned food quanta at settlement 1 and 200 communal quanta at 2.
Initial food expires at 48h. Two cultivation units at 1 and four at 2 use the
unchanged farming recipe: 1h sowing, 12h growth, 1h harvest, 24 food. No completed
crops, carts or infrastructure are injected. A local communal board at 1 has a
60-quanta request; all existing rationing, ownership and priorities remain.

Security is the same controlled pressure in both arms: a locally posted voluntary
request at 1, muster 1, attendance 4, 12h service, closing at 60h. At 1h05m the
harness attempts one existing `issue_unit_protect(0, 1, 2)` command. Only actually
assembled local members can receive it. Failure or zero receipts is recorded;
there are no retries or magical late orders. Three seeded raiders start their
existing objective at 2h, each seeking up to ten quanta from the communal stock
at 2. Strike and theft work remain 15min and 5min. No defense benefit is assumed.

## Information and controlled interventions

Every initial Person receives explicitly labelled, one-hour-old map-only
observations of the node-adjacent edges from prior visits. These contain no remote
goods, people or boards. Current remote inventories are never initialized as
personal knowledge. This is controlled map familiarity, not an autonomous
exploration result.

Courier 2 actually observes node 2 at time zero and captures that exact local
`NodeView`. The existing research purpose asks it to deliver the immutable report
to steward 1 at rendezvous 1 by ordinary Person policy. No direct handoff is
injected. The steward's deliberation is held until 6h in both arms, using F12's
explicit stationary-recipient control; physiology, time, observation and other
people continue. The fixed six-person-hours of unavailable steward labor are
reported separately and are not attributed to endogenous institutions. After 6h
the steward becomes eligible at the next normal policy opportunity. There is no
unscheduled wake consumed early by arbitrary observation checkpoints. Long routes,
missed meetings, hunger, competing
undertakings and death may prevent delivery. No remote tracking is added.

## Evidence and limits

Record configuration, initial goods and beliefs, every event (seed 0), all-seed
quarter-hour timelines, final Persons, goods, projects, offices, report receipts,
unit snapshots and activity ledgers. Summary columns include authority hours,
commissions, construction effort/participants/completion, route use, farming,
care, starvation, mortality, military attendance and costs, combat, stolen food,
report receipt age, paths, migration, peak Persons, runtime and hashes. Run the
normal audit at checkpoints and completion. Raw output is create-new only.

Compare paired differences, not a monetized welfare score. Initial food provenance
distinguishes original stock, consumption, waste and attacker custody. A low final
stock cannot be called raid loss without the actual theft events. More throughput
cannot be called benefit without its care, labor and starvation consequences.

The readiness control passes at registered scope if all physical/institutional
audits pass and the observed integrated causal paths or failures are explainable.
Construction, report delivery, protection and improved survival are not required.
An all-zero downstream result limits the claim to successful composition and
identification of the first blocking stage; it does not establish a kingdom
gameplay loop. This control cannot pass full F13, authorize conscription, create
taxation or requisition, or turn factual reports into orders. Any demonstrated
implementation defect is isolated and fixed with evidence; negative allocation
outcomes are preserved.

## Reproduction

`cargo run --release --bin kingdom_readiness_probe -- CASE ARM NEW_DIRECTORY 30`

Source: `src/provisioning/kingdom_research.rs` and
`src/bin/kingdom_readiness_probe.rs`. The implementation and output revision are
recorded by the final report after validation. Existing research is untouched.
