use super::*;
fn world() -> World {
    let mut w = World::new(super::super::super::Config {
        barter: true,
        market_nodes: vec![0, 2],
        autonomous: false,
        starts: vec![0, 0, 0],
        food: vec![],
        boards: vec![],
        ..super::super::super::Config::default()
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
fn deposit_is_exclusive_and_keeps_age() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    let o = w.barter.offers[0].clone();
    let b = &w.goods.batches[o.batch];
    assert_eq!(
        (
            b.owner,
            b.holder,
            b.quantity,
            b.origin,
            b.created_at,
            b.expires_at()
        ),
        (Owner::Agent(0), Holder::MarketEscrow(0), 6, 0, 0, 48 * HOUR)
    );
    assert_eq!(w.goods.batches[0].quantity, 14);
    assert_eq!(o.expires_at, 12 * HOUR);
    for intent in [
        Intent::Eat {
            batch: o.batch,
            quantity: 1,
        },
        Intent::Pickup {
            batch: o.batch,
            quantity: 1,
        },
        Intent::Gather {
            batch: o.batch,
            communal: false,
        },
        Intent::Drop {
            batch: o.batch,
            quantity: 1,
        },
    ] {
        let before = w.goods.clone();
        w.apply_intents(vec![(0, intent)]).unwrap();
        assert_eq!(w.goods, before);
    }
    assert!(
        !w.barter_view(0)
            .unwrap()
            .own
            .iter()
            .any(|b| b.id == o.batch)
    );
    assert!(w.post_barter(0, o.batch, 1, Material::Wood, 1).is_err());
    w.audit().unwrap();
}
#[test]
fn departed_busy_seller_can_settle_without_remote_notification() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.agents[0].location = Location::Node(2);
    w.agents[0].action = Action::Sleep { until: HOUR };
    w.wake.clear();
    let before = w.agents[0].clone();
    w.expire_barter();
    assert_eq!(w.barter.offers[0].status, "OPEN");
    assert_eq!(w.barter_view(1).unwrap().offers.len(), 1);
    assert!(w.barter_view(0).unwrap().offers.is_empty());
    w.accept_barter(1, 0, 1).unwrap();
    assert_eq!(w.agents[0], before);
    assert!(!w.wake.contains(&0));
    let x = &w.barter.exchanges[0];
    assert_eq!(
        w.goods.batches[x.received_by_proposer].holder,
        Holder::Loose(0)
    );
    assert_eq!(
        w.goods.batches[x.received_by_proposer].owner,
        Owner::Agent(0)
    );
    assert_eq!(
        w.goods.batches[x.received_by_taker].holder,
        Holder::Loose(0)
    );
    w.audit().unwrap();
}
#[test]
fn atomic_failures_and_racing_buyers() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    for (buyer, pay) in [(1, 2), (1, 0), (0, 0)] {
        let before = w.goods.clone();
        assert!(w.accept_barter(buyer, 0, pay).is_err());
        assert_eq!(w.goods, before);
    }
    w.agents[1].location = Location::Node(2);
    assert_eq!(w.accept_barter(1, 0, 1).unwrap_err(), "NO_COLOCATION");
    w.agents[1].location = Location::Node(0);
    w.accept_barter(1, 0, 1).unwrap();
    let before = w.goods.clone();
    assert!(w.accept_barter(2, 0, 2).is_err());
    assert_eq!(w.goods, before);
    w.audit().unwrap();
}
#[test]
fn cancellation_requires_local_owner_and_retrieval_requires_actual_return() {
    let mut w = world();
    w.goods.batches[0].quantity = 6;
    w.goods.initial[0] = 6;
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    let b = w.barter.offers[0].batch;
    w.agents[0].location = Location::Node(2);
    assert_eq!(w.revoke_barter(0, 0).unwrap_err(), "NO_COLOCATION");
    assert_eq!(w.barter.offers[0].status, "OPEN");
    w.agents[0].location = Location::Node(0);
    w.revoke_barter(0, 0).unwrap();
    assert_eq!(w.goods.batches[b].quantity, 0);
    let b = w.barter.receipts[0].batch;
    assert_eq!(w.goods.batches[b].holder, Holder::Loose(0));
    assert_eq!(w.goods.batches[b].owner, Owner::Agent(0));
    w.apply_intents(vec![(
        1,
        Intent::Pickup {
            batch: b,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.batches[b].quantity, 6);
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 6,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.load(0), 6);
    assert_eq!(w.barter.receipts[0].retrieved, 6);
    w.audit().unwrap();
}
#[test]
fn expiry_leaves_property_local_and_dead_owner_does_not_transfer_rights() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    let b = w.barter.offers[0].batch;
    w.agents[0].location = Location::Node(2);
    w.agents[0].action = Action::Dead;
    w.expire_barter();
    assert_eq!(w.barter.offers[0].status, "OPEN");
    w.advance_to(12 * HOUR).unwrap();
    assert_eq!(w.barter.offers[0].status, "EXPIRED");
    assert_eq!(w.goods.batches[b].quantity, 0);
    let b = w.barter.receipts[0].batch;
    assert_eq!(w.goods.batches[b].holder, Holder::Loose(0));
    assert_eq!(w.goods.batches[b].owner, Owner::Agent(0));
    assert!(w.accept_barter(1, 0, 1).is_err());
    assert_eq!(w.market_stocks()["dead_owner_receipt_Food"], 6);
    w.audit().unwrap();
}
#[test]
fn deceased_proposers_prior_deposit_may_settle_but_payment_is_locked_property() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.agents[0].action = Action::Dead;
    w.agents[0].location = Location::Node(2);
    w.expire_barter();
    w.accept_barter(1, 0, 1).unwrap();
    let pay = w.barter.exchanges[0].received_by_proposer;
    assert_eq!(w.goods.batches[pay].owner, Owner::Agent(0));
    assert_eq!(w.market_stocks()["dead_owner_receipt_Wood"], 10);
    w.audit().unwrap();
}
#[test]
fn escrow_spoils_at_original_deadline_without_disappearing() {
    let mut w = world();
    w.now = 47 * HOUR;
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    let b = w.barter.offers[0].batch;
    assert_eq!(w.barter.offers[0].expires_at, 59 * HOUR);
    w.advance_to(48 * HOUR).unwrap();
    assert_eq!(w.barter.offers[0].status, "GOODS_SPOILED");
    assert_eq!(w.goods.batches[b].quantity, 0);
    let b = w.barter.receipts[0].batch;
    assert_eq!(w.barter.receipts[0].remaining, 6);
    assert_eq!(
        (
            w.goods.batches[b].quantity,
            w.goods.batches[b].material,
            w.goods.batches[b].holder
        ),
        (20, Material::Waste, Holder::Loose(0))
    );
    assert!(w.accept_barter(1, 0, 1).is_err());
    w.audit().unwrap();
}
#[test]
fn payment_can_only_be_retrieved_locally_and_is_not_a_capacity_upgrade() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.agents[0].location = Location::Node(2);
    w.accept_barter(1, 0, 1).unwrap();
    let b = w.barter.exchanges[0].received_by_proposer;
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 10,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.load(0), 0);
    w.agents[0].location = Location::Node(0);
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 10,
        },
    )])
    .unwrap();
    assert_eq!(w.goods.load(0), 10);
    assert_eq!(w.barter.receipts[0].retrieved, 10);
    assert_eq!(w.barter.receipts[0].remaining, 0);
    w.audit().unwrap();
}
#[test]
fn unseen_offer_changes_do_not_change_remote_views() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.agents[2].location = Location::Node(2);
    let a = w.barter_view(2).unwrap();
    w.accept_barter(1, 0, 1).unwrap();
    let b = w.barter_view(2).unwrap();
    assert_eq!(a.offers, b.offers);
    assert_eq!(a.own, b.own);
    assert_eq!(a.needs, b.needs);
}
#[test]
fn formation_is_identical_before_custody_diverges_and_deposit_does_not_self_cancel() {
    let mut d = research::world("colocated", true, 0, false).unwrap();
    let mut m = research::market_world("colocated", 0, false).unwrap();
    for w in [&mut d, &mut m] {
        w.observe_capital(0);
        let v = w.node_view(0);
        w.agents[0].memory.nodes.insert(0, v);
    }
    let fallback = Decision {
        intent: Intent::Wait,
        reason: "control".into(),
    };
    let od = d.observe(0).unwrap();
    let om = m.observe(0).unwrap();
    let a = d.barter_decision(0, &od, fallback.clone());
    let b = m.barter_decision(0, &om, fallback.clone());
    assert_eq!(a, b);
    assert!(matches!(a.intent, Intent::OfferBarter { .. }));
    m.apply_intents(vec![(0, b.intent)]).unwrap();
    let v = m.barter_view(0).unwrap();
    let f = &m.barter.offers[0];
    assert!(
        willingness(
            &m.offer_owner_view(&v, f),
            f.give,
            f.quantity,
            f.want,
            f.want_quantity
        )
        .is_ok()
    );
}
#[test]
fn market_replay_is_deterministic() {
    let mut a = research::market_world("colocated", 7, false).unwrap();
    let mut b = a.clone();
    a.advance_to(72 * HOUR).unwrap();
    for h in 1..=72 {
        b.advance_to(h * HOUR).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
    a.audit().unwrap();
    b.audit().unwrap();
}

#[test]
fn unchanged_buyer_policy_can_accept_after_seller_physically_departs() {
    // Conformance witness, not a live outcome: suppress autonomous reconsideration
    // only to test the executable institution and the unmodified buyer valuation.
    let mut w = research::market_world("colocated", 0, false).unwrap();
    w.config.autonomous = false;
    for id in 0..2 {
        w.observe_capital(id);
        let v = w.node_view(0);
        w.agents[id].memory.nodes.insert(0, v);
    }
    let o = w.observe(0).unwrap();
    let d = w.barter_decision(
        0,
        &o,
        Decision {
            intent: Intent::Wait,
            reason: "static conformance".into(),
        },
    );
    assert!(matches!(d.intent, Intent::OfferBarter { .. }));
    w.apply_intents(vec![(0, d.intent)]).unwrap();
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 2,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(HOUR).unwrap();
    assert!(!matches!(w.agents[0].location, Location::Node(0)));
    w.observe_capital(1);
    let v = w.node_view(0);
    w.agents[1].memory.nodes.insert(0, v);
    let o = w.observe(1).unwrap();
    let a = &mut w.agents[1];
    let base = policy::decide_capital(
        &o,
        &a.memory,
        &mut a.rng,
        None,
        &w.capital.memories[1],
        &w.config.capital.as_ref().unwrap().rules,
    );
    let d = w.barter_decision(1, &o, base);
    assert!(matches!(d.intent, Intent::AcceptBarter { .. }));
    w.apply_intents(vec![(1, d.intent)]).unwrap();
    assert_eq!(w.barter.exchanges.len(), 1);
    let x = &w.barter.exchanges[0];
    assert_eq!(
        w.goods.batches[x.received_by_proposer].holder,
        Holder::Loose(0)
    );
    assert_eq!(w.goods.batches[x.received_by_proposer].quantity, 10);
    w.audit().unwrap();
}

#[test]
fn remote_settlement_cannot_change_admission_to_another_market() {
    let mut a = world();
    a.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    a.agents[0].location = Location::Node(2);
    let batch = a.goods.add(
        Material::Food,
        6,
        Owner::Agent(0),
        Holder::Loose(2),
        0,
        48 * HOUR,
    );
    let mut b = a.clone();
    b.accept_barter(1, 0, 1).unwrap();
    assert_eq!(
        a.post_barter(0, batch, 6, Material::Wood, 10),
        b.post_barter(0, batch, 6, Material::Wood, 10),
        "one-outstanding-offer admission must depend on the proposer's local record, not remote settlement truth"
    );
}

#[test]
fn escrow_release_uses_existing_goods_coalescing_semantics() {
    let mut w = world();
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.revoke_barter(0, 0).unwrap();
    assert_eq!(
        w.goods.batches[0].quantity, 20,
        "research receipt tracking must not fragment an otherwise identical owned food lot"
    );
    assert_eq!(
        w.goods
            .batches
            .iter()
            .filter(|b| b.quantity > 0 && b.material == Material::Food)
            .count(),
        1
    );
    w.audit().unwrap();
}

#[test]
fn settlement_merges_receipts_using_normal_goods_rules() {
    let mut w = world();
    let existing = w.goods.move_part(0, 2, Owner::Agent(1), Holder::Loose(0));
    w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
    w.accept_barter(1, 0, 1).unwrap();
    assert_eq!(w.barter.exchanges[0].received_by_taker, existing);
    assert_eq!(w.goods.batches[existing].quantity, 8);
    assert_eq!(w.barter.receipts[1].initial, 6);
    w.apply_intents(vec![(
        1,
        Intent::Eat {
            batch: existing,
            quantity: 4,
        },
    )])
    .unwrap();
    assert_eq!(
        w.barter.receipts[1].used_locally, 2,
        "preexisting untracked food is attributed first"
    );
    assert_eq!(w.barter.receipts[1].remaining, 4);
    w.audit().unwrap();
}
#[test]
fn repeated_cancellation_and_redeposit_do_not_duplicate_receipt_matter() {
    let mut w = world();
    for id in 0..10 {
        w.post_barter(0, 0, 6, Material::Wood, 10).unwrap();
        w.revoke_barter(0, id).unwrap();
        w.audit().unwrap();
        assert_eq!(w.goods.batches[0].quantity, 20);
    }
    assert!(w.barter.receipts.iter().map(|r| r.remaining).sum::<u64>() <= 20);
}
#[test]
fn receipt_journal_retention_does_not_change_physics_or_policy() {
    let mut a = research::market_world("colocated", 0, false).unwrap();
    let mut b = a.clone();
    a.advance_to(24 * HOUR).unwrap();
    for m in 1..=24 * 60 {
        b.advance_to(m * MINUTE).unwrap();
        b.barter.receipts.clear();
        b.barter.receipt_batches.clear();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
