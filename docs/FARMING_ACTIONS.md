# F3 executable action contracts

The only new command is `Cultivate { plot }`. The crop stage determines its work.
It is legal only for a living agent at that plot, when the unit is empty or its
active crop belongs to that agent. Growing crops cannot be worked/harvested early.
Concurrent attempts use the kernel's deterministic command order; seeded local
plot selection does not allocate a prescribed number of workers.

- Empty unit: create personal crop and begin sowing. One nominal hour of work.
- Sowing: resume retained work. No second crop can occupy this unit.
- Sowing completion: agent becomes idle; growth deadline is now + configured time.
- Growing: deadline changes authoritative crop state to mature. No agent is
  remotely notified, teleported, assigned or fed. A remembered deadline can be
  inferred, but personal memory remains the last actual observation.
- Mature: owner may begin one nominal hour of harvesting.
- Harvesting: resume retained work; only completion creates edible food.
- Harvest completion: create loose personal batch at plot, record environmental
  input in production ledger, free unit. Carrying capacity does not constrain
  creation on the ground; normal pickup/travel/eating and ownership do constrain use.

Sow/harvest use the kernel's continuous impairment and event boundaries. Eating,
sleep, travel or cancellation can interrupt work; work remains on the crop. Death
ends action but never extinguishes ownership. Growth can complete after owner
 death. Unharvested crops have no decay in this deliberately narrow model.

A frozen cultivation forecast executes sow work, growth wait and harvest work with
no intermediate meals/sleep. This is a conditional, bounded candidate, not optimal
planning. The field policy's current-nutrition survivability test is unchanged by
believed future meals. Existing six-hour preference is used to trigger investment
when known food coverage falls below the candidate duration plus six hours; it
is not a promise that the candidate ends with six hours of nutrition.

The new opportunity sits below immediate eating, critical food seeking and critical
sleep, and is unavailable during accepted delivery jobs. It precedes ordinary
self-provisioning and optional new communal work. This is an explicit personal
investment preference, not a proven correction to prior NPC behavior. Only farming
worlds use it. The finite-stock policy and every historical diagnostic remain intact.

The field memory is separate from `PolicyMemory` to keep historical serialization
and hashes identical. Farming worlds include the entire field state/memory in their
state hash. No-farming worlds have exactly the old state/event hash representation.

## Accounting boundary

No water, carbon, seed stock, fertility depletion or sunlight budget exists at F3.
Production records environmental food input, rather than pretending crops transform
preexisting food. There is no meaningful closed atomic-mass claim for these omitted
inputs. Within modeled goods, the exact identity is initial + harvested production
= food in every holder + waste in every holder + consumed. Labor/growth without
harvesting cannot add food. Source batches retain original age under all transfers.

## Research metrics

`attempts` counts Cultivate command submissions (including continuation checks and
rejected collisions), not distinct planted crops. `started` is successful creation.
`interruptions`/`resumptions` describe work actions. No abandonment/renunciation
institution exists. Instead measure active crops, living/dead-owner stranded crops,
and inaccessible mature crops separately; do not call a paused living crop lost.
`output_consumed` and `output_spoiled` track all descendants of harvest origins.
Material audit is checked at every kernel boundary and reported checkpoint.
