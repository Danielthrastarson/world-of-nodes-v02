use super::*;
fn manual(count: usize) -> World {
    World::new(Config {
        starts: vec![0; count],
        autonomous: false,
        trace: true,
        ..Config::default()
    })
    .unwrap()
}
fn act(w: &mut World, id: usize, intent: Intent) {
    w.apply_intents(vec![(id, intent)]).unwrap();
}
#[test]
fn no_food_death_is_exactly_36h_and_not_before() {
    let mut w = manual(2);
    w.advance_to(36 * HOUR - 1).unwrap();
    assert_eq!(w.metrics.deaths, 0);
    w.advance_to(36 * HOUR).unwrap();
    assert_eq!(w.metrics.deaths, 2);
    assert!(w.agents.iter().all(|a| a.death_time == Some(36 * HOUR)));
    assert_eq!(w.metrics.deficit_ms, 48 * HOUR as u128);
    w.audit().unwrap();
}
#[test]
fn split_gather_transfer_never_refresh_age() {
    let mut w = World::new(Config {
        starts: vec![2],
        autonomous: false,
        ..Config::default()
    })
    .unwrap();
    // Keep the fixture alive without adding new matter; consumption is explicitly ledgered.
    w.goods.move_part(0, 48, Owner::Agent(0), Holder::Loose(2));
    for h in [0, 12, 24, 36] {
        w.advance_to(h * HOUR).unwrap();
        let batch = w
            .goods
            .batches
            .iter()
            .find(|b| b.holder == Holder::Loose(2) && b.quantity > 0)
            .unwrap()
            .id;
        act(
            &mut w,
            0,
            Intent::Eat {
                batch,
                quantity: 12,
            },
        );
    }
    w.advance_to(47 * HOUR).unwrap();
    act(
        &mut w,
        0,
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    );
    w.advance_to(47 * HOUR + 10_000).unwrap();
    let carried = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(0) && b.quantity > 0)
        .unwrap()
        .id;
    w.goods
        .move_part(carried, 1, Owner::Commons(0), Holder::Commons(2));
    w.advance_to(48 * HOUR).unwrap();
    assert!(
        w.goods
            .batches
            .iter()
            .filter(|b| b.quantity > 0)
            .all(|b| b.material == Material::Waste && b.created_at == 0)
    );
    w.audit().unwrap();
}
#[test]
fn communal_consumption_is_immediate_capped_and_preserves_owner() {
    let mut w = manual(1);
    w.agents[0].nutrition = 8 * HOUR;
    let batch = w
        .goods
        .move_part(0, 10, Owner::Commons(0), Holder::Commons(0));
    act(&mut w, 0, Intent::Eat { batch, quantity: 4 });
    assert_eq!(w.agents[0].nutrition, 12 * HOUR);
    assert_eq!(w.goods.load(0), 0);
    act(&mut w, 0, Intent::Pickup { batch, quantity: 1 });
    act(&mut w, 0, Intent::Eat { batch, quantity: 1 });
    assert_eq!(w.goods.batches[batch].quantity, 6);
    assert_eq!(w.goods.consumed, 4);
    w.audit().unwrap();
}
#[test]
fn communal_carrier_eating_reduces_delivery() {
    let mut w = manual(1);
    w.agents[0].nutrition = 5 * HOUR;
    let batch = w
        .goods
        .move_part(0, 10, Owner::Commons(0), Holder::Agent(0));
    act(&mut w, 0, Intent::Eat { batch, quantity: 7 });
    assert_eq!(w.goods.batches[batch].quantity, 3);
    assert_eq!(w.agents[0].nutrition, 12 * HOUR);
    assert_eq!(w.goods.batches[batch].owner, Owner::Commons(0));
    w.audit().unwrap();
}
#[test]
fn mid_edge_death_preserves_position_property_and_material() {
    let mut w = manual(1);
    w.config.edges[0].nominal_ms = 100 * HOUR;
    w.goods.move_part(0, 3, Owner::Agent(0), Holder::Agent(0));
    act(
        &mut w,
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    );
    w.advance_to(36 * HOUR).unwrap();
    let Location::Edge {
        edge,
        from,
        progress,
        ..
    } = w.agents[0].location
    else {
        panic!("teleported")
    };
    assert!(progress > 0 && progress < 100 * HOUR as u128 * SCALE);
    let cargo = w
        .goods
        .batches
        .iter()
        .find(|b| {
            b.holder
                == Holder::Edge {
                    edge,
                    from,
                    progress,
                }
        })
        .unwrap();
    assert_eq!(cargo.owner, Owner::Agent(0));
    assert_eq!(cargo.quantity, 3);
    assert_eq!(w.goods.load(0), 0);
    w.audit().unwrap();
}
#[test]
fn death_does_not_remotely_clear_board_claim() {
    let mut w = manual(2);
    w.boards[0].quantity = 10;
    act(
        &mut w,
        0,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    );
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 23 * HOUR;
    w.config.edges[0].nominal_ms = 3 * HOUR;
    act(
        &mut w,
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    );
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert!(w.agents[0].job.is_none());
    assert_eq!(w.boards[0].claims.len(), 1);
    w.advance_to(12 * HOUR).unwrap();
    assert!(w.boards[0].claims.is_empty());
    act(
        &mut w,
        1,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    );
    assert!(w.agents[1].job.is_some());
    w.audit().unwrap();
}
#[test]
fn fifo_is_independent_of_submission_order_and_supports_zero_capacity() {
    let mut a = manual(3);
    a.config.edges[0].capacity = 1;
    let mut b = a.clone();
    let intents: Vec<_> = (0..3)
        .map(|id| {
            (
                id,
                Intent::Travel {
                    edge: 0,
                    target: 1,
                    food: false,
                },
            )
        })
        .collect();
    let mut reverse = intents.clone();
    reverse.reverse();
    a.apply_intents(intents).unwrap();
    b.apply_intents(reverse).unwrap();
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.event_hash, b.event_hash);
    a.advance_to(2 * HOUR).unwrap();
    assert_eq!(a.agents[1].action, Action::Travel);
    assert!(matches!(a.agents[2].action, Action::Queued { .. }));
    let mut zero = manual(1);
    zero.config.edges[0].capacity = 0;
    act(
        &mut zero,
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    );
    zero.advance_to(36 * HOUR).unwrap();
    assert_eq!(zero.agents[0].location, Location::Node(0));
    assert_eq!(zero.metrics.deaths, 1);
}
#[test]
fn local_eating_cancels_queue_without_advancing_it() {
    let mut w = manual(2);
    w.config.edges[0].capacity = 1;
    w.apply_intents(
        (0..2)
            .map(|id| {
                (
                    id,
                    Intent::Travel {
                        edge: 0,
                        target: 1,
                        food: false,
                    },
                )
            })
            .collect(),
    )
    .unwrap();
    w.agents[1].nutrition = 5 * HOUR;
    let batch = w
        .goods
        .move_part(0, 7, Owner::Commons(0), Holder::Commons(0));
    act(&mut w, 1, Intent::Eat { batch, quantity: 7 });
    assert_eq!(w.agents[1].action, Action::Idle);
    assert_eq!(w.agents[0].action, Action::Travel);
}
#[test]
fn reservation_contention_interruption_and_spoilage_are_conservative() {
    let mut w = manual(2);
    w.goods.batches[0].holder = Holder::Deposit(0);
    w.goods.batches[0].quantity = 1;
    w.goods.initial[0] = 1;
    act(
        &mut w,
        0,
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    );
    act(
        &mut w,
        1,
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    );
    assert_eq!(w.reserved(0), 1);
    act(&mut w, 0, Intent::Sleep);
    act(
        &mut w,
        1,
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    );
    assert_eq!(w.reserved(0), 1);
    w.goods.batches[0].spoil_after = 1000;
    w.advance_to(2000).unwrap();
    assert_eq!(w.reserved(0), 0);
    assert_eq!(w.metrics.gathered, 0);
    assert_eq!(w.metrics.spoiled, 1);
    w.audit().unwrap();
}
#[test]
fn local_observation_does_not_reveal_destinations_or_remote_changes() {
    let w = manual(1);
    let observation = w.observe(0).unwrap();
    let local = observation.local.unwrap();
    assert!(local.food.is_empty());
    assert_eq!(local.edges.len(), 1);
    assert_eq!(local.edges[0].edge.b, 1);
    let mut m = PolicyMemory::default();
    m.nodes.insert(0, local);
    let mut changed = w.clone();
    changed.goods.batches[0].holder = Holder::Loose(2);
    changed.goods.batches[0].material = Material::Waste;
    assert_eq!(
        policy::decide(&w.observe(0).unwrap(), &m, &mut 1),
        policy::decide(&changed.observe(0).unwrap(), &m, &mut 1)
    );
}
#[test]
fn live_advance_partition_does_not_change_state_or_events() {
    let cfg = Config {
        starts: vec![0; 3],
        ..Config::default()
    };
    let mut a = World::new(cfg).unwrap();
    let mut b = a.clone();
    a.advance_to(8 * HOUR).unwrap();
    for t in (0..8 * HOUR).step_by(137_137) {
        b.advance_to(t).unwrap();
    }
    b.advance_to(8 * HOUR).unwrap();
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.goods, b.goods);
    assert_eq!(a.metrics, b.metrics);
    assert_eq!(a.event_hash, b.event_hash);
}
#[test]
fn reversed_policy_evaluation_is_identical() {
    let mut a = World::new(Config {
        starts: vec![0; 5],
        ..Config::default()
    })
    .unwrap();
    let mut b = a.clone();
    b.config.reverse_evaluation = true;
    a.advance_to(40 * HOUR).unwrap();
    b.advance_to(40 * HOUR).unwrap();
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.goods, b.goods);
    assert_eq!(a.event_hash, b.event_hash);
    assert_eq!(a.metrics, b.metrics);
}

#[test]
fn remote_queue_and_inflight_positions_are_not_local_observations() {
    let mut w = manual(2);
    w.agents[1].location = Location::Node(1);
    let before = w.observe(0).unwrap().local.unwrap();
    w.agents[1].action = Action::Queued {
        edge: 0,
        to: 0,
        joined: 0,
    };
    assert_eq!(before, w.observe(0).unwrap().local.unwrap());
    w.agents[1].action = Action::Travel;
    w.agents[1].location = Location::Edge {
        edge: 0,
        from: 1,
        to: 0,
        progress: 1000,
    };
    assert_eq!(before, w.observe(0).unwrap().local.unwrap());
}
#[test]
fn dead_personal_property_is_inaccessible_even_to_starving_neighbours() {
    let mut w = manual(2);
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 24 * HOUR;
    w.goods.move_part(0, 5, Owner::Agent(0), Holder::Agent(0));
    w.advance_to(0).unwrap();
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Loose(0) && b.quantity > 0)
        .unwrap()
        .id;
    w.agents[1].nutrition = 0;
    act(&mut w, 1, Intent::Eat { batch, quantity: 5 });
    act(&mut w, 1, Intent::Pickup { batch, quantity: 5 });
    assert_eq!(w.agents[1].nutrition, 0);
    assert_eq!(w.goods.batches[batch].quantity, 5);
    w.audit().unwrap();
}
#[test]
fn expired_claim_does_not_cancel_original_worker_or_duplicate_goods() {
    let mut w = manual(2);
    w.boards[0].quantity = 10;
    act(
        &mut w,
        0,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    );
    act(
        &mut w,
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    );
    w.advance_to(12 * HOUR).unwrap();
    assert!(w.agents[0].job.is_some());
    assert!(w.boards[0].claims.is_empty());
    act(
        &mut w,
        1,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    );
    assert!(w.agents.iter().all(|a| a.job.is_some()));
    assert_eq!(w.goods.initial[0], 3000);
    w.audit().unwrap();
}
#[test]
fn exploration_has_no_hidden_food_target_under_relabeling() {
    let mut cfg = Config {
        starts: vec![0],
        nodes: 3,
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR,
                capacity: 1,
            },
            Edge {
                a: 0,
                b: 2,
                nominal_ms: HOUR,
                capacity: 1,
            },
        ],
        food: vec![(1, 100, 0, 48 * HOUR)],
        autonomous: false,
        ..Config::default()
    };
    let a = World::new(cfg.clone()).unwrap();
    cfg.food[0].0 = 2;
    let b = World::new(cfg).unwrap();
    let mut ma = PolicyMemory::default();
    ma.nodes.insert(0, a.node_view(0));
    let mut mb = PolicyMemory::default();
    mb.nodes.insert(0, b.node_view(0));
    assert_eq!(
        policy::decide(&a.observe(0).unwrap(), &ma, &mut 5),
        policy::decide(&b.observe(0).unwrap(), &mb, &mut 5)
    );
    let d = policy::decide(&a.observe(0).unwrap(), &ma, &mut 5);
    assert!(d.reason.contains("explore unknown"));
    assert!(matches!(d.intent, Intent::Travel { food: false, .. }));
}
#[test]
fn nearer_feasible_food_beats_fatal_or_predictably_spoiled_food() {
    let cfg = Config {
        starts: vec![0],
        nodes: 3,
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR / 10,
                capacity: 1,
            },
            Edge {
                a: 0,
                b: 2,
                nominal_ms: 12 * HOUR,
                capacity: 1,
            },
        ],
        food: vec![(1, 2, 0, 48 * HOUR), (2, 3000, 0, HOUR)],
        autonomous: false,
        ..Config::default()
    };
    let mut w = World::new(cfg).unwrap();
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 23 * HOUR;
    let mut m = PolicyMemory::default();
    for n in 0..3 {
        m.nodes.insert(n, w.node_view(n));
    }
    let decision = policy::decide(&w.observe(0).unwrap(), &m, &mut 1);
    assert!(
        matches!(decision.intent, Intent::Travel { target: 1, .. }),
        "{decision:?}"
    );
}
#[test]
fn policy_does_not_force_a_return_from_food() {
    let w = World::new(Config {
        starts: vec![2],
        autonomous: false,
        ..Config::default()
    })
    .unwrap();
    let mut m = PolicyMemory::default();
    m.nodes.insert(2, w.node_view(2));
    let d = policy::decide(&w.observe(0).unwrap(), &m, &mut 4);
    assert!(!matches!(d.intent, Intent::Travel { .. }));
}
#[test]
fn communal_gathering_ownership_delivery_and_partial_consumption() {
    let mut w = manual(1);
    w.boards[0].quantity = 3;
    act(
        &mut w,
        0,
        Intent::Claim {
            board: 0,
            quantity: 3,
        },
    );
    w.agents[0].location = Location::Node(2);
    for _ in 0..3 {
        act(
            &mut w,
            0,
            Intent::Gather {
                batch: 0,
                communal: true,
            },
        );
        w.advance_to(w.now + 3000).unwrap();
    }
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(0) && b.quantity > 0)
        .unwrap()
        .id;
    assert_eq!(w.goods.batches[batch].owner, Owner::Commons(0));
    w.agents[0].nutrition = 11 * HOUR;
    act(&mut w, 0, Intent::Eat { batch, quantity: 1 });
    w.agents[0].location = Location::Node(0);
    act(&mut w, 0, Intent::Deliver { batch, quantity: 2 });
    assert_eq!(w.metrics.delivered, 2);
    assert!(w.agents[0].job.is_none());
    w.audit().unwrap();
}
#[test]
fn communal_hauling_requires_job_and_does_not_create_personal_property() {
    let mut w = manual(1);
    let batch = w
        .goods
        .move_part(0, 10, Owner::Commons(0), Holder::Commons(1));
    w.agents[0].location = Location::Node(1);
    act(&mut w, 0, Intent::LoadCargo { batch, quantity: 4 });
    assert_eq!(w.goods.load(0), 0);
    w.agents[0].location = Location::Node(0);
    w.boards[0].quantity = 4;
    act(
        &mut w,
        0,
        Intent::Claim {
            board: 0,
            quantity: 4,
        },
    );
    w.agents[0].location = Location::Node(1);
    act(&mut w, 0, Intent::LoadCargo { batch, quantity: 4 });
    assert_eq!(w.goods.load(0), 4);
    assert!(
        w.goods
            .batches
            .iter()
            .filter(|b| b.holder == Holder::Agent(0) && b.quantity > 0)
            .all(|b| b.owner == Owner::Commons(0))
    );
    w.audit().unwrap();
}
#[test]
fn sleep_recovery_continues_across_atomic_sleep_ticks() {
    let mut w = World::new(Config {
        starts: vec![0],
        initial_nutrition: 24 * HOUR,
        initial_fatigue: 22 * HOUR,
        food: vec![],
        ..Config::default()
    })
    .unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert!(w.agents[0].fatigue <= 5 * HOUR);
    assert!(w.metrics.sleep_ms >= 8 * HOUR as u128);
}
#[test]
fn physiology_changes_speed_during_a_trip_not_only_at_departure() {
    let mut w = manual(1);
    w.config.edges[0].nominal_ms = 8 * HOUR;
    act(
        &mut w,
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    );
    w.advance_to(8 * HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Travel);
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents[0].location, Location::Node(1));
}
#[test]
fn stale_depletion_fixture_changes_truth_without_refreshing_absent_memories() {
    let result = experiment::run("stale_depletion", 0, false).unwrap();
    assert!(result.world.metrics.stale_failures > 0);
    assert_eq!(result.world.goods.initial[0], 6);
    assert_eq!(result.world.goods.consumed, 6);
    result.world.audit().unwrap();
}
