use super::*;

// Controlled institutional fixture: all parties initially meet at a real project
// site. Node 0 is reachable by an ordinary road; edge 1 remains an unfinished
// crossing. No material, office acceptance, route or courier labor is implicit.
fn world() -> World {
    World::new(Config {
        commands: true,
        information: true,
        authority: Some(authority::Config {
            polity_id: 42,
            sovereign: 0,
            settlements: vec![0, 1],
        }),
        nodes: 3,
        starts: vec![1; 4],
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR,
                capacity: 4,
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: HOUR,
                capacity: 4,
            },
        ],
        food: vec![],
        boards: vec![],
        autonomous: false,
        initial_nutrition: 24 * HOUR,
        initial_fatigue: 0,
        trace: true,
        infrastructure: Some(infrastructure::Config {
            enabled: true,
            wood: vec![(1, 10)],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 1,
                work_ms: HOUR,
                opened_nominal_ms: HOUR / 2,
                opened_capacity: 4,
            }],
        }),
        ..Config::default()
    })
    .unwrap()
}
// Causal-event pointers legitimately change when communication is logged. Every
// embodied, possession, action, memory and identity field must remain identical.
fn assert_same_person_except_event(actual: &Agent, expected: &Agent) {
    let mut expected = expected.clone();
    expected.last_event = actual.last_event;
    assert_eq!(actual, &expected);
}
fn offer(w: &mut World) {
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: 1,
            settlement: 1,
        },
    )])
    .unwrap();
}
fn accepted() -> World {
    let mut w = world();
    offer(&mut w);
    w.accept_office(1, 0).unwrap();
    w.observe_command_sites(0).unwrap();
    w
}
fn order(w: &mut World) -> u64 {
    w.issue_order(
        0,
        0,
        Objective::CommissionCrossing {
            project: 0,
            settlement: 1,
        },
        w.now + 24 * HOUR,
    )
    .unwrap()
}
fn travel(w: &mut World, id: usize, target: usize) {
    let arrival = w.now + HOUR;
    w.apply_intents(vec![(
        id,
        Intent::Travel {
            edge: 0,
            target,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(arrival).unwrap();
    assert_eq!(w.agents[id].location, Location::Node(target));
}
fn choice(w: &World, id: usize) -> (Decision, u64) {
    let observation = w.observe(id).unwrap();
    let mut rng = w.agents[id].rng;
    let decision = person::decide(
        person::Context {
            observation: &observation,
            beliefs: &w.agents[id].memory,
            capabilities: w.person_capabilities(id),
        },
        &mut rng,
    );
    (decision, rng)
}

#[test]
fn appointment_is_an_offer_until_the_candidate_accepts_in_person() {
    let mut w = world();
    let candidate = w.agents[1].clone();
    offer(&mut w);
    assert_same_person_except_event(&w.agents[1], &candidate);
    assert!(!w.office_active(0));
    assert!(w.command_memory(1).unwrap().accepted_offices.is_empty());
    assert_eq!(w.command_memory(1).unwrap().offered_offices.len(), 1);
    assert!(w.accept_office(2, 0).is_err());
    travel(&mut w, 1, 0);
    assert!(w.accept_office(1, 0).is_err());
    assert!(!w.office_active(0));
    travel(&mut w, 1, 1);
    let body = w.agents[1].clone();
    let goods = w.goods.clone();
    w.accept_office(1, 0).unwrap();
    assert!(w.office_active(0));
    assert_eq!(w.command_memory(1).unwrap().accepted_offices.len(), 1);
    assert_eq!(w.agents[1].location, body.location);
    assert_eq!(w.agents[1].nutrition, body.nutrition);
    assert_eq!(w.agents[1].fatigue, body.fatigue);
    assert_eq!(w.goods, goods);
    assert!(w.accept_office(1, 0).is_err());
    w.audit().unwrap();
}

#[test]
fn issuance_is_information_and_cannot_remotely_form_an_obligation() {
    let mut w = accepted();
    travel(&mut w, 0, 0);
    let recipient = w.command_memory(1).unwrap().clone();
    let person = w.agents[1].clone();
    let goods = w.goods.clone();
    let projects = w.infrastructure.projects.clone();
    let command = order(&mut w);
    assert_eq!(w.command_memory(1).unwrap(), &recipient);
    assert_same_person_except_event(&w.agents[1], &person);
    assert_eq!(w.goods, goods);
    assert_eq!(w.infrastructure.projects, projects);
    assert!(!w.crossing_authorized(0));
    assert!(w.receive_command(0, 1, command).is_err());
    assert!(w.receive_command(2, 1, command).is_err());
    assert!(w.receive_command(0, 0, command).is_err());
    assert!(w.receive_command(0, 99, command).is_err());
    assert_eq!(w.command_memory(1).unwrap(), &recipient);
    w.audit().unwrap();
}

#[test]
fn physical_carriage_preserves_payload_and_does_not_transport_goods_or_people() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 2, command).unwrap();
    let held = w.command_memory(2).unwrap().envelopes[&command].clone();
    travel(&mut w, 1, 0);
    assert!(w.receive_command(2, 1, command).is_err());
    w.apply_intents(vec![(
        2,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])
    .unwrap();
    assert!(w.receive_command(2, 1, command).is_err());
    w.advance_to(2 * HOUR).unwrap();
    let person = w.agents[1].clone();
    let goods = w.goods.clone();
    w.receive_command(2, 1, command).unwrap();
    assert_eq!(w.command_memory(2).unwrap().envelopes[&command], held);
    assert!(
        w.command_memory(1)
            .unwrap()
            .envelopes
            .contains_key(&command)
    );
    assert!(
        w.command_memory(1)
            .unwrap()
            .obligations
            .contains_key(&command)
    );
    assert_same_person_except_event(&w.agents[1], &person);
    assert_eq!(w.goods, goods);
    assert!(!w.crossing_authorized(0));
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn remote_revocation_cannot_change_a_receivers_knowledge_or_choice() {
    let mut a = accepted();
    let command = order(&mut a);
    a.receive_command(0, 1, command).unwrap();
    travel(&mut a, 0, 0);
    let mut b = a.clone();
    let revocation = b.issue_revocation(0, command).unwrap();
    assert_eq!(a.command_memory(1), b.command_memory(1));
    assert_eq!(choice(&a, 1), choice(&b, 1));
    assert!(
        !b.command_memory(1)
            .unwrap()
            .envelopes
            .contains_key(&revocation)
    );
    assert!(b.receive_command(0, 1, revocation).is_err());
    travel(&mut b, 0, 1);
    b.receive_command(0, 1, revocation).unwrap();
    assert_eq!(
        b.command_memory(1).unwrap().obligations[&command].status,
        Status::Revoked
    );
    assert!(!b.crossing_authorized(0));
    a.audit().unwrap();
    b.audit().unwrap();
}

#[test]
fn revocation_received_before_order_survives_duplicates_and_delayed_order() {
    let mut w = accepted();
    let command = order(&mut w);
    let revoked = w.issue_revocation(0, command).unwrap();
    w.receive_command(0, 1, revoked).unwrap();
    let early_receipt = w.command_memory(1).unwrap().envelopes[&revoked].clone();
    w.advance_to(MINUTE).unwrap();
    w.receive_command(0, 1, command).unwrap();
    assert!(
        w.command_memory(1)
            .unwrap()
            .obligations
            .get(&command)
            .is_none_or(|o| o.status == Status::Revoked)
    );
    assert_eq!(w.command_journal().count("orders_valid"), 0);
    assert_eq!(w.command_journal().count("obligations_formed"), 0);
    assert_eq!(w.command_journal().count("orders_revoked_at_receipt"), 1);
    let memory = w.command_memory(1).unwrap().clone();
    w.advance_to(2 * MINUTE).unwrap();
    w.receive_command(0, 1, command).unwrap();
    w.receive_command(0, 1, revoked).unwrap();
    assert_eq!(w.command_memory(1).unwrap(), &memory);
    assert_eq!(
        w.command_memory(1).unwrap().envelopes[&revoked],
        early_receipt
    );
    assert!(!w.crossing_authorized(0));
    w.audit().unwrap();
}

#[test]
fn only_the_authorized_sovereign_can_issue_and_revoke_an_order() {
    let mut w = accepted();
    let objective = Objective::CommissionCrossing {
        project: 0,
        settlement: 1,
    };
    for issuer in [1, 2, 99] {
        assert!(w.issue_order(issuer, 0, objective.clone(), HOUR).is_err());
    }
    assert!(w.issue_order(0, 99, objective, HOUR).is_err());
    let command = order(&mut w);
    for issuer in [1, 2, 99] {
        assert!(w.issue_revocation(issuer, command).is_err());
    }
    assert!(w.issue_revocation(0, 99).is_err());
    assert!(w.command_memory(1).unwrap().obligations.is_empty());
    w.audit().unwrap();
}

#[test]
fn expired_order_is_retained_as_information_but_never_creates_active_compliance() {
    let mut w = accepted();
    let command = w
        .issue_order(
            0,
            0,
            Objective::CommissionCrossing {
                project: 0,
                settlement: 1,
            },
            HOUR,
        )
        .unwrap();
    w.advance_to(HOUR).unwrap();
    w.receive_command(0, 1, command).unwrap();
    assert!(
        w.command_memory(1)
            .unwrap()
            .envelopes
            .contains_key(&command)
    );
    assert!(
        w.command_memory(1)
            .unwrap()
            .obligations
            .get(&command)
            .is_none_or(|o| o.status == Status::Expired)
    );
    assert_eq!(w.command_journal().count("orders_valid"), 0);
    assert_eq!(w.command_journal().count("obligations_formed"), 0);
    assert_eq!(w.command_journal().count("orders_expired_at_receipt"), 1);
    assert!(!w.crossing_authorized(0));
    assert!(w.fulfil_order(1, command).is_err());
    w.audit().unwrap();
}

#[test]
fn critical_fatigue_can_interrupt_a_valid_order_without_refusal() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    w.agents[1].fatigue = 23 * HOUR;
    w.command_before_decision(1);
    let selected = choice(&w, 1).0;
    assert_eq!(selected.intent, Intent::Sleep);
    assert_ne!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Refused
    );
    assert!(!w.crossing_authorized(0));
    w.apply_intents(vec![(1, selected.intent)]).unwrap();
    assert!(matches!(w.agents[1].action, Action::Sleep { .. }));
    assert!(
        w.command_memory(1)
            .unwrap()
            .obligations
            .contains_key(&command)
    );
    w.audit().unwrap();
}

#[test]
fn death_prevents_handoff_and_never_inherits_command_duties() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 2, command).unwrap();
    let stranger = w.command_memory(3).unwrap().clone();
    w.agents[1].nutrition = 0;
    w.agents[1].starvation = 24 * HOUR;
    w.advance_to(1).unwrap();
    assert_eq!(w.agents[1].action, Action::Dead);
    assert!(w.receive_command(2, 1, command).is_err());
    assert!(!w.office_active(0));
    assert_eq!(w.command_memory(3).unwrap(), &stranger);
    assert!(!w.crossing_authorized(0));
    w.audit().unwrap();
}

#[test]
fn disabled_command_institution_has_no_effect_on_historical_state() {
    let mut w = World::new(Config {
        autonomous: false,
        ..Config::default()
    })
    .unwrap();
    let before = w.state_hash();
    assert!(w.accept_office(1, 0).is_err());
    assert!(
        w.issue_order(
            0,
            0,
            Objective::CommissionCrossing {
                project: 0,
                settlement: 0
            },
            HOUR
        )
        .is_err()
    );
    assert!(w.issue_revocation(0, 0).is_err());
    assert!(w.receive_command(0, 1, 0).is_err());
    assert!(w.begin_command_delivery(0, 0, 1, 0).is_err());
    assert!(w.fulfil_order(1, 0).is_err());
    assert!(w.refuse_order(1, 0, "explicit controlled refusal").is_err());
    assert_eq!(w.state_hash(), before);
    assert!(w.command_memory(0).is_none());
    w.audit().unwrap();
}

#[test]
fn receipt_and_delivery_start_do_not_create_caller_partition_dependent_decisions() {
    for courier in [false, true] {
        let mut a = accepted();
        a.advance_to(MINUTE / 2).unwrap();
        let command = order(&mut a);
        if courier {
            a.receive_command(0, 2, command).unwrap();
            a.begin_command_delivery(2, command, 1, 1).unwrap();
        } else {
            a.receive_command(0, 1, command).unwrap();
        }
        a.config.autonomous = true;
        a.next_policy = MINUTE;
        a.wake.clear();
        let mut b = a.clone();
        a.advance_to(3 * MINUTE).unwrap();
        for t in [
            MINUTE / 2 + 1,
            MINUTE - 1,
            MINUTE + 7,
            2 * MINUTE + 13,
            3 * MINUTE,
        ] {
            b.advance_to(t).unwrap();
        }
        assert_eq!(a.state_hash(), b.state_hash(), "courier={courier}");
        assert_eq!(a.event_hash(), b.event_hash(), "courier={courier}");
        assert!(
            !b.events
                .iter()
                .any(|e| e.kind == "Decision" && e.at > MINUTE / 2 && e.at < MINUTE)
        );
        a.audit().unwrap();
        b.audit().unwrap();
    }
}

#[test]
fn authenticated_out_of_scope_message_is_information_not_valid_authority() {
    let mut w = accepted();
    let message = w
        .issue_order(
            0,
            0,
            Objective::Unsupported {
                label: "SEIZE_PRIVATE_CART".into(),
            },
            HOUR,
        )
        .unwrap();
    let goods = w.goods.clone();
    w.receive_command(0, 1, message).unwrap();
    assert!(
        w.command_memory(1)
            .unwrap()
            .envelopes
            .contains_key(&message)
    );
    assert!(
        w.command_memory(1)
            .unwrap()
            .obligations
            .get(&message)
            .is_none_or(|o| o.status == Status::Failed)
    );
    assert!(w.fulfil_order(1, message).is_err());
    assert_eq!(w.goods, goods);
    assert!(!w.crossing_authorized(0));
    w.audit().unwrap();
}

#[test]
fn receiving_another_offices_message_does_not_grant_the_receivers_authority() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 2, command).unwrap();
    assert!(
        w.command_memory(2)
            .unwrap()
            .envelopes
            .contains_key(&command)
    );
    assert!(w.command_memory(2).unwrap().obligations.is_empty());
    assert!(w.fulfil_order(2, command).is_err());
    assert!(w.refuse_order(2, command, "not my office").is_err());
    assert!(w.command_memory(2).unwrap().accepted_offices.is_empty());
    assert!(!w.crossing_authorized(0));
    w.audit().unwrap();
}

#[test]
fn explicit_refusal_is_terminal_and_does_not_punish_or_execute_the_person() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    let physical = w.agents[1].clone();
    let goods = w.goods.clone();
    w.refuse_order(
        1,
        command,
        "controlled informed choice to decline office duty",
    )
    .unwrap();
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Refused
    );
    assert_eq!(w.agents[1].nutrition, physical.nutrition);
    assert_eq!(w.agents[1].fatigue, physical.fatigue);
    assert_eq!(w.agents[1].location, physical.location);
    assert_eq!(w.goods, goods);
    assert!(w.office_active(0));
    assert!(!w.crossing_authorized(0));
    assert!(w.fulfil_order(1, command).is_err());
    w.receive_command(0, 1, command).unwrap();
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Refused
    );
    w.audit().unwrap();
}

#[test]
fn completing_commission_does_not_complete_construction_or_revoke_private_rights() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    let goods = w.goods.clone();
    let projects = w.infrastructure.projects.clone();
    let strangers = [w.agents[2].clone(), w.agents[3].clone()];
    w.fulfil_order(1, command).unwrap();
    assert!(w.crossing_authorized(0));
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Completed
    );
    assert_eq!(w.infrastructure.projects, projects);
    assert_eq!(w.goods, goods);
    assert_eq!(w.agents[2], strangers[0]);
    assert_eq!(w.agents[3], strangers[1]);
    assert!(w.crossing_projects()[0].completed_at.is_none());
    w.audit().unwrap();
}

#[test]
fn critical_food_need_wins_but_ordinary_self_provision_does_not_erase_duty() {
    let mut c = world().config;
    c.food = vec![(1, 8, 0, 48 * HOUR)];
    let mut w = World::new(c).unwrap();
    offer(&mut w);
    w.accept_office(1, 0).unwrap();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    w.capture_report(1).unwrap();
    w.agents[1].nutrition = 9 * HOUR;
    w.command_before_decision(1);
    assert_eq!(choice(&w, 1).0.intent, Intent::FulfilOrder { command });
    w.agents[1].nutrition = 3 * HOUR;
    w.command_before_decision(1);
    let survival = choice(&w, 1).0;
    assert!(matches!(
        survival.intent,
        Intent::Gather {
            communal: false,
            ..
        }
    ));
    assert_ne!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Refused
    );
    assert!(!w.crossing_authorized(0));
    w.audit().unwrap();
}

#[test]
fn unsupported_module_combinations_are_rejected_before_a_world_is_created() {
    let c = world().config;
    let mut without_information = c.clone();
    without_information.information = false;
    assert!(World::new(without_information).is_err());
    let mut without_authority = c;
    without_authority.authority = None;
    assert!(World::new(without_authority).is_err());
}

#[test]
fn later_revocation_cannot_undo_a_completed_physical_or_institutional_action() {
    let mut w = accepted();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    w.fulfil_order(1, command).unwrap();
    assert!(w.crossing_authorized(0));
    let commissions = w.commissions().to_vec();
    let goods = w.goods.clone();
    let revocation = w.issue_revocation(0, command).unwrap();
    w.receive_command(0, 1, revocation).unwrap();
    assert_eq!(w.commissions(), commissions);
    assert_eq!(w.goods, goods);
    assert!(w.crossing_authorized(0));
    assert!(
        w.command_memory(1).unwrap().obligations[&command]
            .status
            .terminal()
    );
    w.audit().unwrap();
}

#[test]
fn removing_observational_journal_cannot_change_order_arbitration_or_execution() {
    let mut a = accepted();
    let command = order(&mut a);
    a.receive_command(0, 1, command).unwrap();
    let mut b = a.clone();
    b.commands.journal = Journal::default();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(choice(&a, 1), choice(&b, 1));
    a.config.autonomous = true;
    b.config.autonomous = true;
    a.advance_to(3 * MINUTE).unwrap();
    b.advance_to(3 * MINUTE).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.goods, b.goods);
    assert_eq!(a.commands.memories, b.commands.memories);
    a.audit().unwrap();
    b.audit().unwrap();
}

#[test]
fn an_office_certificate_does_not_create_remote_project_knowledge() {
    let mut known = accepted();
    travel(&mut known, 0, 0);
    let mut unknown = known.clone();
    // Same true world and office acceptance. This controlled memory contrast
    // removes only the sovereign's earlier personal site description.
    unknown.commands.memories[0].known_sites.clear();
    let objective = Objective::CommissionCrossing {
        project: 0,
        settlement: 1,
    };
    assert!(
        unknown
            .issue_order(0, 0, objective.clone(), 24 * HOUR)
            .is_err()
    );
    assert!(unknown.command_memory(1).unwrap().obligations.is_empty());
    assert!(known.issue_order(0, 0, objective, 24 * HOUR).is_ok());
    for (a, b) in known.agents.iter().zip(&unknown.agents) {
        assert_same_person_except_event(a, b);
    }
    assert_eq!(known.goods, unknown.goods);
    known.audit().unwrap();
    unknown.audit().unwrap();
}

#[test]
fn unobserved_remote_holder_death_does_not_become_an_issuance_oracle() {
    let mut alive = accepted();
    travel(&mut alive, 0, 0);
    let mut dead = alive.clone();
    // Fatal exposure is an explicit paired physical initial condition. The
    // sovereign is elsewhere and retains precisely the same personal knowledge.
    dead.agents[1].nutrition = 0;
    dead.agents[1].starvation = 24 * HOUR;
    dead.advance_to(HOUR + 1).unwrap();
    alive.advance_to(HOUR + 1).unwrap();
    assert_eq!(dead.agents[1].action, Action::Dead);
    assert_eq!(dead.command_memory(0), alive.command_memory(0));
    assert_eq!(dead.agents[0], alive.agents[0]);
    let command_alive = order(&mut alive);
    let command_dead = order(&mut dead);
    assert_eq!(command_alive, command_dead);
    assert_eq!(alive.command_memory(0), dead.command_memory(0));
    assert!(dead.command_memory(1).unwrap().obligations.is_empty());
    assert!(!dead.office_active(0));
    alive.audit().unwrap();
    dead.audit().unwrap();
}

// The office is accepted at its project site; both the sovereign and steward
// then physically depart. The steward remembers the return road from observation.
fn away_from_office() -> World {
    let mut c = world().config;
    c.edges[0].capacity = 1;
    let mut w = World::new(c).unwrap();
    offer(&mut w);
    w.accept_office(1, 0).unwrap();
    w.observe_command_sites(0).unwrap();
    travel(&mut w, 0, 0);
    travel(&mut w, 1, 0);
    w.capture_report(1).unwrap();
    w
}

#[test]
fn physically_received_revocation_releases_only_the_stewards_queued_route() {
    let mut w = away_from_office();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    // A real earlier traveler occupies the only ordinary road slot.
    w.apply_intents(vec![(
        2,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[2].action, Action::Travel);
    let selected = choice(&w, 1).0;
    assert_eq!(
        selected.intent,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false
        }
    );
    w.record_command_choice(1, &selected);
    w.apply_intents(vec![(1, selected.intent)]).unwrap();
    assert!(matches!(w.agents[1].action, Action::Queued { .. }));
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Active
    );
    let body = w.agents[1].clone();
    let earlier = w.agents[2].clone();
    let revocation = w.issue_revocation(0, command).unwrap();
    assert!(matches!(w.agents[1].action, Action::Queued { .. }));
    w.receive_command(0, 1, revocation).unwrap();
    assert_eq!(w.agents[1].action, Action::Idle);
    assert_eq!(w.agents[1].goal, None);
    assert_eq!(w.agents[1].location, body.location);
    assert_eq!(w.agents[1].nutrition, body.nutrition);
    assert_eq!(w.agents[1].fatigue, body.fatigue);
    assert_eq!(w.agents[2], earlier);
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Revoked
    );
    assert_eq!(w.command_memory(1).unwrap().pursuing, None);
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn expiration_during_traversal_cannot_teleport_the_person_out_of_the_edge() {
    let mut w = away_from_office();
    let start = w.now;
    let command = w
        .issue_order(
            0,
            0,
            Objective::CommissionCrossing {
                project: 0,
                settlement: 1,
            },
            start + HOUR / 2,
        )
        .unwrap();
    w.receive_command(0, 1, command).unwrap();
    let selected = choice(&w, 1).0;
    w.record_command_choice(1, &selected);
    w.apply_intents(vec![(1, selected.intent)]).unwrap();
    assert_eq!(w.agents[1].action, Action::Travel);
    w.advance_to(start + HOUR / 2).unwrap();
    let midpoint = w.agents[1].location.clone();
    w.command_before_decision(1);
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Expired
    );
    assert_eq!(w.agents[1].location, midpoint);
    assert_eq!(w.agents[1].action, Action::Travel);
    assert_eq!(w.agents[1].goal, None);
    w.advance_to(start + HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(1));
    assert_eq!(w.agents[1].action, Action::Idle);
    assert!(w.fulfil_order(1, command).is_err());
    assert!(!w.crossing_authorized(0));
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn critical_hunger_without_a_known_food_source_leaves_the_role_queue_but_keeps_duty() {
    let mut w = away_from_office();
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    w.apply_intents(vec![(
        2,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])
    .unwrap();
    let selected = choice(&w, 1).0;
    w.record_command_choice(1, &selected);
    w.apply_intents(vec![(1, selected.intent)]).unwrap();
    assert!(matches!(w.agents[1].action, Action::Queued { .. }));
    // Controlled physiological checkpoint: no edible source is present or known.
    // Merely returning Wait would leave an applicant admitted into a deadly march.
    w.agents[1].nutrition = 0;
    w.command_before_decision(1);
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Paused
    );
    assert!(!matches!(
        w.agents[1].action,
        Action::Travel | Action::Queued { .. }
    ));
    assert_eq!(w.agents[1].location, Location::Node(0));
    assert_eq!(w.agents[2].action, Action::Travel);
    assert!(!w.crossing_authorized(0));
    let survival = choice(&w, 1).0;
    // Ordinary exploration may independently choose the same neighbor. The
    // role duty cannot compel that choice or retain the old queue application.
    assert!(!matches!(survival.intent, Intent::FulfilOrder { .. }));
    assert_ne!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Refused
    );
    w.advance_to(w.now + HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(0));
    assert_eq!(w.agents[1].nutrition, 0);
    assert!(
        w.command_memory(1)
            .unwrap()
            .obligations
            .contains_key(&command)
    );
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn explanatory_reason_text_does_not_determine_role_undertaking_identity() {
    let mut a = away_from_office();
    let command = order(&mut a);
    a.receive_command(0, 1, command).unwrap();
    let mut b = a.clone();
    let first = choice(&a, 1).0;
    let mut reworded = first.clone();
    reworded.reason = "different diagnostic prose for the same selected typed action".into();
    a.record_command_choice(1, &first);
    b.record_command_choice(1, &reworded);
    assert_eq!(a.command_memory(1).unwrap().pursuing, Some(command));
    assert_eq!(a.command_memory(1), b.command_memory(1));
    a.apply_intents(vec![(1, first.intent)]).unwrap();
    b.apply_intents(vec![(1, reworded.intent)]).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    a.audit().unwrap();
    b.audit().unwrap();
}

#[test]
fn controlled_physical_carriage_can_deliver_an_already_expired_order() {
    let mut w = accepted();
    let command = w
        .issue_order(
            0,
            0,
            Objective::CommissionCrossing {
                project: 0,
                settlement: 1,
            },
            HOUR / 2,
        )
        .unwrap();
    w.receive_command(0, 2, command).unwrap();
    // Explicitly controlled travel and meeting; no claim of autonomous delivery.
    w.apply_intents(vec![
        (
            1,
            Intent::Travel {
                edge: 0,
                target: 0,
                food: false,
            },
        ),
        (
            2,
            Intent::Travel {
                edge: 0,
                target: 0,
                food: false,
            },
        ),
    ])
    .unwrap();
    w.advance_to(HOUR / 2).unwrap();
    assert_eq!(w.agents[2].action, Action::Travel);
    assert!(w.command_memory(1).unwrap().obligations.is_empty());
    assert!(w.receive_command(2, 1, command).is_err());
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(0));
    assert_eq!(w.agents[2].location, Location::Node(0));
    w.receive_command(2, 1, command).unwrap();
    let m = w.command_memory(1).unwrap();
    assert_eq!(m.envelopes[&command].received_at, HOUR);
    assert_eq!(m.obligations[&command].status, Status::Expired);
    assert_eq!(w.command_journal().count("orders_expired_at_receipt"), 1);
    assert_eq!(w.command_journal().count("obligations_formed"), 0);
    assert!(!matches!(
        choice(&w, 1).0.intent,
        Intent::FulfilOrder { .. }
    ));
    assert!(w.fulfil_order(1, command).is_err());
    assert!(!w.crossing_authorized(0));
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn controlled_stale_command_delivery_does_not_refresh_spoiled_food_beliefs() {
    let mut c = world().config;
    // Labeled mechanism initial stock. Food ages under the same spoilage law;
    // nothing modifies the primary experimental worlds or physiological rules.
    c.food = vec![(1, 8, 0, HOUR / 2)];
    let mut w = World::new(c).unwrap();
    offer(&mut w);
    w.accept_office(1, 0).unwrap();
    w.capture_report(0).unwrap();
    w.capture_report(1).unwrap();
    let believed_source = w.agents[1].memory.nodes[&1].clone();
    assert_eq!(believed_source.food[0].quantity, 8);
    let command = order(&mut w);
    w.receive_command(0, 2, command).unwrap();
    w.apply_intents(vec![
        (
            1,
            Intent::Travel {
                edge: 0,
                target: 0,
                food: false,
            },
        ),
        (
            2,
            Intent::Travel {
                edge: 0,
                target: 0,
                food: false,
            },
        ),
    ])
    .unwrap();
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.metrics.spoiled, 8);
    assert_eq!(w.goods.batches[0].material, Material::Waste);
    assert_eq!(w.goods.batches[0].quantity, 8);
    w.receive_command(2, 1, command).unwrap();
    assert_eq!(w.agents[1].memory.nodes[&1], believed_source);
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Received
    );
    assert_eq!(
        w.command_memory(1).unwrap().envelopes[&command].received_at,
        HOUR
    );
    // Use the real Person selector for the role undertaking and commission.
    // The harness supplies when to deliberate; it does not replace the decision.
    let toward_office = choice(&w, 1).0;
    assert_eq!(
        toward_office.intent,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false
        }
    );
    w.record_command_choice(1, &toward_office);
    w.apply_intents(vec![(1, toward_office.intent)]).unwrap();
    w.advance_to(2 * HOUR).unwrap();
    w.capture_report(1).unwrap();
    assert!(
        w.agents[1].memory.nodes[&1]
            .food
            .iter()
            .all(|f| f.quantity == 0 || f.expires_at <= w.now)
    );
    let commission = choice(&w, 1).0;
    assert_eq!(commission.intent, Intent::FulfilOrder { command });
    w.record_command_choice(1, &commission);
    w.apply_intents(vec![(1, commission.intent)]).unwrap();
    assert!(w.crossing_authorized(0));
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Completed
    );
    assert!(w.crossing_projects()[0].completed_at.is_none());
    assert_eq!(w.metrics.spoiled, 8);
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn controlled_late_revocation_travels_after_the_steward_and_cannot_undo_prior_action() {
    let mut w = away_from_office();
    travel(&mut w, 2, 0);
    let start = w.now;
    let command = order(&mut w);
    w.receive_command(0, 1, command).unwrap();
    let departure = choice(&w, 1).0;
    w.record_command_choice(1, &departure);
    w.apply_intents(vec![(1, departure.intent)]).unwrap();
    w.advance_to(start + HOUR / 4).unwrap();
    assert_eq!(w.agents[1].action, Action::Travel);
    let underway = w.agents[1].clone();
    let old_memory = w.command_memory(1).unwrap().clone();
    let revocation = w.issue_revocation(0, command).unwrap();
    w.receive_command(0, 2, revocation).unwrap();
    assert_eq!(w.command_memory(1).unwrap(), &old_memory);
    assert_eq!(w.agents[1], underway);
    assert!(w.receive_command(2, 1, revocation).is_err());
    w.apply_intents(vec![(
        2,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    assert!(matches!(w.agents[2].action, Action::Queued { .. }));
    w.advance_to(start + HOUR).unwrap();
    assert_eq!(w.agents[1].location, Location::Node(1));
    assert_eq!(w.agents[2].action, Action::Travel);
    assert!(w.receive_command(2, 1, revocation).is_err());
    assert!(
        !w.command_memory(1)
            .unwrap()
            .received_revocations
            .contains(&command)
    );
    let local_act = choice(&w, 1).0;
    assert_eq!(local_act.intent, Intent::FulfilOrder { command });
    w.record_command_choice(1, &local_act);
    w.apply_intents(vec![(1, local_act.intent)]).unwrap();
    assert!(w.crossing_authorized(0));
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Completed
    );
    w.advance_to(start + 2 * HOUR).unwrap();
    assert_eq!(w.agents[2].location, Location::Node(1));
    w.receive_command(2, 1, revocation).unwrap();
    assert!(
        w.command_memory(1)
            .unwrap()
            .received_revocations
            .contains(&command)
    );
    assert_eq!(
        w.command_memory(1).unwrap().envelopes[&revocation].received_at,
        start + 2 * HOUR
    );
    assert_eq!(
        w.command_memory(1).unwrap().obligations[&command].status,
        Status::Completed
    );
    assert!(w.crossing_authorized(0));
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn a_known_order_deadline_cancels_the_old_queue_before_fifo_admission() {
    let mut w = away_from_office();
    let traversal_start = w.now;
    w.apply_intents(vec![(
        2,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])
    .unwrap();
    // Align a real traveler's remaining service to 45 seconds. The order's
    // deadline is 30 seconds away; the next ordinary policy turn is 60 seconds
    // away. No change to road, physiology, capacity or FIFO is involved.
    w.advance_to(traversal_start + HOUR - 45_000).unwrap();
    let checkpoint = w.now;
    let command = w
        .issue_order(
            0,
            0,
            Objective::CommissionCrossing {
                project: 0,
                settlement: 1,
            },
            checkpoint + 30_000,
        )
        .unwrap();
    w.receive_command(0, 1, command).unwrap();
    let selected = choice(&w, 1).0;
    w.record_command_choice(1, &selected);
    w.apply_intents(vec![(1, selected.intent)]).unwrap();
    assert!(matches!(w.agents[1].action, Action::Queued { .. }));
    for autonomous in [false, true] {
        let mut direct = w.clone();
        direct.config.autonomous = autonomous;
        direct.transport_research.actors = Some(BTreeSet::from([1]));
        direct.next_policy = checkpoint + MINUTE;
        direct.wake.clear();
        let mut partitioned = direct.clone();
        direct.advance_to(checkpoint + 45_000).unwrap();
        for t in [
            checkpoint + 17_003,
            checkpoint + 30_000,
            checkpoint + 40_001,
            checkpoint + 45_000,
        ] {
            partitioned.advance_to(t).unwrap();
        }
        let duty = &direct.command_memory(1).unwrap().obligations[&command];
        assert_eq!(duty.status, Status::Expired);
        assert_eq!(duty.changed_at, checkpoint + 30_000);
        assert_eq!(direct.command_memory(1).unwrap().pursuing, None);
        assert!(direct.events.iter().any(|e| e.agent == Some(1)
            && e.at == checkpoint + 30_000
            && e.kind == "Interrupted"
            && e.detail.contains("role undertaking ended")));
        if !autonomous {
            // Isolate the old role application: after it expires, FIFO must not
            // admit it into the slot released fifteen seconds later.
            assert_eq!(direct.agents[1].location, Location::Node(0));
            assert_ne!(direct.agents[1].action, Action::Travel);
        } else {
            // A new ordinary exploration choice may legally target this same
            // neighbor. Do not rewrite that historical policy to force stasis.
            assert!(direct.events.iter().any(|e| e.agent == Some(1)
                && e.at == checkpoint + 30_000
                && e.kind == "Decision"));
        }
        assert_eq!(direct.agents[2].location, Location::Node(0));
        assert_eq!(direct.metrics.overtakes, 0);
        assert_eq!(
            direct.state_hash(),
            partitioned.state_hash(),
            "autonomous={autonomous}"
        );
        assert_eq!(
            direct.event_hash(),
            partitioned.event_hash(),
            "autonomous={autonomous}"
        );
        direct.audit().unwrap();
        partitioned.audit().unwrap();
    }
}
