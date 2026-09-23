use super::*;
impl World {
    pub fn body(&self, id: usize) -> Option<&Body> {
        self.combat.bodies.get(id)
    }
    pub fn combat_journal(&self) -> &Journal {
        &self.combat.journal
    }
    pub(crate) fn body_factor(&self, id: usize) -> u128 {
        self.body(id).map_or(SCALE, Body::factor)
    }
    pub(crate) fn is_raider(&self, id: usize) -> bool {
        self.combat.views.get(id).is_some_and(|v| v.raid.is_some())
    }
    pub(crate) fn observe_combat(&mut self, id: usize) {
        if self.config.combat.is_none() {
            return;
        }
        let enemies = (0..self.agents.len())
            .filter(|&t| self.can_contact(id, t))
            .collect();
        let serving = self.military_role(id).is_some();
        let v = &mut self.combat.views[id];
        v.injuries = self.combat.bodies[id].injuries;
        v.serving = serving;
        v.enemies = enemies;
    }
    fn side(&self, id: usize) -> Option<bool> {
        let v = self.combat.views.get(id)?;
        if v.raid.as_ref().is_some_and(|r| self.now >= r.begins) {
            Some(true)
        } else if v.protect.is_some()
            && self
                .military_memory(id)
                .and_then(|m| m.undertaking.as_ref())
                .is_some_and(|u| u.arrived_at.is_some() && u.until.is_some_and(|t| t > self.now))
        {
            Some(false)
        } else {
            None
        }
    }
    /// Losing contact is a kernel event, not an observer-polling event. A target
    /// can depart after the boundary's strike resolution, during action/admission.
    /// Cancel any now-invalid strike at the next clock instant, not at whichever
    /// future checkpoint a renderer or research observer happens to request.
    pub(crate) fn contact_next_boundary(&self, next: Time) -> Time {
        if self.config.combat.is_some()
            && self.agents.iter().any(|person| {
                matches!(person.action, Action::Strike { target, .. } if !self.can_contact(person.id, target))
            })
        {
            next.min(self.now + 1)
        } else {
            next
        }
    }
    fn can_contact(&self, a: usize, b: usize) -> bool {
        let Some(c) = &self.config.combat else {
            return false;
        };
        a != b
            && a < self.agents.len()
            && b < self.agents.len()
            && self.side(a).is_some()
            && self.side(b).is_some()
            && self.side(a) != self.side(b)
            && self.body_factor(a) > 0
            && self.body_factor(b) > 0
            && self.agents[a].starvation < 24 * HOUR
            && self.agents[b].starvation < 24 * HOUR
            && self.agents[a].action != Action::Dead
            && self.agents[b].action != Action::Dead
            && self.agents[a].location == self.agents[b].location
            && matches!(self.agents[a].location,Location::Node(n) if c.nodes.contains(&n))
    }
    pub fn issue_unit_protect(
        &mut self,
        unit: usize,
        node: usize,
        target: usize,
    ) -> Result<usize, String> {
        if self.config.combat.is_none() {
            return Err("contact disabled".into());
        }
        let n = self.issue_unit_move(unit, node, target)?;
        for id in 0..self.agents.len() {
            if self.military_role(id) == Some(unit)
                && self.agents[id].location == Location::Node(node)
            {
                self.combat.views[id].protect = Some(target);
            }
        }
        self.log(
            None,
            "ProtectIssued",
            format!("unit={unit} target={target} local_recipients={n}"),
        );
        Ok(n)
    }
    pub(crate) fn begin_strike(&mut self, id: usize, target: usize) -> Result<(), String> {
        if !self.can_contact(id, target) {
            return Err("no capable local opposing contact".into());
        }
        if matches!(self.agents[id].action,Action::Strike{target:t,..}if t==target) {
            return Ok(());
        }
        self.cancel_action(id, "begin contact strike");
        self.agents[id].action = Action::Strike {
            target,
            remaining: self.config.combat.as_ref().unwrap().strike_work as u128 * SCALE,
        };
        let j = &mut self.combat.journal;
        j.add("strikes_started");
        j.engaged.extend([id, target]);
        j.first_contact.get_or_insert(self.now);
        j.last_contact = Some(self.now);
        self.log(
            Some(id),
            "StrikeStarted",
            format!("target={target} location={:?}", self.agents[id].location),
        );
        Ok(())
    }
    fn can_raid(&self, id: usize, batch: usize) -> bool {
        let Some(v) = self.combat.views.get(id) else {
            return false;
        };
        let Some(r) = &v.raid else { return false };
        let Some(b) = self.goods.batches.get(batch) else {
            return false;
        };
        self.now >= r.begins
            && v.stolen < r.quota
            && self.body_factor(id) > 0
            && self.agents[id].starvation < 24 * HOUR
            && self.agents[id].action != Action::Dead
            && self.agents[id].location == Location::Node(r.target)
            && b.owner == Owner::Commons(0)
            && b.holder == Holder::Commons(r.target)
            && b.material == Material::Food
            && b.quantity > 0
            && b.expires_at() > self.now
            && self.carrier_load(id) < self.carrier_capacity(id)
            && !(0..self.agents.len()).any(|t| self.can_contact(id, t))
    }
    pub(crate) fn begin_raid(&mut self, id: usize, batch: usize) -> Result<(), String> {
        if !self.can_raid(id, batch) {
            return Err("raid lacks stock/capacity/uncontested local access".into());
        }
        self.cancel_action(id, "begin timed theft");
        self.agents[id].action = Action::RaidStock {
            batch,
            remaining: self.config.combat.as_ref().unwrap().theft_work as u128 * SCALE,
        };
        self.combat.journal.add("thefts_started");
        self.log(
            Some(id),
            "RaidStarted",
            format!("batch={batch}; quantum=1; title unchanged"),
        );
        Ok(())
    }
    pub(crate) fn resolve_strikes(&mut self) {
        if self.config.combat.is_none() {
            return;
        }
        let mut hits = BTreeMap::<usize, u8>::new();
        let mut completions = vec![];
        for a in &self.agents {
            if let Action::Strike { target, remaining } = a.action {
                if !self.can_contact(a.id, target) {
                    completions.push((a.id, target, false));
                } else if remaining == 0 {
                    let n = hits.entry(target).or_default();
                    *n = n.saturating_add(1);
                    completions.push((a.id, target, true));
                }
            }
        }
        for (id, target, hit) in completions {
            if hit {
                self.combat.journal.last_contact = Some(self.now);
            }
            self.agents[id].action = Action::Idle;
            self.wake.insert(id);
            self.combat
                .journal
                .add(if hit { "hits" } else { "strikes_invalidated" });
            self.log(
                Some(id),
                if hit {
                    "StrikeHit"
                } else {
                    "StrikeInvalidated"
                },
                format!("target={target}; simultaneous pre-damage validation"),
            );
        }
        for (id, hits) in hits {
            let before = self.combat.bodies[id].injuries;
            let after = before.saturating_add(hits).min(3);
            self.combat.bodies[id].injuries = after;
            self.combat.views[id].injuries = after;
            if before == 0 && after > 0 {
                self.combat.journal.add("wounded_people");
            }
            if before < 2 && after >= 2 {
                self.combat.journal.add("incapacitations");
            }
            if after == 3 {
                self.combat.journal.add("lethal_injuries");
            }
            if after >= 2 {
                self.cancel_action(id, "incapacitated by contact");
                self.agents[id].goal = None;
            }
            self.wake.insert(id);
            self.log(
                Some(id),
                "Injured",
                format!("before={before} after={after} hits={hits}; own body; no unit HP"),
            );
        }
    }
    pub(crate) fn resolve_raids(&mut self) {
        if self.config.combat.is_none() {
            return;
        }
        for id in 0..self.agents.len() {
            if let Action::RaidStock { batch, remaining } = self.agents[id].action {
                if !self.can_raid(id, batch) {
                    self.cancel_action(id, "raid interrupted by contact/expiry/capacity");
                    self.combat.journal.add("thefts_interrupted");
                    self.wake.insert(id);
                } else if remaining == 0 {
                    let owner = self.goods.batches[batch].owner;
                    let target = self.goods.move_part(batch, 1, owner, Holder::Agent(id));
                    self.combat.views[id].stolen += 1;
                    self.combat.journal.stolen += 1;
                    self.combat.journal.add("thefts_completed");
                    self.agents[id].action = Action::Idle;
                    self.wake.insert(id);
                    self.log(Some(id),"FoodStolen",format!("from={batch} to={target} quantity=1 owner={owner:?}; custody=Agent({id}); age retained"));
                }
            }
        }
    }
    pub(crate) fn audit_combat(&self) -> Result<(), String> {
        if self.config.combat.is_none() {
            return Ok(());
        }
        if self.combat.bodies.len() != self.agents.len()
            || self.combat.views.len() != self.agents.len()
        {
            return Err("body/person cardinality".into());
        }
        for a in &self.agents {
            let b = &self.combat.bodies[a.id];
            if b.injuries > 3
                || b.injuries == 3 && a.action != Action::Dead
                || b.injuries >= 2
                    && !matches!(a.action, Action::Idle | Action::Sleep { .. } | Action::Dead)
            {
                return Err("incapacitated body acting".into());
            }
        }
        Ok(())
    }
}
