# Transport probe — 2026-09-22

This is a controlled physical hauling workload, not a settlement policy or a successful S1 experiment. Source revision: `14466cb`. The package version `0.1.0` is a Rust package identifier, not a reproduction of the missing historical V0.1 simulation.

## Method

Thirty paired seeds (0–29), 50 agents, a ten-minute horizon, one-second command cadence, a 1 m/s speed, four simultaneous travelers per edge, and two seconds of gathering per unit. Finite wood varies from seed to seed; all agents start at the origin. The harness explicitly asks agents to gather wood and return it. It has omniscient research access and imposes a hauling task: it is not evidence of emergent job allocation.

Cross route length (10 m / 100 m) with carrying limit (2 / 10 units), plus a transport-disabled control: 150 runs. Initial food is unused. No hunger, resource regrowth, mortality, settlement layout, or economic utility is modeled. Geometry is fixed; distance changes routed edge length, not endpoint coordinates. These are test paths, not built roads.

## Results

| Route | Carry limit | Transport | Mean delivered wood | Range across seeds |
|---|---:|---|---:|---:|
| 10 m | 2 | enabled | 227.87 | 203–232 |
| 10 m | 10 | enabled | 248.60 | 203–297 |
| 100 m | 2 | enabled | 16.00 | 16–16 |
| 100 m | 10 | enabled | 80.00 | 80–80 |
| 10 m | 10 | disabled | 0.00 | 0–0 |

All 150 runs passed material audits. Every CSV row independently balances initial wood against delivered wood, carried wood, and remaining deposits. Twelve tests pass, including 60,000 randomized command attempts and exact equality of world/event histories under different advance partitions with the same command cadence. Clippy passes with warnings denied.

The short-route, large-load case exhausts all wood deposits in these seeds. Its variance is initial supply variance, not evidence of complex emergent behavior. The long-route cases produce identical deliveries across seeds: route occupancy and horizon dominate while deposits are sufficient. These seeds only vary finite deposits; they do not establish robustness to geometry, ordering, or policy changes.

## Causal trace and a dispatch failure

For seed 0, 100 m, carrying limit 10, agent 0:

- At 0 ms, departure accepted (event 0).
- At 100,000 ms, arrival (4604, referencing event 0); gathering begins (4608).
- At 120,000 ms, ten wood units enter the inventory (5456, referencing 4608).
- Return attempts are rejected while the edge is occupied. At 200,000 ms, return departure succeeds (9144).
- At 300,000 ms, arrival (13676) and local delivery of ten units (13680).

The full trace preserves all rejected attempts. Outbound empty travelers can delay loaded returners because there is no dispatch queue or loaded-worker priority. Stable command order is another potential source of unfairness. The current experiment demonstrates a bottleneck but cannot determine an economically appropriate policy. It also produces many repeated rejection events; logging and retry frequency will need attention before long runs.

Pickup/drop have no handling duration. Gathering is all-or-nothing at completion; interruptions lose partial labor. Edge capacity means concurrent occupancy, not persons per second. These assumptions can materially change throughput and must remain explicit.

## Reproduction and next question

Run:

```sh
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
cargo run --offline --release --bin transport_probe > /tmp/transport-probe.csv
cargo run --offline --release --bin transport_probe -- --trace > /tmp/transport-trace.txt
```

[Raw runs](transport-probe.csv), [readable trace excerpt](transport-trace-excerpt.txt), [full compressed trace](transport-trace-seed-0.txt.gz), and [provenance/hashes](transport-provenance.json) preserve the evidence.

Next falsifiable question: can a local observation-based job policy supply hungry agents without omniscient stock access or starvation caused by dispatch contention? Before a settlement claim, implement needs, action interruption, observations, and explicit policy cadence; compare dispatch ordering and paired distance/load changes. Farming, construction, and the 180-day settlement gate remain untested. No military/security or later systems are justified by this probe.
