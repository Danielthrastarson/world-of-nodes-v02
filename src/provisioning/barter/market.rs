//! F9B deposited standing consent. No matching, offer motivation, or remote reports.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pending {
    pub offer: usize,
    pub node: usize,
    pub expires_at: Time,
}
#[derive(Clone, Debug)]
pub struct TrackedBatch {
    pub last_quantity: u64,
    pub untracked: u64,
}
#[derive(Clone, Debug)]
pub struct Receipt {
    pub offer: usize,
    pub batch: usize,
    pub kind: String,
    pub owner: usize,
    pub node: usize,
    pub at: Time,
    pub initial: u64,
    pub remaining: u64,
    pub last_quantity: u64,
    pub commingled: u64,
    pub retrieved: u64,
    pub used_locally: u64,
    pub redeposited: u64,
    pub other_removed: u64,
    pub first_retrieval: Option<Time>,
}
impl World {
    /// Private record of consent, never refreshed by a remote acceptance/death.
    pub(super) fn refresh_market_record(&mut self, id: usize) {
        let Some(p) = self.barter.pending.get(&id) else {
            return;
        };
        if p.expires_at <= self.now
            || (self.agents[id].location == Location::Node(p.node)
                && self.barter.offers[p.offer].status != "OPEN")
        {
            self.barter.pending.remove(&id);
        }
    }
    pub fn market_enabled(&self) -> bool {
        !self.config.market_nodes.is_empty()
    }
    pub fn market_receipts(&self) -> &[Receipt] {
        &self.barter.receipts
    }
    pub(super) fn market_input(&self, o: &Offer) -> Result<Batch, String> {
        let b = self.goods.batches.get(o.batch).ok_or("MISSING_ESCROW")?;
        if b.holder != Holder::MarketEscrow(o.id) || b.owner != Owner::Agent(o.proposer) {
            return Err("OWNERSHIP_OR_ESCROW_BLOCK".into());
        }
        if b.material == Material::Waste
            || (b.material == Material::Food && b.expires_at() <= self.now)
        {
            return Err("GOODS_SPOILED".into());
        }
        if b.material != o.give || b.quantity != o.quantity {
            return Err("MISSING_ESCROW".into());
        }
        Ok(b.clone())
    }
    pub(super) fn market_receipt(&mut self, offer: usize, batch: usize, quantity: u64, kind: &str) {
        let b = &self.goods.batches[batch];
        let Owner::Agent(owner) = b.owner else {
            unreachable!()
        };
        let Holder::Loose(node) = b.holder else {
            unreachable!()
        };
        // Register the known incoming part without changing any physical batch.
        if let Some(t) = self.barter.receipt_batches.get_mut(&batch) {
            t.last_quantity += quantity;
        } else {
            self.barter.receipt_batches.insert(
                batch,
                TrackedBatch {
                    last_quantity: b.quantity,
                    untracked: b.quantity - quantity,
                },
            );
        }
        self.barter.receipts.push(Receipt {
            offer,
            batch,
            kind: kind.into(),
            owner,
            node,
            at: self.now,
            initial: quantity,
            remaining: quantity,
            last_quantity: b.quantity,
            commingled: 0,
            retrieved: 0,
            used_locally: 0,
            redeposited: 0,
            other_removed: 0,
            first_retrieval: None,
        });
    }
    pub(super) fn release_market(&mut self, offer: usize, reason: &str) {
        let o = &self.barter.offers[offer];
        let b = &self.goods.batches[o.batch];
        // Use ordinary transfer/coalescing. Observation must not fragment stock.
        let q = b.quantity;
        let batch = self
            .goods
            .move_part(o.batch, q, b.owner, Holder::Loose(o.location));
        self.market_receipt(offer, batch, q, reason);
    }
    pub(super) fn offer_owner_view(&self, v: &View, o: &Offer) -> View {
        let mut out = v.clone();
        if self.market_enabled()
            && v.node == o.location
            && v.agent == o.proposer
            && let Ok(b) = self.market_input(o)
        {
            match b.material {
                Material::Food => out.needs.owned_food += b.quantity,
                Material::Wood => out.needs.owned_wood += b.quantity,
                _ => (),
            }
        }
        out
    }
    pub(crate) fn integrate_market(&mut self, dt: Time) {
        if !self.market_enabled() {
            return;
        }
        let q: u64 = self
            .goods
            .batches
            .iter()
            .filter(|b| matches!(b.holder, Holder::MarketEscrow(_)))
            .map(|b| b.quantity)
            .sum();
        *self
            .barter
            .counters
            .entry("escrow_quantum_ms".into())
            .or_default() += q * dt as u64;
    }
    /// Research-only attribution of fungible receipts. Untracked goods are removed
    /// first, then receipt cohorts in posting order. Physical merging stays untouched.
    /// Receipt-specific pickup is a conservative attribution, not molecular identity.
    pub(crate) fn market_event(&mut self, e: &Event) {
        if !self.market_enabled() {
            return;
        }
        for (batch, t) in &mut self.barter.receipt_batches {
            let q = self.goods.batches[*batch].quantity;
            if q > t.last_quantity {
                t.untracked += q - t.last_quantity;
            }
            let removed = t.last_quantity.saturating_sub(q);
            let untracked = removed.min(t.untracked);
            t.untracked -= untracked;
            let mut left = removed - untracked;
            t.last_quantity = q;
            let mut first = true;
            for r in self
                .barter
                .receipts
                .iter_mut()
                .filter(|r| r.batch == *batch)
            {
                r.last_quantity = q;
                r.commingled = if first { t.untracked } else { 0 };
                first = false;
                let traced = left.min(r.remaining);
                left -= traced;
                r.remaining -= traced;
                if traced == 0 {
                    continue;
                }
                let local_owner = e.agent == Some(r.owner)
                    && self.agents[r.owner].location == Location::Node(r.node);
                if local_owner && e.kind == "Pickup" {
                    r.retrieved += traced;
                    r.first_retrieval.get_or_insert(e.at);
                } else if local_owner
                    && matches!(
                        e.kind.as_str(),
                        "Consumed" | "CartStarted" | "CrossingDonation"
                    )
                {
                    r.used_locally += traced;
                } else if local_owner && e.kind == "MarketDeposited" {
                    r.redeposited += traced;
                } else {
                    r.other_removed += traced;
                }
            }
        }
    }
    pub fn market_stocks(&self) -> BTreeMap<String, u64> {
        let mut out = BTreeMap::new();
        for b in &self.goods.batches {
            if let Holder::MarketEscrow(_) = b.holder {
                *out.entry(format!("locked_{:?}", b.material)).or_default() += b.quantity;
                if let Owner::Agent(id) = b.owner
                    && self.agents[id].action == Action::Dead
                {
                    *out.entry(format!("dead_owner_escrow_{:?}", b.material))
                        .or_default() += b.quantity;
                }
            }
        }
        for r in &self.barter.receipts {
            let b = &self.goods.batches[r.batch];
            *out.entry(format!("remaining_{}_{:?}", r.kind, b.material))
                .or_default() += r.remaining;
            if self.agents[r.owner].action == Action::Dead {
                *out.entry(format!("dead_owner_receipt_{:?}", b.material))
                    .or_default() += r.remaining;
            }
        }
        out
    }
    pub(super) fn audit_market(&self) -> Result<(), String> {
        for b in &self.goods.batches {
            if b.quantity == 0 {
                continue;
            }
            if let Holder::MarketEscrow(id) = b.holder {
                let o = self.barter.offers.get(id).ok_or("orphan escrow")?;
                if !self.market_enabled()
                    || !self.config.market_nodes.contains(&o.location)
                    || o.status != "OPEN"
                    || o.batch != b.id
                    || o.quantity != b.quantity
                    || b.owner != Owner::Agent(o.proposer)
                    || self.reserved(b.id) != 0
                {
                    return Err("invalid market escrow".into());
                }
            }
        }
        if self.market_enabled() {
            for o in &self.barter.offers {
                if o.status == "OPEN" {
                    self.market_input(o)?;
                }
            }
        }
        for (batch, t) in &self.barter.receipt_batches {
            let tracked: u64 = self
                .barter
                .receipts
                .iter()
                .filter(|r| r.batch == *batch)
                .map(|r| r.remaining)
                .sum();
            if tracked + t.untracked != self.goods.batches[*batch].quantity {
                return Err("market pooled receipt accounting".into());
            }
        }
        for r in &self.barter.receipts {
            if r.initial
                != r.remaining + r.retrieved + r.used_locally + r.redeposited + r.other_removed
                || r.remaining > self.goods.batches[r.batch].quantity
            {
                return Err("market receipt accounting".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests;
