use super::*;
fn fixture() -> World {
    let mut w = World::new(super::super::Config {
        barter: true,
        autonomous: false,
        starts: vec![0, 0, 0],
        food: vec![],
        boards: vec![],
        capital: Some(capital::Config {
            rules: capital::Rules::default(),
            wood: vec![],
        }),
        ..super::super::Config::default()
    })
    .unwrap();
    w.goods.add(
        Material::Food,
        20,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    w.goods.add(
        Material::Wood,
        10,
        Owner::Agent(1),
        Holder::Agent(1),
        0,
        Time::MAX,
    );
    w.goods.add(
        Material::Wood,
        10,
        Owner::Agent(2),
        Holder::Agent(2),
        0,
        Time::MAX,
    );
    w
}
#[test]
fn atomic_swap_preserves_age_origin_and_physical_node() {
    let mut w = fixture();
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    w.accept_barter(1, 0, 1).unwrap();
    let x = &w.barter.exchanges[0];
    assert_eq!(w.goods.batches[x.received_by_taker].owner, Owner::Agent(1));
    assert_eq!(
        w.goods.batches[x.received_by_taker].holder,
        Holder::Loose(0)
    );
    assert_eq!(w.goods.batches[x.received_by_taker].created_at, 0);
    assert_eq!(w.goods.batches[x.received_by_taker].origin, 0);
    assert_eq!(w.goods.batches[1].quantity, 7);
    assert_eq!(w.goods.batches[0].quantity, 14);
    w.audit().unwrap();
}
#[test]
fn failures_are_atomic_and_do_not_reserve_goods() {
    let mut w = fixture();
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    for (id, payment, reason) in [
        (1, 2, "OWNERSHIP_BLOCK"),
        (1, 0, "OWNERSHIP_BLOCK"),
        (0, 0, "SELF_EXCHANGE"),
    ] {
        let before = w.goods.clone();
        assert_eq!(w.accept_barter(id, 0, payment).unwrap_err(), reason);
        assert_eq!(before, w.goods);
    }
    w.goods.batches[0].quantity = 2;
    let before = w.goods.clone();
    assert_eq!(
        w.accept_barter(1, 0, 1).unwrap_err(),
        "INSUFFICIENT_OWNED_QUANTITY"
    );
    assert_eq!(w.goods, before);
}
#[test]
fn same_offer_cannot_settle_twice() {
    let mut w = fixture();
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    w.accept_barter(1, 0, 1).unwrap();
    let before = w.goods.clone();
    assert!(w.accept_barter(2, 0, 2).is_err());
    assert_eq!(before, w.goods);
    w.audit().unwrap();
}
#[test]
fn expiry_revocation_departure_death_and_spoilage_preserve_property() {
    for case in 0..5 {
        let mut w = fixture();
        w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
        match case {
            0 => w.advance_to(HOUR).unwrap(),
            1 => w.revoke_barter(0, 0).unwrap(),
            2 => w.agents[0].location = Location::Node(1),
            3 => w.agents[0].action = Action::Dead,
            _ => w.goods.batches[0].material = Material::Waste,
        }
        w.expire_barter();
        let before = w.goods.clone();
        assert!(w.accept_barter(1, 0, 1).is_err());
        assert_eq!(before, w.goods);
        assert_eq!(w.goods.batches[0].owner, Owner::Agent(0));
    }
}
#[test]
fn exact_deadline_cannot_be_exchanged_and_age_never_refreshes() {
    let mut w = fixture();
    w.now = 47 * HOUR;
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    w.accept_barter(1, 0, 1).unwrap();
    w.advance_to(48 * HOUR).unwrap();
    assert!(
        w.goods
            .batches
            .iter()
            .filter(|b| b.origin == 0 && b.quantity > 0)
            .all(|b| b.material == Material::Waste)
    );
    w.audit().unwrap();
}
#[test]
fn owner_and_counterparty_must_both_be_local_and_idle() {
    let mut w = fixture();
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    w.agents[1].location = Location::Node(2);
    assert_eq!(w.accept_barter(1, 0, 1).unwrap_err(), "NO_COLOCATION");
    w.agents[1].location = Location::Node(0);
    w.agents[0].action = Action::Sleep { until: HOUR };
    assert_eq!(w.accept_barter(1, 0, 1).unwrap_err(), "PARTY_BUSY");
    w.agents[0].action = Action::Idle;
    w.accept_barter(1, 0, 1).unwrap();
    w.audit().unwrap();
}
#[test]
fn foreign_and_communal_goods_cannot_be_offered() {
    let mut w = fixture();
    assert_eq!(
        w.post_barter(1, 0, 6, Material::Wood, 3).unwrap_err(),
        "OWNERSHIP_BLOCK"
    );
    w.goods.batches[0].owner = Owner::Commons(0);
    assert_eq!(
        w.post_barter(0, 0, 6, Material::Wood, 3).unwrap_err(),
        "OWNERSHIP_BLOCK"
    );
}
#[test]
fn refusal_is_not_a_global_offer_cancellation() {
    let mut w = fixture();
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    let v = w.barter_view(1).unwrap();
    assert_eq!(
        willingness(&v, Material::Wood, 3, Material::Food, 6),
        Err("COUNTERPARTY_DOES_NOT_WANT_GOOD")
    );
    assert_eq!(w.barter.offers[0].status, "OPEN");
    assert!(w.barter.exchanges.is_empty());
}
#[test]
fn offers_and_own_inventory_are_local_only() {
    let mut w = fixture();
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    assert_eq!(w.barter_view(1).unwrap().offers.len(), 1);
    assert!(
        w.barter_view(1)
            .unwrap()
            .own
            .iter()
            .all(|b| b.owner == Owner::Agent(1))
    );
    w.agents[1].location = Location::Node(2);
    let before = w.barter_view(1).unwrap();
    assert!(before.offers.is_empty());
    w.goods.batches[0].quantity = 1;
    w.barter.offers[0].want_quantity = 9;
    let after = w.barter_view(1).unwrap();
    assert_eq!(before.own, after.own);
    assert_eq!(before.offers, after.offers);
    assert_eq!(before.needs, after.needs);
}
#[test]
fn exchanging_full_backpacks_does_not_overfill_or_teleport() {
    let mut w = fixture();
    w.goods.batches[0].holder = Holder::Agent(0);
    w.goods.batches[0].quantity = 10;
    w.goods.initial[0] = 10;
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.accept_barter(1, 0, 1).unwrap();
    assert_eq!(w.goods.load(0), 4);
    assert_eq!(w.goods.load(1), 0);
    assert!(
        w.barter
            .exchanges
            .iter()
            .all(|x| w.goods.batches[x.received_by_proposer].holder == Holder::Loose(0))
    );
    w.audit().unwrap();
}
#[test]
fn communal_job_arbitration_is_unchanged() {
    let mut w = fixture();
    w.agents[1].job = Some(Job {
        request: 0,
        destination: 0,
        target: 4,
        acquired: 0,
    });
    w.post_barter(0, 0, 6, Material::Wood, 3).unwrap();
    let o = w.observe(1).unwrap();
    let d = Decision {
        intent: Intent::Travel {
            edge: 0,
            target: 2,
            food: true,
        },
        reason: "preserved".into(),
    };
    assert_eq!(w.barter_decision(1, &o, d.clone()), d);
}
#[test]
fn enabled_world_replays_deterministically() {
    let mut a = research::world("colocated", true, 7, false).unwrap();
    let mut b = a.clone();
    a.advance_to(24 * HOUR).unwrap();
    for h in 1..=24 {
        b.advance_to(h * HOUR).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    a.audit().unwrap();
}
#[test]
fn double_coincidence_requires_willing_seller_and_wrong_actual_payment_good() {
    let mut w = research::world("colocated", true, 0, false).unwrap();
    w.config.autonomous = false;
    // Research fixture: A wants wood for its remembered cart opportunity but
    // only owns stone. B explicitly offers spare wood for food it needs.
    w.goods.batches[0].owner = Owner::None;
    w.goods.batches[0].holder = Holder::Deposit(0);
    let local = w.node_view(0);
    w.agents[0].memory.nodes.insert(0, local);
    w.goods.add(
        Material::Stone,
        2,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        Time::MAX,
    );
    w.agents[1].nutrition = 8 * HOUR;
    w.observe_capital(0);
    w.observe_capital(1);
    w.post_barter(1, 2, 6, Material::Food, 4).unwrap();
    let a = w.barter_view(0).unwrap();
    assert_eq!(a.needs.wood, 10);
    assert!(research::coincidence_failure(&w, &a, &w.barter.offers[0]));
    w.goods.add(
        Material::Food,
        1,
        Owner::Agent(0),
        Holder::Loose(0),
        0,
        48 * HOUR,
    );
    let a = w.barter_view(0).unwrap();
    assert!(
        !research::coincidence_failure(&w, &a, &w.barter.offers[0]),
        "small quantity of desired payment is quantity mismatch, not missing coincidence"
    );
    w.audit().unwrap();
}
#[test]
fn no_money_or_resale_motive_is_hidden_in_willingness() {
    let w = fixture();
    let mut v = w.barter_view(1).unwrap();
    v.needs.food = 0;
    v.needs.wood = 0;
    for resource in [Material::Food, Material::Wood, Material::Stone] {
        assert_eq!(
            willingness(&v, Material::Wood, 1, resource, 100),
            Err("COUNTERPARTY_DOES_NOT_WANT_GOOD")
        );
    }
}
#[test]
fn two_willing_local_policies_can_settle_and_use_received_goods() {
    // Static conformance witness, not a tuned live settlement arm: both owners
    // are at a legal idle decision point and buyer nutrition is explicitly 8h.
    let mut w = research::world("colocated", true, 0, false).unwrap();
    w.config.autonomous = false;
    w.agents[1].nutrition = 8 * HOUR;
    for id in 0..2 {
        w.observe_capital(id);
        let local = w.node_view(0);
        w.agents[id].memory.nodes.insert(0, local);
    }
    for id in 0..2 {
        let o = w.observe(id).unwrap();
        let a = &mut w.agents[id];
        let base = policy::decide_capital(
            &o,
            &a.memory,
            &mut a.rng,
            None,
            &w.capital.memories[id],
            &w.config.capital.as_ref().unwrap().rules,
        );
        let d = w.barter_decision(id, &o, base);
        if id == 0 {
            assert!(matches!(d.intent, Intent::OfferBarter { .. }));
        } else {
            assert!(matches!(d.intent, Intent::AcceptBarter { .. }));
        }
        w.apply_intents(vec![(id, d.intent)]).unwrap();
    }
    assert_eq!(w.barter.exchanges.len(), 1);
    assert_eq!(
        w.agents[1].nutrition,
        8 * HOUR,
        "exchange alone is not nourishment"
    );
    let x = w.barter.exchanges[0].clone();
    w.apply_intents(vec![(
        1,
        Intent::Eat {
            batch: x.received_by_taker,
            quantity: x.give_quantity,
        },
    )])
    .unwrap();
    assert_eq!(w.agents[1].nutrition, 14 * HOUR);
    w.apply_intents(vec![(0, Intent::BuildCart)]).unwrap();
    assert_eq!(w.capital.projects[0].wood, 10);
    assert!(w.carts().is_empty(), "barter does not bypass capital labor");
    w.audit().unwrap();
}
