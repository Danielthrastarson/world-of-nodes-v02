use super::*;
fn world() -> World {
    World::new(super::super::Config {
        authority: Some(Config {
            polity_id: 42,
            sovereign: 0,
            settlements: vec![0, 1],
        }),
        nodes: 2,
        starts: vec![0; 3],
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: HOUR,
            capacity: 4,
        }],
        food: vec![],
        boards: vec![0],
        autonomous: false,
        trace: true,
        infrastructure: Some(infrastructure::Config {
            enabled: true,
            wood: vec![(0, 10)],
            sites: vec![infrastructure::Site {
                edge: 0,
                node: 0,
                wood_cost: 1,
                work_ms: HOUR,
                opened_nominal_ms: HOUR / 2,
                opened_capacity: 4,
            }],
        }),
        ..super::super::Config::default()
    })
    .unwrap()
}
fn appoint(w: &mut World, holder: usize, node: usize) {
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: holder,
            settlement: node,
        },
    )])
    .unwrap();
}
fn go(w: &mut World, id: usize, to: usize) {
    w.apply_intents(vec![(
        id,
        Intent::Travel {
            edge: 0,
            target: to,
            food: false,
        },
    )])
    .unwrap();
}
#[test]
fn appointment_is_local_and_does_not_assign_action_or_knowledge() {
    let mut w = world();
    let candidate = w.agents[1].clone();
    assert!(w.appoint_steward(2, 1, 0).is_err());
    assert!(w.appoint_steward(0, 1, 99).is_err());
    appoint(&mut w, 1, 1);
    assert_eq!(candidate, w.agents[1]);
    assert_eq!(w.known_appointments(1).len(), 1);
    assert!(w.known_appointments(2).is_empty());
    assert_eq!(w.polity().unwrap().id, 42);
    assert_eq!(w.offices()[0].id, 0);
    assert!(w.administered_settlements().is_empty());
    assert!(w.appoint_steward(0, 1, 1).is_err());
    go(&mut w, 1, 1);
    assert_eq!(w.known_appointments(1)[0].settlement, 1);
    assert!(w.appoint_steward(0, 1, 0).is_err());
    w.advance_to(HOUR).unwrap();
    assert!(w.appoint_steward(0, 1, 0).is_err());
    assert!(w.office_active(0));
    assert_eq!(w.administered_settlements(), BTreeSet::from([1]));
}
#[test]
fn death_retains_records_and_removes_actual_authority_at_exact_boundary() {
    let mut w = world();
    appoint(&mut w, 1, 0);
    w.advance_to(36 * HOUR - 1).unwrap();
    assert!(w.office_active(0));
    w.advance_to(36 * HOUR).unwrap();
    assert!(!w.office_active(0));
    assert_eq!(w.offices()[0].status, OfficeStatus::Appointed);
    assert!(w.polity().is_some());
    assert_eq!(w.authority_journal().active_ms[&0], (36 * HOUR) as u128);
    assert_eq!(
        w.authority_journal().transitions.last().unwrap().reason,
        "HOLDER_DIED"
    );
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: 2,
            settlement: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.offices().len(), 1);
}
#[test]
fn sovereign_death_does_not_revoke_a_living_steward() {
    let mut w = world();
    appoint(&mut w, 1, 0);
    // Explicit physiological test initial condition, not a research intervention.
    w.agents[1].nutrition = 24 * HOUR;
    w.advance_to(36 * HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert!(w.office_active(0));
    assert!(w.appoint_steward(0, 2, 1).is_err());
    assert!(w.commission_crossing(1, 0, 0).is_ok());
}
#[test]
fn departure_and_return_derive_territory_without_duplicate_hours() {
    let mut w = world();
    appoint(&mut w, 1, 0);
    appoint(&mut w, 2, 0);
    w.advance_to(HOUR).unwrap();
    go(&mut w, 1, 1);
    assert!(!w.office_active(0));
    assert!(w.office_active(1));
    w.advance_to(2 * HOUR).unwrap();
    assert_eq!(w.authority_journal().active_ms[&0], (2 * HOUR) as u128);
    go(&mut w, 2, 1);
    assert!(w.administered_settlements().is_empty());
    go(&mut w, 1, 0);
    w.advance_to(3 * HOUR).unwrap();
    assert!(w.office_active(0));
    assert_eq!(w.authority_journal().active_ms[&0], (2 * HOUR) as u128);
}
#[test]
fn local_observation_does_not_reveal_remote_office_status() {
    let mut w = world();
    appoint(&mut w, 1, 1);
    let local = w.observe_authority(0).unwrap();
    go(&mut w, 1, 1);
    w.advance_to(HOUR).unwrap();
    assert!(w.office_active(0));
    assert_eq!(
        local.present_offices,
        w.observe_authority(0).unwrap().present_offices
    );
    assert_eq!(w.observe_authority(1).unwrap().present_offices.len(), 1);
    assert!(!w.agents[0].memory.nodes.contains_key(&1));
}
#[test]
fn commission_requires_local_holder_and_never_creates_material_or_work() {
    let mut w = world();
    appoint(&mut w, 1, 0);
    assert!(w.commission_crossing(2, 0, 0).is_err());
    let goods = w.goods.clone();
    let projects = w.infrastructure.projects.clone();
    let memories: Vec<_> = w.agents.iter().map(|a| a.memory.clone()).collect();
    w.apply_intents(vec![(
        1,
        Intent::CommissionCrossing {
            office: 0,
            project: 0,
        },
    )])
    .unwrap();
    assert_eq!(w.goods, goods);
    assert_eq!(w.infrastructure.projects, projects);
    assert_eq!(
        memories,
        w.agents
            .iter()
            .map(|a| a.memory.clone())
            .collect::<Vec<_>>()
    );
    assert!(w.commission_crossing(1, 0, 0).is_err());
    go(&mut w, 1, 1);
    assert!(!w.office_active(0));
    assert!(w.crossing_authorized(0));
    assert_eq!(w.observe_authority(0).unwrap().posted_commissions.len(), 1);
}
#[test]
fn public_objective_requires_commission_but_not_forced_participation() {
    let mut w = world();
    assert!(w.infrastructure_view(0, 2).projects.is_empty());
    w.apply_intents(vec![(
        2,
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    )])
    .unwrap();
    while matches!(w.agents[2].action, Action::Gather { .. }) {
        w.advance_to(w.next_time(w.now + HOUR)).unwrap();
    }
    let batch = w
        .batches()
        .iter()
        .find(|b| b.owner == Owner::Agent(2) && b.quantity > 0)
        .unwrap()
        .id;
    assert!(w.donate_crossing(2, 0, batch, 1).is_err());
    assert!(w.begin_crossing(2, 0).is_err());
    appoint(&mut w, 1, 0);
    w.apply_intents(vec![(
        1,
        Intent::CommissionCrossing {
            office: 0,
            project: 0,
        },
    )])
    .unwrap();
    assert_eq!(w.infrastructure_view(0, 2).projects.len(), 1);
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.crossing_projects()[0].wood, 0);
    w.apply_intents(vec![(
        2,
        Intent::ContributeCrossing {
            project: 0,
            batch,
            quantity: 1,
        },
    )])
    .unwrap();
    w.apply_intents(vec![(2, Intent::BuildCrossing { project: 0 })])
        .unwrap();
    w.advance_to(3 * HOUR).unwrap();
    assert!(w.crossing_projects()[0].completed_at.is_some());
    assert_eq!(w.crossing_projects()[0].owner, Owner::Commons(0));
    assert_eq!(w.effective_edge(0).nominal_ms, HOUR / 2);
    w.audit().unwrap();
}
#[test]
fn one_holder_cannot_administer_two_locations_at_once() {
    let mut w = world();
    appoint(&mut w, 1, 0);
    appoint(&mut w, 1, 1);
    assert_eq!(w.administered_settlements(), BTreeSet::from([0]));
    go(&mut w, 1, 1);
    assert!(w.administered_settlements().is_empty());
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.administered_settlements(), BTreeSet::from([1]));
}
#[test]
fn observation_journal_cannot_change_decisions_or_hashes() {
    let mut a = world();
    appoint(&mut a, 1, 0);
    let mut b = a.clone();
    b.authority.journal = Journal::default();
    a.advance_to(4 * HOUR).unwrap();
    b.advance_to(4 * HOUR).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn invalid_configuration_is_rejected() {
    let mut c = super::super::Config {
        authority: Some(Config {
            polity_id: 0,
            sovereign: 50,
            settlements: vec![0],
        }),
        ..super::super::Config::default()
    };
    assert!(World::new(c.clone()).is_err());
    c.authority.as_mut().unwrap().sovereign = 0;
    c.authority.as_mut().unwrap().settlements = vec![0, 0];
    assert!(World::new(c).is_err());
}
