use super::*;
fn ready(case: &str) -> World {
    let mut w = research::fixture(case, 0).unwrap();
    research::prepare_command(&mut w, case).unwrap();
    w
}
#[test]
fn one_order_moves_actual_people_and_preserves_identity() {
    let mut w = ready("short");
    let before: Vec<_> = w
        .agents
        .iter()
        .map(|a| (a.id, w.biology(a.id).cloned()))
        .collect();
    assert_eq!(w.unit_snapshot(0).unwrap().ever_assembled, 4);
    assert_eq!(w.issue_unit_move(0, 0, 1).unwrap(), 4);
    w.advance_to(4 * HOUR).unwrap();
    let s = w.unit_snapshot(0).unwrap();
    assert_eq!(s.departed, 4);
    assert_eq!(s.arrived, 4);
    assert_eq!(s.at_objective, 4);
    assert_eq!(s.arrival_spread, Some(0));
    assert_eq!(w.formation.journal.count("commands"), 1);
    assert_eq!(
        w.agents
            .iter()
            .map(|a| (a.id, w.biology(a.id).cloned()))
            .collect::<Vec<_>>(),
        before
    );
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}
#[test]
fn partial_and_late_members_do_not_receive_orders_remotely() {
    let mut w = ready("partial");
    assert_eq!(w.unit_snapshot(0).unwrap().enlisted, 4);
    assert_eq!(w.issue_unit_move(0, 1, 2).unwrap(), 2);
    w.advance_to(6 * HOUR).unwrap();
    assert_eq!(w.unit_snapshot(0).unwrap().arrived, 2);
    let mut w = ready("late");
    let received = w.issue_unit_move(0, 1, 2).unwrap();
    assert_eq!(received, 2);
    w.advance_to(6 * HOUR).unwrap();
    let s = w.unit_snapshot(0).unwrap();
    assert!(s.ever_assembled > received);
    assert_eq!(w.formation.journal.count("receipts"), 2);
    assert_eq!(s.arrived, 2);
    for v in w.formation.views.iter().filter(|v| v.order.is_none()) {
        assert!(v.departed_at.is_none())
    }
}
#[test]
fn critical_recovery_keeps_membership_then_resumes() {
    let mut w = ready("fatigue_stress");
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(2 * HOUR).unwrap();
    assert!(matches!(w.agents[0].action, Action::Sleep { .. }));
    assert_eq!(w.military_role(0), Some(0));
    assert!(w.formation.views[0].order.is_some());
    w.advance_to(11 * HOUR).unwrap();
    assert!(w.formation.views[0].departed_at.is_some());
    assert!(w.formation.journal.count("PAUSED_FATIGUE") > 0);
    assert_eq!(w.unit_snapshot(0).unwrap().enlisted, 4);
}
#[test]
fn hunger_pause_cancels_queue_without_erasing_person_or_membership() {
    let mut w = ready("bottleneck");
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(HOUR + MINUTE).unwrap();
    let id = (0..4)
        .find(|&i| matches!(w.agents[i].action, Action::Queued { .. }))
        .unwrap();
    w.agents[id].nutrition = 0;
    for b in &mut w.goods.batches {
        if b.owner == Owner::Agent(id) {
            b.material = Material::Waste;
        }
    }
    // Controlled already-spoiled stock: food becomes waste, with no quantity duplication.
    // Rebuild the controlled food accounting for this forced spoilage initial condition.
    let q: u64 = w
        .goods
        .batches
        .iter()
        .filter(|b| b.owner == Owner::Agent(id) && b.material == Material::Waste)
        .map(|b| b.quantity)
        .sum();
    w.metrics.spoiled += q;
    w.advance_to(HOUR + 2 * MINUTE).unwrap();
    assert!(!matches!(
        w.agents[id].action,
        Action::Queued { .. } | Action::Travel
    ));
    assert_eq!(w.military_role(id), Some(0));
    assert!(w.formation.views[id].order.is_some());
}
#[test]
fn expired_service_does_not_teleport_or_keep_a_queue_reservation() {
    let mut w = ready("bottleneck");
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(HOUR + MINUTE).unwrap();
    let moving = (0..4)
        .find(|&i| w.agents[i].action == Action::Travel)
        .unwrap();
    let waiting = (0..4)
        .find(|&i| matches!(w.agents[i].action, Action::Queued { .. }))
        .unwrap();
    w.config.autonomous = false;
    let due = w.now + MINUTE;
    for id in [moving, waiting] {
        w.military.memories[id].undertaking.as_mut().unwrap().until = Some(due);
    }
    w.advance_to(due).unwrap();
    assert_eq!(w.agents[moving].action, Action::Travel);
    assert_eq!(w.agents[waiting].action, Action::Idle);
    assert!(!w.formation.views[waiting].active);
    assert!(w.units()[&0].members.contains_key(&waiting));
    w.audit().unwrap();
}
#[test]
fn unknown_route_is_not_created_by_the_objective() {
    let mut w = ready("unknown_route");
    w.issue_unit_move(0, 0, 2).unwrap();
    w.advance_to(4 * HOUR).unwrap();
    assert_eq!(w.unit_snapshot(0).unwrap().departed, 0);
    assert_eq!(w.unit_snapshot(0).unwrap().active, 4);
    assert!(w.formation.journal.count("PAUSED_NO_KNOWN_ROUTE") > 0);
}
#[test]
fn new_order_does_not_update_an_absent_straggler() {
    let mut w = ready("fatigue_stress");
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(4 * HOUR).unwrap();
    let old = w.formation.views[0].order.clone();
    assert_eq!(w.issue_unit_move(0, 1, 0).unwrap(), 3);
    assert_eq!(w.formation.views[0].order, old);
    assert_ne!(w.formation.views[1].order, old);
    w.audit().unwrap();
}
#[test]
fn dying_remains_on_roster_and_uses_ordinary_property_death() {
    let mut w = ready("short");
    w.issue_unit_move(0, 0, 1).unwrap();
    w.config.autonomous = false;
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 24 * HOUR;
    w.advance_to(w.now + 1).unwrap();
    assert_eq!(w.unit_snapshot(0).unwrap().dead, 1);
    assert_eq!(w.unit_snapshot(0).unwrap().enlisted, 4);
    assert_eq!(w.military_role(0), None);
    assert!(
        w.goods
            .batches
            .iter()
            .any(|b| b.owner == Owner::Agent(0) && b.holder == Holder::Loose(0) && b.quantity > 0)
    );
    w.audit().unwrap();
}
#[test]
fn existing_paths_crossings_and_carts_use_common_physics() {
    let mut timings = vec![];
    for case in ["path_on", "path_off"] {
        let mut w = ready(case);
        w.issue_unit_move(0, 0, 1).unwrap();
        w.advance_to(4 * HOUR).unwrap();
        timings.push(w.formation.views[0].arrived_at.unwrap());
        assert_eq!(w.config.edges[0].capacity, 4);
    }
    assert!(timings[0] < timings[1]);
    let mut w = ready("crossing");
    assert_eq!(w.crossing_projects()[0].wood, 1);
    assert!(w.crossing_projects()[0].completed_at.is_some());
    assert_eq!(w.effective_edge(0).capacity, 1);
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(14 * HOUR).unwrap();
    assert!(w.unit_snapshot(0).unwrap().arrived > 0);
    assert_eq!(w.metrics.overtakes, 0);
    let mut w = ready("cart");
    assert_eq!(w.carts()[0].owner, Owner::Agent(0));
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.carts()[0].location, Location::Node(1));
    assert_eq!(w.carts()[0].user, Some(0));
    w.audit().unwrap();
}
#[test]
fn command_choices_use_personal_history_only() {
    let mut w = ready("sequence");
    w.issue_unit_move(0, 0, 2).unwrap();
    let first = social::research::decision(&w, 0);
    let old = w.formation.views[0].clone();
    w.formation
        .units
        .get_mut(&0)
        .unwrap()
        .objective
        .as_mut()
        .unwrap()
        .target = 0;
    w.agents[3].nutrition = 0;
    assert_eq!(social::research::decision(&w, 0), first);
    assert_eq!(w.formation.views[0], old);
}
#[test]
fn birth_adds_a_person_without_membership_or_inherited_orders() {
    let mut c = lifecycle::research::fixture("basic", 0).unwrap().config;
    c.autonomous = false;
    c.formation = true;
    c.military = Some(military::Config {
        enabled: true,
        requests: vec![],
    });
    let mut w = World::new(c).unwrap();
    w.apply_intents(vec![
        (0, Intent::Reproduce { partner: 1 }),
        (1, Intent::Reproduce { partner: 0 }),
    ])
    .unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 3);
    assert_eq!(w.unit_view(2), Some(&View::default()));
    assert!(w.units().is_empty());
    w.audit().unwrap();
}
#[test]
fn observer_sampling_does_not_change_unit_or_person_execution() {
    let mut a = ready("bottleneck");
    let mut b = ready("bottleneck");
    a.issue_unit_move(0, 0, 1).unwrap();
    b.issue_unit_move(0, 0, 1).unwrap();
    a.advance_to(13 * HOUR).unwrap();
    for minute in 61..=780 {
        b.advance_to(minute * MINUTE).unwrap();
        let _ = b.unit_snapshot(0);
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn people_together_mid_edge_are_not_measured_as_separated() {
    let mut w = ready("short");
    w.issue_unit_move(0, 0, 1).unwrap();
    w.advance_to(2 * HOUR).unwrap();
    let s = w.unit_snapshot(0).unwrap();
    assert_eq!(s.moving, 4);
    assert_eq!(s.modal_fraction, 0.0);
    assert_eq!(s.coincident_fraction, 1.0);
    assert_eq!(s.separated, 0);
}
