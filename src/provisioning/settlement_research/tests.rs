use super::*;
fn manual(life: Time) -> World {
    let mut w = World::new(Config {
        starts: vec![0, 0],
        autonomous: false,
        trace: true,
        food: vec![(0, 10, 0, life)],
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
        ..Config::default()
    })
    .unwrap();
    w.enable_settlement_research(vec![0, CORRIDOR, 1]).unwrap();
    w
}
fn gather(w: &mut World, q: u64) {
    for _ in 0..q {
        w.apply_intents(vec![(
            0,
            Intent::Gather {
                batch: 0,
                communal: false,
            },
        )])
        .unwrap();
        w.advance_to(w.now + 2 * SECOND).unwrap();
    }
}
fn travel(w: &mut World, edge: usize, target: usize) {
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge,
            target,
            food: false,
        },
    )])
    .unwrap();
    while w.agents[0].action == Action::Travel {
        let at = w.next_time(w.now + HOUR);
        w.advance_to(at).unwrap();
    }
}
#[test]
fn journal_never_changes_physics_events_policy_or_rng() {
    let mut c = configuration("paths", 7, true).unwrap();
    c.starts = vec![0, 2, 0, 2];
    c.food = vec![(0, 24, 0, 48 * HOUR), (2, 24, 0, 48 * HOUR)];
    c.farming.as_mut().unwrap().units = vec![0, 0, 2, 2];
    let mut plain = World::new(c).unwrap();
    let mut observed = plain.clone();
    observed
        .enable_settlement_research(vec![0, CORRIDOR, 1])
        .unwrap();
    for hour in 0..=72 {
        plain.advance_to(hour * HOUR).unwrap();
        observed.advance_to(hour * HOUR).unwrap();
        observed.audit_settlement_research().unwrap();
        assert_eq!(plain.state_hash(), observed.state_hash());
        assert_eq!(plain.event_hash(), observed.event_hash());
    }
    assert_eq!(plain.events, observed.events);
}
#[test]
fn arrived_flow_excludes_food_eaten_at_corridor_and_retains_ownership() {
    let mut w = manual(48 * HOUR);
    gather(&mut w, 5);
    travel(&mut w, 0, 2);
    assert!(w.settlement_journal().flows.is_empty());
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(0) && b.quantity > 0)
        .unwrap()
        .id;
    w.apply_intents(vec![(
        0,
        Intent::Eat {
            batch: b,
            quantity: 2,
        },
    )])
    .unwrap();
    travel(&mut w, 1, 2);
    let j = w.settlement_journal();
    assert_eq!(j.flows.len(), 1);
    assert_eq!(
        (
            j.flows[0].from,
            j.flows[0].to,
            j.flows[0].quantity,
            j.flows[0].owner
        ),
        (0, 1, 3, Owner::Agent(0))
    );
    assert!(
        j.transfers
            .iter()
            .all(|t| t.before == Owner::None && t.after == Owner::Agent(0))
    );
    assert_eq!(j.stats[&(CORRIDOR, "consumed_food".into())], 2);
    w.audit_settlement_research().unwrap();
}
#[test]
fn spoiled_transit_cargo_is_waste_flow_not_delivered_food() {
    let mut w = manual(HOUR);
    gather(&mut w, 1);
    travel(&mut w, 0, 2);
    travel(&mut w, 1, 2);
    let flow = &w.settlement_journal().flows[0];
    assert_eq!(
        (flow.material, flow.quantity, flow.owner),
        (Material::Waste, 1, Owner::Agent(0))
    );
    assert_eq!(w.metrics.delivered, 0);
    w.audit_settlement_research().unwrap();
}

#[test]
fn arrival_exactly_at_expiry_is_not_edible_flow() {
    let mut w = manual(48 * HOUR);
    w.settlement_research.as_mut().unwrap().node_places[1] = 1;
    w.goods.add(
        Material::Food,
        1,
        Owner::Agent(0),
        Holder::Agent(0),
        0,
        HOUR,
    );
    travel(&mut w, 0, 1);
    assert_eq!(w.now, HOUR);
    assert_eq!(w.settlement_journal().flows[0].material, Material::Waste);
    w.audit_settlement_research().unwrap();
}
#[test]
fn research_place_labels_do_not_change_decisions() {
    let mut c = configuration("complementary", 9, false).unwrap();
    c.starts = vec![0, 2];
    let mut a = World::new(c).unwrap();
    let mut b = a.clone();
    a.enable_settlement_research(vec![0, CORRIDOR, 1]).unwrap();
    b.enable_settlement_research(vec![1, CORRIDOR, 0]).unwrap();
    a.advance_to(20 * HOUR).unwrap();
    b.advance_to(20 * HOUR).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn other_owners_food_is_inaccessible_but_own_food_can_enter_legal_communal_job() {
    let mut w = manual(48 * HOUR);
    let batch = w.goods.add(
        Material::Food,
        3,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    w.apply_intents(vec![(1, Intent::Pickup { batch, quantity: 3 })])
        .unwrap();
    assert_eq!(w.goods.batches[batch].quantity, 3);
    assert_eq!(w.goods.load(1), 0);
    assert!(
        w.events
            .last()
            .unwrap()
            .detail
            .contains("ownership forbids access")
    );
    // A test job is obtained through actual board presence, never remote claiming.
    w.boards.push(Board {
        node: 2,
        quantity: 3,
        created_at: 0,
        refreshed_at: 0,
        claims: vec![],
    });
    travel(&mut w, 0, 2);
    travel(&mut w, 1, 2);
    w.apply_intents(vec![(
        0,
        Intent::Claim {
            board: 2,
            quantity: 3,
        },
    )])
    .unwrap();
    travel(&mut w, 1, 0);
    travel(&mut w, 0, 0);
    w.apply_intents(vec![(0, Intent::LoadCargo { batch, quantity: 3 })])
        .unwrap();
    let t = w.settlement_journal().transfers.last().unwrap();
    assert_eq!(
        (t.before, t.after, t.quantity),
        (Owner::Agent(0), Owner::Commons(0), 3)
    );
    travel(&mut w, 0, 2);
    travel(&mut w, 1, 2);
    assert_eq!(
        w.settlement_journal().flows.last().unwrap().owner,
        Owner::Commons(0)
    );
    w.audit_settlement_research().unwrap();
}
#[test]
fn observer_accounts_exact_exclusive_labor_and_death_stops_living_time() {
    let mut w = manual(48 * HOUR);
    w.advance_to(48 * HOUR).unwrap();
    assert!(w.agents.iter().all(|a| a.action == Action::Dead));
    assert_eq!(
        w.settlement_journal().labor_ms.values().sum::<u128>(),
        72 * HOUR as u128
    );
    assert_eq!(w.metrics.deficit_ms, 48 * HOUR as u128);
    w.audit_settlement_research().unwrap();
}
