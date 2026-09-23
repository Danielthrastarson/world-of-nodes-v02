# Player action language — long-term design principle

The ruler uses a small, composable vocabulary with deep causal consequences.
Normal play should not require assigning individual agents walking, hauling,
farming or building instructions. Atomic actions remain autonomous execution.
As an analogy only: the physical kernel supplies machine semantics, atomic actions
are low-level instructions, NPC plans are execution, institutions provide a runtime,
and ruler actions form a high-level domain-specific language. No compiler or large
planning framework is implied.

| Candidate verb | Meaning | Boundary |
|---|---|---|
| OBSERVE | Inspect, request a report, send a scout, investigate | Changes legitimate knowledge; cannot reveal hidden truth instantly. |
| DECLARE | Change a law, tax, ration or obligation | Changes institutional rules, not resources; authority/enforcement matter. |
| COMMISSION | Request a crossing, road, granary or fortification | Creates an attempted objective; never spawns a completed object. |
| ALLOCATE | Reserve resources, grant use or access | Only things the institution actually controls; no invented ownership. |
| APPOINT | Delegate authority/responsibility | Appointees remain actors; delegation is not guaranteed performance. |
| NEGOTIATE | Propose trade, treaties, access or alliances | Other actors may refuse; no automatic agreement. |
| MOBILIZE | Redirect existing people, equipment and food | Reconfigures the economy; no free soldiers or transport. |
| REVOKE | Withdraw a permission, commission, appointment or policy | Communication, sunk costs and implementation delays remain real. |

An eventual command should be capable of recording type, scope, authority,
information basis, lawful resource claim/limit, priority, communication/execution
time, enforcement responsibility and status: proposed, issued, received, attempted,
blocked, completed, failed, revoked. These are conceptual semantics, not an F6 API.

Commands generally change information, rights, constraints, incentives, priorities
or institutional objectives. Physical changes still require agents, lawful goods,
local access, work, travel capacity and elapsed time. Simple input should produce
an understandable causal history, including failure and unintended consequences.

Example: COMMISSION crossing at a known ford, public access, bounded authorized
resources. This creates an objective. Materials must be acquired and transported;
people may undertake or refuse work. Construction may compete with food transport,
remain incomplete, or open a crossing that changes routes and congestion. Those
consequences must follow the world, not a scripted commission outcome.

F6 implements only physical communal projects, material commitments, labor and
public use within the existing community. No ruler interface, player authority,
communication institution, taxes, appointments or generalized command runtime is
introduced. Future commissions can reference the same project identities and
resource ledger instead of bypassing construction.
