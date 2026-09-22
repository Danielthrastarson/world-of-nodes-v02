use world_of_nodes::*;

fn world() -> World {
    World::new(reference_scenario(7, 10_000, 10)).unwrap()
}

#[test]
fn seeded_initialization_and_identity_are_reproducible() {
    assert_eq!(world(), world());
    assert_ne!(
        world().initial_stock(),
        World::new(reference_scenario(8, 10_000, 10))
            .unwrap()
            .initial_stock()
    );
    assert_eq!(world().agents().len(), 50);
    world().audit().unwrap();
}

#[test]
fn transfers_preserve_material_and_enforce_shared_carrying_capacity() {
    let mut w = world();
    let before = w.initial_stock();
    w.command(
        0,
        Command::Pickup {
            resource: Resource::Food,
            amount: 8,
        },
    )
    .unwrap();
    assert_eq!(
        w.command(
            0,
            Command::Pickup {
                resource: Resource::Food,
                amount: 3
            }
        ),
        Err(Error::Capacity)
    );
    assert_eq!(w.nodes()[0].loose()[0], 492);
    assert_eq!(w.agents()[0].inventory(), [8, 0, 0]);
    w.command(
        0,
        Command::Drop {
            resource: Resource::Food,
            amount: 8,
        },
    )
    .unwrap();
    assert_eq!(w.nodes()[0].loose()[0], 500);
    assert_eq!(w.totals(), before);
    w.audit().unwrap();
}

#[test]
fn arrival_is_required_and_travel_excludes_work_and_stock_access() {
    let mut w = world();
    assert_eq!(
        w.command(
            0,
            Command::Gather {
                resource: Resource::Wood,
                amount: 1
            }
        ),
        Err(Error::InsufficientStock)
    );
    w.command(0, Command::Travel { edge: 0 }).unwrap();
    for cmd in [
        Command::Pickup {
            resource: Resource::Food,
            amount: 1,
        },
        Command::Gather {
            resource: Resource::Wood,
            amount: 1,
        },
        Command::Travel { edge: 1 },
    ] {
        assert_eq!(w.command(0, cmd), Err(Error::Busy));
    }
    w.advance_to(9_999).unwrap();
    assert!(matches!(w.agents()[0].action(), Action::Travel { .. }));
    w.advance_to(10_000).unwrap();
    assert_eq!(w.agents()[0].action(), &Action::Idle { node: 1 });
    assert_eq!(
        w.command(0, Command::Travel { edge: 1 }),
        Err(Error::NotLocal)
    );
    w.command(
        0,
        Command::Gather {
            resource: Resource::Wood,
            amount: 1,
        },
    )
    .unwrap();
    w.advance_to(11_999).unwrap();
    assert_eq!(w.agents()[0].inventory()[1], 0);
    w.advance_to(12_000).unwrap();
    assert_eq!(w.agents()[0].inventory()[1], 1);
}

#[test]
fn individual_edge_capacity_allows_progress_and_is_shared_between_directions() {
    let mut s = reference_scenario(0, 10_000, 10);
    s.homes[1] = 1;
    s.edges[0].capacity = 1;
    let mut w = World::new(s).unwrap();
    w.command(0, Command::Travel { edge: 0 }).unwrap();
    assert_eq!(
        w.command(1, Command::Travel { edge: 0 }),
        Err(Error::Capacity)
    );
    assert_eq!(
        w.command(2, Command::Travel { edge: 0 }),
        Err(Error::Capacity)
    );
    w.advance_to(10_000).unwrap();
    w.command(1, Command::Travel { edge: 0 }).unwrap();
    w.advance_to(20_000).unwrap();
    w.command(2, Command::Travel { edge: 0 }).unwrap();
    w.advance_to(30_000).unwrap();
    w.audit().unwrap();
}

#[test]
fn last_deposit_unit_is_reserved_once_and_cancel_releases_it() {
    let mut s = reference_scenario(0, 10_000, 10);
    s.nodes[0].deposits = [0, 1, 0];
    let mut w = World::new(s).unwrap();
    let gather = Command::Gather {
        resource: Resource::Wood,
        amount: 1,
    };
    w.command(0, gather).unwrap();
    assert_eq!(w.nodes()[0].deposits()[1], 1);
    assert_eq!(w.agents()[0].inventory()[1], 0);
    assert_eq!(w.command(1, gather), Err(Error::InsufficientStock));
    w.advance_to(1_999).unwrap();
    w.command(0, Command::CancelGather).unwrap();
    w.command(1, gather).unwrap();
    w.advance_to(3_999).unwrap();
    assert_eq!(w.agents()[0].inventory()[1], 0);
    assert_eq!(w.agents()[1].inventory()[1], 1);
    assert_eq!(w.nodes()[0].reserved(), [0; 3]);
    w.audit().unwrap();
}

#[test]
fn distance_changes_arrival_without_whole_tick_rounding() {
    for distance in [10_000, 10_001, 20_000] {
        let mut w = World::new(reference_scenario(0, distance, 10)).unwrap();
        w.command(0, Command::Travel { edge: 0 }).unwrap();
        w.advance_to(distance - 1).unwrap();
        assert!(matches!(w.agents()[0].action(), Action::Travel { .. }));
        w.advance_to(distance).unwrap();
        assert!(matches!(w.agents()[0].action(), Action::Idle { .. }));
        assert_eq!(w.events().last().unwrap().time_ms, distance);
    }
}

#[test]
fn completion_times_and_events_do_not_depend_on_advance_partition() {
    let mut coarse = world();
    coarse.command(0, Command::Travel { edge: 0 }).unwrap();
    coarse.command(1, Command::Travel { edge: 1 }).unwrap();
    let mut fine = coarse.clone();
    coarse.advance_to(12_345).unwrap();
    for t in (0..12_345).step_by(137) {
        fine.advance_to(t).unwrap();
    }
    fine.advance_to(12_345).unwrap();
    assert_eq!(fine, coarse);
    for w in [&mut fine, &mut coarse] {
        w.command(
            0,
            Command::Gather {
                resource: Resource::Wood,
                amount: 3,
            },
        )
        .unwrap();
    }
    coarse.advance_to(30_000).unwrap();
    for t in 12_345..=30_000 {
        fine.advance_to(t).unwrap();
    }
    assert_eq!(fine, coarse);
    let completion = coarse.events().last().unwrap();
    let cause = &coarse.events()[completion.cause.unwrap()];
    assert_eq!(completion.time_ms - cause.time_ms, 6_000);
}

#[test]
fn overflow_and_invalid_commands_are_atomic() {
    let mut w = world();
    w.advance_to(u64::MAX - 1).unwrap();
    assert_eq!(
        w.command(0, Command::Travel { edge: 0 }),
        Err(Error::TimeOverflow)
    );
    assert_eq!(w.agents()[0].action(), &Action::Idle { node: 0 });
    assert_eq!(
        w.command(99, Command::Travel { edge: 0 }),
        Err(Error::UnknownAgent)
    );
    assert_eq!(
        w.command(
            0,
            Command::Pickup {
                resource: Resource::Food,
                amount: 0
            }
        ),
        Err(Error::InvalidAmount)
    );
    assert_eq!(w.advance_to(0), Err(Error::TimeReversal));
    w.audit().unwrap();
    let mut s = reference_scenario(0, 10_000, 10);
    s.edges[0].length_mm = 999; // shorter than physical endpoint distance
    assert_eq!(World::new(s), Err(Error::InvalidScenario));
}

#[test]
fn mixed_resources_share_one_load_limit() {
    let mut w = world();
    w.command(
        0,
        Command::Pickup {
            resource: Resource::Food,
            amount: 8,
        },
    )
    .unwrap();
    w.command(0, Command::Travel { edge: 0 }).unwrap();
    w.advance_to(10_000).unwrap();
    assert_eq!(
        w.command(
            0,
            Command::Gather {
                resource: Resource::Wood,
                amount: 3
            }
        ),
        Err(Error::Capacity)
    );
    w.command(
        0,
        Command::Gather {
            resource: Resource::Wood,
            amount: 2,
        },
    )
    .unwrap();
    w.advance_to(14_000).unwrap();
    assert_eq!(w.agents()[0].inventory(), [8, 2, 0]);
}

#[test]
fn randomized_command_sequences_preserve_invariants() {
    for seed in 0..30 {
        let mut w = World::new(reference_scenario(seed, 1_000, 10)).unwrap();
        let mut random = seed + 1;
        for step in 0..2_000 {
            random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
            let agent = ((random >> 32) % 50) as usize;
            let resource =
                [Resource::Food, Resource::Wood, Resource::Stone][((random >> 20) % 3) as usize];
            let amount = (random >> 8) % 12;
            let command = match random % 6 {
                0 => Command::Travel {
                    edge: ((random >> 16) % 2) as usize,
                },
                1 => Command::Gather { resource, amount },
                2 => Command::Pickup { resource, amount },
                3 => Command::Drop { resource, amount },
                4 => Command::CancelGather,
                _ => Command::Travel { edge: 9 },
            };
            let _ = w.command(agent, command);
            w.advance_to(step * 500).unwrap();
            w.audit().unwrap();
        }
        w.advance_to(2_000_000).unwrap();
        w.audit().unwrap();
    }
}
