# WORLD OF NODES — PC DEVELOPMENT HANDOFF
## Causal Kingdom Strategy / V0.2 "The Price of Security"

### Purpose
This file is the authoritative handoff for continuing the project on a PC with ChatGPT/Codex/Astra or another capable coding agent.

The project is not a conventional content-heavy strategy game. The goal is to construct the smallest causal world simulation that produces deep kingdom warfare, economy, frontier pressure, diplomacy, demographic consequences, and later institutional development from a small number of interpretable rules.

Do not expand the feature list unless the existing causal core has been tested first.

---

# 1. CURRENT GAME DEFINITION — LOCKED

## Platform / presentation
- PC first.
- Browser-capable.
- Real-time.
- Top-down 2.5D.
- Orthographic or very-low-perspective camera.
- Continuous-looking terrain.
- Primitive 3D rendering.
- Semantic zoom.
- Simulation independent from rendering.

## Visual language
The visuals are intentionally cheap so complexity can be invested in gameplay.

Examples:
- people = dots / circles / simple capsules
- armies = masses of dots in visible formations
- army facing/front = line, bracket, shield-wall-like edge, or another obvious primitive
- houses = boxes
- fields = simple patterned planes
- roads = lines/ribbons
- rivers = lines/flows
- forests = simple repeated primitives
- fire = triangles/particles/smoke
- borders = explicit political boundaries
- wilderness = visibly distinct decentralized territory
- forts/walls = primitive geometry with real gates and constrained passage

Graphics may improve later with funding, but the simulation must not depend on graphical complexity.

---

# 2. CORE GAMEPLAY LOOP — LOCKED

The player acts as a kingdom, not as one individual.

The main strategic loop is:

ECONOMY
↕
SECURITY / MILITARY
↕
TERRITORIAL CONTROL
↕
DIPLOMACY

The army is one of the most important gameplay systems.

An army is NOT an independently spawned resource.

An army is a temporary reconfiguration of society for security and war.

Soldiers are actual people removed from civilian labor.
Army food comes from actual production.
Weapons come from actual materials and production.
Transport uses actual people, animals, carts, roads, bridges, and time.

Therefore:

more army
→ less civilian labor
→ less production / slower construction / lower household capacity

but:

too little army
→ successful raids / destruction / displacement / lost infrastructure / weaker authority

The desired game problem is NOT a fixed "best army percentage."

The desired problem is:

"How much productive society should I redirect toward security right now, given threat, season, geography, roads, frontier length, stores, and political goals?"

---

# 3. MATHEMATICAL SKELETON — TREAT AS THE SIMULATION CONSTITUTION

Represent the world as:

G_t = (V, E, X_t)

Where:

V = spatial/institutional nodes
    farm
    household
    village
    town
    fort
    manor
    church
    market
    mine
    granary
    etc.

E = routes / connectivity
    roads
    trails
    rivers
    bridges
    gates
    paths

X_t = state at time t
    people
    matter
    information
    institutions
    claims
    authority
    threats
    stored goods
    moving goods
    armies
    households

The project should be explained primarily through:

NODES
+
EDGES
+
STOCKS
+
PEOPLE
+
TRAVEL TIME
+
CAPACITY
+
STATE TRANSITIONS

---

# 4. UNIVERSAL WORLD LAWS

## Conservation
Matter cannot appear from nowhere except through explicit natural production or extraction.

## Exclusivity
A person, horse, cart, weapon, tool, or quantity of grain cannot be simultaneously used in two places.

## Travel
Movement from A to B requires time and capacity.

## Locality
Most effects occur through proximity or through a traversable path.

## Transformation
Production is modeled as:

inputs + labor + knowledge + facility + time -> outputs

## Agency
Autonomous people and groups choose among physically available actions.

The decision system may decide intent, but it must never bypass the world model.

## Single clock
All production, travel, construction, aging, campaigns, harvests, messages, and recovery happen under one coherent simulation clock.

---

# 5. CIVILIZATION AS SUSTAINED MOVEMENT — CORE PRINCIPLE

Civilization is not merely map color.

Civilization is the sustained ability to move:

- people
- food
- animals
- tools
- construction material
- soldiers
- officials
- clergy
- merchants
- taxes
- messages
- knowledge

through space and time.

Political claim != actual control.

A kingdom may draw a border around an area while being unable to effectively administer or defend it.

Effective state capacity should arise from actual reach:
- travel time
- road capacity
- response time
- supply access
- administrative visits
- institutional connectivity
- communication delay

Do not immediately collapse this into one "control %" stat.

---

# 6. ROADS — LOCKED CONCEPT

Roads are not "+15% movement."

Each road/edge should eventually contain quantities such as:

- distance
- capacity
- travel time / speed
- condition
- risk
- possibly direction or seasonal constraints

Roads carry both beneficial and hostile flows.

They can carry:
- grain
- workers
- traders
- officials
- soldiers
- messages
- construction material
- settlers
- raiders

This dual use matters.

A road can increase state capacity while also making a frontier easier to raid.

The road simulation must avoid artificial cliffs caused by whole-party movement or coarse rounding.

---

# 7. TERRITORY — LOCKED STRUCTURE

There are three meaningful territorial conditions:

1. CORE KINGDOM TERRITORY
2. CONTESTED / FRONTIER TERRITORY
3. WILDERNESS

## Core
Strong institutional reach, security, economic integration, and regular administration.

## Frontier / contested
Claimed or occupied, but authority remains weak because institutions and connectivity have not yet been established.

## Wilderness
Not empty land.
Contains decentralized societies:
- households
- clans
- tribes
- mobile groups
- camps
- migration/grazing/hunting areas

Wilderness populations do not automatically respect kingdom borders.

They are not scripted "barbarian units."

They act through the same underlying people/household logic, but with different institutions and network structure.

---

# 8. DECENTRALIZED SOCIETIES / TRIBES

Groups can:
- migrate
- trade
- raid
- retaliate
- negotiate
- hide
- fight one another
- cooperate
- ally temporarily
- confederate under shared threat

CRITICAL RULE:

Do NOT secretly scale them upward because a player is winning.

Escalation must have a causal pathway, e.g.:

kingdom expands
→ tribal land/access shrinks
→ displacement
→ casualties / grievances
→ shared threat increases
→ inter-group cooperation becomes more attractive
→ groups communicate
→ physically assemble
→ coordinate raids / war

Coalition preference alone is insufficient.

Coordination needs actual affordances:
- messenger/contact
- agreement
- rendezvous
- travel
- troop reservation
- shared or compatible supply
- coordinated departure

No spawn_confederation().

---

# 9. PEOPLE / HOUSEHOLDS

Long term, the household/family is the social atom.

However, the current research does NOT yet justify a full family tree in the first playable slice.

For now:
- identifiable people
- households
- age cohorts or individual ages
- labor availability
- food
- location
- military status
- migration
- births/deaths at sufficient resolution

Later family/kinship becomes justified only when it materially drives:
- marriage
- inheritance
- revenge
- migration
- alliance
- property
- loyalty
- dynasties

Do not add detailed family simulation just because it sounds deep.

---

# 10. PRODUCTION / MATERIAL WORLD

Start small.

Likely physical resources:
- grain
- timber
- stone
- iron
- livestock

But V0.2 should stay even smaller if possible.

Matter should exist as stocks and flows.

Examples:

forest
→ labor + tool + time
→ logs
→ transport
→ processed timber

land + seed + labor + season
→ standing crop
→ harvest labor
→ grain
→ cart
→ granary
→ household or army consumption

Avoid abstract "+25 wood/month" if a traceable process can remain computationally cheap.

---

# 11. TIME — MAJOR GAMEPLAY PRINCIPLE

Time is not just simulation speed.

Actions take time:
- travel
- construction
- training
- harvest
- planting
- military response
- message travel
- recovery
- aging
- demographic change

Season matters.

Prototype calendar:

SPRING
- planting
- preparation
- campaigning possible

SUMMER
- growth
- construction
- campaigning

AUTUMN
- harvest
- mobilization can be extremely expensive

WINTER
- production lower
- stored food matters
- mobilization opportunity cost may differ
- travel may later become terrain/climate constrained

A mobilization of 100 workers during harvest must not be equivalent to the same mobilization during winter.

---

# 12. SECURITY — V0.2 CENTRAL HYPOTHESIS

Previous research v0.1 falsified the first attempt.

Observed:
- military mobilization had strong real economic/demographic costs
- high mobilization could collapse the kingdom
- zero mobilization still performed best in the tested security grid
- the reason was structural: civilian raid damage was too weak

Therefore V0.2 must NOT tune military numbers.

V0.2 must repair what security actually protects.

Security should protect real vulnerable things:
- standing crops
- harvested grain
- livestock
- carts
- roads
- bridges
- buildings
- households
- people
- local authority
- settlement continuity

Raiders should physically:
travel
→ select target
→ arrive
→ steal/burn/kill/capture if possible
→ carry loot
→ attempt escape

Defenders should:
detect
→ receive information
→ move/intercept
→ fight
→ pursue / return

No arbitrary "raid damage = 10" if the damage can be tied to actual assets.

---

# 13. V0.2 EXPERIMENT — "THE PRICE OF SECURITY"

Keep the world deliberately small.

Suggested:
- 1 kingdom
- 3 settlements
- 1 clear frontier
- 2 decentralized groups initially
- a few hundred identifiable people
- grain
- livestock
- basic equipment
- standing crops
- a small road graph
- one river / bridge if useful
- no church system yet
- no technology tree
- no functioning market unless real exchange is implemented
- no elaborate family tree

Use one fixed, understandable map before randomizing geography.

Example conceptual map:

                         WILDERNESS

              Group A                 Group B
                . . .                   . . .

                    forest / hills

                        village
                           |
                           | poor road
                           |
 fields ----------- frontier town
                           |
                           |
                     ==== bridge ====
                           |
                           |
                         capital

                         fields

---

# 14. V0.2 TEST MATRIX

Sweep at minimum:

THREAT
- none
- low
- medium
- high

MOBILIZATION / SECURITY POLICY
- 0%
- 5%
- 10%
- 20%
- 35%
- 50%
or better adaptive policies if implemented carefully

TIMING
- permanent / standing force
- winter-heavy
- summer-heavy
- harvest mobilization
- responsive mobilization

CONNECTIVITY
- poor
- medium
- good

Do not target a predetermined "balanced" army share.

Search for regimes.

Desired evidence would be worlds where:

too little security
→ meaningful civilian destruction / displacement / economic loss

and:

too much security
→ labor shortage / supply burden / demographic decline / economic loss

The viable region should move when changing:
- threat
- season
- road network
- distance
- frontier length
- agricultural productivity
- stored reserves

If this does not emerge, report failure and diagnose the causal structure.

Do not tune until it looks fun.

---

# 15. METRICS TO TRACK

At minimum:

- population
- working population
- mobilized population
- farm labor
- planting labor
- harvest labor
- standing crop
- grain stock
- livestock
- civilian food adequacy
- military food consumption
- civilian deaths from raids
- military deaths
- houses/buildings lost
- carts/equipment lost
- road damage
- settlement abandonment
- migration/displacement
- raid frequency
- raid success
- loot taken
- time to detect raids
- time to respond
- percent of raids intercepted
- economic value lost to insecurity
- economic value spent on security
- frontier authority / administrative reach
- controlled vs merely claimed territory
- travel times
- transport bottlenecks

Derived metrics:
- security burden
- prevented expected loss
- expansion cost
- military sustainability
- response-time coverage
- state capacity / reach

---

# 16. MONTE CARLO / SCIENTIFIC METHOD

Use reproducible random seeds.

Separate:
1. parameters
2. world state
3. actor policies
4. randomness

Run broad enough sweeps to identify qualitative regimes.

Do not force sigmoid behavior.

Classify mechanisms correctly:
- conservation/accounting
- network flow
- physical constraints
- stochastic hazard
- behavioral choice
- threshold/tipping
- adaptation/learning

Use Monte Carlo to discover:
- regime changes
- dominant strategies
- unstable feedback loops
- dead mechanics
- accidental numerical cliffs
- parameter sensitivity
- non-identifiable parameters

Preserve causal traces.

When a run collapses, produce a readable chain like:

late-summer mobilization
→ harvest labor unavailable
→ standing crop unharvested
→ grain storage lower
→ winter ration shortage
→ household mortality / migration
→ spring labor shortage
→ frontier garrison unsustainable
→ raids penetrate deeper
→ settlement abandoned

Never invent links that are not actually modeled.

---

# 17. FORTS / WALLS — LATER, BUT DESIGN THEM NOW AS FLOW GEOMETRY

Do not yet implement unless V0.2 supports them cleanly.

## Wall
A wall is not "+50 defense."

A wall changes movement topology.

It:
- blocks passage
- creates gates
- concentrates attackers/defenders
- changes pathfinding
- constrains trade and movement
- enables defensive concentration

## Gate
A controlled graph edge / chokepoint.

## Fort
A persistent node storing:
- soldiers
- food
- equipment
- information
- shelter

Its strategic value is primarily reduced response time and persistent force projection.

## Bridge
A high-value connectivity edge.

Its existence can dramatically change:
- civilian movement
- trade
- military response
- invasion routes
- administrative reach

## Granary
Moves food through time by allowing storage.

The design rule is:

BUILDINGS SHOULD MODIFY FLOWS, CAPACITIES, TOPOLOGY, STORAGE, OR HUMAN ACTIVITY BEFORE THEY MODIFY ABSTRACT STATS.

---

# 18. LATER INSTITUTIONS

Once movement/security/economy are validated:

## Manor
A persistent administrative node.
Officials must actually reach people to tax/administer.

## Church
Not "+conversion radius."
Creates a recurring social/institutional destination:
- clergy
- congregation
- ritual
- marriage
- burial
- charity
- teaching
- prestige
- generational transmission

Religion should spread through people and relationships, not a passive percentage timer.

## Market
Only add when real exchange exists.
A market should concentrate exchange and information because people physically attend it.

---

# 19. TECHNOLOGY — LATER

No conventional arbitrary tech tree initially.

Long-term concept:

Technology = available knowledge + recipes + skilled people + facilities

Example:

ore
+ charcoal
+ furnace
+ skilled smith
+ time
→ iron

Knowledge should eventually be held/transmitted by people or institutions.

Do not add until the core security/economy simulation is proven.

---

# 20. 2.5D RENDERING DIRECTION

The world simulation should remain mostly 2D/network-based.

Rendering can add visual Z for:
- terrain elevation
- walls
- towers
- buildings
- trees
- smoke
- formations

Semantic zoom:

FAR
- kingdom border
- settlement symbols
- large army mass
- major roads/rivers

MID
- individual buildings
- fields
- army ranks/front
- walls/gates
- carts/traffic

CLOSE
- individual people
- livestock
- small goods
- basic procedural movement

A person can begin as:
- a dot
- a sphere/capsule
- a few primitive body pieces

An army can be:
- repeated small dots/spheres
- formation depth
- visible front line / bracket
- facing direction
- cohesion visible from spacing

Terrain should look continuous even if a hidden grid/hex structure is used for simulation.

Avoid a visibly tiled/isometric board-game aesthetic unless later testing shows it is superior.

---

# 21. AUTONOMOUS BEHAVIOR / JEV-SHAPED INTERFACE

No external AI calls should be required for the game loop.

Create a local structured behavioral interface:

decision = policy(encoded_state, available_actions)

Jev or a Jev-like/local typed decision model may later plug into this interface.

Actors should make bounded choices:
- household migrate/stay
- raider attack/avoid/trade
- group cooperate/refuse
- soldier retreat/hold
- trader choose route
etc.

Do not use natural-language LLM calls as the simulation engine.

Intent does not equal execution.

If an actor chooses "raid village", the world must still determine:
- whether it can assemble
- whether it can travel
- whether it has supplies
- whether defenders detect it
- whether it reaches the target
- what it can physically steal
- whether it can carry it home

---

# 22. QUALITY RULE

DEEPEN EXISTING SYSTEMS BEFORE ADDING NEW ONES.

Bad:
- add disease
- add cavalry
- add naval war
- add religion
- add politics
- add 30 resources

Good:
grain exists
→ grain is physically grown
→ harvest needs labor
→ grain must be moved
→ roads constrain movement
→ army consumes grain
→ raids can steal/burn it
→ households respond to shortage
→ security protects it
→ military labor cost competes with harvest

A small number of well-connected mechanisms is the goal.

---

# 23. CURRENT RESEARCH RESULT — DO NOT LOSE THIS

V0.1 research result:

The first model did NOT establish the intended security/economy balance.

Zero mobilization preserved population best in the tested security grid.

High mobilization strongly damaged the kingdom through:
mobilized people unavailable to farms
→ lower farm effort
→ lower harvest
→ ration shortage
→ higher mortality / lower fertility
→ depleted workforce

This was a useful negative result.

The missing pathway was meaningful civilian damage from insecurity.

Coalition preferences also created alliance links but failed to create useful coordinated operations because actors had no robust rendezvous/assembly/supply action.

Road experiments were contaminated by numerical cliffs from whole-party capacity and rounded travel times; those mechanics must be cleaned up before claiming road phase transitions.

Do not erase these failures by tuning numbers.

---

# 24. IMMEDIATE TASK FOR THE PC AGENT

You are now continuing V0.2 only.

FIRST inspect the existing repository/files if present.

Then:

1. Reproduce the existing v0.1 tests/results.
2. Preserve old source/results.
3. Create a new V0.2 branch or equivalent isolated revision.
4. Fix movement scheduling:
   - avoid whole-party artificial edge-capacity cliffs
   - avoid coarse travel-time rounding artifacts where possible
   - preserve matter/person exclusivity
5. Introduce vulnerable civilian assets:
   - standing crops
   - grain stores
   - livestock
   - carts/equipment
   - buildings/infrastructure where minimally necessary
   - civilians can suffer meaningful consequences
6. Make raids operate on real targets and real carried loot.
7. Make security/interception spatial and time-dependent.
8. Add seasonal labor/harvest timing.
9. Run no-threat / low-threat / high-threat experiments.
10. Sweep mobilization/security policies without aiming at a target optimum.
11. Produce plots/tables of:
    - civilian loss vs security burden
    - population outcome
    - food outcome
    - intercepted raids
    - response times
    - seasonal mobilization consequences
12. Produce causal traces from interesting runs.
13. Explicitly identify:
    - observed regimes
    - artifacts
    - failure cases
    - dead mechanics
    - next falsifiable question

Do not add technology, church conversion, walls, diplomacy depth, cavalry, markets, or detailed family trees yet.

Those remain design candidates, not V0.2 requirements.

---

# 25. SUCCESS GATE FOR V0.2

Do not call V0.2 successful merely because graphs look balanced.

The central loop is validated only if:

A. insufficient security can destroy valuable civilian production/assets for explicit causal reasons,

AND

B. excessive security can destroy productive capacity for explicit causal reasons,

AND

C. the viable security policy changes when threat, season, geography/connectivity, or reserves change,

AND

D. there is no hidden rubber-banding or arbitrary army penalty creating the result.

If the gate fails, diagnose it.

Do not move on until the result is understood.

---

# 26. SHORT STARTUP PROMPT

Use this after loading this file:

"Read WORLD_OF_NODES_HANDOFF.md completely and treat it as the current project specification. Inspect the existing repository and v0.1 research outputs before changing code. Continue only V0.2 — The Price of Security. Preserve the simulation constitution: nodes, edges, stocks, people, travel time, capacity, conservation, exclusivity, locality, transformation, agency, and one coherent clock. Reproduce v0.1 first, then repair meaningful civilian vulnerability, movement scheduling, seasonal labor, raids, and spatial interception. Do not tune toward a desired military optimum. Use Monte Carlo experiments to try to falsify the security/economy loop. Work in small tested commits and report assumptions, failures, causal traces, and evidence before proposing V0.3."

