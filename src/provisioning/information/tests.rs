use super::*;

fn world(starts: Vec<usize>) -> World {
    World::new(Config {
        information: true,
        starts,
        nodes: 3,
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
        food: vec![(2, 20, 0, 48 * HOUR)],
        boards: vec![],
        communal: false,
        autonomous: false,
        initial_nutrition: 24 * HOUR,
        initial_fatigue: 0,
        trace: true,
        ..Config::default()
    })
    .unwrap()
}

fn travel(w: &mut World, id: usize, edge: usize, target: usize) {
    w.apply_intents(vec![(
        id,
        Intent::Travel {
            edge,
            target,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(w.now + HOUR).unwrap();
    assert_eq!(w.agents[id].location, Location::Node(target));
}

fn refresh_local(w: &mut World, id: usize) {
    let local = w.observe(id).unwrap().local.unwrap();
    w.agents[id].memory.nodes.insert(local.node, local);
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
fn capture_contains_exact_local_evidence_and_no_remote_observation() {
    let mut w = world(vec![2, 0, 0]);
    let observed = w.observe(0).unwrap().local.unwrap();
    let id = w.capture_report(0).unwrap();
    let receipt = &w.information_memory(0).unwrap().reports[&id];
    assert_eq!(receipt.report.author, 0);
    assert_eq!(receipt.report.observation, observed);
    assert_eq!(receipt.report.observation.observed_at, 0);
    assert_eq!(receipt.received_at, 0);
    assert_eq!(w.agents[0].memory.nodes.len(), 1);
    assert!(w.agents[1].memory.nodes.is_empty());
    assert!(w.information_memory(1).unwrap().reports.is_empty());
    assert_eq!(w.information_journal().count("reports_created"), 1);
    w.audit().unwrap();
}

#[test]
fn unheld_nonlocal_self_and_unknown_handoffs_have_no_receipt_side_effects() {
    let mut w = world(vec![2, 0, 0]);
    let id = w.capture_report(0).unwrap();
    let before = w.state_hash();
    assert!(w.receive_report(0, 1, id).is_err());
    assert!(w.receive_report(1, 2, id).is_err());
    assert!(w.receive_report(0, 0, id).is_err());
    assert!(w.receive_report(0, 99, id).is_err());
    assert!(w.receive_report(99, 0, id).is_err());
    assert_eq!(before, w.state_hash());
    assert_eq!(w.information_journal().count("handoffs"), 0);
    assert!(w.information_memory(2).unwrap().reports.is_empty());
    w.audit().unwrap();
}

#[test]
fn real_travel_prevents_capture_and_communication_until_node_contact() {
    let mut w = world(vec![2, 2, 2]);
    let id = w.capture_report(0).unwrap();
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 1,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[0].action, Action::Travel);
    assert!(w.capture_report(0).is_err());
    assert!(w.receive_report(0, 1, id).is_err());
    w.advance_to(HOUR).unwrap();
    assert!(w.receive_report(0, 1, id).is_err());
    w.apply_intents(vec![(
        1,
        Intent::Travel {
            edge: 1,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    assert!(w.receive_report(0, 1, id).is_err());
    w.advance_to(2 * HOUR).unwrap();
    w.receive_report(0, 1, id).unwrap();
    let receipt = &w.information_memory(1).unwrap().reports[&id];
    assert_eq!(receipt.report.observation.node, 2);
    assert_eq!(receipt.report.observation.observed_at, 0);
    assert_eq!(receipt.received_at, 2 * HOUR);
    assert_eq!(w.metrics.overtakes, 0);
    w.audit().unwrap();
}

#[test]
fn actual_source_depletion_does_not_mutate_an_old_report() {
    let mut w = world(vec![2, 2, 2]);
    let id = w.capture_report(0).unwrap();
    let original = w.information_memory(0).unwrap().reports[&id].report.clone();
    let batch = original.observation.food[0].batch;
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch,
            communal: false,
        },
    )])
    .unwrap();
    w.advance_to(MINUTE).unwrap();
    assert_eq!(w.goods.batches[batch].quantity, 19);
    let goods = w.goods.clone();
    let recipient = w.agents[1].clone();
    w.receive_report(0, 1, id).unwrap();
    assert_eq!(
        w.information_memory(1).unwrap().reports[&id].report,
        original
    );
    assert_eq!(w.agents[1].memory.nodes[&2].food[0].quantity, 20);
    assert_eq!(w.goods, goods);
    assert_eq!(w.agents[1].nutrition, recipient.nutrition);
    assert_eq!(w.agents[1].location, recipient.location);
    assert_eq!(w.agents[1].action, recipient.action);
    assert_eq!(w.agents[1].rng, recipient.rng);
    w.audit().unwrap();
}

#[test]
fn older_report_does_not_overwrite_newer_direct_evidence() {
    let mut w = world(vec![2, 2, 2]);
    let old = w.capture_report(0).unwrap();
    let batch = w.information_memory(0).unwrap().reports[&old]
        .report
        .observation
        .food[0]
        .batch;
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch,
            communal: false,
        },
    )])
    .unwrap();
    w.advance_to(MINUTE).unwrap();
    refresh_local(&mut w, 1);
    let direct = w.agents[1].memory.nodes[&2].clone();
    w.receive_report(0, 1, old).unwrap();
    assert_eq!(w.agents[1].memory.nodes[&2], direct);
    assert_eq!(direct.food[0].quantity, 19);
    assert_eq!(
        w.information_memory(1).unwrap().reports[&old]
            .report
            .observation
            .food[0]
            .quantity,
        20
    );
    assert!(
        !w.information_journal()
            .handoffs
            .last()
            .unwrap()
            .spatial_belief_updated
    );
    w.audit().unwrap();
}

#[test]
fn equal_timestamp_local_evidence_is_retained() {
    let mut w = world(vec![2, 2, 2]);
    let report = w.capture_report(0).unwrap();
    // Gathering reserves one real quantum immediately. A second local observation
    // at the same clock time sees that reservation; a previous report must not undo it.
    let batch = w.information_memory(0).unwrap().reports[&report]
        .report
        .observation
        .food[0]
        .batch;
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch,
            communal: false,
        },
    )])
    .unwrap();
    refresh_local(&mut w, 1);
    let direct = w.agents[1].memory.nodes[&2].clone();
    assert_eq!(direct.food[0].available, 19);
    w.receive_report(0, 1, report).unwrap();
    assert_eq!(w.agents[1].memory.nodes[&2], direct);
    w.audit().unwrap();
}

#[test]
fn out_of_order_reports_relay_and_duplicates_never_freshen_the_observation() {
    let mut w = world(vec![2, 2, 2]);
    let old = w.capture_report(0).unwrap();
    w.advance_to(MINUTE).unwrap();
    let newer = w.capture_report(0).unwrap();
    w.receive_report(0, 1, newer).unwrap();
    w.receive_report(0, 1, old).unwrap();
    assert_eq!(w.agents[1].memory.nodes[&2].observed_at, MINUTE);
    let first = w.information_memory(1).unwrap().reports[&old].clone();
    w.advance_to(2 * MINUTE).unwrap();
    w.receive_report(0, 1, old).unwrap();
    assert_eq!(w.information_memory(1).unwrap().reports[&old], first);
    w.receive_report(1, 2, old).unwrap();
    let relayed = &w.information_memory(2).unwrap().reports[&old];
    assert_eq!(relayed.report.author, 0);
    assert_eq!(relayed.sender, 1);
    assert_eq!(relayed.report.observation.observed_at, 0);
    assert_eq!(relayed.received_at, 2 * MINUTE);
    assert_eq!(w.agents[2].memory.nodes[&2].observed_at, 0);
    assert_eq!(w.information_journal().count("duplicate_handoffs"), 1);
    w.audit().unwrap();
}

#[test]
fn identical_physical_receiver_can_choose_a_different_route_after_local_receipt() {
    let mut w = world(vec![2, 0, 0]);
    let report = w.capture_report(0).unwrap();
    travel(&mut w, 0, 1, 1);
    travel(&mut w, 0, 0, 0);
    refresh_local(&mut w, 1);
    let before = w.clone();
    let without = choice(&before, 1);
    w.receive_report(0, 1, report).unwrap();
    let with = choice(&w, 1);
    assert_ne!(without.0.intent, with.0.intent);
    assert!(matches!(
        with.0.intent,
        Intent::Travel {
            edge: 0,
            target: 2,
            food: true
        }
    ));
    assert_eq!(w.agents[1].nutrition, before.agents[1].nutrition);
    assert_eq!(w.agents[1].fatigue, before.agents[1].fatigue);
    assert_eq!(w.agents[1].location, before.agents[1].location);
    assert_eq!(w.agents[1].action, before.agents[1].action);
    assert_eq!(w.agents[1].rng, before.agents[1].rng);
    assert_eq!(w.goods, before.goods);
    w.audit().unwrap();
}

#[test]
fn remote_world_changes_do_not_refresh_the_report_or_receiver_choice() {
    let mut a = world(vec![2, 0, 0]);
    let report = a.capture_report(0).unwrap();
    travel(&mut a, 0, 1, 1);
    travel(&mut a, 0, 0, 0);
    refresh_local(&mut a, 1);
    a.receive_report(0, 1, report).unwrap();
    let mut b = a.clone();
    // Research-only counterfactual remote truth. Held reports and recipient inputs
    // stay unchanged; no trajectory/accounting claim is made for this mutation.
    b.goods.batches[0].quantity = 0;
    b.config.edges[1].capacity = 0;
    assert_eq!(choice(&a, 1), choice(&b, 1));
    assert_eq!(a.information_memory(1), b.information_memory(1));
    assert_eq!(a.agents[1].memory, b.agents[1].memory);
}

#[test]
fn delivery_purpose_creates_no_route_and_cannot_track_an_absent_recipient() {
    let mut a = world(vec![2, 0, 0]);
    let report = a.capture_report(0).unwrap();
    a.begin_report_delivery(0, report, 1, 0).unwrap();
    assert_eq!(a.agents[0].memory.nodes.len(), 1);
    let before = choice(&a, 0);
    assert!(before.0.reason.contains("PAUSED_NO_KNOWN_ROUTE"));
    let mut b = a.clone();
    b.agents[1].location = Location::Node(1);
    b.agents[1].nutrition = 0;
    assert_eq!(before, choice(&b, 0));
    travel(&mut a, 0, 1, 1);
    travel(&mut a, 0, 0, 0);
    // Recipient's departure is a controlled initial-state contrast. The carrier
    // only sees absence here; it cannot infer the recipient's new destination.
    a.agents[1].location = Location::Node(2);
    let d = choice(&a, 0).0;
    assert_eq!(d.intent, Intent::CancelReportDelivery);
    assert!(d.reason.contains("RENDEZVOUS_RECIPIENT_ABSENT"));
    a.apply_intents(vec![(0, d.intent)]).unwrap();
    assert!(a.information_memory(0).unwrap().delivery.is_none());
    assert!(
        a.information_memory(0)
            .unwrap()
            .reports
            .contains_key(&report)
    );
    assert!(a.information_memory(1).unwrap().reports.is_empty());
    a.audit().unwrap();
}

#[test]
fn death_blocks_capture_and_handoff_without_inheriting_knowledge() {
    let mut w = world(vec![2, 2, 2]);
    let report = w.capture_report(0).unwrap();
    w.receive_report(0, 1, report).unwrap();
    let remote_before = w.information_memory(2).unwrap().clone();
    // Fatal exposure is a labeled primitive-test initial condition. Ordinary
    // Person death performs the transition and no information is inherited.
    w.agents[0].nutrition = 0;
    w.agents[0].starvation = 24 * HOUR;
    w.advance_to(1).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert!(w.capture_report(0).is_err());
    assert!(w.receive_report(0, 2, report).is_err());
    assert!(w.receive_report(1, 0, report).is_err());
    assert_eq!(w.information_memory(2).unwrap(), &remote_before);
    // A previously received copy remains usable; it is not inherited from death.
    w.receive_report(1, 2, report).unwrap();
    assert_eq!(
        w.information_memory(2).unwrap().reports[&report]
            .report
            .author,
        0
    );
    assert_eq!(w.information_memory(2).unwrap().reports[&report].sender, 1);
    w.audit().unwrap();
}

#[test]
fn birth_extends_empty_information_state_without_parental_report_inheritance() {
    let mut config = lifecycle::research::fixture("basic", 0).unwrap().config;
    config.information = true;
    config.autonomous = false;
    let mut w = World::new(config).unwrap();
    let report = w.capture_report(0).unwrap();
    w.receive_report(0, 1, report).unwrap();
    w.apply_intents(vec![
        (0, Intent::Reproduce { partner: 1 }),
        (1, Intent::Reproduce { partner: 0 }),
    ])
    .unwrap();
    w.advance_to(9 * HOUR).unwrap();
    assert_eq!(w.agents.len(), 3);
    assert_eq!(w.information_memory(2), Some(&Memory::default()));
    assert!(w.agents[2].memory.nodes.is_empty());
    assert_eq!(w.biology(2).unwrap().parents, Some([0, 1]));
    w.audit().unwrap();
}

#[test]
fn sorted_simultaneous_handoffs_reproduce_under_reversed_submission() {
    let mut a = world(vec![2, 2, 2]);
    let first = a.capture_report(0).unwrap();
    let second = a.capture_report(1).unwrap();
    let mut b = a.clone();
    let mut intents = vec![
        (
            0,
            Intent::ShareReport {
                report: first,
                recipient: 2,
            },
        ),
        (
            1,
            Intent::ShareReport {
                report: second,
                recipient: 2,
            },
        ),
    ];
    a.apply_intents(intents.clone()).unwrap();
    intents.reverse();
    b.apply_intents(intents).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    assert_eq!(a.information_memory(2).unwrap().reports.len(), 2);
    a.audit().unwrap();
}

#[test]
fn journal_queries_and_disabled_information_do_not_create_permissions_or_state() {
    let mut disabled = World::new(Config {
        autonomous: false,
        ..Config::default()
    })
    .unwrap();
    let hash = disabled.state_hash();
    assert!(disabled.capture_report(0).is_err());
    assert!(disabled.receive_report(0, 1, 0).is_err());
    assert!(disabled.begin_report_delivery(0, 0, 1, 0).is_err());
    assert_eq!(disabled.state_hash(), hash);
    assert!(disabled.information_memory(0).is_none());
    let mut w = world(vec![2, 2, 2]);
    let report = w.capture_report(0).unwrap();
    let physical_goods = w.goods.clone();
    w.receive_report(0, 1, report).unwrap();
    let before = w.state_hash();
    for _ in 0..10 {
        let _ = w.information_memory(1);
        let _ = w.information_journal();
    }
    assert_eq!(w.state_hash(), before);
    assert_eq!(w.goods, physical_goods);
    assert!(w.offices().is_empty());
    assert!(w.units().is_empty());
    assert!(w.agents[1].job.is_none());
    w.audit().unwrap();
}

#[test]
fn receipt_and_delivery_start_obey_policy_time_under_irregular_caller_partitions() {
    for begin_delivery in [false, true] {
        let mut a = world(vec![0, 0, 0]);
        for edge in &mut a.config.edges {
            edge.capacity = 0;
        }
        a.config.autonomous = true;
        a.advance_to(MINUTE / 2).unwrap();
        let report = a.capture_report(0).unwrap();
        if begin_delivery {
            a.begin_report_delivery(0, report, 1, 0).unwrap();
        } else {
            a.receive_report(0, 1, report).unwrap();
        }
        let mut b = a.clone();
        a.advance_to(3 * MINUTE).unwrap();
        for t in [
            MINUTE / 2 + 1,
            MINUTE - 7,
            MINUTE + 13,
            2 * MINUTE + 71,
            3 * MINUTE,
        ] {
            b.advance_to(t).unwrap();
        }
        assert_eq!(
            a.state_hash(),
            b.state_hash(),
            "begin_delivery={begin_delivery}"
        );
        assert_eq!(
            a.event_hash(),
            b.event_hash(),
            "begin_delivery={begin_delivery}"
        );
        // The new undertaking/report cannot trigger an early action merely
        // because an external researcher requested an intermediate checkpoint.
        assert!(
            !b.events
                .iter()
                .any(|e| { e.kind == "Decision" && e.at > MINUTE / 2 && e.at < MINUTE })
        );
        a.audit().unwrap();
        b.audit().unwrap();
    }
}

#[test]
fn already_selected_local_report_handoff_is_not_replaced_by_barter() {
    let mut w = barter::research::world("colocated", true, 0, true).unwrap();
    w.config.information = true;
    w.information = State::new(&w.config);
    w.config.autonomous = false;
    w.agents[0].nutrition = 24 * HOUR;
    w.observe_farming(0);
    w.observe_capital(0);
    w.observe_infrastructure(0);
    let report = w.capture_report(0).unwrap();
    w.begin_report_delivery(0, report, 1, 0).unwrap();
    let selected = choice(&w, 0).0;
    assert_eq!(
        selected.intent,
        Intent::ShareReport {
            report,
            recipient: 1
        }
    );
    // Witness that a real conflicting barter opportunity exists. The ordinary
    // exchange stage would replace this selected step if the runtime forgot to
    // preserve the already accepted courier undertaking's chosen action.
    let mut probe = w.clone();
    let observation = probe.observe(0).unwrap();
    let competing = probe.barter_decision(0, &observation, selected);
    assert!(matches!(competing.intent, Intent::OfferBarter { .. }));
    w.config.autonomous = true;
    w.advance_to(0).unwrap();
    assert!(
        w.information_memory(1)
            .unwrap()
            .reports
            .contains_key(&report)
    );
    assert!(w.information_memory(0).unwrap().delivery.is_none());
    assert!(!w.barter.offers.iter().any(|offer| offer.proposer == 0));
    w.audit().unwrap();
}

#[test]
fn active_report_provenance_clears_on_direct_capture_and_runtime_observation() {
    for explicit_capture in [true, false] {
        let mut w = world(vec![2, 0, 0]);
        let report = w.capture_report(0).unwrap();
        let second_old_report = w.capture_report(0).unwrap();
        travel(&mut w, 0, 1, 1);
        travel(&mut w, 0, 0, 0);
        w.receive_report(0, 1, report).unwrap();
        assert_eq!(
            w.information_memory(1).unwrap().reported_nodes.get(&2),
            Some(&report)
        );
        let receipt = w.information_memory(1).unwrap().reports[&report].clone();
        w.receive_report(0, 1, report).unwrap();
        assert_eq!(
            w.information_memory(1).unwrap().reported_nodes.get(&2),
            Some(&report)
        );
        assert_eq!(w.information_memory(1).unwrap().reports[&report], receipt);

        travel(&mut w, 1, 0, 1);
        travel(&mut w, 1, 1, 2);
        // Merely reading the body/location or reaching a node in a manual control
        // is not a new cognitive observation. Exercise both legitimate refreshes.
        assert!(
            w.information_memory(1)
                .unwrap()
                .reported_nodes
                .contains_key(&2)
        );
        if explicit_capture {
            w.capture_report(1).unwrap();
        } else {
            w.config.autonomous = true;
            w.next_policy = w.now + MINUTE;
            w.wake.clear();
            w.advance_to(w.now + MINUTE).unwrap();
            w.config.autonomous = false;
        }
        assert!(
            !w.information_memory(1)
                .unwrap()
                .reported_nodes
                .contains_key(&2)
        );
        assert_eq!(w.information_memory(1).unwrap().reports[&report], receipt);
        let fresh = w.agents[1].memory.nodes[&2].clone();
        assert!(fresh.observed_at > receipt.report.observation.observed_at);

        // The original author physically catches up; neither duplicate receipt nor
        // an unreceived older report may turn direct evidence back into hearsay.
        travel(&mut w, 0, 0, 1);
        travel(&mut w, 0, 1, 2);
        w.receive_report(0, 1, report).unwrap();
        w.receive_report(0, 1, second_old_report).unwrap();
        assert!(
            !w.information_memory(1)
                .unwrap()
                .reported_nodes
                .contains_key(&2)
        );
        assert_eq!(w.agents[1].memory.nodes[&2], fresh);
        assert_eq!(w.information_memory(1).unwrap().reports[&report], receipt);
        assert!(
            w.information_memory(1)
                .unwrap()
                .reports
                .contains_key(&second_old_report)
        );
        w.audit().unwrap();
    }
}

#[test]
fn discarding_research_journal_does_not_change_courier_or_receiver_execution() {
    let mut a = research::fixture("short", true, 0).unwrap();
    a.advance_to(MINUTE).unwrap();
    assert!(!a.information.journal.counts.is_empty());
    let mut b = a.clone();
    b.information.journal = Journal::default();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    a.advance_to(8 * HOUR).unwrap();
    b.advance_to(8 * HOUR).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.goods, b.goods);
    assert_eq!(a.information.memories, b.information.memories);
    a.audit().unwrap();
    b.audit().unwrap();
}

#[test]
fn cancellation_without_information_or_an_undertaking_rejects_without_panicking() {
    let mut disabled = World::new(Config {
        autonomous: false,
        trace: true,
        ..Config::default()
    })
    .unwrap();
    disabled
        .apply_intents(vec![(0, Intent::CancelReportDelivery)])
        .unwrap();
    assert!(disabled.events.iter().any(|e| {
        e.kind == "Rejected" && e.detail.contains("information institution disabled")
    }));
    assert_eq!(disabled.agents[0].action, Action::Idle);
    assert_eq!(
        disabled.information_journal().count("deliveries_cancelled"),
        0
    );

    let mut available = world(vec![2, 0, 0]);
    available
        .apply_intents(vec![(0, Intent::CancelReportDelivery)])
        .unwrap();
    assert!(
        available.events.iter().any(|e| {
            e.kind == "Rejected" && e.detail.contains("no report delivery undertaking")
        })
    );
    assert_eq!(
        available
            .information_journal()
            .count("deliveries_cancelled"),
        0
    );
    disabled.audit().unwrap();
    available.audit().unwrap();
}

#[test]
fn cancellation_distinguishes_explicit_choice_from_missing_report() {
    for lose_report in [false, true] {
        let mut w = world(vec![2, 0, 0]);
        let report = w.capture_report(0).unwrap();
        w.begin_report_delivery(0, report, 1, 0).unwrap();
        if lose_report {
            // Adversarial damaged-memory fixture only: no normal mechanic removes
            // this held report. Cancellation must diagnose rather than panic.
            w.information.memories[0].reports.remove(&report);
        }
        w.apply_intents(vec![(0, Intent::CancelReportDelivery)])
            .unwrap();
        assert!(w.information_memory(0).unwrap().delivery.is_none());
        let expected = if lose_report {
            "REPORT_NOT_HELD"
        } else {
            "EXPLICIT_CANCEL"
        };
        let cancellations: Vec<_> = w
            .events
            .iter()
            .filter(|e| e.kind == "ReportDeliveryCancelled")
            .collect();
        assert_eq!(cancellations.len(), 1);
        assert!(
            cancellations[0]
                .detail
                .contains(&format!("reason={expected}"))
        );
        assert!(
            !cancellations[0]
                .detail
                .contains("RENDEZVOUS_RECIPIENT_ABSENT")
        );
        assert_eq!(
            w.information_memory(0)
                .unwrap()
                .reports
                .contains_key(&report),
            !lose_report
        );
        assert!(w.information_memory(1).unwrap().reports.is_empty());
        w.audit().unwrap();
    }
}
