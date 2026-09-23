use super::*;
fn fixture() -> World {
    World::new(super::super::Config {
        nodes: 2,
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: HOUR,
            capacity: 1,
        }],
        starts: vec![0, 0],
        food: vec![],
        boards: vec![1],
        communal: true,
        initial_request: 100,
        autonomous: false,
        trace: true,
        capital: Some(Config {
            rules: Rules::default(),
            wood: vec![(0, 20)],
        }),
        ..super::super::Config::default()
    })
    .unwrap()
}
fn gather_wood(w: &mut World, id: usize, q: u64) {
    for _ in 0..q {
        w.apply_intents(vec![(
            id,
            Intent::Gather {
                batch: 0,
                communal: false,
            },
        )])
        .unwrap();
        while matches!(w.agents[id].action, Action::Gather { .. }) {
            w.advance_to(w.next_time(w.now + HOUR)).unwrap();
        }
    }
}
fn build(w: &mut World, id: usize) {
    gather_wood(w, id, 10);
    w.apply_intents(vec![(id, Intent::BuildCart)]).unwrap();
    while matches!(w.agents[id].action, Action::BuildCart { .. }) {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
}
#[test]
fn completed_cart_has_owner_and_separate_user_and_conserved_wood() {
    let mut w = fixture();
    gather_wood(&mut w, 0, 10);
    w.apply_intents(vec![(0, Intent::BuildCart)]).unwrap();
    assert!(w.carts().is_empty());
    assert_eq!(w.capital.projects[0].wood, 10);
    assert_eq!(w.goods.initial[1], 20);
    assert_eq!(
        w.goods
            .batches
            .iter()
            .filter(|b| b.material == Material::Wood)
            .map(|b| b.quantity)
            .sum::<u64>(),
        10
    );
    while w.carts().is_empty() {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
    assert_eq!(w.carts()[0].owner, Owner::Agent(0));
    assert_eq!(w.carts()[0].user, None);
    assert_eq!(w.carrier_capacity(0), 10);
    w.apply_intents(vec![(1, Intent::HitchCart { cart: 0 })])
        .unwrap();
    assert_eq!(w.carts()[0].user, None);
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    assert_eq!(w.carts()[0].user, Some(0));
    assert_eq!(w.carrier_capacity(0), 30);
    w.audit().unwrap();
}
#[test]
fn interruption_retains_work_inputs_and_death_does_not_create_cart() {
    let mut w = fixture();
    gather_wood(&mut w, 0, 10);
    w.apply_intents(vec![(0, Intent::BuildCart)]).unwrap();
    w.advance_to(w.now + HOUR).unwrap();
    w.apply_intents(vec![(0, Intent::CancelAction)]).unwrap();
    let remaining = w.capital.projects[0].remaining;
    w.advance_to(36 * HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert!(w.carts().is_empty());
    assert_eq!(w.capital.projects[0].remaining, remaining);
    w.audit().unwrap();
}
#[test]
fn actual_container_load_parking_travel_fifo_and_age() {
    let mut w = fixture();
    build(&mut w, 0);
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    let b = w.goods.add(
        Material::Food,
        30,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 30,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.load(0), 10);
    assert_eq!(w.cart_load(0), 20);
    assert!(
        w.batches()
            .iter()
            .filter(|b| b.material == Material::Food)
            .all(|b| b.created_at == 0)
    );
    w.apply_intents(vec![(0, Intent::ParkCart)]).unwrap();
    assert_eq!(w.carrier_capacity(0), 10);
    assert_eq!(w.cart_load(0), 20);
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(w.now + 2 * HOUR).unwrap();
    assert_eq!(w.carts()[0].location, Location::Node(0));
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    assert_eq!(w.carts()[0].user, None);
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(w.now + 4 * HOUR).unwrap();
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    w.apply_intents(vec![
        (
            1,
            Intent::Travel {
                edge: 0,
                target: 1,
                food: false,
            },
        ),
        (
            0,
            Intent::Travel {
                edge: 0,
                target: 1,
                food: false,
            },
        ),
    ])
    .unwrap();
    assert_eq!(w.agents[0].action, Action::Travel);
    assert!(matches!(w.agents[1].action, Action::Queued { .. }));
    assert_eq!(w.carts()[0].location, w.agents[0].location);
    w.audit().unwrap();
}
#[test]
fn death_mid_edge_keeps_cart_and_cargo_without_inheritance() {
    let mut w = fixture();
    build(&mut w, 0);
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    let b = w.goods.add(
        Material::Food,
        30,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 30,
        },
    )])
    .unwrap();
    w.advance_to(35 * HOUR).unwrap();
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(36 * HOUR).unwrap();
    assert_eq!(w.carts()[0].user, None);
    assert_eq!(w.carts()[0].owner, Owner::Agent(0));
    assert!(matches!(w.carts()[0].location, Location::Edge { .. }));
    assert_eq!(w.carts()[0].location, w.agents[0].location);
    assert_eq!(w.cart_load(0), 20);
    w.advance_to(48 * HOUR).unwrap();
    assert!(
        w.batches()
            .iter()
            .filter(|b| b.holder == Holder::Cart(0))
            .all(|b| b.material == Material::Waste && b.owner == Owner::Agent(0))
    );
    w.audit().unwrap();
}
#[test]
fn no_wood_or_no_work_cannot_spawn_capital() {
    let mut w = fixture();
    w.apply_intents(vec![(0, Intent::BuildCart)]).unwrap();
    assert!(w.cart_projects().is_empty());
    gather_wood(&mut w, 0, 10);
    w.advance_to(36 * HOUR).unwrap();
    assert!(w.carts().is_empty());
    w.audit().unwrap();
}
#[test]
fn frozen_atomic_wood_and_construction_forecast_matches_kernel() {
    for n in [6, 12, 24] {
        for f in [0, 12, 20, 24] {
            let mut w = fixture();
            w.agents[0].nutrition = n * HOUR;
            w.agents[0].fatigue = f * HOUR;
            let forecast = policy::coherent::capital_work(
                &w.observe(0).unwrap(),
                10,
                4 * HOUR as u128 * SCALE,
            )
            .unwrap();
            build(&mut w, 0);
            assert_eq!(
                forecast,
                (w.now, w.agents[0].nutrition, w.agents[0].fatigue)
            );
            w.audit().unwrap();
        }
    }
}
#[test]
fn remote_wood_is_not_a_policy_input() {
    let mut c = fixture().config;
    c.autonomous = true;
    c.starts = vec![0];
    c.capital.as_mut().unwrap().wood = vec![(1, 10)];
    let mut a = World::new(c.clone()).unwrap();
    c.capital.as_mut().unwrap().wood[0].1 = 500;
    let mut b = World::new(c).unwrap();
    a.advance_to(0).unwrap();
    b.advance_to(0).unwrap();
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.event_hash(), b.event_hash());
    assert!(a.cart_memory(0).unwrap()[&0].wood.is_empty());
}
#[test]
fn partial_delivery_across_containers_keeps_same_job_until_payload_unloaded() {
    let mut w = fixture();
    build(&mut w, 0);
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    w.agents[0].job = Some(Job {
        request: 1,
        destination: 1,
        target: 30,
        acquired: 30,
    });
    let b = w.goods.add(
        Material::Food,
        30,
        Owner::Commons(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    w.load_carrier(0, b, 30, Owner::Commons(0));
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(w.now + 2 * HOUR).unwrap();
    let cargo: Vec<_> = w
        .batches()
        .iter()
        .filter(|b| w.carried_by(0, b.holder) && b.quantity > 0)
        .map(|b| (b.id, b.quantity))
        .collect();
    for (i, (batch, q)) in cargo.iter().enumerate() {
        w.apply_intents(vec![(
            0,
            Intent::Deliver {
                batch: *batch,
                quantity: *q,
            },
        )])
        .unwrap();
        assert_eq!(w.agents[0].job.is_some(), i + 1 < cargo.len());
    }
    assert_eq!(w.metrics.delivered, 30);
    w.audit().unwrap();
}
#[test]
fn loaded_cart_never_overtakes_an_earlier_unloaded_traveler() {
    let mut w = fixture();
    build(&mut w, 0);
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    let b = w.goods.add(
        Material::Food,
        30,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 30,
        },
    )])
    .unwrap();
    w.apply_intents(vec![(
        1,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[1].action, Action::Travel);
    assert!(matches!(w.agents[0].action, Action::Queued { .. }));
    w.advance_to(w.now + HOUR).unwrap();
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}
#[test]
fn investment_compares_demand_not_cart_availability() {
    fn evaluate_for(request: u64) -> Evaluation {
        let c = super::super::Config {
            prior_memory: true,
            initial_request: request,
            capital: Some(Config {
                rules: Rules::default(),
                wood: vec![(0, 500)],
            }),
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: HOUR / 4,
                    capacity: 4,
                },
                Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: HOUR / 4,
                    capacity: 4,
                },
            ],
            ..super::super::Config::default()
        };
        let mut w = World::new(c).unwrap();
        w.observe_capital(0);
        let o = w.observe(0).unwrap();
        evaluate(
            &o,
            &w.agents[0].memory,
            &w.capital.memories[0],
            &Rules::default(),
        )
    }
    let low = evaluate_for(30);
    let high = evaluate_for(600);
    assert_eq!(low.gate, "NO_TIME_PAYBACK");
    assert!(low.intent.is_none());
    assert_eq!(high.gate, "INVEST");
    assert!(matches!(
        high.intent,
        Some(Intent::Gather {
            communal: false,
            ..
        })
    ));
}
