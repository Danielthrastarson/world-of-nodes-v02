use super::*;
use crate::provisioning::{
    Config, HOUR, Holder, MINUTE, Material, Owner, World, authority, barter,
};
fn observe_domains(w: &mut World, id: usize) {
    w.observe_farming(id);
    w.observe_capital(id);
    w.observe_infrastructure(id);
    let local = w.observe(id).unwrap().local.unwrap();
    w.agents[id].memory.nodes.insert(local.node, local);
}
#[test]
fn person_is_the_same_embodied_record_not_a_second_agent() {
    let mut w = World::new(Config {
        autonomous: false,
        authority: Some(authority::Config {
            polity_id: 17,
            sovereign: 0,
            settlements: vec![0],
        }),
        ..Config::default()
    })
    .unwrap();
    let a: &Person = &w.agents[1];
    assert_eq!(a.id, 1);
    assert_eq!(a.needs().nutrition, 12 * HOUR);
    assert!(std::ptr::eq(a, w.person_state(1).unwrap().person));
    let before = w.agents[1].clone();
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: 1,
            settlement: 0,
        },
    )])
    .unwrap();
    let s = w.person_state(1).unwrap();
    assert_eq!(s.person, &before);
    assert_eq!(
        s.roles,
        vec![Role::Steward {
            polity: 17,
            office: 0,
            settlement: 0
        }]
    );
    w.advance_to(36 * HOUR).unwrap();
    let s = w.person_state(1).unwrap();
    assert_eq!(s.person.action, Action::Dead);
    assert!(s.observation.is_none());
    assert!(!s.person.can_reconsider());
    assert!(!w.office_active(0));
}
#[test]
fn role_assignment_does_not_add_a_brain_or_work_preference() {
    let mut w = World::new(Config {
        autonomous: false,
        authority: Some(authority::Config {
            polity_id: 17,
            sovereign: 0,
            settlements: vec![0],
        }),
        ..Config::default()
    })
    .unwrap();
    observe_domains(&mut w, 1);
    let o = w.observe(1).unwrap();
    let mut r1 = w.agents[1].rng;
    let before = decide(
        Context {
            observation: &o,
            beliefs: &w.agents[1].memory,
            capabilities: w.person_capabilities(1),
        },
        &mut r1,
    );
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: 1,
            settlement: 0,
        },
    )])
    .unwrap();
    let mut r2 = w.agents[1].rng;
    let after = decide(
        Context {
            observation: &o,
            beliefs: &w.agents[1].memory,
            capabilities: w.person_capabilities(1),
        },
        &mut r2,
    );
    assert_eq!(before, after);
    assert_eq!(r1, r2);
}
#[test]
fn composed_capabilities_use_personal_history_not_remote_truth() {
    let mut w = barter::research::world("complementary", true, 7, false).unwrap();
    observe_domains(&mut w, 0);
    let o = w.observe(0).unwrap();
    let m = w.agents[0].memory.clone();
    let f = w.farm.memories[0].clone();
    let c = w.capital.memories[0].clone();
    let fr = w.config.farming.as_ref().unwrap().rules.clone();
    let cr = w.config.capital.as_ref().unwrap().rules.clone();
    let ctx = Context {
        observation: &o,
        beliefs: &m,
        capabilities: Capabilities {
            life: None,
            social: None,
            care: None,
            cultivation: Some((&f, &fr)),
            capital: Some((&c, &cr)),
            public_works: None,
        },
    };
    let mut r1 = 91;
    let a = decide(ctx, &mut r1);
    for b in &mut w.goods.batches {
        if b.holder == Holder::Deposit(2) {
            b.quantity = 0;
        }
    }
    w.agents[1].nutrition = 0;
    let mut r2 = 91;
    let b = decide(ctx, &mut r2);
    assert_eq!(a, b);
    assert_eq!(r1, r2);
}
#[test]
fn survival_reflex_dominates_all_available_economic_capabilities() {
    let mut w = barter::research::world("complementary", true, 2, false).unwrap();
    observe_domains(&mut w, 0);
    w.agents[0].nutrition = HOUR;
    // Move one existing local food quantum into personal custody through real actions.
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.material == Material::Food && b.holder == Holder::Deposit(0))
        .unwrap()
        .id;
    w.config.autonomous = false;
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch: b,
            communal: false,
        },
    )])
    .unwrap();
    w.advance_to(MINUTE).unwrap();
    let o = w.observe(0).unwrap();
    let mut rng = 2;
    let d = decide(
        Context {
            observation: &o,
            beliefs: &w.agents[0].memory,
            capabilities: w.person_capabilities(0),
        },
        &mut rng,
    );
    assert!(matches!(d.intent, Intent::Eat { .. }));
}
#[test]
fn one_step_plan_does_not_erase_or_invent_a_longer_undertaking() {
    let mut w = World::new(Config {
        autonomous: false,
        initial_request: 600,
        ..Config::default()
    })
    .unwrap();
    w.apply_intents(vec![(
        0,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    )])
    .unwrap();
    let o = w.observe(0).unwrap();
    let plan = ActionPlan::new(
        &o,
        Decision {
            intent: Intent::Sleep,
            reason: "existing interruption".into(),
        },
    );
    assert!(plan.undertaking.is_some());
    assert_eq!(plan.command(), &Intent::Sleep);
    w.apply_intents(vec![(0, plan.proposed.intent)]).unwrap();
    assert_eq!(w.agents[0].job, plan.undertaking);
    w.apply_intents(vec![(0, Intent::CancelJob)]).unwrap();
    assert!(w.person_state(0).unwrap().reasoning.undertaking.is_none());
}
#[test]
fn domain_state_is_belief_not_a_remote_inventory_projection() {
    let mut w = barter::research::world("complementary", true, 0, false).unwrap();
    observe_domains(&mut w, 0);
    let stored = w.farm.memories[0].clone();
    let remote = w.farm.plots.iter_mut().find(|p| p.node == 2).unwrap();
    remote.crop = Some(farming::Crop {
        id: 99,
        owner: Owner::Agent(1),
        started_at: 0,
        stage: farming::Stage::Mature,
        remaining: 0,
    });
    let state = w.person_state(0).unwrap();
    assert_eq!(state.capabilities.cultivation.unwrap().0, &stored);
    assert!(
        state
            .observation
            .unwrap()
            .own
            .iter()
            .all(|b| b.holder != Holder::Loose(2))
    );
}
#[test]
fn legal_exchange_stage_is_pure_and_keeps_physical_payment_out_of_choice() {
    let mut w = barter::research::world("colocated", true, 0, false).unwrap();
    observe_domains(&mut w, 0);
    let o = w.observe(0).unwrap();
    let v = w.barter_view(0).unwrap();
    let input = barter::selection::Input {
        local: v,
        owner_views: Default::default(),
        market: false,
    };
    let fallback = Decision {
        intent: Intent::Wait,
        reason: "same fallback".into(),
    };
    let first = select_exchange(&input, &o, fallback.clone());
    w.goods.batches[2].quantity = 0; // Hidden counterparty goods cannot feed the captured choice.
    let again = select_exchange(&input, &o, fallback);
    assert_eq!(first.decision, again.decision);
    assert_eq!(first.notes, again.notes);
    // Selection alone never settles ownership or inventory.
    assert!(w.barter.exchanges.is_empty());
}
#[test]
fn repeated_person_inspection_is_observational() {
    let mut w = barter::research::market_world("colocated", 3, false).unwrap();
    observe_domains(&mut w, 0);
    let mut untouched = w.clone();
    for _ in 0..10 {
        let s = w.person_state(0).unwrap();
        assert_eq!(s.person.id, 0);
    }
    assert_eq!(w.state_hash(), untouched.state_hash());
    w.advance_to(2 * HOUR).unwrap();
    untouched.advance_to(2 * HOUR).unwrap();
    assert_eq!(w.event_hash(), untouched.event_hash());
    assert_eq!(w.state_hash(), untouched.state_hash());
}
#[test]
fn no_atomic_command_can_turn_role_into_free_goods_or_mid_edge_action() {
    let mut w = World::new(Config {
        autonomous: false,
        trace: true,
        ..Config::default()
    })
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
    assert!(!w.agents[0].can_reconsider());
    let o = w.observe(0).unwrap();
    let p = ActionPlan::new(
        &o,
        Decision {
            intent: Intent::AppointSteward {
                candidate: 1,
                settlement: 1,
            },
            reason: "external invalid command".into(),
        },
    );
    w.apply_intents(vec![(0, p.proposed.intent)]).unwrap();
    assert_eq!(w.agents[0].action, Action::Travel);
    assert!(w.offices().is_empty());
    w.audit().unwrap();
}
