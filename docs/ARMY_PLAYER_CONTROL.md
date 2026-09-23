# Future army player-control architecture

Design target only. F7 implements civilian paths; no armies, combat, military
commands, formation solver, ruler UI or Jev are authorized by this note.

## Stable gameplay spine

Civil society remains emergent: the ruler changes institutions, priorities,
infrastructure, allocation and policy, then observes consequences. Military play
should provide immediate spatial decisions about objectives, movement, fronts,
formations, commitment and supply priority. The player controls formations and
objectives, never individual soldier coordinates. Target: simple control, deep
consequences, rather than passive civilization observation.

An army must derive from actual people, food, equipment, transport, fatigue,
location, cohesion, information and command structure. Mobilization draws these
from the civilian world; no free unit counters. Military power is temporary
reconfiguration of the economy. This does not itself authorize seizure of private
carts or other property: future institutions must define lawful resource claims.

## Stable categories, provisional controls

| Army key/category | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| Q — OBSERVE | Recon/scouts | Terrain | Enemy estimate | Supply/readiness |
| W — ORDER | Move | Attack | Hold | Withdraw |
| E — DOCTRINE / CONFIGURATION | Line | Column | Dispersed | Depth/reserve |
| R — COMMIT / ALLOCATE | Reinforce | Supply priority | Attach/mobilize | Release/demobilize |

Exact bindings require play-testing. These are explicit contextual specializations
of the stable observation/order/governance-or-configuration/allocation categories
in the player language, not AI-driven remapping of top-level keys. Jev must never
silently change these meanings.

LMB selects, drag-selects where useful, targets locations and confirms spatial
commands. RMB supplies a fast contextual order: ground → move, observed enemy →
attack, friendly settlement → move/defend as appropriate. While composing a command,
RMB may back/cancel. Basic military play should work with selection + RMB; QWER and
1–4 provide precision. An intentional RMB order is an explicit player issuance
gesture, not permission for an AI to issue autonomous player orders. Avoid complex
mandatory gestures.

## Typed intent, physical realization

Select army → W/ORDER → 1/MOVE → LMB ridge resolves to a typed objective such as:

```text
MoveArmy { army_id, target: ridge }
AttackFormation { army_id, target: observed_enemy_formation }
DesiredFront { point_a, point_b, depth, orientation }
```

Other objectives include hold line, withdraw, change formation, assign reserve,
prioritize supply and reconnoiter. The command system validates subjects, targets,
authority, information and resource claims. The army's internal command structure
translates an objective into individual execution. A move cannot teleport; attack
cannot manufacture contact or determine combat success.

A dragged front specifies desired geometry. Actual occupation depends on real
soldier count, terrain, obstacles, fatigue, enemy pressure and cohesion. Line gives
width/frontage with less depth; column concentrates depth with narrower frontage;
dispersal changes spacing and occupied area. Those should have physical geometric
consequences, not arbitrary “formation +10% attack” bonuses. Geometry and cohesion
are future causal systems, not decorative states to add during F7.

## Responsive feedback within information limits

Within ruler/commander communication reach, a valid issued order should immediately
show intended route/front, target, stance and the estimated supply state known to
the player. Distinguish intended geometry from actual positions and order receipt
from completion. Physical execution still takes time. Later communication distance
may delay receipt; feedback must not reveal hidden enemy or logistics state. Do
not implement communication or these overlays yet.

## One physical logistics world

F5 carts, F6 crossings and F7 paths/roads must remain shared geography, capacities,
FIFO/travel mechanics and bottlenecks. Future armies use that same system through
the actual people and transport they comprise, not a separate privileged movement
layer or one free occupancy slot for an arbitrarily large formation.

Possible future causal chains: requisitioned carts reduce civilian transport;
armies occupy crossing capacity and civilians wait; refugees delay supply traffic;
a physically destroyed bridge changes available routes. These outcomes must follow
explicit rights and shared physical constraints, not scripted military modifiers.
Neither priority admission nor destruction is implemented by documenting them.

The intended loop is economy ↔ logistics ↔ military/security ↔ territory ↔ economy.
Positioning, scouting, route control, crossings, supply, formation, commitment and
withdrawal provide meaningful spatial choices; the economy limits sustainability,
and territorial outcomes alter economic possibilities. The player controls intent,
geometry and commitment. The simulation controls execution and consequence.
