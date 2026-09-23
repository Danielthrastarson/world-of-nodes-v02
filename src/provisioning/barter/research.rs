//! Registered research fixtures; never called by autonomous policy.
use super::*;
pub const FIXTURES: &[&str] = &[
    "complementary",
    "symmetric",
    "low_cost",
    "high_cost",
    "colocated",
    "separated",
];
pub fn world(fixture: &str, enabled: bool, seed: u64, trace: bool) -> Result<World, String> {
    if !FIXTURES.contains(&fixture) {
        return Err("unknown barter fixture".into());
    }
    let opportunity = matches!(fixture, "colocated" | "separated");
    let mut c = settlement_research::configuration(
        if opportunity {
            "complementary"
        } else {
            fixture
        },
        seed,
        trace,
    )?;
    c.barter = enabled;
    if opportunity {
        c.starts = vec![0, if fixture == "colocated" { 0 } else { 2 }];
        c.food = vec![(0, 24, 0, 48 * HOUR), (2, 2976, 0, 48 * HOUR)];
        c.farming.as_mut().unwrap().units = vec![0, 0];
        c.capital.as_mut().unwrap().wood = vec![(c.starts[1], 10), (2, 490)];
    }
    let mut w = World::new(c)?;
    if opportunity {
        // Initial endowment, before time or actions: reallocates existing matter.
        w.goods.batches[0].owner = Owner::Agent(0);
        w.goods.batches[0].holder = Holder::Loose(0);
        w.goods.batches[2].owner = Owner::Agent(1);
        w.goods.batches[2].holder = Holder::Loose(w.config.starts[1]);
        for node in 0..3 {
            let mut v = w.node_view(node);
            v.observed_at = -HOUR;
            // Only route/request history is endowed. Initial t=0 food/people
            // cannot have been observed at t=-1h.
            v.food.clear();
            v.people.clear();
            if node == 2 {
                v.board = Some(Board {
                    node: 2,
                    quantity: 100,
                    created_at: -HOUR,
                    refreshed_at: -HOUR,
                    claims: vec![],
                });
            }
            w.agents[0].memory.nodes.insert(node, v);
        }
    }
    w.audit()?;
    Ok(w)
}

/// Omniscient diagnostic of a *locally observed* failed offer. Never a policy input.
/// Require a willing seller, a wanted good, zero desired payment resource, and
/// actual alternative possessions which the seller does not want. Mere insufficient
/// quantity or an owner unwilling to sell does not qualify.
pub(super) fn coincidence_failure(w: &World, taker: &View, offer: &Offer) -> bool {
    let Some(maker) = w.barter_view(offer.proposer) else {
        return false;
    };
    let maker = w.offer_owner_view(&maker, offer);
    if w.market_enabled() && maker.node != offer.location {
        return false;
    }
    if willingness(
        &maker,
        offer.give,
        offer.quantity,
        offer.want,
        offer.want_quantity,
    )
    .is_err()
        || taker.needs.demand(offer.give) == 0
        || taker.own.iter().any(|b| b.material == offer.want)
        || taker.own.is_empty()
    {
        return false;
    }
    taker
        .own
        .iter()
        .all(|b| maker.needs.demand(b.material) == 0)
}

/// Identical F9A worlds; only settlement-node deposited offer institution differs.
pub fn market_world(fixture: &str, seed: u64, trace: bool) -> Result<World, String> {
    let mut w = world(fixture, true, seed, trace)?;
    w.config.market_nodes = vec![0, 2];
    w.audit()?;
    Ok(w)
}
