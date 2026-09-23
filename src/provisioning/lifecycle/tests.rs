use super::*;
fn manual() -> World {
    let mut w = research::fixture("basic", 0).unwrap();
    w.config.autonomous = false;
    w
}
fn conceive(w: &mut World) {
    w.apply_intents(vec![
        (0, Intent::Reproduce { partner: 1 }),
        (1, Intent::Reproduce { partner: 0 }),
    ])
    .unwrap();
    w.advance_to(HOUR).unwrap();
    assert!(w.biology(0).unwrap().pregnancy.is_some());
}
#[test]
fn bilateral_time_and_birth_are_real() {
    let mut w = manual();
    w.apply_intents(vec![(0, Intent::Reproduce { partner: 1 })])
        .unwrap();
    assert!(w.biology(0).unwrap().undertaking.is_none());
    conceive(&mut w);
    assert_eq!(w.agents.len(), 2);
    w.advance_to(9 * HOUR - 1).unwrap();
    assert_eq!(w.agents.len(), 2);
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 3);
    assert_eq!(w.biology(2).unwrap().parents, Some([0, 1]));
    assert_eq!(w.agents[2].nutrition, 0);
    assert_eq!(w.household_view(2).unwrap().kin.len(), 2);
    assert_eq!(w.household_view(0).unwrap().dependents, vec![2]);
    w.audit().unwrap();
}
#[test]
fn consent_does_not_survive_a_separate_command_batch() {
    let mut w = manual();
    w.apply_intents(vec![(0, Intent::Reproduce { partner: 1 })])
        .unwrap();
    w.apply_intents(vec![(1, Intent::Reproduce { partner: 0 })])
        .unwrap();
    assert_eq!(w.lifecycle_journal().count("undertakings_started"), 0);
}
#[test]
fn no_colocation_no_consent_and_zero_fertility() {
    for case in ["separated", "decline", "hungry", "infertile"] {
        let mut w = research::fixture(case, 0).unwrap();
        w.advance_to(12 * HOUR).unwrap();
        assert_eq!(w.lifecycle_journal().count("births"), 0, "{case}");
    }
}
#[test]
fn interruption_releases_both_people() {
    let mut w = manual();
    w.apply_intents(vec![
        (0, Intent::Reproduce { partner: 1 }),
        (1, Intent::Reproduce { partner: 0 }),
    ])
    .unwrap();
    w.apply_intents(vec![(1, Intent::Sleep)]).unwrap();
    assert!(w.biology(0).unwrap().undertaking.is_none());
    w.advance_to(2 * HOUR).unwrap();
    assert_eq!(w.lifecycle_journal().count("conceptions"), 0);
}
#[test]
fn death_precedes_due_birth() {
    let mut w = manual();
    conceive(&mut w);
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 16 * HOUR;
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 2);
    assert_eq!(
        w.lifecycle_journal().count("pregnancy_lost_parent_death"),
        1
    );
}
#[test]
fn birth_extends_all_enabled_memories() {
    let mut c = research::fixture("dependency_social", 0).unwrap().config;
    c.autonomous = false;
    c.farming = Some(farming::Config {
        units: vec![0],
        rules: farming::Rules::default(),
    });
    c.capital = Some(capital::Config {
        wood: vec![],
        rules: capital::Rules::default(),
    });
    c.communal = true;
    c.infrastructure = Some(infrastructure::Config {
        enabled: true,
        sites: vec![],
        wood: vec![],
    });
    let mut w = World::new(c).unwrap();
    conceive(&mut w);
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.farm.memories.len(), 3);
    assert_eq!(w.capital.memories.len(), 3);
    assert_eq!(w.infrastructure.memories.len(), 3);
    assert_eq!(w.social.memories.len(), 3);
    assert_eq!(w.social.memories[0].relationships[&2].attachment, 0);
    w.audit().unwrap();
}
#[test]
fn child_commands_cannot_bypass_capability_gates() {
    let mut w = manual();
    conceive(&mut w);
    w.advance_to(9 * HOUR).unwrap();
    for intent in [
        Intent::BuildCart,
        Intent::Cultivate { plot: 0 },
        Intent::Reproduce { partner: 0 },
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    ] {
        w.apply_intents(vec![(2, intent)]).unwrap();
        assert_eq!(w.agents[2].action, Action::Idle);
    }
    assert!(
        w.events
            .iter()
            .filter(|e| e.kind == "Rejected")
            .all(|e| e.detail.contains("child capability"))
    );
}
#[test]
fn maturation_preserves_person_history_and_goods() {
    let mut w = manual();
    conceive(&mut w);
    w.advance_to(9 * HOUR).unwrap();
    let batch = 0;
    w.apply_intents(vec![(
        0,
        Intent::GiveCareFood {
            dependent: 2,
            batch,
            quantity: 6,
        },
    )])
    .unwrap();
    let gift = w
        .goods
        .batches
        .iter()
        .find(|b| b.owner == Owner::Agent(2) && b.quantity > 0)
        .unwrap()
        .id;
    w.apply_intents(vec![(
        2,
        Intent::Eat {
            batch: gift,
            quantity: 6,
        },
    )])
    .unwrap();
    let rng = w.agents[2].rng;
    let view = w.node_view(0);
    w.agents[2].memory.nodes.insert(0, view);
    let memory = w.agents[2].memory.clone();
    w.advance_to(33 * HOUR).unwrap();
    assert!(!w.is_child(2));
    assert_eq!(w.agents[2].rng, rng);
    assert_eq!(w.agents[2].memory, memory);
    assert_eq!(w.agents[2].consumed, 6);
    assert_eq!(w.biology(2).unwrap().birth_time, 9 * HOUR);
    assert_eq!(w.lifecycle_journal().count("maturations"), 1);
}
#[test]
fn remote_birth_does_not_create_paternal_social_knowledge() {
    let mut w = research::fixture("dependency_social", 0).unwrap();
    w.config.nodes = 2;
    w.config.edges = vec![Edge {
        a: 0,
        b: 1,
        nominal_ms: HOUR,
        capacity: 4,
    }];
    w.config.autonomous = false;
    conceive(&mut w);
    w.apply_intents(vec![(
        1,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(1));
    assert!(!w.social.memories[1].relationships.contains_key(&2));
    assert!(!w.household.memories[1].sightings.contains_key(&2));
    assert_eq!(w.household_view(1).unwrap().dependents, vec![2]);
}
#[test]
fn sex_swap_preserves_nonreproductive_decisions() {
    let mut a = manual();
    let mut b = a.clone();
    for p in &mut b.lifecycle.people {
        p.sex = if p.sex == Sex::Female {
            Sex::Male
        } else {
            Sex::Female
        };
    }
    for w in [&mut a, &mut b] {
        for p in &mut w.lifecycle.people {
            p.drive = false;
        }
        w.config.autonomous = true;
        w.advance_to(12 * HOUR).unwrap();
    }
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.metrics, b.metrics);
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn partition_and_submission_order_are_deterministic() {
    let mut a = manual();
    let mut b = a.clone();
    a.apply_intents(vec![
        (0, Intent::Reproduce { partner: 1 }),
        (1, Intent::Reproduce { partner: 0 }),
    ])
    .unwrap();
    b.apply_intents(vec![
        (1, Intent::Reproduce { partner: 0 }),
        (0, Intent::Reproduce { partner: 1 }),
    ])
    .unwrap();
    a.advance_to(12 * HOUR).unwrap();
    for t in 1..=24 {
        b.advance_to(t * HOUR / 2).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn known_child_death_uses_existing_attachment_and_remote_death_does_not() {
    let mut base = research::fixture("dependency_social", 0).unwrap();
    base.config.autonomous = false;
    base.config.nodes = 2;
    base.config.edges = vec![Edge {
        a: 0,
        b: 1,
        nominal_ms: HOUR,
        capacity: 4,
    }];
    conceive(&mut base);
    base.advance_to(9 * HOUR).unwrap();
    // Explicit matched attachment initial condition; no special child-death rule.
    base.social.memories[0]
        .relationships
        .get_mut(&2)
        .unwrap()
        .attachment = 20;
    let mut remote = base.clone();
    remote
        .apply_intents(vec![(
            2,
            Intent::Travel {
                edge: 0,
                target: 1,
                food: false,
            },
        )])
        .unwrap();
    remote.advance_to(12 * HOUR).unwrap();
    base.advance_to(12 * HOUR).unwrap();
    for w in [&mut base, &mut remote] {
        w.agents[2].starvation = 23 * HOUR;
        w.advance_to(13 * HOUR).unwrap();
        assert_eq!(w.agents[2].action, Action::Dead);
    }
    assert_eq!(base.social.memories[0].affect.grief.value(base.now), 10);
    assert_eq!(remote.social.memories[0].affect.grief.value(remote.now), 0);
}
#[test]
fn no_household_merge_no_property_or_remote_need_transfer() {
    let mut w = manual();
    w.config.household.as_mut().unwrap().households = vec![
        household::Household {
            id: 0,
            members: vec![0],
            home: None,
        },
        household::Household {
            id: 1,
            members: vec![1],
            home: None,
        },
    ];
    let initial = w.goods.clone();
    conceive(&mut w);
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.household_view(2).unwrap().households[0].id, 0);
    assert_eq!(w.config.household.as_ref().unwrap().households.len(), 2);
    assert_eq!(w.goods.batches, initial.batches);
    assert!(!w.goods.batches.iter().any(|b| b.owner == Owner::Agent(2)));
}
#[test]
fn due_birth_waits_for_node_without_extra_queue_slot() {
    let mut w = manual();
    w.config.nodes = 2;
    w.config.edges = vec![Edge {
        a: 0,
        b: 1,
        nominal_ms: 12 * HOUR,
        capacity: 1,
    }];
    conceive(&mut w);
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 2);
    w.advance_to(22 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 3);
    assert_eq!(w.agents[2].location, Location::Node(1));
    assert!(w.lifecycle.journal.delivery_delay_ms > 0);
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}
#[test]
fn controlled_generations_use_real_support_and_autonomous_mutual_choice() {
    let mut w = research::fixture("generational_control", 0).unwrap();
    for h in 0..=60 {
        w.advance_to(h * HOUR).unwrap();
        research::support_adults(&mut w).unwrap();
        research::seed_descendant_partner(&mut w);
    }
    assert!(w.lifecycle.people.iter().any(|b| b.generation == 3));
    assert!(w.events.iter().any(|e| e.kind == "ResearchAdultSupport"));
    assert!(w.events.iter().any(|e| e.kind == "Matured"));
    assert!(w.care_journal().count("food_transferred") > 0);
    w.audit().unwrap();
}
#[test]
fn children_do_not_perform_work_and_newborns_have_no_hidden_reserve() {
    let mut w = research::fixture("dependency", 0).unwrap();
    w.advance_to(30 * HOUR).unwrap();
    for a in w.agents.iter().filter(|a| w.is_child(a.id)) {
        for ((id, act), ms) in &w.care_journal().activity_ms {
            if *id == a.id {
                assert!(["idle", "sleep"].contains(&act.as_str()) || *ms == 0);
            }
        }
        assert_eq!(w.biology(a.id).unwrap().birth_time, 9 * HOUR);
    }
    assert!(w.lifecycle_journal().child_ms > 0);
}
#[test]
fn death_at_maturation_boundary_does_not_retroactively_create_an_adult() {
    let mut w = manual();
    conceive(&mut w);
    w.advance_to(33 * HOUR).unwrap();
    assert_eq!(w.agents[2].action, Action::Dead);
    assert!(!w.biology(2).unwrap().matured);
    assert_eq!(w.lifecycle_journal().count("maturations"), 0);
}
