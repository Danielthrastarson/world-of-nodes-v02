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
