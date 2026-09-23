//! F9A spot barter. Law validates physical consent; policy sees only a local View.
use super::*;
pub mod market;
pub mod research;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offer {
    pub id: usize,
    pub proposer: usize,
    pub batch: usize,
    pub give: Material,
    pub quantity: u64,
    pub want: Material,
    pub want_quantity: u64,
    pub created_at: Time,
    pub expires_at: Time,
    pub location: usize,
    pub status: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exchange {
    pub at: Time,
    pub offer: usize,
    pub proposer: usize,
    pub taker: usize,
    pub node: usize,
    pub give: Batch,
    pub payment: Batch,
    pub give_quantity: u64,
    pub payment_quantity: u64,
    pub received_by_taker: usize,
    pub received_by_proposer: usize,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub receipts: Vec<market::Receipt>,
    pub pending: BTreeMap<usize, market::Pending>,
    pub offers: Vec<Offer>,
    pub exchanges: Vec<Exchange>,
    pub counters: BTreeMap<String, u64>,
    seen: BTreeSet<(usize, usize)>,
    reasons: BTreeSet<(usize, usize, String)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Needs {
    pub food: u64,
    pub wood: u64,
    pub protect_food: u64,
    pub protect_wood: u64,
    pub owned_food: u64,
    pub owned_wood: u64,
    pub capital_gate: String,
}
#[derive(Clone, Debug)]
pub struct View {
    pub now: Time,
    pub agent: usize,
    pub node: usize,
    pub own: Vec<Batch>,
    pub offers: Vec<Offer>,
    pub people: Vec<usize>,
    pub needs: Needs,
}
impl Needs {
    fn demand(&self, m: Material) -> u64 {
        match m {
            Material::Food => self.food,
            Material::Wood => self.wood,
            _ => 0,
        }
    }
    fn surplus(&self, m: Material) -> u64 {
        match m {
            Material::Food => self.owned_food.saturating_sub(self.protect_food),
            Material::Wood => self.owned_wood.saturating_sub(self.protect_wood),
            _ => 0,
        }
    }
}
/// Pure, disclosed use-based rule; no World reference or counterparty preferences.
pub fn willingness(
    v: &View,
    give: Material,
    q: u64,
    receive: Material,
    incoming: u64,
) -> Result<(), &'static str> {
    if incoming == 0 || v.needs.demand(receive) == 0 {
        return Err("COUNTERPARTY_DOES_NOT_WANT_GOOD");
    }
    if q == 0 || v.needs.surplus(give) < q {
        return Err("OFFER_NOT_USEFUL");
    }
    Ok(())
}
fn acceptable_material(m: Material) -> bool {
    !matches!(m, Material::Waste)
}
impl World {
    pub fn barter_enabled(&self) -> bool {
        self.config.barter
    }
    pub fn barter_offers(&self) -> &[Offer] {
        &self.barter.offers
    }
    pub fn barter_exchanges(&self) -> &[Exchange] {
        &self.barter.exchanges
    }
    pub fn barter_counters(&self) -> &BTreeMap<String, u64> {
        &self.barter.counters
    }
    fn barter_count(&mut self, key: &str) {
        *self.barter.counters.entry(key.into()).or_default() += 1;
    }
    pub(super) fn record_barter_failure(&mut self, reason: &str) {
        if self.config.barter {
            self.barter_count(&format!("settlement_rejected_{reason}"));
        }
    }
    fn barter_node(&self, id: usize, idle: bool) -> Result<usize, String> {
        if !self.config.barter {
            return Err("TRANSFER_DISABLED".into());
        }
        let a = self.agents.get(id).ok_or("UNKNOWN_AGENT")?;
        if a.action == Action::Dead {
            return Err("DEAD_OWNER".into());
        }
        if idle && a.action != Action::Idle {
            return Err("PARTY_BUSY".into());
        }
        match a.location {
            Location::Node(n) => Ok(n),
            _ => Err("NO_COLOCATION".into()),
        }
    }
    fn barter_input(&self, id: usize, batch: usize, q: u64, node: usize) -> Result<Batch, String> {
        let b = self
            .goods
            .batches
            .get(batch)
            .ok_or("INSUFFICIENT_OWNED_QUANTITY")?;
        if b.owner != Owner::Agent(id) {
            return Err("OWNERSHIP_BLOCK".into());
        }
        if b.material == Material::Waste
            || (b.material == Material::Food && b.expires_at() <= self.now)
        {
            return Err("GOODS_SPOILED".into());
        }
        if !(b.holder == Holder::Loose(node)
            || self.carried_by(id, b.holder)
            || self.accessible_cart(id, b.holder))
        {
            return Err("NO_COLOCATION".into());
        }
        if q == 0 || q > b.quantity.saturating_sub(self.reserved(batch)) {
            return Err("INSUFFICIENT_OWNED_QUANTITY".into());
        }
        Ok(b.clone())
    }
    pub(super) fn post_barter(
        &mut self,
        id: usize,
        batch: usize,
        quantity: u64,
        want: Material,
        want_quantity: u64,
    ) -> Result<(), String> {
        let node = self.barter_node(id, true)?;
        if self.market_enabled() && !self.config.market_nodes.contains(&node) {
            return Err("NO_LOCAL_MARKET".into());
        }
        let b = self.barter_input(id, batch, quantity, node)?;
        if !acceptable_material(want) || b.material == want || want_quantity == 0 {
            return Err("INVALID_BUNDLE".into());
        }
        let market = self.market_enabled();
        if market {
            self.refresh_market_record(id);
        }
        let existing = if market {
            self.barter.pending.contains_key(&id)
        } else {
            self.barter
                .offers
                .iter()
                .any(|o| o.proposer == id && o.status == "OPEN" && o.expires_at > self.now)
        };
        if existing {
            return Err("OFFER_ALREADY_OPEN".into());
        }
        let index = self.barter.offers.len();
        let expires_at = if market {
            self.now + 12 * HOUR
        } else {
            (self.now + HOUR).min(if b.material == Material::Food {
                b.expires_at()
            } else {
                Time::MAX
            })
        };
        let batch = if market {
            self.goods
                .move_part(batch, quantity, b.owner, Holder::MarketEscrow(index))
        } else {
            batch
        };
        self.barter.offers.push(Offer {
            id: index,
            proposer: id,
            batch,
            give: b.material,
            quantity,
            want,
            want_quantity,
            created_at: self.now,
            expires_at,
            location: node,
            status: "OPEN".into(),
        });
        if market {
            self.barter.pending.insert(
                id,
                market::Pending {
                    offer: index,
                    node,
                    expires_at,
                },
            );
        }
        self.barter_count("offers_created");
        if market {
            self.barter_count("offers_deposited");
            self.log(Some(id), "MarketDeposited", format!("offer={index} batch={batch} q={quantity} owner={:?} node={node} expires={expires_at}", b.owner));
        } else {
            self.log(Some(id),"BarterOffered",format!("offer={index} batch={batch} give={:?}:{quantity} want={want:?}:{want_quantity} node={node} expires={expires_at}; standing voluntary consent; no reservation",b.material));
        }
        Ok(())
    }
    pub(super) fn revoke_barter(&mut self, id: usize, offer: usize) -> Result<(), String> {
        let node = self.barter_node(id, false)?;
        let market = self.market_enabled();
        let o = self.barter.offers.get_mut(offer).ok_or("UNKNOWN_OFFER")?;
        if o.proposer != id || o.status != "OPEN" {
            return Err("OWNERSHIP_BLOCK".into());
        }
        if market && o.location != node {
            return Err("NO_COLOCATION".into());
        }
        o.status = "REVOKED".into();
        if market {
            self.release_market(offer, "cancelled");
            self.barter.pending.remove(&id);
        }
        self.barter_count("offers_revoked");
        self.log(Some(id), "BarterRevoked", format!("offer={offer}"));
        Ok(())
    }
    pub(super) fn accept_barter(
        &mut self,
        id: usize,
        offer: usize,
        payment: usize,
    ) -> Result<(), String> {
        // All validation precedes either transfer. No partial commit or resource reservation.
        let node = self.barter_node(id, true)?;
        let o = self
            .barter
            .offers
            .get(offer)
            .ok_or("UNKNOWN_OFFER")?
            .clone();
        if o.status != "OPEN" || o.expires_at <= self.now {
            return Err("OFFER_EXPIRED_OR_CLOSED".into());
        }
        if o.proposer == id {
            return Err("SELF_EXCHANGE".into());
        }
        let market = self.market_enabled();
        if o.location != node || (!market && self.barter_node(o.proposer, true)? != node) {
            return Err("NO_COLOCATION".into());
        }
        let give = if market {
            self.market_input(&o)?
        } else {
            self.barter_input(o.proposer, o.batch, o.quantity, node)?
        };
        let pay = self.barter_input(id, payment, o.want_quantity, node)?;
        if give.material != o.give || pay.material != o.want {
            return Err("WRONG_RESOURCE".into());
        }
        let received_by_taker = if market {
            self.goods.move_part_unmerged(
                give.id,
                o.quantity,
                Owner::Agent(id),
                Holder::Loose(node),
            )
        } else {
            self.goods
                .move_part(give.id, o.quantity, Owner::Agent(id), Holder::Loose(node))
        };
        let received_by_proposer = if market {
            self.goods.move_part_unmerged(
                pay.id,
                o.want_quantity,
                Owner::Agent(o.proposer),
                Holder::Loose(node),
            )
        } else {
            self.goods.move_part(
                pay.id,
                o.want_quantity,
                Owner::Agent(o.proposer),
                Holder::Loose(node),
            )
        };
        if market {
            self.market_receipt(offer, received_by_proposer, "payment");
            self.market_receipt(offer, received_by_taker, "purchase");
        }
        self.barter.offers[offer].status = "ACCEPTED".into();
        self.barter.exchanges.push(Exchange {
            at: self.now,
            offer,
            proposer: o.proposer,
            taker: id,
            node,
            give: give.clone(),
            payment: pay.clone(),
            give_quantity: o.quantity,
            payment_quantity: o.want_quantity,
            received_by_taker,
            received_by_proposer,
        });
        self.barter_count("offers_accepted");
        self.log(Some(id),"BarterSettled",format!("offer={offer} proposer={} taker={id} node={node} give_batch={} q={} payment_batch={payment} q={} output_taker={received_by_taker} output_proposer={received_by_proposer}; simultaneous owner transfer; both outputs Loose({node})",o.proposer,give.id,o.quantity,o.want_quantity));
        self.wake.insert(id);
        if !market || self.agents[o.proposer].location == Location::Node(node) {
            self.wake.insert(o.proposer);
        }
        Ok(())
    }
    pub(super) fn expire_barter(&mut self) {
        if !self.config.barter {
            return;
        }
        let mut closed = vec![];
        for o in &self.barter.offers {
            if o.status != "OPEN" {
                continue;
            }
            let why = if o.expires_at <= self.now {
                Some("EXPIRED".to_string())
            } else if self.market_enabled() {
                self.market_input(o).err()
            } else if self.agents[o.proposer].action == Action::Dead {
                Some("DEAD_OWNER".into())
            } else if self.agents[o.proposer].location != Location::Node(o.location) {
                Some("PROPOSER_LEFT".into())
            } else {
                self.barter_input(o.proposer, o.batch, o.quantity, o.location)
                    .err()
            };
            if let Some(why) = why {
                closed.push((o.id, o.proposer, why));
            }
        }
        for (id, proposer, why) in closed {
            self.barter.offers[id].status = why.clone();
            self.barter_count(&format!("closed_{why}"));
            if self.market_enabled() {
                self.release_market(id, &why);
            }
            self.log(
                if self.market_enabled() {
                    None
                } else {
                    Some(proposer)
                },
                "BarterClosed",
                format!("offer={id} reason={why}; local offer not remotely broadcast"),
            );
        }
    }
    /// Own goods and visible offers only. Other owners' inventories/needs are absent.
    pub fn barter_view(&self, id: usize) -> Option<View> {
        if !self.config.barter {
            return None;
        }
        let o = self.observe(id)?;
        let Location::Node(node) = o.location else {
            return None;
        };
        let own: Vec<_> = self
            .goods
            .batches
            .iter()
            .filter(|b| {
                b.owner == Owner::Agent(id)
                    && b.quantity > 0
                    && b.material != Material::Waste
                    && (b.material != Material::Food || b.expires_at() > self.now)
                    && (b.holder == Holder::Loose(node)
                        || self.carried_by(id, b.holder)
                        || self.accessible_cart(id, b.holder))
            })
            .cloned()
            .collect();
        let owned_food = own
            .iter()
            .filter(|b| b.material == Material::Food)
            .map(|b| b.quantity)
            .sum::<u64>();
        let owned_wood = own
            .iter()
            .filter(|b| b.material == Material::Wood)
            .map(|b| b.quantity)
            .sum::<u64>();
        let shortfall = ((12 * HOUR - o.nutrition).max(0) as u64).div_ceil(HOUR as u64);
        let mut needs = Needs {
            food: shortfall.saturating_sub(owned_food),
            wood: 0,
            protect_food: shortfall + 6,
            protect_wood: 0,
            owned_food,
            owned_wood,
            capital_gate: "NO_CAPITAL".into(),
        };
        if let Some(c) = &self.config.capital {
            let mut memory = self.capital.memories[id].clone();
            if let Some(local) = memory.get_mut(&node) {
                // Conditional mechanics: evaluate what owned recipe inputs would enable,
                // without altering world stock, observation or actual capital policy.
                local.wood.push(capital::Wood {
                    batch: usize::MAX,
                    available: c.rules.wood_cost,
                    owner: Owner::Agent(id),
                    holder: Holder::Loose(node),
                });
                let e = capital::evaluate(&o, &self.agents[id].memory, &memory, &c.rules);
                needs.capital_gate = e.gate.clone();
                if e.gate == "INVEST" && !local_project_exists(&memory, node, id) {
                    needs.protect_wood = c.rules.wood_cost;
                    needs.wood = c.rules.wood_cost.saturating_sub(owned_wood);
                }
            }
        }
        let people = o.local.as_ref()?.people.clone();
        let offers = self
            .barter
            .offers
            .iter()
            .filter(|f| {
                f.status == "OPEN"
                    && f.location == node
                    && f.expires_at > self.now
                    && (self.market_enabled() || people.contains(&f.proposer))
            })
            .cloned()
            .collect();
        Some(View {
            now: self.now,
            agent: id,
            node,
            own,
            offers,
            people,
            needs,
        })
    }
    pub(super) fn barter_decision(
        &mut self,
        id: usize,
        o: &Observation,
        fallback: Decision,
    ) -> Decision {
        if !self.config.barter {
            return fallback;
        }
        if self.market_enabled() {
            self.refresh_market_record(id);
        }
        let Some(v) = self.barter_view(id) else {
            return fallback;
        };
        // Offer observation is local even when an agent has higher-priority work.
        for f in &v.offers {
            if f.proposer != id && self.barter.seen.insert((id, f.id)) {
                self.barter_count("offers_observed");
            }
        }
        if o.action != Action::Idle
            || o.job.is_some()
            || matches!(
                fallback.intent,
                Intent::Eat { .. } | Intent::Sleep | Intent::Deliver { .. }
            )
        {
            if self.market_enabled() {
                let why = if o.action != Action::Idle {
                    "DEFERRED_BUSY"
                } else if o.job.is_some() {
                    "DEFERRED_JOB"
                } else {
                    "DEFERRED_PRIORITY"
                };
                for f in &v.offers {
                    if f.proposer != id && self.barter.reasons.insert((id, f.id, why.into())) {
                        self.barter_count(why);
                    }
                }
            }
            return fallback;
        }
        self.barter_count("eligible_evaluations");
        if v.needs.wood > 0 {
            self.barter_count("missing_wood_plan_evaluations");
        }
        if v.needs.food > 0 {
            self.barter_count("unmet_food_evaluations");
        }
        let decision = |intent, reason: String| Decision { intent, reason };
        for f in &v.offers {
            if f.proposer == id {
                let own_offer_view = self.offer_owner_view(&v, f);
                if willingness(&own_offer_view, f.give, f.quantity, f.want, f.want_quantity)
                    .is_err()
                {
                    return decision(
                        Intent::RevokeBarter { offer: f.id },
                        format!("own use changed; needs={:?}", v.needs),
                    );
                }
                continue;
            }
            let payment = v
                .own
                .iter()
                .find(|b| b.material == f.want && b.quantity >= f.want_quantity);
            let reason = if v.needs.demand(f.give) == 0 {
                Some("COUNTERPARTY_DOES_NOT_WANT_GOOD")
            } else if payment.is_none() {
                Some("INSUFFICIENT_OWNED_QUANTITY")
            } else {
                willingness(&v, f.want, f.want_quantity, f.give, f.quantity).err()
            };
            if let Some(reason) = reason {
                if self.barter.reasons.insert((id, f.id, reason.into())) {
                    if reason == "INSUFFICIENT_OWNED_QUANTITY"
                        && research::coincidence_failure(self, &v, f)
                    {
                        self.barter_count("research_double_coincidence");
                    }
                    self.barter_count(&format!("rejected_{reason}"));
                    self.log(Some(id),"BarterDeclined",format!("offer={} reason={reason} needs={:?}; rejection by taker, offer remains open",f.id,v.needs));
                }
            } else {
                return decision(
                    Intent::AcceptBarter {
                        offer: f.id,
                        payment: payment.unwrap().id,
                    },
                    format!(
                        "positive own use; surrender outside protected uses; needs={:?}",
                        v.needs
                    ),
                );
            }
        }
        if v.offers.iter().any(|f| f.proposer == id) {
            return fallback;
        }
        self.barter_count("formation_evaluations");
        for (give, want, limit) in [
            (Material::Food, Material::Wood, 6),
            (Material::Wood, Material::Food, 10),
        ] {
            let q = v.needs.surplus(give).min(limit);
            let ask = v.needs.demand(want);
            if q == 0 || ask == 0 {
                continue;
            }
            if let Some(b) = v.own.iter().find(|b| b.material == give) {
                let q = q.min(b.quantity);
                if v.people.len() < 2 {
                    self.barter_count("offer_without_counterparty");
                }
                return decision(
                    Intent::OfferBarter {
                        batch: b.id,
                        quantity: q,
                        want,
                        want_quantity: ask,
                    },
                    format!(
                        "blocked own use; surplus={q}; no resale assumption; needs={:?}",
                        v.needs
                    ),
                );
            }
        }
        fallback
    }
    pub(super) fn audit_barter(&self) -> Result<(), String> {
        if !self.config.barter
            && (!self.barter.offers.is_empty() || !self.barter.exchanges.is_empty())
        {
            return Err("barter without institution".into());
        }
        self.audit_market()?;
        let mut open = BTreeSet::new();
        for (id, o) in self.barter.offers.iter().enumerate() {
            if o.id != id
                || o.quantity == 0
                || o.want_quantity == 0
                || o.proposer >= self.agents.len()
                || o.location >= self.config.nodes
                || o.give == o.want
                || o.expires_at <= o.created_at
            {
                return Err("invalid barter terms".into());
            }
            if o.status == "OPEN" && !open.insert(o.proposer) {
                return Err("duplicate standing consent".into());
            }
        }
        for x in &self.barter.exchanges {
            let o = &self.barter.offers[x.offer];
            if o.status != "ACCEPTED"
                || x.proposer == x.taker
                || x.give.owner != Owner::Agent(x.proposer)
                || x.payment.owner != Owner::Agent(x.taker)
                || x.give_quantity > x.give.quantity
                || x.payment_quantity > x.payment.quantity
            {
                return Err("barter consent/accounting".into());
            }
            for (source, id) in [
                (&x.give, x.received_by_taker),
                (&x.payment, x.received_by_proposer),
            ] {
                let b = &self.goods.batches[id];
                if b.origin != source.origin
                    || b.created_at != source.created_at
                    || b.spoil_after != source.spoil_after
                {
                    return Err("barter refreshed goods".into());
                }
            }
        }
        Ok(())
    }
}
fn local_project_exists(m: &capital::Memory, node: usize, id: usize) -> bool {
    m.get(&node).is_some_and(|v| {
        v.projects
            .iter()
            .any(|p| p.investor == id && p.cart.is_none())
    })
}
#[cfg(test)]
mod tests;
