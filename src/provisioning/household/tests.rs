use super::*;
fn local(w: &mut World, id: usize) -> Observation {
    w.observe_household(id);
    let o = w.observe(id).unwrap();
    if let Some(v) = &o.local {
        w.agents[id].memory.nodes.insert(v.node, v.clone());
    }
    o
}
fn choose(w: &mut World, id: usize) -> Decision {
    let o = local(w, id);
    person::decide(
        person::Context {
            observation: &o,
            beliefs: &w.agents[id].memory,
            capabilities: w.person_capabilities(id),
        },
        &mut w.agents[id].rng.clone(),
    )
}
#[test]
fn relations_are_directed_and_care_is_not_implied() {
    let w = research::fixture("colocated", true, 0).unwrap();
    let a = w.household_view(0).unwrap();
    let b = w.household_view(1).unwrap();
    assert_eq!(a.kin[0].kind, Kin::Parent);
    assert_eq!(b.kin[0].kind, Kin::Child);
    assert_eq!(a.dependents, vec![1]);
    assert!(b.dependents.is_empty());
    assert_eq!(a.households[0].id, b.households[0].id);
    let c = Config {
        kin: vec![
            Relation {
                from: 0,
                to: 1,
                kind: Kin::Partner,
            },
            Relation {
                from: 0,
                to: 2,
                kind: Kin::Sibling,
            },
        ],
        ..Config::default()
    };
    assert_eq!(c.relations(1)[0].kind, Kin::Partner);
    assert_eq!(c.relations(2)[0].kind, Kin::Sibling);
    assert!(c.care.is_empty());
}
#[test]
fn explicit_gift_preserves_age_matter_and_requires_recipient_consumption() {
    let mut w = research::fixture("colocated", true, 0).unwrap();
    let d = choose(&mut w, 0);
    assert!(matches!(d.intent, Intent::GiveCareFood { quantity: 6, .. }));
    w.apply_intents(vec![(0, d.intent)]).unwrap();
    assert_eq!(w.agents[1].nutrition, 0);
    assert_eq!(w.goods.consumed, 0);
    let b = w.goods.batches.iter().find(|b| b.quantity > 0).unwrap();
    assert_eq!(
        (b.owner, b.holder, b.quantity, b.created_at, b.spoil_after),
        (Owner::Agent(1), Holder::Loose(0), 6, 0, 48 * HOUR)
    );
    let q = b.id;
    w.apply_intents(vec![(
        1,
        Intent::Eat {
            batch: q,
            quantity: 6,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[1].nutrition, 6 * HOUR);
    assert_eq!(w.goods.consumed, 6);
    w.audit().unwrap();
}
#[test]
fn personal_gift_cannot_take_commons_or_remote_food() {
    let mut w = research::fixture("colocated", true, 0).unwrap();
    w.goods.batches[0].owner = Owner::Commons(0);
    assert!(w.give_care_food(0, 1, 0, 1).is_err());
    w.goods.batches[0].owner = Owner::Agent(0);
    w.agents[1].location = Location::Node(99);
    assert!(w.give_care_food(0, 1, 0, 1).is_err());
    assert_eq!(w.goods.batches[0].quantity, 6);
}
#[test]
fn family_members_cannot_take_one_anothers_personal_goods() {
    let mut w = research::fixture("colocated", true, 0).unwrap();
    w.apply_intents(vec![(
        1,
        Intent::Eat {
            batch: 0,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[1].nutrition, 0);
    assert_eq!(w.goods.batches[0].quantity, 6);
    assert!(w.events.iter().any(|e| e.kind == "Rejected"));
}
#[test]
fn existing_self_survival_wins_scarcity_conflict() {
    let mut w = research::fixture("self_conflict", true, 0).unwrap();
    assert!(matches!(
        choose(&mut w, 0).intent,
        Intent::Eat { quantity: 6, .. }
    ));
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.care_journal().count("food_transferred"), 0);
    assert_eq!(w.agents[0].consumed, 6);
    assert_eq!(w.agents[1].consumed, 0);
}
#[test]
fn hidden_remote_need_and_death_do_not_change_personal_choice() {
    let mut a = research::fixture("unknown", true, 5).unwrap();
    let mut b = a.clone();
    b.agents[1].nutrition = 20 * HOUR;
    b.agents[1].action = Action::Dead;
    assert_eq!(choose(&mut a, 0), choose(&mut b, 0));
    assert!(a.household.memories[0].sightings.is_empty());
    let mut a = research::fixture("near_known", true, 5).unwrap();
    let mut b = a.clone();
    b.agents[1].nutrition = 20 * HOUR;
    b.agents[1].action = Action::Dead;
    assert_eq!(choose(&mut a, 0), choose(&mut b, 0));
    assert_eq!(a.household.memories[0], b.household.memories[0]);
}
#[test]
fn known_need_creates_physical_travel_and_absence_only_updates_locally() {
    let mut w = research::fixture("near_known", true, 0).unwrap();
    let d = choose(&mut w, 0);
    assert!(matches!(d.intent, Intent::TravelForCare { target: 1, .. }));
    w.apply_intents(vec![(0, d.intent)]).unwrap();
    assert!(matches!(w.agents[0].location, Location::Edge { .. }));
    assert_eq!(w.goods.batches[0].holder, Holder::Agent(0));
    w.agents[0].location = Location::Node(1);
    w.agents[0].action = Action::Idle;
    w.agents[1].location = Location::Node(0);
    w.observe_household(0);
    let s = &w.household.memories[0].sightings[&1];
    assert!(!s.present);
    assert_eq!(s.node, 1);
}
#[test]
fn stale_need_and_infeasible_distance_can_block_help() {
    let mut w = research::fixture("near_known", true, 0).unwrap();
    let mut o = local(&mut w, 0);
    o.now = 13 * HOUR;
    assert_eq!(
        evaluate(&o, &w.agents[0].memory, &w.household.memories[0]),
        Err("STALE_NEED")
    );
    let mut w = research::fixture("far_known", true, 0).unwrap();
    w.agents[0].nutrition = 12 * HOUR;
    let o = local(&mut w, 0);
    assert_eq!(
        evaluate(&o, &w.agents[0].memory, &w.household.memories[0]),
        Err("SELF_SURVIVAL_BUFFER")
    );
}
#[test]
fn no_obligation_and_disabled_care_do_not_create_gifts() {
    for (case, enabled) in [("nonkin", true), ("colocated", false)] {
        let mut w = research::fixture(case, enabled, 0).unwrap();
        w.advance_to(HOUR).unwrap();
        assert_eq!(w.care_journal().count("food_transferred"), 0);
    }
}
#[test]
fn death_retains_family_and_personal_property_without_remote_notification() {
    let mut w = research::fixture("caregiver_death", true, 0).unwrap();
    w.goods.add(
        Material::Wood,
        1,
        Owner::Agent(0),
        Holder::Agent(0),
        0,
        Time::MAX,
    );
    w.advance_to(2 * HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(
        w.household_view(1).unwrap().households[0].members,
        vec![0, 1]
    );
    let remaining = w.goods.batches.iter().find(|b| b.quantity > 0).unwrap();
    assert_eq!(remaining.owner, Owner::Agent(0));
    assert_eq!(remaining.holder, Holder::Loose(0));
    assert_eq!(w.care_journal().losses.len(), 1);
    assert!(w.household.memories[1].sightings.is_empty());
    w.audit().unwrap();
}
#[test]
fn care_can_pause_for_sleep_and_resume_without_forcing_commitment() {
    let mut w = research::fixture("near_known", true, 0).unwrap();
    let o = local(&mut w, 0);
    let d = choose(&mut w, 0);
    w.record_care_decision(0, &o, &d);
    let sleep = Decision {
        intent: Intent::Sleep,
        reason: "existing sleep priority".into(),
    };
    w.record_care_decision(0, &o, &sleep);
    assert!(w.household.memories[0].undertaking.as_ref().unwrap().paused);
    w.record_care_decision(0, &o, &d);
    assert!(!w.household.memories[0].undertaking.as_ref().unwrap().paused);
    w.agents[1].location = Location::Node(0);
    w.agents[1].action = Action::Dead;
    let o = local(&mut w, 0);
    w.record_care_decision(0, &o, &sleep);
    assert!(w.household.memories[0].undertaking.is_none());
}
#[test]
fn no_mid_edge_gift_and_no_spoilage_refresh() {
    let mut w = research::fixture("near_known", true, 0).unwrap();
    let d = choose(&mut w, 0);
    w.apply_intents(vec![(0, d.intent)]).unwrap();
    w.apply_intents(vec![(
        0,
        Intent::GiveCareFood {
            dependent: 1,
            batch: 0,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.batches[0].quantity, 6);
    let mut w = research::fixture("colocated", true, 0).unwrap();
    w.config.autonomous = false;
    w.give_care_food(0, 1, 0, 3).unwrap();
    w.advance_to(48 * HOUR).unwrap();
    assert_eq!(w.metrics.spoiled, 6);
    w.audit().unwrap();
    assert!(
        w.goods
            .batches
            .iter()
            .filter(|b| b.quantity > 0)
            .all(|b| b.material == Material::Waste && b.created_at == 0)
    );
}
#[test]
fn identical_personal_histories_and_seed_replay_identically() {
    let mut a = research::fixture("farming_distant", true, 4).unwrap();
    let mut b = a.clone();
    a.advance_to(48 * HOUR).unwrap();
    b.advance_to(48 * HOUR).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    assert_eq!(a.care_journal(), b.care_journal());
}
#[test]
fn duplicate_membership_is_rejected_and_disabled_hash_is_unchanged() {
    let c = super::super::Config {
        household: Some(Config {
            households: vec![Household {
                id: 0,
                members: vec![0, 0],
                home: None,
            }],
            ..Config::default()
        }),
        ..super::super::Config::default()
    };
    assert!(World::new(c).is_err());
    let mut a = World::new(super::super::Config::default()).unwrap();
    let mut b = a.clone();
    b.household.journal.add("unread_observer", 100);
    a.advance_to(HOUR).unwrap();
    b.advance_to(HOUR).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn observer_journal_and_advance_partition_do_not_change_care_execution() {
    let mut a = research::fixture("near_known", true, 0).unwrap();
    let mut b = a.clone();
    b.household.journal.add("UNREAD_RESEARCH_COUNTER", 999);
    a.advance_to(12 * HOUR).unwrap();
    for t in 1..=144 {
        b.advance_to(t * 5 * MINUTE).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    assert_eq!(
        a.care_journal().care_travel_ms,
        b.care_journal().care_travel_ms
    );
}
#[test]
fn care_cancels_waiting_without_priority_admission_or_moving_recipient() {
    let mut w = research::fixture("near_known", true, 0).unwrap();
    w.config.edges[0].capacity = 0;
    w.agents[1].location = Location::Node(0);
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
    let recipient = w.agents[1].clone();
    let d = choose(&mut w, 0);
    assert!(matches!(d.intent, Intent::GiveCareFood { .. }));
    w.apply_intents(vec![(0, d.intent)]).unwrap();
    assert_eq!(w.agents[0].action, Action::Idle);
    assert_eq!(w.agents[1], recipient);
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}
#[test]
fn multiple_dependents_do_not_duplicate_food_or_optimize_away_scarcity() {
    let mut w = research::fixture("multiple", true, 0).unwrap();
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.care_journal().count("food_transferred"), 6);
    assert_eq!(w.agents[1].consumed, 6);
    assert_eq!(w.agents[2].consumed, 0);
    w.audit().unwrap();
}
