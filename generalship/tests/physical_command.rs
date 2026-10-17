use generalship_g0::{model::*, sim::Game};

fn running() -> Game {
    let mut g = Game::new(7, Config::default());
    for side in 0..2 {
        g.input(side, Input::Connection { connected: true })
            .unwrap();
        g.input(side, Input::Ready).unwrap();
    }
    g
}
fn step(g: &mut Game, n: usize) {
    for _ in 0..n {
        g.step();
    }
}
fn separated(g: &Game) {
    let d = i64::from(g.config.person_radius * 2).pow(2);
    for a in g.people.iter().filter(|p| p.alive()) {
        for b in g.people.iter().filter(|p| p.alive() && p.id > a.id) {
            assert!(
                a.pos.distance2(b.pos) >= d,
                "tick {}: {} {:?} overlaps {} {:?}",
                g.tick,
                a.id,
                a.pos,
                b.id,
                b.pos
            );
        }
    }
}
fn relay_scene() -> Game {
    let mut g = running();
    for (id, x) in [(0, 8000), (2, 10000), (3, 12500), (4, 15000), (5, 24000)] {
        g.people[id].pos = Point::new(x, 15000);
        g.people[id].plan = Plan::hold(g.people[id].pos);
        g.knowledge[0].people.get_mut(&id).unwrap().pos = g.people[id].pos;
    }
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::General { id: 0 },
            recipients: vec![2, 3, 4, 5],
            plan: Plan::hold(Point::new(10000, 15000)),
        },
    )
    .unwrap();
    g
}
#[test]
fn formation_handoff_has_per_link_delay_and_cannot_reach_a_detached_member() {
    let mut g = relay_scene();
    step(&mut g, 3);
    assert_eq!(g.people[2].sequence, 1);
    assert_eq!(g.people[3].sequence, 0);
    step(&mut g, 3);
    assert_eq!(g.people[3].sequence, 1);
    assert_eq!(g.people[4].sequence, 0);
    step(&mut g, 3);
    assert_eq!(g.people[4].sequence, 1);
    assert_eq!(g.people[5].sequence, 0);
    for id in [2, 3, 4] {
        assert_eq!(g.people[id].accepted_group, vec![2, 3, 4, 5]);
    }
    assert_eq!(g.knowledge[0].orders[0].confirmed, vec![2, 3, 4]);
}
#[test]
fn broken_or_superseded_handoff_cannot_transmit() {
    let mut g = relay_scene();
    step(&mut g, 6);
    g.people[3].hp = 0;
    step(&mut g, 10);
    assert_eq!(g.people[4].sequence, 0);
    let mut g = relay_scene();
    step(&mut g, 6);
    g.people[3].role = Role::Trainee;
    g.people[3].training = 600;
    step(&mut g, 10);
    assert_eq!(
        g.people[4].sequence, 0,
        "new duty cancels the old relay work"
    );
    let mut g = relay_scene();
    step(&mut g, 3);
    // A later, already received instruction wins over the in-flight old handoff.
    g.people[3].sequence = 2;
    let old = g.people[3].plan.clone();
    step(&mut g, 10);
    assert_eq!(g.people[3].sequence, 2);
    assert_eq!(g.people[3].plan, old);
    assert_eq!(g.people[4].sequence, 0);
}
#[test]
fn courier_delivers_to_a_local_encounter_without_tracking_hidden_positions() {
    let mut g = running();
    g.people[2].pos = Point::new(14000, 27500);
    g.people[2].plan = Plan::hold(g.people[2].pos);
    g.knowledge[0].people.get_mut(&2).unwrap().pos = Point::new(30000, 27500);
    g.input(
        0,
        Input::Order {
            rendezvous: Some(Point::new(30000, 27500)),
            source: Source::Hq,
            recipients: vec![2],
            plan: Plan::hold(Point::new(14000, 27500)),
        },
    )
    .unwrap();
    for _ in 0..150 {
        g.step();
        if g.people[2].sequence > 0 {
            break;
        }
    }
    assert_eq!(g.people[2].sequence, 1);
    let messenger = g.missions[0].courier.unwrap();
    assert!(g.people[messenger].pos.x < 17000);
    assert!(g.missions[0].returning);
}
#[test]
fn an_unseen_receipt_is_not_instantly_reported_to_the_player() {
    let mut g = running();
    g.people[2].pos = Point::new(30000, 55000);
    g.people[2].plan = Plan::hold(g.people[2].pos);
    g.knowledge[0].people.get_mut(&2).unwrap().pos = g.people[2].pos;
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: vec![2],
            plan: Plan::hold(g.people[2].pos),
        },
    )
    .unwrap();
    for _ in 0..1200 {
        g.step();
        if g.people[2].sequence > 0 {
            break;
        }
    }
    assert_eq!(g.people[2].sequence, 1);
    assert!(g.knowledge[0].orders[0].confirmed.is_empty());
    assert!(g.knowledge[0].orders[0].status.starts_with("Dispatched"));
}
#[test]
fn people_keep_space_during_marching_shared_destinations_and_idle_support_work() {
    let mut g = running();
    separated(&g);
    for ids in [(2..12).collect(), (12..22).collect(), (22..32).collect()] {
        g.input(
            0,
            Input::Order {
                rendezvous: None,
                source: Source::Hq,
                recipients: ids,
                plan: Plan {
                    verb: Verb::Move,
                    then: Some(Verb::Hold),
                    manner: Manner::Together,
                    ..Plan::hold(Point::new(22000, 28000))
                },
            },
        )
        .unwrap();
    }
    for _ in 0..1200 {
        g.step();
        separated(&g);
    }
    assert!(
        g.people[2..32].iter().all(|p| p.pos.x > 17000),
        "Crowding must not stop the march: {:?}",
        g.people[2..32].iter().map(|p| p.pos).collect::<Vec<_>>()
    );
    g.audit().unwrap();
    assert_eq!(Game::replay_run(&g.replay()).unwrap().hash(), g.hash());
}
#[test]
fn opposite_traffic_passes_without_interpenetration_or_permanent_deadlock() {
    let mut g = running();
    g.people[0].pos = Point::new(20000, 15000);
    g.people[1].pos = Point::new(26000, 15000);
    for (id, target) in [(0, Point::new(26000, 15000)), (1, Point::new(20000, 15000))] {
        g.input(0, Input::MoveGeneral { id, target }).unwrap();
    }
    for _ in 0..400 {
        g.step();
        separated(&g);
    }
    assert!(
        g.people[0].pos.x > 25000 && g.people[1].pos.x < 21000,
        "head-on traffic stalled: {:?} {:?}",
        g.people[0].pos,
        g.people[1].pos
    );
}
