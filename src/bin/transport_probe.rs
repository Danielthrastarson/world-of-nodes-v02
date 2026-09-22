//! Controlled workload, NOT an autonomous settlement policy.
use world_of_nodes::*;

fn run(seed: u64, distance: u64, load: u64, transport: bool, advance_chunk: u64) -> World {
    let mut w = World::new(reference_scenario(seed, distance, load)).unwrap();
    // Explicit experimental commands: haul finite wood home for ten simulated minutes.
    // Debug state access is intentional here and is not suitable for actor policies.
    for second in 0..600 {
        for id in 0..w.agents().len() {
            let a = &w.agents()[id];
            let command = match *a.action() {
                Action::Idle { node: 0 } if a.inventory()[1] > 0 => Some(Command::Drop {
                    resource: Resource::Wood,
                    amount: a.inventory()[1],
                }),
                Action::Idle { node: 0 } if transport => Some(Command::Travel { edge: 0 }),
                Action::Idle { node: 1 } if a.inventory()[1] > 0 && transport => {
                    Some(Command::Travel { edge: 0 })
                }
                Action::Idle { node: 1 } => {
                    let available = w.nodes()[1].deposits()[1] - w.nodes()[1].reserved()[1];
                    if available > 0 {
                        Some(Command::Gather {
                            resource: Resource::Wood,
                            amount: load.min(available),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(command) = command {
                let _ = w.command(id, command);
            }
        }
        let end = (second + 1) * 1000;
        while w.now_ms() < end {
            w.advance_to((w.now_ms() + advance_chunk).min(end)).unwrap();
        }
        w.audit().unwrap();
    }
    w
}

fn main() {
    if std::env::args().any(|a| a == "--trace") {
        let w = run(0, 100_000, 10, true, 1_000);
        println!("# seed=0 distance_mm=100000 carry_units=10 transport=true horizon_ms=600000");
        for event in w.events() {
            println!("{event:?}");
        }
        println!(
            "# final_stock={:?} delivered_wood={} audit={:?}",
            w.totals(),
            w.nodes()[0].loose()[1],
            w.audit()
        );
        return;
    }
    println!(
        "seed,distance_mm,carry_units,transport,horizon_ms,initial_wood,delivered_wood,carried_wood,remaining_deposit,capacity_rejections,arrivals,audit_ok"
    );
    for seed in 0..30 {
        for (distance, load, enabled) in [
            (10_000, 2, true),
            (10_000, 10, true),
            (100_000, 2, true),
            (100_000, 10, true),
            (10_000, 10, false),
        ] {
            let w = run(seed, distance, load, enabled, 1_000);
            let carried: u64 = w.agents().iter().map(|a| a.inventory()[1]).sum();
            let rejected = w
                .events()
                .iter()
                .filter(|e| matches!(e.kind, EventKind::Rejected(_, Error::Capacity)))
                .count();
            let arrived = w
                .events()
                .iter()
                .filter(|e| e.kind == EventKind::Arrived)
                .count();
            println!(
                "{seed},{distance},{load},{enabled},600000,{},{},{carried},{},{rejected},{arrived},{}",
                w.initial_stock()[1],
                w.nodes()[0].loose()[1],
                w.nodes()[1].deposits()[1],
                w.audit().is_ok()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_command_cadence_is_independent_of_advance_chunk() {
        assert_eq!(
            run(0, 10_000, 10, true, 500),
            run(0, 10_000, 10, true, 2_000)
        );
    }
    #[test]
    fn disabling_transport_prevents_remote_delivery() {
        let w = run(0, 10_000, 10, false, 1_000);
        assert_eq!(w.nodes()[0].loose()[1], 0);
        assert_eq!(w.nodes()[1].deposits()[1] as u128, w.initial_stock()[1]);
        assert!(w.events().is_empty());
    }
}
