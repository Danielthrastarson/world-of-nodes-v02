# Provisioning increment — locked contract (2026-09-22)

This contract supersedes the earlier settlement build order for this increment only. Implement physiology, physical food spoilage, observations/memory, communal requests, voluntary decisions, interruptions, FIFO transport, and research. No farming, construction, markets, safety, graphics, combat, or physics.

## Locked reference

50 agents at C, zero cargo, nutrition 12h, fatigue 4h. C–X–F has two 2h unimpaired edges, each with four shared directional slots. Carrying capacity stays 10 existing material units; gathering work stays 2 seconds per unit. F alone contains 3000 integer food quanta (500 six-quantum rations), created at t=0 and expiring at 48h. Duration 72h; checkpoints at 36/48/72h. No replenishment. Each quantum provides one nutrition-hour. Biological reserves plus physical food equal 3600 nutrition-hours, but temporal, spatial, information, transport and institutional access remain separate constraints.

Initial knowledge is only C and adjacent C–X; destination contents require a physical visit. A paired prior-information control is an explicit initial-memory intervention, never the reference. There is no home tether or compulsory return. Migration toward F is legitimate. Jobs are persistent voluntary attempts, not guaranteed deliveries.

## Physiology

Nutrition cap 24h, metabolic use 1h/h, performance 100% above 6h, 80% at positive nutrition up to 6h, 50% at zero. Starvation duration increases only at zero and resets at positive nutrition. Death at 24 continuous hours of zero nutrition. This deliberately accelerated physiology is not realism. Fatigue rises 1h/h awake and falls 2h/h asleep, clamped 0–24h; performance bands at 12/16/20h are 90/75/50%. Seek sleep at 22h, unless accessible food and immediate starvation risk dominate. Multiply nutrition and fatigue performance. No sleeping/eating or changing intent mid-edge; death is possible there.

## Goods and institutions

Immutable batch age survives gathering, splitting, merging, ownership/custody transfers. Spoilage makes physical waste in the same location. Keep owner separate from holder. Natural goods are open access; personal gathering assigns Agent ownership; accepted communal-job gathering assigns Commons ownership. Communal withdrawals are immediate local consumption and cannot exceed 12h nutrition. Integer quanta require flooring available nutrition headroom. Carrier consumption of communal cargo obeys this ceiling and reduces delivery.

One explicit CommunalProvisioning institution. A physical board at C records locally observed shortage contributions, not a global hunger query. A local request arises below 8h nutrition when locally accessible food cannot cover demand. Claims last 12h from local acceptance, independent of travel estimates. Remote cancellation/death releases actual work/reservations but leaves board records unchanged until a local report or expiry. Expiry does not cancel the worker; duplicate effort is allowed, duplicate matter is not.

Death preserves ownership and moves cargo to actual node/edge position. Dead personal property stays inaccessible; no inheritance/scavenging/appropriation. Node-local communal remains communal; edge cargo recovery is deferred.

## Behavior and scheduler

Policies receive local observations, timestamped memories and self state only. Memory confidence exp(-age/12h) affects scores, not truth. Prioritize survival, critical sleep, atomic action, self-provisioning, communal work, exploration, waiting. Consider predicted survival, edibility on arrival, travel time and remembered queue evidence. Exploration visits unknown locations, never hidden resource targets. Atomic gathering is one quantum; queue/work can be cancelled, traversal cannot.

FIFO keys are (join time, AgentId), shared both directions, with timestamp-batched submissions before admission. Capacity zero blocks passage. No urgency/cargo priority. Overtakes must be zero. Use a 60s policy cadence with arrival/work/critical-state wakeups on the same clock.

## Evidence and completion

Retain the old transport kernel and its evidence as a frozen legacy baseline. New provisioning is versioned separately within the Rust crate. Preserve old invariants; compare intentional FIFO changes separately, never overwrite old CSVs.

Run seeds 0–29 for reference, prior memory, self-only, no transport, scarcity, stale information, unequal access, contested demand, reversed processing, congestion, distance, spoilage stress and near/fresh versus far/old food. Publish all requested metrics, raw results, configuration and source hashes, paired effects, failed cases and actual causal traces. No minimum survival is an acceptance criterion. Report no institutional effect and dead mechanics without rebalancing.

## Disclosed implementation choices (P1)

The policy uses a six-quantum personal buffer and requires at least six predicted nutrition-hours remaining before volunteering for a complete request trip. These are explicit heuristics, not survival guarantees. Request urgency is `1 + min(recorded_demand, 600)/600`; freshness weight is remaining shelf life after estimated delivery divided by 48h, capped at one. Source-memory confidence uses the locked 12h decay. Policies only claim requests while physically at their board. A visit to a remembered board may prove unnecessary or fail.

Adjacent-edge observations expose only the queue at the observer's own endpoint. In-flight and opposite-end traffic is unobserved; estimates use nominal service time and remembered visible queue length. Equal-time admission is by AgentId. Seed streams affect equivalent choices, not quotas. No general face-to-face information exchange exists: personal discoveries are not automatically broadcast.

Board demand is refreshed from cooperating people physically present (their public local shortage signals) only when a local actor observes the board. It is the sum of deficits to 12h among people below 8h, minus their edible personal supplies and accessible communal stock. A deserted board retains recorded demand. Lease expiry is a deterministic known deadline, not a remote report. Jobs/claims never reserve distant matter.

Eat/pickup/drop/claim are instantaneous local actions; sleep uses one-minute atomic intervals. Sleep continues toward fatigue 4h unless interrupted. Gathering one quantum uses the kernel's 2 seconds of unimpaired work. Food metabolism does not depend on work. Carried waste occupies capacity until dropped at a node. Node-local communal property of a dead carrier remains accessible; personal dead property is not.

The prior-information control injects documented t=0 observations as an explicit information-only initial-condition intervention; it does not claim to simulate prehistory. The stale-depletion fixture places actor 49 at F with six quanta, seeds t=0 source memories, then records actual one-quantum gathering and consumption by actor 49 during a 12-second controlled preparation period before autonomous decisions. It is a targeted mechanism fixture, not the 3000-quantum reference.

At a coincident event: integrate physiology/progress, settle arrivals, spoil expired food, process deaths, finish valid gathering/sleep, expire board leases, observe/decide, then admit queues. A death or expiry cannot be reversed by a same-time meal. Caller advance partition is independent of the fixed decision cadence.

Instantaneous transfers/eating complete locally, then the agent waits for its next scheduled decision; timed action completions wake decisions early. There is no unbounded same-timestamp decision loop.
