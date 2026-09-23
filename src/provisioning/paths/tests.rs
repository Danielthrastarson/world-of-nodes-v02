use super::*;
fn fixture(agents: usize, capacity: usize, enabled: bool, nominal: Time) -> World {
    World::new(super::super::Config {
        nodes: 2,
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: nominal,
            capacity,
        }],
        starts: vec![0; agents],
        food: vec![],
        boards: vec![0],
        initial_nutrition: 24 * HOUR,
        autonomous: false,
        trace: true,
        paths: Some(Config::ground(vec![0], enabled)),
        ..super::super::Config::default()
    })
    .unwrap()
}
fn travel(w: &mut World, id: usize) {
    let Location::Node(n) = w.agents[id].location else {
        panic!()
    };
    w.apply_intents(vec![(
        id,
        Intent::Travel {
            edge: 0,
            target: 1 - n,
            food: false,
        },
    )])
    .unwrap();
}
fn arrive(w: &mut World, id: usize) {
    while matches!(w.agents[id].action, Action::Travel | Action::Queued { .. }) {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
}
#[test]
fn actual_completed_passages_change_surface_not_idle_or_queued_people() {
    let mut w = fixture(3, 1, true, HOUR);
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.path_surfaces()[&0].compaction, 0);
    travel(&mut w, 0);
    travel(&mut w, 1);
    w.advance_to(w.now + HOUR / 2).unwrap();
    assert_eq!(w.path_surfaces()[&0].passages, 0);
    assert_eq!(w.path_surfaces()[&0].compaction, 0);
    arrive(&mut w, 0);
    assert_eq!(w.path_surfaces()[&0].compaction, 1);
    assert_eq!(w.path_trips()[0].nominal_ms, HOUR);
    assert_eq!(w.path_trips()[1].nominal_ms, HOUR * 499 / 500);
    arrive(&mut w, 1);
    travel(&mut w, 0);
    arrive(&mut w, 0);
    assert_eq!(w.path_surfaces()[&0].passages, 3);
    assert_eq!(w.metrics.overtakes, 0);
    assert_eq!(w.effective_edge(0).capacity, 1);
}
#[test]
fn simultaneous_arrivals_retain_admission_contract_and_saturate_at_twenty_percent() {
    let mut w = fixture(102, 101, true, HOUR);
    let intents = (0..101)
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
    w.apply_intents(intents).unwrap();
    w.advance_to(HOUR).unwrap();
    assert!(
        w.path_trips()
            .iter()
            .all(|t| t.arrived == Some(HOUR) && t.nominal_ms == HOUR)
    );
    assert_eq!(w.path_surfaces()[&0].compaction, 100);
    assert_eq!(w.path_surfaces()[&0].passages, 101);
    assert_eq!(w.effective_edge(0).nominal_ms, 4 * HOUR / 5);
    travel(&mut w, 101);
    assert_eq!(w.path_trips()[101].nominal_ms, 4 * HOUR / 5);
    w.audit().unwrap();
}
#[test]
fn known_surface_forecast_matches_kernel_across_physiology_thresholds() {
    for (n, f) in [(24, 4), (7, 11), (5, 17), (0, 20)] {
        let mut w = fixture(51, 50, true, HOUR);
        w.apply_intents(
            (0..50)
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
        w.advance_to(HOUR).unwrap();
        w.agents[50].nutrition = n * HOUR;
        w.agents[50].fatigue = f * HOUR;
        let local = w.node_view(0);
        w.agents[50].memory.nodes.insert(0, local);
        let o = w.observe(50).unwrap();
        let r = policy::coherent::route(&o, &w.agents[50].memory, 1).unwrap();
        let at = w.now;
        travel(&mut w, 50);
        arrive(&mut w, 50);
        assert_eq!(
            (w.now - at, w.agents[50].nutrition, w.agents[50].starvation),
            (r.time, r.nutrition, r.starvation)
        );
    }
}
#[test]
fn recovery_uses_the_authoritative_clock_and_movement_maintains_surface() {
    let mut w = fixture(1, 1, true, SECOND);
    travel(&mut w, 0);
    arrive(&mut w, 0);
    let first = w.now;
    w.advance_to(first + 5 * HOUR).unwrap();
    assert_eq!(w.path_surfaces()[&0].compaction, 1);
    travel(&mut w, 0);
    arrive(&mut w, 0);
    let last = w.now;
    w.advance_to(first + 6 * HOUR).unwrap();
    assert_eq!(w.path_surfaces()[&0].compaction, 2);
    w.advance_to(last + 6 * HOUR).unwrap();
    assert_eq!(w.path_surfaces()[&0].compaction, 1);
    w.advance_to(last + 12 * HOUR).unwrap();
    assert_eq!(w.path_surfaces()[&0].compaction, 0);
    assert_eq!(w.path_surfaces()[&0].next_recovery, None);
}
#[test]
fn dead_mid_edge_does_not_complete_a_passage_or_lose_cargo() {
    let mut w = fixture(1, 1, true, HOUR);
    let b = w.goods.add(
        Material::Food,
        1,
        Owner::Agent(0),
        Holder::Agent(0),
        0,
        48 * HOUR,
    );
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 24 * HOUR - SECOND;
    travel(&mut w, 0);
    w.advance_to(SECOND).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(w.path_surfaces()[&0].passages, 0);
    assert!(w.paths.active.is_empty());
    assert!(w.goods.batches.iter().any(|x| x.quantity == 1
        && matches!(x.holder, Holder::Edge { .. })
        && x.owner == Owner::Agent(0)));
    assert_eq!(w.goods.batches[b].quantity, 0);
    w.audit().unwrap();
}
#[test]
fn frozen_control_records_use_without_surface_effect_and_remote_memory_stays_stale() {
    let mut a = fixture(2, 1, true, HOUR);
    let mut b = fixture(2, 1, false, HOUR);
    let old = a.node_view(0);
    a.agents[1].memory.nodes.insert(0, old.clone());
    travel(&mut a, 0);
    travel(&mut b, 0);
    arrive(&mut a, 0);
    arrive(&mut b, 0);
    assert_eq!(a.agents[1].memory.nodes[&0], old);
    assert_eq!(b.path_surfaces()[&0].passages, 1);
    assert_eq!(b.path_surfaces()[&0].compaction, 0);
    assert_eq!(b.effective_edge(0).nominal_ms, HOUR);
    assert!(
        a.observe(1).unwrap().local.unwrap().edges[0]
            .edge
            .nominal_ms
            < HOUR
    );
}
#[test]
fn path_law_cannot_open_blocked_crossings_or_overlap_constructed_infrastructure() {
    let c = super::super::Config {
        paths: Some(Config::ground(vec![1], true)),
        infrastructure: Some(infrastructure::Config {
            enabled: true,
            wood: vec![],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 20,
                work_ms: 8 * HOUR,
                opened_nominal_ms: HOUR,
                opened_capacity: 4,
            }],
        }),
        ..super::super::Config::default()
    };
    assert!(World::new(c).is_err());
    let mut c = super::super::Config {
        paths: Some(Config::ground(vec![0], true)),
        ..super::super::Config::default()
    };
    c.edges[0].capacity = 0;
    assert!(World::new(c).is_err());
}
#[test]
fn movement_and_recovery_are_partition_independent() {
    let mut a = fixture(2, 1, true, HOUR);
    travel(&mut a, 0);
    travel(&mut a, 1);
    let mut b = a.clone();
    let end = 18 * HOUR;
    a.advance_to(end).unwrap();
    while b.now < end {
        b.advance_to((b.now + 137 * SECOND).min(end)).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn fifo_opposite_direction_survives_path_improvement() {
    let mut w = fixture(3, 1, true, HOUR);
    w.agents[2].location = Location::Node(1);
    travel(&mut w, 0);
    travel(&mut w, 2);
    w.advance_to(MINUTE).unwrap();
    travel(&mut w, 1);
    w.advance_to(4 * HOUR).unwrap();
    assert_eq!(
        w.path_trips().iter().map(|t| t.agent).collect::<Vec<_>>(),
        vec![0, 2, 1]
    );
    assert_eq!(w.metrics.overtakes, 0);
}
#[test]
fn topology_relabeling_preserves_ground_and_crossing_identity() {
    let c = super::super::Config {
        paths: Some(Config::ground(vec![0], true)),
        infrastructure: Some(infrastructure::Config {
            enabled: true,
            wood: vec![(0, 20)],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 20,
                work_ms: 8 * HOUR,
                opened_nominal_ms: HOUR,
                opened_capacity: 4,
            }],
        }),
        ..super::super::Config::default()
    };
    let renamed = geography::reversed_labels(c);
    assert_eq!(renamed.paths.as_ref().unwrap().ground_edges, vec![1]);
    assert_eq!(renamed.infrastructure.as_ref().unwrap().sites[0].edge, 0);
    assert_eq!(renamed.infrastructure.as_ref().unwrap().wood, vec![(2, 20)]);
    World::new(renamed).unwrap().audit().unwrap();
}
