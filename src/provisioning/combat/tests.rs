use super::*;
fn manual() -> World {
    let mut w = research::fixture("local", true, true, 0).unwrap();
    research::prepare(&mut w, true).unwrap();
    w.config.autonomous = false;
    // Explicit controlled contact initial condition, not a movement/protection proof.
    for id in 0..9 {
        w.agents[id].location = Location::Node(2);
        w.agents[id].action = Action::Idle;
        w.agents[id].goal = None;
    }
    w.advance_to(2 * HOUR).unwrap();
    w
}
#[test]
fn strikes_take_time_and_simultaneous_hits_ignore_actor_order() {
    let mut a = manual();
    let mut b = a.clone();
    let mut hits = vec![
        (0, Intent::Strike { target: 6 }),
        (1, Intent::Strike { target: 6 }),
        (2, Intent::Strike { target: 6 }),
        (6, Intent::Strike { target: 0 }),
    ];
    a.apply_intents(hits.clone()).unwrap();
    hits.reverse();
    b.apply_intents(hits).unwrap();
    a.advance_to(2 * HOUR + 15 * MINUTE - 1).unwrap();
    assert_eq!(a.body(6).unwrap().injuries, 0);
    a.advance_to(2 * HOUR + 15 * MINUTE).unwrap();
    b.advance_to(2 * HOUR + 15 * MINUTE).unwrap();
    assert_eq!(a.body(0).unwrap().injuries, 1);
    assert_eq!(a.agents[6].action, Action::Dead);
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    assert!(
        a.goods
            .batches
            .iter()
            .any(|x| x.owner == Owner::Agent(6) && x.holder == Holder::Loose(2))
    );
    a.audit().unwrap();
}
#[test]
fn target_must_remain_local_and_contact_capable() {
    let mut w = manual();
    w.apply_intents(vec![(0, Intent::Strike { target: 6 })])
        .unwrap();
    w.apply_intents(vec![(
        6,
        Intent::Travel {
            edge: 1,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(2 * HOUR + 15 * MINUTE).unwrap();
    assert_eq!(w.body(6).unwrap().injuries, 0);
    assert_eq!(w.combat.journal.count("strikes_invalidated"), 1);
    w.apply_intents(vec![(0, Intent::Strike { target: 5 })])
        .unwrap();
    assert!(!matches!(w.agents[0].action, Action::Strike { .. }));
}
#[test]
fn theft_is_timed_conserved_age_preserving_and_capacity_limited() {
    let mut w = research::fixture("local", false, true, 0).unwrap();
    w.config.autonomous = false;
    w.advance_to(2 * HOUR).unwrap();
    w.agents[6].location = Location::Node(2);
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Commons(2))
        .unwrap()
        .id;
    let before = w.goods.batches[batch].clone();
    for _ in 0..4 {
        w.apply_intents(vec![(6, Intent::RaidStock { batch })])
            .unwrap();
        w.advance_to(w.now + 5 * MINUTE).unwrap();
    }
    assert_eq!(w.combat.journal.stolen, 4);
    assert_eq!(w.carrier_load(6), 10);
    assert_eq!(w.goods.batches[batch].quantity, before.quantity - 4);
    let stolen = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(6) && b.owner == Owner::Commons(0))
        .unwrap();
    assert_eq!(stolen.created_at, before.created_at);
    assert_eq!(stolen.expires_at(), before.expires_at());
    w.apply_intents(vec![(6, Intent::RaidStock { batch })])
        .unwrap();
    w.advance_to(w.now + HOUR).unwrap();
    assert_eq!(w.combat.journal.stolen, 4);
    w.audit().unwrap();
}
#[test]
fn defender_arrival_interrupts_unfinished_theft_without_refunding_completed_theft() {
    let mut w = manual();
    for id in 0..4 {
        w.agents[id].location = Location::Node(0);
    }
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Commons(2))
        .unwrap()
        .id;
    w.apply_intents(vec![(6, Intent::RaidStock { batch })])
        .unwrap();
    w.advance_to(w.now + 5 * MINUTE).unwrap();
    w.apply_intents(vec![(6, Intent::RaidStock { batch })])
        .unwrap();
    w.advance_to(w.now + MINUTE).unwrap();
    // Counterfactual local appearance for primitive conformance only; live fixtures travel.
    w.agents[0].location = Location::Node(2);
    w.advance_to(w.now + MINUTE).unwrap();
    assert_eq!(w.combat.journal.stolen, 1);
    assert_eq!(w.combat.journal.count("thefts_interrupted"), 1);
}
#[test]
fn own_wound_forecast_matches_actual_travel_and_incapacity_blocks_work() {
    let mut w = manual();
    w.agents[0].location = Location::Node(0);
    w.combat.bodies[0].injuries = 1;
    w.observe_combat(0);
    let o = w.observe(0).unwrap();
    let r = policy::coherent::route(&o, &w.agents[0].memory, 2).unwrap();
    let start = w.now;
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 2,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(start + r.time).unwrap();
    assert_eq!(w.agents[0].location, Location::Node(2));
    assert_eq!(w.agents[0].nutrition, r.nutrition);
    w.combat.bodies[0].injuries = 2;
    w.agents[0].action = Action::Idle;
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[0].action, Action::Idle);
    assert!(policy::coherent::route(&w.observe(0).unwrap(), &w.agents[0].memory, 0).is_none());
    w.audit().unwrap();
}
#[test]
fn service_expiry_invalidates_pending_strike() {
    let mut w = manual();
    w.apply_intents(vec![(0, Intent::Strike { target: 6 })])
        .unwrap();
    w.military.memories[0].undertaking.as_mut().unwrap().until = Some(w.now + MINUTE);
    w.advance_to(w.now + 15 * MINUTE).unwrap();
    assert_eq!(w.body(6).unwrap().injuries, 0);
    assert!(w.military_role(0).is_none());
}
#[test]
fn unobserved_remote_opponents_do_not_enter_personal_view() {
    let mut w = manual();
    w.agents[0].location = Location::Node(0);
    w.observe_combat(0);
    assert!(w.combat.views[0].enemies.is_empty());
    let before = w.combat.views[0].clone();
    w.agents[6].nutrition = 0;
    w.observe_combat(0);
    assert_eq!(before, w.combat.views[0]);
}
#[test]
fn live_raid_reaches_physical_stock_and_local_defense_prevents_theft() {
    let mut result = vec![];
    for defense in [false, true] {
        let mut w = research::fixture("local", defense, true, 0).unwrap();
        research::prepare(&mut w, defense).unwrap();
        w.advance_to(12 * HOUR).unwrap();
        w.audit().unwrap();
        result.push(w.combat.journal.stolen);
        if defense {
            assert!(w.military.journal.count("arrivals") > 0);
            assert!(w.combat.journal.count("hits") > 0);
        }
    }
    assert!(result[0] > 0);
    assert!(result[1] < result[0], "{result:?}");
}
#[test]
fn raiders_cannot_bypass_theft_with_normal_communal_withdrawal() {
    let mut w = manual();
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Commons(2))
        .unwrap()
        .id;
    w.agents[6].nutrition = 0;
    let q = w.goods.batches[batch].quantity;
    w.apply_intents(vec![
        (6, Intent::Eat { batch, quantity: 1 }),
        (7, Intent::Pickup { batch, quantity: 1 }),
    ])
    .unwrap();
    assert_eq!(w.goods.batches[batch].quantity, q);
}
#[test]
fn simultaneous_theft_of_last_quantum_cannot_duplicate_matter() {
    let mut w = research::fixture("local", false, true, 0).unwrap();
    w.config.autonomous = false;
    w.advance_to(2 * HOUR).unwrap();
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Commons(2))
        .unwrap()
        .id;
    // Explicit one-quantum fixture; preserve the rest as physically separate commons goods.
    w.goods
        .move_part(batch, 199, Owner::Commons(0), Holder::Loose(3));
    for id in [6, 7] {
        w.agents[id].location = Location::Node(2);
    }
    w.apply_intents(vec![
        (6, Intent::RaidStock { batch }),
        (7, Intent::RaidStock { batch }),
    ])
    .unwrap();
    w.advance_to(w.now + 5 * MINUTE).unwrap();
    assert_eq!(w.combat.journal.stolen, 1);
    assert_eq!(w.goods.batches[batch].quantity, 0);
    w.audit().unwrap();
}
#[test]
fn injury_slows_actual_gathering_work() {
    let mut w = manual();
    w.combat.bodies[0].injuries = 1;
    let batch = w.goods.add(
        Material::Food,
        1,
        Owner::None,
        Holder::Deposit(2),
        0,
        48 * HOUR,
    );
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch,
            communal: false,
        },
    )])
    .unwrap();
    let start = w.now;
    w.advance_to(start + 3999).unwrap();
    assert!(matches!(w.agents[0].action, Action::Gather { .. }));
    w.advance_to(start + 4000).unwrap();
    assert_eq!(w.goods.batches[batch].quantity, 0);
    w.audit().unwrap();
}
#[test]
fn partitioned_execution_preserves_contact_outcomes_and_events() {
    let mut a = research::fixture("local", true, true, 5).unwrap();
    let mut b = a.clone();
    research::prepare(&mut a, true).unwrap();
    research::prepare(&mut b, true).unwrap();
    a.advance_to(10 * HOUR).unwrap();
    for t in 66..=600 {
        b.advance_to(t * MINUTE).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn reverse_evaluation_preserves_physics_and_event_meaning() {
    let mut a = research::fixture("local", true, true, 5).unwrap();
    let mut b = a.clone();
    b.config.reverse_evaluation = true;
    research::prepare(&mut a, true).unwrap();
    research::prepare(&mut b, true).unwrap();
    a.advance_to(10 * HOUR).unwrap();
    b.advance_to(10 * HOUR).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    // Existing service/formation evaluation logs follow evaluation order.
    // Compare semantic events, excluding ordinal IDs/causal pointers into that order.
    let events = |w: &World| {
        let mut x: Vec<_> = w
            .events
            .iter()
            .map(|e| (e.at, e.agent, e.kind.clone(), e.detail.clone()))
            .collect();
        x.sort();
        x
    };
    assert_eq!(events(&a), events(&b));
}
#[test]
fn under_strength_run_does_not_enter_millisecond_polling() {
    let mut w = research::fixture("under", true, true, 0).unwrap();
    research::prepare(&mut w, true).unwrap();
    w.advance_to(19 * HOUR + 30 * MINUTE).unwrap();
    let dt = w.next_time(20 * HOUR) - w.now;
    w.advance_to(24 * HOUR).unwrap();
    w.audit().unwrap();
    assert!(dt > 1);
}
#[test]
fn raider_cannot_predict_legal_eating_from_unstolen_communal_stock() {
    let mut w = manual();
    w.agents[6].nutrition = 8 * HOUR;
    for b in &mut w.goods.batches {
        if b.holder == Holder::Agent(6) {
            b.holder = Holder::Loose(1);
        }
    }
    w.observe_combat(6);
    let o = w.observe(6).unwrap();
    let d = policy::decide(&o, &w.agents[6].memory, &mut 0);
    assert!(!matches!(d.intent, Intent::Eat { .. }));
}
#[test]
fn birth_extends_body_state_without_inheriting_combat_objectives() {
    let mut c = lifecycle::research::fixture("basic", 0).unwrap().config;
    c.autonomous = false;
    c.combat = Some(Config {
        nodes: vec![0],
        raids: vec![],
        strike_work: 15 * MINUTE,
        theft_work: 5 * MINUTE,
    });
    let mut w = World::new(c).unwrap();
    w.apply_intents(vec![
        (0, Intent::Reproduce { partner: 1 }),
        (1, Intent::Reproduce { partner: 0 }),
    ])
    .unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 3);
    assert_eq!(w.combat.bodies.len(), 3);
    assert_eq!(w.body(2).unwrap().injuries, 0);
    assert_eq!(w.combat.views[2], View::default());
    w.audit().unwrap();
}
#[test]
fn contact_death_uses_existing_household_and_known_grief_paths() {
    let mut c = research::fixture("household", true, true, 0)
        .unwrap()
        .config;
    c.social = Some(social::Config::default());
    let mut w = World::new(c).unwrap();
    w.config.autonomous = false;
    w.social.memories[9]
        .relationships
        .entry(0)
        .or_default()
        .attachment = 40;
    w.agents[0].location = Location::Node(2);
    w.agents[9].location = Location::Node(0);
    w.combat.bodies[0].injuries = 3;
    w.advance_to(1).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(w.social.memories[9].affect.grief.value(w.now), 0);
    w.agents[9].location = Location::Node(2);
    w.observe_social(9);
    assert!(w.social.memories[9].affect.grief.value(w.now) > 0);
    assert!(
        w.events
            .iter()
            .any(|e| e.kind == "Died" && e.agent == Some(0) && e.detail.contains("contact"))
    );
    w.audit().unwrap();
}
