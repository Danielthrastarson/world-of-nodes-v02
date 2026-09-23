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

## Long-term UX constraint: stable, compositional controls

Target **low input complexity + high expressive power + high consequence
complexity**. The player should learn a small language and discover deeper
combinations and consequences, rather than operate large menus or micromanage
individual NPCs. Preserve this abstraction boundary during the simulation
milestones; **do not implement Jev or the final UI now**.

Prefer primary physical controls around Q/W/E/R, 1–4 and the two mouse buttons.
The exact bindings are provisional and require later play-testing. The constraint
is stable top-level semantics: AI prediction must never redefine the fundamental
meaning of Q/W/E/R.

| Provisional key | Stable semantic category | Examples |
|---|---|---|
| Q | OBSERVE | Inspect, investigate, request information. |
| W | COMMISSION / ORDER | Attempt physical projects or operational objectives. |
| E | DECLARE / GOVERN | Laws, rights, rationing, taxation, access rules. |
| R | MOBILIZE / ALLOCATE | Redirect authorized resources, authority, military or logistical capacity. |

These categories organize the larger verb vocabulary above; four keys do not
limit the eventual system to four command types. Under a selected category, 1–4
choose contextual objects, subcommands or modifiers. For a settlement, Q could
offer food, population, traffic and authority. For a river, W could offer road,
crossing, storage and fortification where those commands have valid targets.
Those are vocabulary examples, not permission to construct at an unsuitable site.

Left Mouse provisionally selects, targets and confirms where appropriate. Right
Mouse provides inspection/context or back/cancel where appropriate. Core gameplay
must not depend on complicated mouse gestures.

Compose **verb + object + modifier + target → player command**. The interaction
loop is approximately: select world object → choose verb → choose object/modifier
→ target if required → issue. For example: W (Commission), 2 (Crossing), left-click
a river target, 1 (Public modifier), confirm. This yields a command such as:

```text
Commission(project = Crossing, target = RiverCrossing42, access = Public)
Declare(rule = GrainRation, scope = CapitalRegion, amount = …)
Mobilize(resource = Carts, destination = Frontier, priority = MilitarySupply)
```

These examples describe future typed commands, not implemented APIs. Issuing one
creates an authorized attempt or institutional change. Resources, labor,
transport, authority, information and time determine its realization; the command
does not directly mutate the desired final world state. In particular, naming
carts in a mobilization does not itself grant rights over personally owned carts.

## Future command type boundary

Represent actions as typed simulation objects: ObserveCommand, CommissionCommand,
DeclareCommand, AllocateCommand, AppointCommand, NegotiateCommand, MobilizeCommand
and RevokeCommand. Each type defines valid subjects, targets, parameters, authority
requirements, resource claims and information requirements. Invalid commands must
be unrepresentable or explicitly rejected. A phrase such as “build bridge please”
is never authoritative game state; interpretation must resolve to a legal typed
command before issuance.

Contextual choices must respect the player's knowledge. The authoritative kernel
still validates actual preconditions when a command is received/executed; a valid
expression does not guarantee physical completion. Hidden world truth must not
leak through rankings, option availability or rejection explanations. Keep the
information basis and command lifecycle described above inspectable.

## Optional Jev layer: relevance, not semantics

Jev may later rank a finite, game-defined set of valid contextual actions using
current selection, player knowledge, available commands, recent interaction and
UI context. It may return probabilities or relevance scores for typed candidates
such as InspectFood, InspectTraffic, RequestReport, CommissionStorage and
ChangeRationing. This is an optional contextual intent layer, not the authority
that defines what those actions mean.

The UI may use its output to rank contextual 1–4 choices within the selected
category, suggest actions, choose default focus, reduce navigation or interpret
ambiguous intent. These conveniences must preserve predictable top-level verbs
and player confirmation of the resolved command.

Jev must never:

- Invent actions outside the legal type set or change command semantics.
- Mutate simulation state directly or obtain/reveal hidden world information.
- Issue commands without confirmation, unless a separate, explicit future
  authorization enables that behavior; none is enabled by this design note.
- Remap Q/W/E/R according to predicted intent.

Use AI for **contextual relevance, not semantic instability**. Q should retain
its observation meaning even when the contextual choices beneath it change.
The simulation remains authoritative, muscle memory remains useful, and the
player's language remains small while causal consequences can be large.
