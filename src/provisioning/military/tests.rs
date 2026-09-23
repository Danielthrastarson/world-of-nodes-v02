use super::*;
fn manual() -> World {
    let mut w = research::fixture("large", true, 0).unwrap();
    w.config.autonomous = false;
    w.observe_military(0);
    let o = w.observe(0).unwrap();
    let v = o.local.unwrap();
    w.agents[0].memory.nodes.insert(v.node, v);
    w
}
#[test]
fn request_is_local_and_child_cannot_enlist() {
    let mut w = research::fixture("unknown", true, 0).unwrap();
    w.advance_to(12 * HOUR).unwrap();
    assert_eq!(w.military.journal.count("volunteers"), 0);
    let mut w = research::fixture("caregiver", true, 0).unwrap();
    w.config.autonomous = false;
    w.observe_military(4);
    w.apply_intents(vec![(4, Intent::Volunteer { request: 0 })])
        .unwrap();
    assert!(w.military.memories[4].undertaking.is_none());
}
#[test]
fn arrival_is_physical_role_preserves_person_and_property() {
    let mut w = manual();
    let before = w.agents[0].clone();
    let bio = w.lifecycle.people[0].clone();
    let goods = w.goods.batches.clone();
    w.apply_intents(vec![(0, Intent::Volunteer { request: 0 })])
        .unwrap();
    assert_eq!(w.military_role(0), None);
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    assert_eq!(w.military_role(0), None);
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(HOUR).unwrap();
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    assert_eq!(w.military_role(0), Some(0));
    assert_eq!(w.agents[0].id, before.id);
    assert_eq!(w.lifecycle.people[0], bio);
    assert_eq!(w.goods.batches, goods);
    assert_eq!(w.agents[0].nutrition, before.nutrition - HOUR);
    assert!(
        w.person_state(0)
            .unwrap()
            .roles
            .contains(&person::Role::Soldier { unit: 0 })
    );
    w.advance_to(3 * HOUR).unwrap();
    assert_eq!(w.military_role(0), None);
    assert_eq!(w.military.journal.duty_ms, 2 * HOUR as u128);
    assert_eq!(w.military.journal.count("TERM_COMPLETED"), 1);
    w.audit().unwrap();
}
#[test]
fn danger_declines_and_death_ends_role_without_inheritance() {
    let mut w = research::fixture("food_constrained", true, 0).unwrap();
    w.advance_to(12 * HOUR).unwrap();
    assert_eq!(w.military.journal.count("volunteers"), 0);
    let mut w = manual();
    w.config.military.as_mut().unwrap().requests[0].muster = 0;
    w.observe_military(0);
    w.apply_intents(vec![(0, Intent::Volunteer { request: 0 })])
        .unwrap();
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 24 * HOUR;
    w.advance_to(1).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(w.military_role(0), None);
    assert!(
        w.goods
            .batches
            .iter()
            .any(|b| b.quantity > 0 && b.owner == Owner::Agent(0) && b.holder == Holder::Loose(0))
    );
    w.audit().unwrap();
}
#[test]
fn voluntary_withdrawal_and_care_priority_keep_relations() {
    let mut w = manual();
    w.apply_intents(vec![(0, Intent::Volunteer { request: 0 })])
        .unwrap();
    w.apply_intents(vec![(0, Intent::LeaveService)]).unwrap();
    assert!(w.military.memories[0].undertaking.is_none());
    let mut w = research::fixture("caregiver", true, 0).unwrap();
    let care = w.config.household.clone();
    w.advance_to(HOUR).unwrap();
    assert!(w.household.journal.count("food_transferred") > 0);
    assert_eq!(w.config.household, care);
}
#[test]
fn unchanged_personal_history_gives_same_military_choice_and_no_hidden_truth() {
    let mut w = manual();
    let d = social::research::decision(&w, 0);
    // Remote stock/request occupancy cannot enter a copied personal history.
    w.agents[1].location = Location::Node(2);
    w.agents[1].nutrition = 0;
    w.goods.add(
        Material::Food,
        500,
        Owner::None,
        Holder::Deposit(2),
        0,
        48 * HOUR,
    );
    assert_eq!(social::research::decision(&w, 0), d);
}
#[test]
fn active_service_eating_uses_real_goods_and_same_ration() {
    let mut w = manual();
    w.config.military.as_mut().unwrap().requests[0].muster = 0;
    w.observe_military(0);
    w.apply_intents(vec![(0, Intent::Volunteer { request: 0 })])
        .unwrap();
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(0))
        .unwrap()
        .id;
    let q = w.goods.batches[b].quantity;
    w.apply_intents(vec![(
        0,
        Intent::Eat {
            batch: b,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.batches[b].quantity, q - 1);
    assert_eq!(w.military.journal.food, 1);
    assert_eq!(w.military_role(0), Some(0));
    assert_eq!(w.agents[0].action, Action::Idle);
    w.apply_intents(vec![(0, Intent::MilitaryDuty)]).unwrap();
    assert!(matches!(w.agents[0].action, Action::MilitaryDuty { .. }));
}
#[test]
fn service_has_no_cart_access_right_and_fifo_not_priority() {
    let mut w = manual();
    w.config.edges[0].capacity = 1;
    w.apply_intents(vec![(
        1,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.apply_intents(vec![(0, Intent::Volunteer { request: 0 })])
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
    assert!(matches!(w.agents[0].action, Action::Queued { .. }));
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(1));
    assert!(matches!(w.agents[0].location, Location::Edge { .. }));
    assert_eq!(w.metrics.overtakes, 0);
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.owner == Owner::Agent(1))
        .unwrap()
        .id;
    w.advance_to(2 * HOUR).unwrap();
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    let q = w.goods.batches[b].quantity;
    w.apply_intents(vec![(
        0,
        Intent::Eat {
            batch: b,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.batches[b].quantity, q);
}
#[test]
fn births_extend_person_substrate_without_automatic_military_knowledge() {
    let mut w = lifecycle::research::fixture("basic", 0).unwrap();
    w.config.military = Some(Config::default());
    w.military = State::new(&w.config).unwrap();
    w.advance_to(10 * HOUR).unwrap();
    assert!(w.agents.len() > 2);
    assert_eq!(w.military.memories.len(), w.agents.len());
    assert_eq!(w.military.memories[2], Memory::default());
    w.audit().unwrap();
}

#[test]
fn term_expiry_does_not_cancel_sleep() {
    let mut w = manual();
    w.config.military.as_mut().unwrap().requests[0].muster = 0;
    w.observe_military(0);
    w.apply_intents(vec![(0, Intent::Volunteer { request: 0 })])
        .unwrap();
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    w.agents[0].fatigue = 22 * HOUR;
    w.apply_intents(vec![(0, Intent::Sleep)]).unwrap();
    // Existing sleep has atomic hourly ends. Research holds one ongoing recovery action across term expiry.
    w.agents[0].action = Action::Sleep { until: 4 * HOUR };
    w.advance_to(2 * HOUR).unwrap();
    assert_eq!(w.military_role(0), None);
    assert!(matches!(w.agents[0].action, Action::Sleep { .. }));
    assert_eq!(w.agents[0].fatigue, 18 * HOUR);
    w.audit().unwrap();
}
#[test]
fn owner_cart_travels_with_volunteer_but_service_cannot_requisition() {
    let mut c = research::fixture("large", true, 0).unwrap().config;
    c.autonomous = false;
    c.capital = Some(capital::Config {
        rules: Default::default(),
        wood: vec![],
    });
    c.initial_nutrition = 24 * HOUR;
    let mut w = World::new(c).unwrap();
    w.goods.add(
        Material::Wood,
        10,
        Owner::Agent(0),
        Holder::Agent(0),
        0,
        Time::MAX / 4,
    );
    w.apply_intents(vec![(0, Intent::BuildCart)]).unwrap();
    while matches!(w.agents[0].action, Action::BuildCart { .. }) {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
    assert_eq!(w.carts().len(), 1);
    w.apply_intents(vec![(0, Intent::HitchCart { cart: 0 })])
        .unwrap();
    for id in [0, 1] {
        w.observe_military(id);
        let v = w.observe(id).unwrap().local.unwrap();
        w.agents[id].memory.nodes.insert(v.node, v);
    }
    w.apply_intents(vec![
        (0, Intent::Volunteer { request: 0 }),
        (1, Intent::Volunteer { request: 0 }),
    ])
    .unwrap();
    w.apply_intents(vec![(1, Intent::HitchCart { cart: 0 })])
        .unwrap();
    assert_eq!(w.carts()[0].user, Some(0));
    assert_eq!(w.carts()[0].owner, Owner::Agent(0));
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    while w.agents[0].action == Action::Travel {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
    w.apply_intents(vec![(0, Intent::JoinMuster)]).unwrap();
    assert_eq!(w.military_role(0), Some(0));
    assert_eq!(w.carts()[0].location, Location::Node(1));
    assert!(w.military.journal.cart_travel_ms > 0);
    w.audit().unwrap();
}

#[test]
fn working_farmer_fixture_really_interrupts_existing_cultivation() {
    for seed in 0..30 {
        let mut w = research::fixture("working_farmer", true, seed).unwrap();
        w.advance_to(4 * HOUR).unwrap();
        assert_eq!(w.military.journal.count("volunteers"), 4);
        assert_eq!(
            w.events
                .iter()
                .filter(|e| e.kind == "Volunteered" && e.detail.contains("previous=Farm"))
                .count(),
            4
        );
        assert_eq!(w.farm.started, 4);
        assert_eq!(w.farm.interruptions, 4);
        w.audit().unwrap();
    }
}
