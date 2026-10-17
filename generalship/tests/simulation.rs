use generalship_g0::{model::*, sim::Game, terrain::Terrain};
fn running() -> Game {
    let mut g = Game::new(7, Config::default());
    for s in 0..2 {
        g.input(s, Input::Connection { connected: true }).unwrap();
        g.input(s, Input::Ready).unwrap();
    }
    g
}
fn advance(g: &mut Game, n: u64) {
    for _ in 0..n {
        g.step();
    }
}
fn march(p: Point) -> Plan {
    Plan {
        area_radius: 0,
        verb: Verb::Move,
        target: p,
        route: vec![],
        manner: Manner::Together,
        then: Some(Verb::Hold),
        fallback: None,
    }
}

#[test]
fn original_population_and_resources_are_explicit() {
    let g = running();
    assert_eq!(g.people.len(), 96);
    assert_eq!(
        g.people.iter().filter(|p| p.role == Role::General).count(),
        4
    );
    assert_eq!(g.initial, [[696, 80, 0]; 2]);
    g.audit().unwrap();
}
#[test]
fn enemy_state_cannot_enter_an_identical_player_view() {
    let g = running();
    let mut b = g.clone();
    b.people[60].pos = Point::new(89000, 55000);
    b.people[70].hp = 0;
    b.stock[1][0] = 5;
    assert_eq!(g.view(0), b.view(0));
}
#[test]
fn hidden_enemy_changes_do_not_change_sensing_output() {
    let mut a = running();
    let mut b = a.clone();
    b.people[60].pos = Point::new(89000, 55000);
    advance(&mut a, 5);
    advance(&mut b, 5);
    assert_eq!(a.view(0), b.view(0));
}
#[test]
fn remote_order_waits_for_physical_delivery() {
    let mut g = running();
    g.people[2].pos = Point::new(30000, 28000);
    g.people[2].plan = Plan::hold(g.people[2].pos);
    g.knowledge[0].people.get_mut(&2).unwrap().pos = g.people[2].pos;
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: vec![2],
            plan: march(Point::new(40000, 28000)),
        },
    )
    .unwrap();
    advance(&mut g, 100);
    assert_eq!(g.people[2].sequence, 0);
    assert!(
        g.people
            .iter()
            .any(|p| p.role == Role::Courier && p.task.is_some())
    );
    advance(&mut g, 3500);
    assert!(g.people[2].sequence > 0);
    assert!(g.knowledge[0].orders[0].status.contains("Acknowledged"));
}
#[test]
fn delayed_older_order_does_not_overwrite_newer() {
    let mut g = running();
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: vec![2],
            plan: march(Point::new(22000, 28000)),
        },
    )
    .unwrap();
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: vec![2],
            plan: Plan::hold(Point::new(8000, 28000)),
        },
    )
    .unwrap();
    g.missions[0].due = 100;
    advance(&mut g, 110);
    assert_eq!(g.people[2].sequence, 2);
    assert_eq!(g.people[2].plan.verb, Verb::Hold);
}
#[test]
fn unseen_courier_loss_does_not_confirm_failure() {
    let mut g = running();
    g.people[2].pos = Point::new(30000, 28000);
    g.knowledge[0].people.get_mut(&2).unwrap().pos = g.people[2].pos;
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: vec![2],
            plan: march(Point::new(36000, 28000)),
        },
    )
    .unwrap();
    advance(&mut g, 5);
    let c = g.missions[0].courier.unwrap();
    g.people[c].pos = Point::new(40000, 55000);
    g.people[c].hp = 0;
    advance(&mut g, 200);
    assert!(g.knowledge[0].orders[0].status.starts_with("Dispatched"));
    assert!(g.knowledge[0].people[&c].hp > 0);
}
#[test]
fn late_report_preserves_fresher_memory() {
    let mut g = running();
    let mut report = g.knowledge[0]
        .reports
        .iter()
        .find(|r| r.people.iter().any(|p| p.id == 0))
        .unwrap()
        .clone();
    report.source = "old courier".into();
    report.people.retain(|p| p.id == 0);
    report.people[0].pos = Point::new(1000, 1000);
    advance(&mut g, 10);
    let fresh = g.knowledge[0].people[&0].pos;
    g.receive(0, report);
    assert_eq!(g.knowledge[0].people[&0].pos, fresh);
}
#[test]
fn signal_requires_two_staff_and_connected_sightline() {
    let mut g = running();
    assert!(g.relay(0).contains_key(&0));
    g.people[42].role = Role::Carrier;
    assert!(!g.relay(0).contains_key(&0));
    g.people[42].role = Role::Staff;
    g.posts[0].pos = Point::new(44000, 56000);
    g.people[42].pos = g.posts[0].pos;
    g.people[43].pos = g.posts[0].pos;
    assert!(!g.relay(0).contains_key(&0));
}
#[test]
fn upgraded_relay_changes_transmission_not_sight() {
    let mut g = running();
    let range = g.config.outpost_sight;
    assert_eq!(g.relay(0)[&0].0, 10);
    g.posts[0].upgraded = true;
    assert_eq!(g.relay(0)[&0].0, 3);
    assert_eq!(g.config.outpost_sight, range);
    g.posts[0].pos = Point::new(40000, 20000);
    g.people[42].pos = g.posts[0].pos;
    g.people[43].pos = g.posts[0].pos;
    assert!(g.relay(0).contains_key(&0));
    g.posts[0].upgraded = false;
    assert!(!g.relay(0).contains_key(&0));
}

#[test]
fn connected_outpost_streams_each_tick_with_unchanged_link_delay() {
    let mut g = running();
    let delay = g.relay(0)[&0].0;
    assert_eq!(delay, 10);
    advance(&mut g, 20);
    for _ in 0..15 {
        g.step();
        let report = g.knowledge[0]
            .reports
            .iter()
            .find(|r| r.source == "outpost:0")
            .unwrap();
        assert_eq!(report.received, g.tick);
        assert_eq!(report.observed + delay, g.tick);
    }
    // Loading an older scenario retains its former sampling cadence.
    let legacy: Config = serde_json::from_str(include_str!("../scenarios/g0-v2.json")).unwrap();
    assert_eq!(legacy.relay_report_ticks, 10);
}
#[test]
fn reassignment_waits_for_receipt_and_preserves_people() {
    let mut g = running();
    g.input(
        0,
        Input::Assign {
            source: Source::Hq,
            people: vec![32],
            role: Role::Courier,
            target: Point::new(8000, 28000),
            post: None,
            escort: None,
        },
    )
    .unwrap();
    assert_eq!(g.people[32].role, Role::Carrier);
    advance(&mut g, 4);
    assert_eq!(g.people[32].role, Role::Courier);
    g.audit().unwrap();
    assert_eq!(g.people.len(), 96);
}
#[test]
fn carrier_meals_and_transfers_conserve_every_unit() {
    let mut g = running();
    for id in [40, 41, 42, 43] {
        g.people[id].role = Role::Guard;
        g.people[id].pos = g.terrain.headquarters[0];
        g.people[id].plan = Plan::hold(g.people[id].pos);
    }
    for p in g
        .people
        .iter_mut()
        .filter(|p| p.side == 0 && matches!(p.role, Role::Line | Role::Ranged | Role::Light))
    {
        p.pos = Point::new(30000, 28000);
        p.plan = Plan::hold(p.pos);
    }
    // G0.2 requires an explicit supply route instead of automatically sending
    // all carriers to the front before the commander has assigned them.
    g.input(
        0,
        Input::Assign {
            source: Source::Hq,
            people: (32..36).collect(),
            role: Role::Carrier,
            target: Point::new(30000, 28000),
            post: None,
            escort: None,
        },
    )
    .unwrap();
    advance(&mut g, 3600);
    g.audit().unwrap();
    assert_eq!(g.consumed[0][0], 144);
    assert!(g.stock[0][0] < 590);
    assert!(g.people[2].cargo[0] > 0);
}
#[test]
fn general_replacement_reuses_a_person_and_food() {
    let mut g = running();
    g.people[0].hp = 0;
    let before = g.stock[0][0];
    g.input(0, Input::Train { person: 46 }).unwrap();
    assert_eq!(g.people[46].role, Role::Trainee);
    assert_eq!(g.stock[0][0], before - 10);
    advance(&mut g, 600);
    assert_eq!(g.people[46].role, Role::General);
    assert_eq!(g.people.len(), 96);
    g.audit().unwrap();
}
#[test]
fn general_limit_rejects_third_general() {
    let mut g = running();
    assert!(g.input(0, Input::Train { person: 46 }).is_err());
}
#[test]
fn construction_requires_delivered_material_and_local_work() {
    let mut g = running();
    g.input(
        0,
        Input::Build {
            site: 1,
            people: vec![46, 47],
        },
    )
    .unwrap();
    let id = 1;
    assert!(!g.posts[id].complete);
    assert_eq!(g.posts[id].stock[1], 0);
    advance(&mut g, 20);
    assert_eq!(g.posts[id].work, 0);
    advance(&mut g, 5000);
    assert!(g.posts[id].complete);
    assert_eq!(g.consumed[0][1], 20);
    g.audit().unwrap();
}
#[test]
fn zero_supply_never_creates_food() {
    let c = Config {
        hq_food: 0,
        starting_food: 0,
        ..Config::default()
    };
    let mut g = Game::new(0, c);
    g.status = Status::Running;
    g.connected = [true; 2];
    advance(&mut g, 2400);
    assert!(g.people.iter().all(|p| p.cargo[0] == 0));
    assert!(g.people[0].missed >= 2);
    g.audit().unwrap();
}
#[test]
fn seed_inputs_and_replay_reproduce_hash() {
    let mut g = running();
    g.input(
        0,
        Input::MoveGeneral {
            id: 0,
            target: Point::new(26000, 20000),
        },
    )
    .unwrap();
    g.input(
        1,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: (50..60).collect(),
            plan: march(Point::new(57000, 28000)),
        },
    )
    .unwrap();
    advance(&mut g, 800);
    let r = g.replay();
    let replay = Game::replay_run(&r).unwrap();
    assert_eq!(replay.hash(), r.hash);
}
#[test]
fn arbitrary_scripts_and_contradictory_plans_are_rejected() {
    let mut g = running();
    let mut p = march(Point::new(20000, 28000));
    p.fallback = Some(Point::new(8000, 28000));
    assert!(!p.valid());
    p.then = None;
    p.verb = Verb::Engage;
    p.manner = Manner::Avoid;
    assert!(!p.valid());
    assert!(
        g.input(
            0,
            Input::Order {
                rendezvous: None,
                source: Source::Hq,
                recipients: vec![50],
                plan: p
            }
        )
        .is_err()
    );
}
#[test]
fn ownership_and_unwalkable_destinations_are_rejected() {
    let mut g = running();
    assert!(
        g.input(
            0,
            Input::MoveGeneral {
                id: 48,
                target: Point::new(1000, 1000)
            }
        )
        .is_err()
    );
    assert!(
        g.input(
            0,
            Input::MoveGeneral {
                id: 0,
                target: Point::new(45000, 10000)
            }
        )
        .is_err()
    );
}
#[test]
fn capture_requires_five_and_uninterrupted_thirty_seconds() {
    let mut g = running();
    let hq = g.terrain.headquarters[1];
    for p in g.people.iter_mut().filter(|p| p.side == 1) {
        p.pos = Point::new(88000, 58000);
        p.plan = Plan::hold(p.pos);
        p.role = Role::Guard;
    }
    for id in 2..7 {
        g.people[id].pos = hq;
        g.people[id].plan = Plan::hold(hq);
    }
    advance(&mut g, 299);
    assert_eq!(g.status, Status::Running);
    advance(&mut g, 1);
    assert!(matches!(
        g.status,
        Status::Finished {
            winner: Some(0),
            ..
        }
    ));
}
#[test]
fn disconnect_grace_and_reconnect_keep_standing_orders() {
    let mut g = running();
    g.input(0, Input::Connection { connected: false }).unwrap();
    advance(&mut g, 599);
    assert_eq!(g.status, Status::Running);
    g.input(0, Input::Connection { connected: true }).unwrap();
    advance(&mut g, 2);
    assert_eq!(g.status, Status::Running);
    g.input(0, Input::Connection { connected: false }).unwrap();
    advance(&mut g, 600);
    assert!(matches!(
        g.status,
        Status::Finished {
            winner: Some(1),
            ..
        }
    ));
}
#[test]
fn terrain_path_uses_real_crossing_and_has_deterministic_ties() {
    let t = Terrain::new(0);
    let a = Point::new(10000, 28000);
    let b = Point::new(80000, 28000);
    let p = t.path(a, b);
    assert!(!p.is_empty());
    assert_eq!(p, t.path(a, b));
    assert!(p.iter().all(|p| t.walkable(*p)));
    assert!(p.iter().any(|p| t.kind(*p) == 4));
}
#[test]
fn twenty_minute_headless_match_finishes_without_balance_errors() {
    let mut g = running();
    let start = std::time::Instant::now();
    advance(&mut g, 12000);
    g.audit().unwrap();
    assert_eq!(g.tick, 12000);
    assert!(matches!(g.status, Status::Finished { winner: None, .. }));
    println!(
        "20-minute simulation: {:?}; payload {} bytes",
        start.elapsed(),
        serde_json::to_vec(&g.view(0)).unwrap().len()
    );
}

#[test]
fn a_shared_connection_loss_keeps_the_reconnect_grace() {
    let mut g = running();
    for side in 0..2 {
        g.input(side, Input::Connection { connected: false })
            .unwrap();
    }
    advance(&mut g, 599);
    assert_eq!(g.status, Status::Running);
    advance(&mut g, 1);
    assert!(matches!(g.status, Status::Finished { winner: None, .. }));
}
#[test]
fn a_broken_relay_drops_in_flight_reports_without_death_notification() {
    let mut g = running();
    advance(&mut g, 21);
    let before = g.knowledge[0]
        .reports
        .iter()
        .find(|r| r.source == "outpost:0")
        .unwrap()
        .observed;
    g.people[42].hp = 0;
    advance(&mut g, 40);
    let after = g.knowledge[0]
        .reports
        .iter()
        .find(|r| r.source == "outpost:0")
        .unwrap()
        .observed;
    assert_eq!(before, after);
    assert!(g.knowledge[0].people[&42].hp > 0);
}
#[test]
fn a_courier_waits_at_the_known_position_then_returns_failure() {
    let mut g = running();
    g.knowledge[0].people.get_mut(&2).unwrap().pos = Point::new(29000, 28000);
    g.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: vec![2],
            plan: march(Point::new(38000, 28000)),
        },
    )
    .unwrap();
    let mut waiting = None;
    for _ in 0..1000 {
        g.step();
        if let Some(t) = g.missions[0].wait_since {
            waiting = Some(t);
            break;
        }
    }
    let waiting = waiting.expect("courier reaches remembered rendezvous");
    advance(&mut g, 149);
    assert!(!g.missions[0].returning);
    assert_eq!(g.people[2].sequence, 0);
    advance(&mut g, 1);
    assert!(g.missions[0].returning);
    assert_eq!(g.tick - waiting, 150);
    advance(&mut g, 1500);
    assert!(g.knowledge[0].orders[0].status.contains("undelivered"));
    assert_eq!(g.people[2].sequence, 0);
}
#[test]
fn mutual_lethal_attacks_resolve_together_and_keep_dropped_cargo() {
    let mut g = running();
    for (id, pos) in [
        (2, Point::new(41000, 27500)),
        (50, Point::new(42000, 27500)),
    ] {
        g.people[id].pos = pos;
        g.people[id].plan = Plan::hold(pos);
        g.people[id].hp = 8;
    }
    let cargo = [g.people[2].cargo, g.people[50].cargo];
    g.step();
    assert_eq!((g.people[2].hp, g.people[50].hp), (0, 0));
    assert_eq!([g.people[2].cargo, g.people[50].cargo], cargo);
    g.audit().unwrap();
}
#[test]
fn shared_bridge_capacity_never_admits_a_fifth_person() {
    let mut g = running();
    for id in 2..12 {
        // A physical queue, rather than ten invalid overlapping spawn points.
        let slot = (id - 2) as i32;
        g.people[id].pos = Point::new(43500 - (slot / 3) * 800, 26600 + (slot % 3) * 800);
        g.people[id].plan = march(Point::new(48000, 27500));
    }
    for _ in 0..400 {
        g.step();
        assert!(
            g.people
                .iter()
                .filter(|p| p.alive() && g.terrain.kind(p.pos) == 4)
                .count()
                <= 4
        );
    }
    assert!(g.people[2..12].iter().all(|p| p.pos.x >= 46000));
}
#[test]
fn hidden_blueprints_do_not_appear_until_material_is_physically_present() {
    let mut a = running();
    let mut b = a.clone();
    b.input(
        1,
        Input::Build {
            site: 1,
            people: vec![94, 95],
        },
    )
    .unwrap();
    advance(&mut a, 20);
    advance(&mut b, 20);
    assert_eq!(a.view(0), b.view(0));
}
#[test]
fn fallback_uses_confirmed_local_casualties_and_is_bounded() {
    let mut g = running();
    let p = Point::new(35000, 27500);
    for id in 2..12 {
        g.people[id].pos = p;
        g.people[id].plan = Plan::hold(p);
    }
    g.people[2].plan.fallback = Some(Point::new(8000, 28000));
    g.people[2].accepted_group = (2..12).collect();
    g.people[2].accepted_strength = 10;
    for id in 7..12 {
        g.people[id].hp = 0;
        g.people[id].pos = Point::new(80000, 55000);
    }
    g.step();
    assert_eq!(g.people[2].plan.verb, Verb::Hold);
    for id in 7..12 {
        g.people[id].pos = p;
    }
    g.step();
    assert_eq!(g.people[2].plan.verb, Verb::Withdraw);
    assert!(g.people[2].plan.fallback.is_none());
}
#[test]
fn initial_scouts_deliver_meaningful_contact_within_one_minute() {
    let mut g = running();
    for _ in 0..600 {
        g.step();
        if g.knowledge[0]
            .reports
            .iter()
            .any(|r| r.people.iter().any(|p| p.side == 1))
        {
            return;
        }
    }
    panic!("no enemy observation received within first minute");
}
#[test]
fn changing_a_recorded_checkpoint_is_detected() {
    let mut g = running();
    advance(&mut g, 200);
    let mut r = g.replay();
    r.checkpoints[0].1 = "tampered".into();
    assert!(Game::replay_run(&r).unwrap_err().contains("checkpoint"));
}

#[test]
fn assigned_couriers_physically_move_to_the_dispatch_position() {
    let mut g = running();
    let start = g.people[36].pos;
    let target = Point::new(20000, 28000);
    g.input(
        0,
        Input::Assign {
            source: Source::Hq,
            people: vec![36],
            role: Role::Courier,
            target,
            post: None,
            escort: None,
        },
    )
    .unwrap();
    advance(&mut g, 400);
    assert!(g.people[36].pos.near(target, 100));
    assert_ne!(g.people[36].pos, start);
    g.audit().unwrap();
}
#[test]
fn observation_uses_an_available_local_courier_to_bring_reports_home() {
    let mut g = running();
    let p = Point::new(39000, 50000);
    for id in [2, 36] {
        g.people[id].pos = p;
        g.people[id].plan = Plan::hold(p);
        g.people[id].duty_target = p;
    }
    g.people[2].plan.verb = Verb::Observe;
    g.people[50].pos = Point::new(46000, 50000);
    g.people[50].plan = Plan::hold(g.people[50].pos);
    advance(&mut g, 301);
    assert!(
        g.missions
            .iter()
            .any(|m| m.courier == Some(36) && matches!(m.payload, Payload::Reports { .. }))
    );
    advance(&mut g, 1500);
    assert!(g.knowledge[0].people.contains_key(&50));
    g.audit().unwrap();
}

#[test]
fn reconnect_starts_a_lobby_when_both_players_were_already_ready() {
    let mut g = Game::new(0, Config::default());
    g.input(0, Input::Connection { connected: true }).unwrap();
    g.input(0, Input::Ready).unwrap();
    g.input(0, Input::Connection { connected: false }).unwrap();
    g.input(1, Input::Connection { connected: true }).unwrap();
    g.input(1, Input::Ready).unwrap();
    assert_eq!(g.status, Status::Lobby);
    g.input(0, Input::Connection { connected: true }).unwrap();
    assert_eq!(g.status, Status::Running);
}
#[test]
fn specified_rendezvous_is_fixed_even_when_recipient_memory_moves() {
    let mut g = running();
    let point = Point::new(32000, 28000);
    g.input(
        0,
        Input::Order {
            rendezvous: Some(point),
            source: Source::Hq,
            recipients: vec![2],
            plan: march(Point::new(38000, 28000)),
        },
    )
    .unwrap();
    g.knowledge[0].people.get_mut(&2).unwrap().pos = Point::new(8000, 28000);
    advance(&mut g, 10);
    assert_eq!(g.missions[0].destination, point);
}

#[test]
fn capture_interruption_resets_progress_and_simultaneous_capture_draws() {
    let mut g = running();
    for p in &mut g.people {
        p.role = Role::Scout;
        p.pos = Point::new(if p.side == 0 { 2000 } else { 88000 }, 55000);
        p.plan = Plan::hold(p.pos);
    }
    for id in 2..7 {
        g.people[id].role = Role::Line;
        g.people[id].pos = g.terrain.headquarters[1];
        g.people[id].plan = Plan::hold(g.people[id].pos);
    }
    advance(&mut g, 150);
    assert_eq!(g.capture[0], 150);
    g.people[50].role = Role::Line;
    g.people[50].pos = g.terrain.headquarters[1];
    g.people[50].plan = Plan::hold(g.people[50].pos);
    g.step();
    assert_eq!(g.capture[0], 0);
    for id in 50..55 {
        g.people[id].role = Role::Line;
        g.people[id].pos = g.terrain.headquarters[0];
        g.people[id].plan = Plan::hold(g.people[id].pos);
    }
    advance(&mut g, 299);
    assert_eq!(g.status, Status::Running);
    g.step();
    assert!(
        matches!(g.status,Status::Finished{winner:None,ref reason} if reason=="simultaneous capture")
    );
}
#[test]
fn relay_upgrade_is_installed_by_physical_deliveries_and_local_labor() {
    let mut g = running();
    advance(&mut g, 11);
    g.input(
        0,
        Input::Upgrade {
            post: 0,
            people: vec![46, 47],
        },
    )
    .unwrap();
    advance(&mut g, 20);
    assert!(!g.posts[0].upgraded);
    assert_eq!(g.posts[0].stock[1], 0);
    advance(&mut g, 3000);
    assert!(g.posts[0].upgraded);
    assert_eq!(g.consumed[0][1], 10);
    g.audit().unwrap();
}

#[test]
fn reported_support_and_generals_do_not_accept_silent_noop_orders() {
    let mut game = running();
    for id in [0, 32, 36, 40, 42] {
        let result = game.input(
            0,
            Input::Order {
                rendezvous: None,
                source: Source::Hq,
                recipients: vec![id],
                plan: march(Point::new(20000, 28000)),
            },
        );
        assert!(result.is_err());
    }
    assert!(game.knowledge[0].orders.is_empty());
}

#[test]
fn a_delivered_formation_arrives_as_distinct_people_and_replays() {
    let mut game = running();
    let target = Point::new(20000, 28000);
    game.input(
        0,
        Input::Order {
            rendezvous: None,
            source: Source::Hq,
            recipients: (2..12).collect(),
            plan: march(target),
        },
    )
    .unwrap();
    advance(&mut game, 700);
    let arrived: Vec<_> = game.people[2..12].iter().map(|p| p.pos).collect();
    assert!(
        arrived.iter().all(|p| p.near(target, 2500)),
        "formation failed to arrive: {arrived:?}"
    );
    for i in 0..arrived.len() {
        for j in i + 1..arrived.len() {
            assert!(
                !arrived[i].near(arrived[j], 300),
                "two people collapsed into one point"
            );
        }
    }
    game.audit().unwrap();
    assert_eq!(
        Game::replay_run(&game.replay()).unwrap().hash(),
        game.hash()
    );
}

#[test]
fn carriers_wait_at_headquarters_until_a_physical_route_arrives() {
    let mut game = running();
    advance(&mut game, 600);
    for p in &game.people[32..36] {
        assert!(
            p.pos
                .near(game.terrain.headquarters[0], game.config.logistics_radius)
        );
        assert_eq!(p.cargo[0], 2);
        assert_eq!(p.sequence, 0);
    }
    assert_eq!(game.stock[0][0], 600);
    game.input(
        0,
        Input::Assign {
            source: Source::Hq,
            people: vec![32],
            role: Role::Carrier,
            target: Point::new(20000, 28000),
            post: None,
            escort: None,
        },
    )
    .unwrap();
    assert_eq!(game.people[32].duty_target, game.terrain.headquarters[0]);
    advance(&mut game, 200);
    assert_eq!(game.people[32].duty_target, Point::new(20000, 28000));
    assert!(game.people[32].pos.x > 12000);
    game.audit().unwrap();
}
