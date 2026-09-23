use super::*;
fn world(capacity: usize, work: Time, cost: u64) -> World {
    World::new(super::super::Config {
        nodes: 2,
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: 2 * HOUR,
            capacity,
        }],
        starts: vec![0; 3],
        food: vec![],
        boards: vec![0],
        initial_nutrition: 24 * HOUR,
        autonomous: false,
        trace: true,
        infrastructure: Some(Config {
            enabled: true,
            sites: vec![Site {
                edge: 0,
                node: 0,
                wood_cost: cost,
                work_ms: work,
                opened_nominal_ms: 3 * HOUR / 2,
                opened_capacity: capacity.max(1),
            }],
            wood: vec![(0, 20)],
        }),
        ..super::super::Config::default()
    })
    .unwrap()
}
fn gather(w: &mut World, id: usize, q: u64) {
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
fn donate(w: &mut World, id: usize, q: u64) {
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.material == Material::Wood && b.owner == Owner::Agent(id) && b.quantity > 0)
        .unwrap()
        .id;
    w.apply_intents(vec![(
        id,
        Intent::ContributeCrossing {
            project: 0,
            batch: b,
            quantity: q,
        },
    )])
    .unwrap();
}
fn finish(w: &mut World, id: usize) {
    w.apply_intents(vec![(id, Intent::BuildCrossing { project: 0 })])
        .unwrap();
    while w.crossing_projects()[0].completed_at.is_none() {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
}
#[test]
fn physically_pooled_inputs_labor_and_public_users_are_distinct() {
    let mut w = world(0, 8 * HOUR, 20);
    gather(&mut w, 0, 10);
    donate(&mut w, 0, 10);
    w.apply_intents(vec![(0, Intent::BuildCrossing { project: 0 })])
        .unwrap();
    assert_eq!(w.agents[0].action, Action::Idle);
    assert_eq!(w.effective_edge(0).capacity, 0);
    gather(&mut w, 1, 10);
    donate(&mut w, 1, 10);
    let start = w.now;
    finish(&mut w, 2);
    let p = &w.crossing_projects()[0];
    assert_eq!(p.owner, Owner::Commons(0));
    assert_eq!(p.completed_by, Some(2));
    assert_eq!(p.donors, BTreeMap::from([(0, 10), (1, 10)]));
    assert!(w.now - start >= 8 * HOUR);
    assert_eq!(w.goods.initial[1], 20);
    assert_eq!(w.goods.batches.iter().map(|b| b.quantity).sum::<u64>(), 0);
    for id in [0, 1] {
        w.apply_intents(vec![(
            id,
            Intent::Travel {
                edge: 0,
                target: 1,
                food: false,
            },
        )])
        .unwrap();
    }
    w.advance_to(w.now + 6 * HOUR).unwrap();
    assert!(
        w.crossing_trips()
            .iter()
            .any(|t| t.agent == 0 && t.improved && t.arrived.is_some())
    );
    assert!(
        w.crossing_trips()
            .iter()
            .any(|t| t.agent == 1 && t.improved && t.arrived.is_some())
    );
    w.audit().unwrap();
}
#[test]
fn donation_is_local_owned_and_cannot_overfill() {
    let mut w = world(0, HOUR, 2);
    gather(&mut w, 0, 3);
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.owner == Owner::Agent(0) && b.quantity > 0)
        .unwrap()
        .id;
    w.apply_intents(vec![(
        1,
        Intent::ContributeCrossing {
            project: 0,
            batch: b,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.crossing_projects()[0].wood, 0);
    donate(&mut w, 0, 3);
    assert_eq!(w.crossing_projects()[0].wood, 0);
    donate(&mut w, 0, 2);
    assert_eq!(w.crossing_projects()[0].wood, 2);
    w.audit().unwrap();
}
#[test]
fn interrupted_construction_retains_public_inputs_after_donor_death() {
    let mut w = world(0, 8 * HOUR, 2);
    gather(&mut w, 0, 2);
    donate(&mut w, 0, 2);
    w.apply_intents(vec![
        (0, Intent::BuildCrossing { project: 0 }),
        (1, Intent::BuildCrossing { project: 0 }),
    ])
    .unwrap();
    assert_eq!(w.agents[1].action, Action::Idle);
    w.advance_to(w.now + HOUR).unwrap();
    w.apply_intents(vec![(0, Intent::CancelAction)]).unwrap();
    let remaining = w.crossing_projects()[0].remaining;
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 24 * HOUR - SECOND;
    w.advance_to(w.now + SECOND).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(w.crossing_projects()[0].remaining, remaining);
    assert_eq!(w.crossing_projects()[0].owner, Owner::Commons(0));
    finish(&mut w, 1);
    assert_eq!(w.crossing_projects()[0].completed_by, Some(1));
    w.audit().unwrap();
}
#[test]
fn completion_does_not_rewrite_inflight_traversal_or_fifo() {
    let mut w = world(1, HOUR / 2, 1);
    gather(&mut w, 0, 1);
    donate(&mut w, 0, 1);
    let start = w.now;
    w.apply_intents(vec![(
        1,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.agents[2].location = Location::Node(1);
    w.apply_intents(vec![
        (
            2,
            Intent::Travel {
                edge: 0,
                target: 0,
                food: false,
            },
        ),
        (0, Intent::BuildCrossing { project: 0 }),
    ])
    .unwrap();
    w.advance_to(start + HOUR).unwrap();
    assert_eq!(w.effective_edge(0).nominal_ms, 3 * HOUR / 2);
    assert_eq!(w.agents[1].action, Action::Travel);
    assert!(matches!(w.agents[2].action, Action::Queued { .. }));
    w.advance_to(start + 2 * HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(1));
    assert_eq!(w.infrastructure.trips[0].arrived, Some(start + 2 * HOUR));
    assert_eq!(w.infrastructure.trips[1].agent, 2);
    assert_eq!(w.infrastructure.trips[1].nominal_ms, 3 * HOUR / 2);
    w.advance_to(start + 7 * HOUR / 2).unwrap();
    assert_eq!(w.agents[2].location, Location::Node(0));
    assert_eq!(w.metrics.overtakes, 0);
}
#[test]
fn distant_memory_does_not_receive_completion() {
    let mut w = world(0, HOUR, 1);
    w.agents[2].location = Location::Node(1);
    let before = w.node_view(1);
    w.agents[2].memory.nodes.insert(1, before.clone());
    gather(&mut w, 0, 1);
    donate(&mut w, 0, 1);
    finish(&mut w, 0);
    assert_eq!(w.agents[2].memory.nodes[&1], before);
    assert_eq!(before.edges[0].edge.capacity, 0);
    assert_eq!(
        w.observe(2).unwrap().local.unwrap().edges[0].edge.capacity,
        1
    );
}
#[test]
fn actual_wood_must_travel_before_donation() {
    let mut w = world(1, HOUR, 1);
    w.infrastructure.projects[0].site.node = 1;
    gather(&mut w, 0, 1);
    donate(&mut w, 0, 1);
    assert_eq!(w.crossing_projects()[0].wood, 0);
    let start = w.now;
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(start + 2 * HOUR).unwrap();
    donate(&mut w, 0, 1);
    assert_eq!(w.crossing_projects()[0].wood, 1);
    assert_eq!(w.infrastructure_counters()["wood_transport"], 1);
    w.audit().unwrap();
}
#[test]
fn construction_work_forecast_matches_physiology() {
    for n in [8, 12, 24] {
        for f in [4, 16, 21] {
            let mut w = world(0, HOUR, 1);
            gather(&mut w, 0, 1);
            donate(&mut w, 0, 1);
            w.agents[0].nutrition = n * HOUR;
            w.agents[0].fatigue = f * HOUR;
            let o = w.observe(0).unwrap();
            let (dt, nut, fat) =
                policy::coherent::capital_work(&o, 0, HOUR as u128 * SCALE).unwrap();
            let start = w.now;
            finish(&mut w, 0);
            assert_eq!(
                (w.now - start, w.agents[0].nutrition, w.agents[0].fatigue),
                (dt, nut, fat)
            );
        }
    }
}
#[test]
fn unobserved_remote_material_and_food_cannot_change_decision() {
    let mut a = world(0, HOUR, 1);
    let mut b = a.clone();
    b.goods.add(
        Material::Food,
        1000,
        Owner::None,
        Holder::Deposit(1),
        0,
        48 * HOUR,
    );
    for w in [&mut a, &mut b] {
        w.observe_infrastructure(0);
        let v = w.node_view(0);
        w.agents[0].memory.nodes.insert(0, v);
    }
    let decide = |w: &World| {
        consider(
            &w.observe(0).unwrap(),
            &w.agents[0].memory,
            &w.infrastructure.memories[0],
            true,
        )
    };
    assert_eq!(decide(&a), decide(&b));
    assert!(decide(&a).is_none());
}
#[test]
fn building_and_movement_share_one_partition_independent_clock() {
    let mut a = world(1, HOUR / 2, 1);
    gather(&mut a, 0, 1);
    donate(&mut a, 0, 1);
    a.apply_intents(vec![
        (0, Intent::BuildCrossing { project: 0 }),
        (
            1,
            Intent::Travel {
                edge: 0,
                target: 1,
                food: false,
            },
        ),
    ])
    .unwrap();
    let mut b = a.clone();
    let end = a.now + 4 * HOUR;
    a.advance_to(end).unwrap();
    while b.now < end {
        b.advance_to((b.now + 137 * SECOND).min(end)).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn renamed_nodes_preserve_the_same_observed_construction_choice() {
    let mut a = world(0, HOUR, 1);
    a.goods.add(
        Material::Food,
        100,
        Owner::None,
        Holder::Deposit(1),
        0,
        48 * HOUR,
    );
    for n in 0..2 {
        let v = a.node_view(n);
        a.agents[0].memory.nodes.insert(n, v);
    }
    a.observe_infrastructure(0);
    let original = consider(
        &a.observe(0).unwrap(),
        &a.agents[0].memory,
        &a.infrastructure.memories[0],
        true,
    )
    .unwrap();
    let mut b = world(0, HOUR, 1);
    b.config.edges[0].a = 1;
    b.config.edges[0].b = 0;
    b.agents[0].location = Location::Node(1);
    b.infrastructure.projects[0].site.node = 1;
    b.goods.batches[0].holder = Holder::Deposit(1);
    b.goods.add(
        Material::Food,
        100,
        Owner::None,
        Holder::Deposit(0),
        0,
        48 * HOUR,
    );
    for n in 0..2 {
        let v = b.node_view(n);
        b.agents[0].memory.nodes.insert(n, v);
    }
    b.observe_infrastructure(0);
    let renamed = consider(
        &b.observe(0).unwrap(),
        &b.agents[0].memory,
        &b.infrastructure.memories[0],
        true,
    )
    .unwrap();
    assert_eq!(original, renamed);
}
#[test]
fn unused_public_construction_pays_material_and_time_without_traffic_benefit() {
    let mut built = world(0, 8 * HOUR, 20);
    let mut idle = built.clone();
    gather(&mut built, 0, 10);
    donate(&mut built, 0, 10);
    gather(&mut built, 1, 10);
    donate(&mut built, 1, 10);
    finish(&mut built, 2);
    idle.advance_to(built.now).unwrap();
    assert!(built.crossing_projects()[0].labor_ms.values().sum::<u128>() >= 8 * HOUR as u128);
    assert_eq!(built.infrastructure_counters()["wood"], 20);
    assert_eq!(idle.infrastructure_counters()["wood"], 0);
    assert_eq!(built.infrastructure_counters()["improved_trips"], 0);
    assert_eq!(built.metrics.consumed, idle.metrics.consumed);
    built.audit().unwrap();
    idle.audit().unwrap();
}
