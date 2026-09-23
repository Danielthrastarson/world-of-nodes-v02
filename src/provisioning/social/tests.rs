use super::*;
#[test]
fn costly_help_changes_compressed_history_more_than_safe_help() {
    let a = research::history("help_safe", 0).unwrap();
    let b = research::history("help_costly", 0).unwrap();
    let r = |w: &World| w.social.memories[1].relationships[&0].clone();
    assert_eq!(r(&a).trust, 6);
    assert_eq!(r(&b).trust, 12);
    assert_eq!(r(&b).obligation, 12);
    assert_eq!(r(&b).attachment, 26);
    assert_eq!(b.social.memories[0].relationships[&1].trust, 0);
    assert_eq!(b.metrics.consumed, 6);
    b.audit().unwrap();
}
#[test]
fn same_unmet_need_appraises_alternatives_not_an_outcome_label() {
    let a = research::history("failed_attempt", 0).unwrap();
    let b = research::history("refusal", 0).unwrap();
    let r = &a.social.memories[1].relationships[&0];
    assert_eq!((r.trust, r.grievance), (-1, 0));
    let r = &b.social.memories[1].relationships[&0];
    assert_eq!((r.trust, r.grievance), (-6, 6));
    assert_eq!(a.social.memories[1].affect.anger.value(0), 0);
    assert_eq!(b.social.memories[1].affect.anger.value(0), 12);
    assert!(matches!(a.agents[0].action, Action::Queued { .. }));
}
#[test]
fn hidden_refusal_and_death_do_not_update_remote_beliefs() {
    let a = research::history("remote_refusal", 0).unwrap();
    assert!(a.social.journal.appraisals.is_empty());
    assert_eq!(a.social.memories[1].relationships[&0].trust, 0);
    let b = research::history("death_unknown", 0).unwrap();
    assert_eq!(b.agents[2].action, Action::Dead);
    assert!(!b.social.memories[1].learned_deaths.contains(&2));
    assert_eq!(b.social.memories[1].affect.grief.value(b.now), 0);
}
#[test]
fn later_local_arrival_can_reveal_death_once() {
    let mut w = research::history("death_unknown", 0).unwrap();
    w.apply_intents(vec![(
        1,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(4 * HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(1));
    w.observe_social(1);
    assert!(w.social.memories[1].learned_deaths.contains(&2));
    let before = w.social.journal.appraisals.len();
    w.observe_social(1);
    assert_eq!(before, w.social.journal.appraisals.len());
    assert_eq!(w.social.memories[1].affect.grief.value(w.now), 10);
}
#[test]
fn grief_requires_attachment_and_known_death() {
    let a = research::history("death_known", 0).unwrap();
    assert_eq!(a.social.memories[1].affect.grief.value(HOUR), 10);
    assert_eq!(a.social.memories[0].affect.grief.value(HOUR), 0);
}
#[test]
fn history_changes_later_person_choice_under_identical_physics() {
    let a = research::matched(&research::history("help_costly", 1).unwrap(), 1).unwrap();
    let b = research::matched(&research::history("refusal", 1).unwrap(), 1).unwrap();
    assert_eq!(
        research::physical_signature(&a),
        research::physical_signature(&b)
    );
    assert!(matches!(
        research::decision(&a, 1).intent,
        Intent::GiveCareFood { .. }
    ));
    assert!(!matches!(
        research::decision(&b, 1).intent,
        Intent::GiveCareFood { .. }
    ));
}
#[test]
fn affect_is_lazy_and_grievance_outlasts_anger() {
    let a = research::history("refusal", 0).unwrap();
    let m = a.social.memories[1].clone();
    assert_eq!(m.affect.anger.value(6 * HOUR), 6);
    assert_eq!(m.affect.anger.value(12 * HOUR), 0);
    assert_eq!(m.relationships[&0].grievance, 6);
    assert_eq!(m, a.social.memories[1]);
}
#[test]
fn fear_deduplicates_and_can_change_a_recovered_persons_choice() {
    let mut h = research::history("fear", 0).unwrap();
    let count = h.social.journal.appraisals.len();
    h.observe_social(1);
    assert_eq!(count, h.social.journal.appraisals.len());
    let a = research::matched(&h, 0).unwrap();
    let b = research::matched(&research::history("no_history", 0).unwrap(), 0).unwrap();
    assert_eq!(
        research::physical_signature(&a),
        research::physical_signature(&b)
    );
    assert!(!matches!(
        research::decision(&a, 1).intent,
        Intent::GiveCareFood { .. }
    ));
    assert!(matches!(
        research::decision(&b, 1).intent,
        Intent::GiveCareFood { .. }
    ));
}
#[test]
fn sparse_graph_is_linear_in_explicit_edges_not_population_squared() {
    let c = super::super::Config {
        social: Some(Config::default()),
        starts: vec![0; 100_000],
        ..super::super::Config::default()
    };
    let s = State::new(&c);
    assert_eq!(s.memories.len(), 100_000);
    assert!(s.known_by.is_empty());
    assert_eq!(
        s.memories
            .iter()
            .map(|m| m.relationships.len())
            .sum::<usize>(),
        0
    );
}
#[test]
fn dynamics_are_deterministic_and_observer_journal_does_not_drive_choice() {
    let mut a = research::live("farming", true, 2).unwrap();
    let mut b = a.clone();
    b.social.journal.add("unread_research");
    a.advance_to(12 * HOUR).unwrap();
    for t in 1..=24 {
        b.advance_to(t * HOUR / 2).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    a.audit().unwrap();
    b.audit().unwrap();
}
#[test]
fn anger_and_grief_can_change_care_without_changing_physics_or_rights() {
    let h = research::history("no_history", 0).unwrap();
    let a = research::matched(&h, 0).unwrap();
    for kind in ["anger", "grief"] {
        let mut calm = a.clone();
        calm.agents[0].nutrition = 2 * HOUR;
        calm.observe_household(1);
        let mut affected = calm.clone();
        if kind == "anger" {
            affected.social.memories[1].affect.anger.raise(6, HOUR);
        } else {
            affected.social.memories[1].affect.grief.raise(6, HOUR);
        }
        assert!(matches!(
            research::decision(&calm, 1).intent,
            Intent::GiveCareFood { .. }
        ));
        assert!(!matches!(
            research::decision(&affected, 1).intent,
            Intent::GiveCareFood { .. }
        ));
        assert_eq!(
            research::physical_signature(&calm),
            research::physical_signature(&affected)
        );
    }
}
#[test]
fn each_relational_dimension_has_an_inspectable_effect() {
    let w = research::matched(&research::history("no_history", 0).unwrap(), 0).unwrap();
    let base = research::assessment(&w, 1).unwrap();
    for kind in ["trust", "attachment", "obligation", "grievance"] {
        let mut b = w.clone();
        let r = b.social.memories[1].relationships.get_mut(&0).unwrap();
        match kind {
            "trust" => r.trust += 6,
            "attachment" => r.attachment += 10,
            "obligation" => r.obligation += 6,
            _ => r.grievance += 6,
        }
        let after = research::assessment(&b, 1).unwrap();
        if kind == "grievance" {
            assert_eq!(after.cost(), base.cost() + 1);
        } else {
            assert_eq!(after.pressure(), base.pressure() + 1);
        }
    }
}
#[test]
fn self_preservation_is_not_appraised_as_deliberate_harm() {
    let mut w = research::history("no_history", 0).unwrap();
    w.agents[0].nutrition = 4 * HOUR;
    let b = w.goods.add(
        Material::Food,
        6,
        Owner::Agent(0),
        Holder::Agent(0),
        0,
        48 * HOUR,
    );
    w.apply_intents(vec![(
        0,
        Intent::Eat {
            batch: b,
            quantity: 6,
        },
    )])
    .unwrap();
    let r = &w.social.memories[1].relationships[&0];
    assert_eq!((r.trust, r.grievance), (-1, 0));
    assert_eq!(w.social.memories[1].affect.anger.value(w.now), 0);
}
#[test]
fn remote_death_does_not_trigger_an_unrelated_self_observation() {
    let mut w = research::history("death_unknown", 0).unwrap();
    w.agents[1].starvation = 18 * HOUR;
    w.social.memories[1].in_danger = false;
    let before = w.social.memories[1].clone();
    w.social_death(2);
    assert_eq!(w.social.memories[1], before);
}
#[test]
fn simultaneous_dead_people_do_not_acquire_grief() {
    let mut w = research::history("no_history", 0).unwrap();
    for id in [1, 2] {
        w.agents[id].location = Location::Node(0);
        w.agents[id].nutrition = 0;
        w.agents[id].starvation = 23 * HOUR;
    }
    w.advance_to(2 * HOUR).unwrap();
    assert_eq!(w.agents[1].action, Action::Dead);
    assert_eq!(w.social.memories[1].affect.grief.value(w.now), 0);
}
