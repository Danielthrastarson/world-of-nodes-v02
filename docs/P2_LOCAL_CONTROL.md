# P2 local military command contract

The playable scenario explicitly seeds a voluntary command agreement naming one
embodied commander and one unit. This is an initial institution, not evidence that
command rights emerged or that a sovereign controls every citizen. The session
holds the agreement; selecting a person or unit does not create consent. The
commander must remain alive, capable, physically at a node, and in active voluntary
service with that unit. No new commander recruitment or coercive power is added.

`playable::military::issue(world, agreement, order, target)` derives the issue node
from that person. It validates only the commander's personally known geography;
knowing a destination does not reveal its current state or provide recipients a
route. Only members already in physical local service receive the order. Absent,
late and mid-edge members retain their earlier orders. Each recipient still uses
the ordinary Person policy, survival priorities, personal beliefs and service
expiry. Receipt does not force immediate action selection.

The four commands have deliberately graph-local meanings:

- **Move:** attempt travel to the known node. Existing recognized hostile contact
  remains real; movement does not grant immunity or erase allegiance.
- **Hold:** remain at the commander's current node and engage existing locally
  recognized hostile Persons. To approach another location use Move or Attack.
- **Attack:** approach a known node and engage existing hostile Persons encountered
  through F11C contact. This cannot designate an arbitrary civilian as an enemy,
  pursue hidden moving targets or create ranged combat.
- **Withdraw:** attempt travel to a different known node, suppressing the person's
  own strike selection while that received undertaking remains active. This does
  not remove hostile status: enemies can still strike a withdrawing person who
  remains physically in contact. It grants no speed, passage or damage immunity.

A replacement order cancels a recipient's pending queue only when it belongs to
the old unit movement undertaking. Move/Withdraw may interrupt the recipient's
own strike through the existing cancellation mechanism. Completed injury is never
undone. An atomic edge traversal is never cancelled or repositioned. Critical
survival, care, fatigue and scarce FIFO capacity can still prevent movement.

Retreat is a sparse map from Person ID to that person's received order serial.
It ends on replacement, arrival, service ending or death. The common Person
capability adapter supplies its own retreat flag to existing combat choice, which
then yields to the existing formation movement undertaking. There is no separate
Soldier AI. Historical worlds have an empty map; adding this state alone leaves
their forecasts, decisions, event strings and hash encoding unchanged. Nonempty
retreat state is hashed. The separate clock correction below is independently
reviewed against historical results.

The returned count means local recipients, not force strength or future arrivals.
It is legitimate issuance feedback because the commander is physically present.
Global unit snapshots and remote arrival, injury, hunger or death remain DEBUG
information unless separately observed or reported. Graph display geometry is
not continuous tactical formation or a combat modifier.

Tests cover physical movement, unknown targets and missing routes, unchanged world
on rejection, absent/late recipients, incapacity and service expiry, queue versus
atomic movement, and incoming damage during withdrawal. A static contact fixture
is explicitly controlled and is not used as evidence of autonomous assembly.

## Contact clock correction discovered during validation

The withdrawal partition test found that a pending strike whose target departed
after contact resolution remained active until the next caller-supplied checkpoint.
Different observer sampling could therefore change cancellation time, activity
accounting and event hashes. A second test reproduces this using only existing
Strike and Travel actions, with no playable command or retreat state.

The kernel now schedules a strike which has lost valid physical contact for
cancellation at the next clock instant (one millisecond). This is a global
mechanical consistency correction, not a special player timing rule. It does not
accelerate valid strikes, undo damage or suppress incoming contact. The failing
and corrected transcripts are preserved under `research/playable/development/`.
Historical comparisons must disclose changed cancellation/activity/event timing
where this correction applies; old output files remain unchanged. Empty retreat
state preserves historical encoding, but is not a claim that this separately
identified clock defect has no historical consequences.
