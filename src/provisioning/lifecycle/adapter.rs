use super::*;
impl World {
    pub fn biology(&self, id: usize) -> Option<&Biology> {
        self.lifecycle.people.get(id)
    }
    pub fn lifecycle_journal(&self) -> &Journal {
        &self.lifecycle.journal
    }
    pub fn is_child(&self, id: usize) -> bool {
        self.config
            .lifecycle
            .as_ref()
            .is_some_and(|c| self.lifecycle.people[id].age(self.now) < c.rules.adult_age)
    }
    fn fertile(&self, id: usize) -> bool {
        let Some(c) = &self.config.lifecycle else {
            return false;
        };
        let b = &self.lifecycle.people[id];
        self.agents[id].action != Action::Dead
            && b.age(self.now) >= c.rules.adult_age
            && b.age(self.now) < c.rules.fertile_until
            && b.pregnancy.is_none()
    }
    pub(crate) fn observe_lifecycle(&mut self, id: usize) {
        if self.config.lifecycle.is_none() {
            return;
        }
        let b = &self.lifecycle.people[id];
        let mut partners = vec![];
        if let Location::Node(node) = self.agents[id].location {
            for rel in self.config.household.as_ref().unwrap().relations(id) {
                if rel.kind == household::Kin::Partner
                    && self.agents[rel.to].location == Location::Node(node)
                    && self.agents[rel.to].action != Action::Dead
                    && !self.is_child(rel.to)
                    && self.lifecycle.people[rel.to].sex != b.sex
                {
                    partners.push(rel.to)
                }
            }
        }
        self.lifecycle.views[id] = View {
            child: self.is_child(id),
            fertile: self.fertile(id),
            motivated: b.drive,
            available_at: b.next_attempt,
            undertaking: b.undertaking.clone(),
            partners,
            observed_at: self.now,
        };
        if self.lifecycle.views[id].fertile && !self.lifecycle.views[id].partners.is_empty() {
            self.lifecycle.journal.add("opportunities_observed")
        }
        // New kinship is structural truth; personal social records require local encounter.
        if self.config.social.is_some() {
            for r in self.config.household.as_ref().unwrap().relations(id) {
                if matches!(r.kind, household::Kin::Parent | household::Kin::Child)
                    && (self.lifecycle.people[id].parents.is_some()
                        || self.lifecycle.people[r.to].parents.is_some())
                    && self.agents[r.to].location == self.agents[id].location
                    && matches!(self.agents[id].location, Location::Node(_))
                {
                    self.social.memories[id]
                        .relationships
                        .entry(r.to)
                        .or_default();
                    self.social.known_by.entry(r.to).or_default().insert(id);
                }
            }
        }
    }
    pub(crate) fn record_lifecycle_choice(&mut self, id: usize, o: &Observation, d: &Decision) {
        if self.config.lifecycle.is_none() {
            return;
        }
        let v = &self.lifecycle.views[id];
        if v.fertile && !v.partners.is_empty() {
            let key = if !v.motivated {
                "decision:drive_disabled"
            } else if o.nutrition < 12 * HOUR {
                "decision:hunger"
            } else if o.fatigue >= 16 * HOUR {
                "decision:fatigue"
            } else if v.available_at > o.now && v.undertaking.is_none() {
                "decision:interval"
            } else if matches!(d.intent, Intent::Reproduce { .. }) {
                "decision:accept"
            } else {
                "decision:other_priority"
            };
            self.lifecycle.journal.add(key);
        }
    }
    pub(crate) fn lifecycle_permits(&self, id: usize, i: &Intent) -> bool {
        !self.is_child(id)
            || matches!(
                i,
                Intent::Eat { .. }
                    | Intent::Sleep
                    | Intent::Wait
                    | Intent::Travel { .. }
                    | Intent::CancelAction
            )
    }
    pub(crate) fn interrupt_reproduction(&mut self, id: usize, reason: &str) {
        if self.config.lifecycle.is_none() {
            return;
        }
        self.lifecycle.consents.remove(&id);
        if let Some(u) = self.lifecycle.people[id].undertaking.take() {
            self.lifecycle.people[u.partner].undertaking = None;
            for actor in [id, u.partner] {
                if matches!(self.agents[actor].action, Action::Reproduce { .. }) {
                    self.agents[actor].action = Action::Idle;
                }
            }
            self.lifecycle.journal.add("undertakings_interrupted");
            self.log(
                Some(id),
                "ReproductionInterrupted",
                format!("partner={} reason={reason}", u.partner),
            );
        }
    }
    pub(crate) fn consent_reproduction(&mut self, id: usize, partner: usize) -> Result<(), String> {
        if partner >= self.agents.len()
            || partner == id
            || !self.fertile(id)
            || !self.fertile(partner)
            || self.agents[id].location != self.agents[partner].location
            || !matches!(self.agents[id].location, Location::Node(_))
            || self.lifecycle.people[id].sex == self.lifecycle.people[partner].sex
            || !self
                .config
                .household
                .as_ref()
                .unwrap()
                .relations(id)
                .iter()
                .any(|r| r.to == partner && r.kind == household::Kin::Partner)
        {
            return Err(
                "reproduction requires living fertile colocated opposite-role Partners".into(),
            );
        }
        let b = &self.lifecycle.people[id];
        if !b.drive
            || b.next_attempt > self.now
            || b.undertaking.is_some()
            || self.agents[id].action != Action::Idle
            || self.agents[id].job.is_some()
            || self.agents[id].nutrition < 12 * HOUR
            || self.agents[id].fatigue >= 16 * HOUR
        {
            return Err("own reproductive willingness/readiness gate".into());
        }
        self.lifecycle.consents.insert(id, (partner, self.now));
        self.lifecycle.journal.add("individual_consents");
        self.log(
            Some(id),
            "ReproductionConsent",
            format!("partner={partner}; no unilateral conception"),
        );
        Ok(())
    }
    pub(crate) fn resolve_reproduction(&mut self) {
        let Some(c) = &self.config.lifecycle else {
            return;
        };
        let r = c.rules.clone();
        let pairs: Vec<_> = self
            .lifecycle
            .consents
            .iter()
            .filter_map(|(&a, &(b, t))| {
                (a < b && t == self.now && self.lifecycle.consents.get(&b) == Some(&(a, t)))
                    .then_some((a, b))
            })
            .collect();
        for (a, b) in pairs {
            if !self.fertile(a)
                || !self.fertile(b)
                || self.agents[a].location != self.agents[b].location
                || self.agents[a].action != Action::Idle
                || self.agents[b].action != Action::Idle
            {
                continue;
            }
            for (id, partner) in [(a, b), (b, a)] {
                self.lifecycle.people[id].undertaking = Some(Undertaking {
                    partner,
                    started_at: self.now,
                    until: self.now + r.undertaking,
                });
                self.lifecycle.people[id].next_attempt = self.now + r.interval;
                self.agents[id].action = Action::Reproduce {
                    partner,
                    until: self.now + r.undertaking,
                };
            }
            self.lifecycle.journal.add("undertakings_started");
            self.log(
                Some(a),
                "ReproductionStarted",
                format!("partner={b} until={}", self.now + r.undertaking),
            );
        }
        // Consent is a joint decision at this instant, not a remotely redeemable promise.
        self.lifecycle.consents.clear();
    }
    pub(crate) fn integrate_lifecycle(&mut self, dt: Time) {
        if self.config.lifecycle.is_none() {
            return;
        }
        for a in &self.agents {
            if a.action == Action::Dead {
                continue;
            }
            let b = &self.lifecycle.people[a.id];
            if b.undertaking.is_some() {
                self.lifecycle.journal.reproduction_ms += dt as u128
            }
            if self.is_child(a.id) {
                self.lifecycle.journal.child_ms += dt as u128;
                if !self
                    .config
                    .household
                    .as_ref()
                    .unwrap()
                    .care
                    .iter()
                    .any(|c| c.dependent == a.id && self.agents[c.person].action != Action::Dead)
                {
                    self.lifecycle.journal.unsupported_ms += dt as u128
                }
            } else {
                self.lifecycle.journal.adult_ms += dt as u128
            }
        }
    }
    pub(crate) fn finish_lifecycle(&mut self) {
        let Some(c) = &self.config.lifecycle else {
            return;
        };
        let r = c.rules.clone();
        for id in 0..self.agents.len() {
            if self.agents[id].action == Action::Dead {
                self.interrupt_reproduction(id, "death");
                if self.lifecycle.people[id].pregnancy.take().is_some() {
                    self.lifecycle.journal.add("pregnancy_lost_parent_death");
                    self.log(
                        Some(id),
                        "PregnancyEnded",
                        "gestating person died; no birth/inheritance".into(),
                    )
                }
                continue;
            }
            if !self.lifecycle.people[id].matured
                && self.lifecycle.people[id].age(self.now) >= r.adult_age
            {
                self.lifecycle.people[id].matured = true;
                self.lifecycle.journal.add("maturations");
                self.wake.insert(id);
                self.log(
                    Some(id),
                    "Matured",
                    "same Person identity, possessions, beliefs and history".into(),
                )
            }
            if let Some(u) = self.lifecycle.people[id].undertaking.clone()
                && u.until <= self.now
                && id < u.partner
            {
                let other = u.partner;
                let valid = self.fertile(id)
                    && self.fertile(other)
                    && self.agents[id].location == self.agents[other].location
                    && matches!(self.agents[id].action,Action::Reproduce {partner,..} if partner==other)
                    && matches!(self.agents[other].action,Action::Reproduce {partner,..} if partner==id)
                    && self.lifecycle.people[other]
                        .undertaking
                        .as_ref()
                        .is_some_and(|x| x.partner == id && x.until == u.until);
                self.lifecycle.people[id].undertaking = None;
                self.lifecycle.people[other].undertaking = None;
                for actor in [id, other] {
                    if matches!(self.agents[actor].action, Action::Reproduce { .. }) {
                        self.agents[actor].action = Action::Idle;
                    }
                }
                if valid {
                    self.lifecycle.journal.add("undertakings_completed");
                    let mother = if self.lifecycle.people[id].sex == Sex::Female {
                        id
                    } else {
                        other
                    };
                    let father = if mother == id { other } else { id };
                    let roll = draw(&mut self.lifecycle.people[mother].rng) % 1_000_000;
                    self.log(
                        Some(mother),
                        "ConceptionTrial",
                        format!(
                            "other_parent={father} roll={roll} threshold={}",
                            r.conception_per_million
                        ),
                    );
                    if roll < r.conception_per_million {
                        self.lifecycle.people[mother].pregnancy = Some(Pregnancy {
                            conception_time: self.now,
                            expected_birth_time: self.now + r.gestation,
                            other_parent: father,
                        });
                        self.lifecycle.journal.add("conceptions");
                        self.log(
                            Some(mother),
                            "PregnancyStarted",
                            format!("other_parent={father} due={}", self.now + r.gestation),
                        );
                    } else {
                        self.lifecycle.journal.add("unsuccessful_conception")
                    }
                } else {
                    self.lifecycle.journal.add("undertakings_invalidated")
                }
                self.wake.insert(id);
                self.wake.insert(other);
            }
        }
        let due: Vec<_> = self
            .lifecycle
            .people
            .iter()
            .enumerate()
            .filter_map(|(id, b)| {
                b.pregnancy
                    .as_ref()
                    .filter(|p| {
                        p.expected_birth_time <= self.now
                            && matches!(self.agents[id].location, Location::Node(_))
                            && self.agents[id].action != Action::Dead
                    })
                    .map(|_| id)
            })
            .collect();
        for mother in due {
            self.birth(mother)
        }
    }
    fn birth(&mut self, mother: usize) {
        let p = self.lifecycle.people[mother].pregnancy.take().unwrap();
        let father = p.other_parent;
        let id = self.agents.len();
        let node = match self.agents[mother].location {
            Location::Node(n) => n,
            _ => unreachable!(),
        };
        let sex = if draw(&mut self.lifecycle.people[mother].rng) & 1 == 0 {
            Sex::Male
        } else {
            Sex::Female
        };
        let generation = 1 + self.lifecycle.people[mother]
            .generation
            .max(self.lifecycle.people[father].generation);
        self.agents.push(Agent {
            id,
            location: Location::Node(node),
            action: Action::Idle,
            nutrition: 0,
            fatigue: 0,
            starvation: 0,
            job: None,
            memory: PolicyMemory::default(),
            rng: self
                .config
                .seed
                .wrapping_add((id as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)),
            goal: None,
            last_event: None,
            death_time: None,
            consumed: 0,
            travel_since_food: 0,
        });
        self.lifecycle.people.push(Biology {
            birth_time: self.now,
            sex,
            drive: true,
            rng: seed_rng(self.config.seed, id),
            next_attempt: 0,
            pregnancy: None,
            undertaking: None,
            parents: Some([mother, father]),
            generation,
            matured: false,
        });
        self.lifecycle.views.push(View::default());
        if self.config.information {
            self.information
                .memories
                .push(super::super::information::Memory::default());
        }
        if self.config.combat.is_some() {
            self.combat
                .bodies
                .push(super::super::combat::Body::default());
            self.combat
                .views
                .push(super::super::combat::View::default());
        }
        if self.config.formation {
            self.formation
                .views
                .push(super::super::formation::View::default());
        }
        if self.config.military.is_some() {
            self.military
                .memories
                .push(super::super::military::Memory::default());
        }
        self.household.memories.push(household::Memory::default());
        if self.config.farming.is_some() {
            self.farm.memories.push(farming::Memory::default())
        }
        if self.config.capital.is_some() {
            self.capital.memories.push(capital::Memory::default())
        }
        if self.config.infrastructure.is_some() {
            self.infrastructure
                .memories
                .push(infrastructure::Memory::default())
        }
        if self.config.social.is_some() {
            self.social.memories.push(social::Memory::default());
            for parent in [mother, father] {
                self.social
                    .expected_by
                    .entry(parent)
                    .or_default()
                    .insert(id);
            }
        }
        let h = self.config.household.as_mut().unwrap();
        if let Some(home) = h
            .households
            .iter_mut()
            .find(|h| h.members.contains(&mother))
        {
            home.members.push(id)
        } else {
            let next = h.households.iter().map(|h| h.id).max().map_or(0, |i| i + 1);
            h.households.push(household::Household {
                id: next,
                members: vec![mother, id],
                home: None,
            })
        }
        for parent in [mother, father] {
            h.kin.push(household::Relation {
                from: parent,
                to: id,
                kind: household::Kin::Parent,
            });
            h.care.push(household::Caregiver {
                person: parent,
                dependent: id,
            });
        }
        self.lifecycle.journal.add("births");
        self.lifecycle.journal.peak = self.agents.len();
        self.lifecycle.journal.delivery_delay_ms += (self.now - p.expected_birth_time) as u128;
        self.log(Some(id),"Born",format!("parents=[{mother},{father}] sex={sex:?} generation={generation} birth_time={} due={} node={node}; nutrition=0 goods=0",self.now,p.expected_birth_time));
        self.wake.insert(id);
        self.wake.insert(mother);
        // No remote father wake/report. Local observation can discover the newborn.
        for person in [id, mother, father] {
            if self.agents[person].location == Location::Node(node)
                && self.agents[person].action != Action::Dead
            {
                self.observe_lifecycle(person);
                self.wake.insert(person);
            }
        }
    }
    pub(crate) fn audit_lifecycle(&self) -> Result<(), String> {
        if self.config.lifecycle.is_none() {
            return Ok(());
        }
        if self.lifecycle.people.len() != self.agents.len()
            || self.lifecycle.views.len() != self.agents.len()
        {
            return Err("lifecycle cardinality".into());
        }
        if self.offices().iter().any(|o| self.is_child(o.holder))
            || self.polity().is_some_and(|p| self.is_child(p.sovereign))
        {
            return Err("child cannot hold adult authority office".into());
        }
        for (id, b) in self.lifecycle.people.iter().enumerate() {
            if b.birth_time > self.now
                || b.parents.is_some_and(|ps| ps.iter().any(|&p| p >= id))
                || b.pregnancy.as_ref().is_some_and(|p| {
                    b.sex != Sex::Female
                        || p.other_parent >= self.agents.len()
                        || p.conception_time > self.now
                        || p.expected_birth_time <= p.conception_time
                })
            {
                return Err("invalid biological state".into());
            }
            if let Some(u) = &b.undertaking
                && (!matches!(self.agents[id].action,Action::Reproduce {partner,until} if partner==u.partner&&until==u.until)
                    || u.partner >= self.agents.len()
                    || self.lifecycle.people[u.partner]
                        .undertaking
                        .as_ref()
                        .is_none_or(|v| v.partner != id || v.until != u.until))
            {
                return Err("joint undertaking exclusivity".into());
            }
        }
        Ok(())
    }
}
fn draw(rng: &mut u64) -> u64 {
    *rng = rng.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *rng;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
