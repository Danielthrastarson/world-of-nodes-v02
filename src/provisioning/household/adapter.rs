//! Local observation, lawful transfer and observer-only instrumentation.
use super::*;
impl World {
    pub fn household_view(&self, id: usize) -> Option<PersonalView<'_>> {
        let c = self.config.household.as_ref()?;
        Some(PersonalView {
            households: c
                .households
                .iter()
                .filter(|h| h.members.contains(&id))
                .collect(),
            kin: c.relations(id),
            dependents: c
                .care
                .iter()
                .filter(|r| r.person == id)
                .map(|r| r.dependent)
                .collect(),
            memory: self.household.memories.get(id)?,
        })
    }
    pub fn care_journal(&self) -> &Journal {
        &self.household.journal
    }
    pub(crate) fn observe_household(&mut self, id: usize) {
        let Some(c) = &self.config.household else {
            return;
        };
        let Location::Node(node) = self.agents[id].location else {
            return;
        };
        for rel in c.care.iter().filter(|r| r.person == id) {
            let b = &self.agents[rel.dependent];
            if b.location == Location::Node(node) {
                let s = Sighting {
                    person: b.id,
                    node,
                    observed_at: self.now,
                    nutrition: b.nutrition,
                    starvation: b.starvation,
                    present: true,
                    alive: b.action != Action::Dead,
                };
                if s.alive && s.nutrition <= 6 * HOUR {
                    self.household.journal.add("observed_opportunities", 1);
                }
                self.household.memories[id].sightings.insert(b.id, s);
            } else if let Some(s) = self.household.memories[id]
                .sightings
                .get_mut(&rel.dependent)
                && s.node == node
            {
                // An empty meeting place reveals absence, NOT the new location or death.
                s.present = false;
                s.observed_at = self.now;
            }
        }
    }
    pub(crate) fn care_relation(&self, id: usize, dependent: usize) -> bool {
        self.config.household.as_ref().is_some_and(|c| {
            c.enabled
                && c.care
                    .iter()
                    .any(|r| r.person == id && r.dependent == dependent)
        })
    }
    pub(crate) fn give_care_food(
        &mut self,
        id: usize,
        dependent: usize,
        batch: usize,
        quantity: u64,
    ) -> Result<(), String> {
        let result = (|| {
            if !self.care_relation(id, dependent) {
                return Err("NO_CARE_RIGHT");
            }
            let Some(b) = self.agents.get(dependent) else {
                return Err("UNKNOWN_RECIPIENT");
            };
            let Location::Node(node) = self.agents[id].location else {
                return Err("NOT_AT_NODE");
            };
            if b.action == Action::Dead || b.location != Location::Node(node) {
                return Err("RECIPIENT_NOT_PRESENT_ALIVE");
            }
            let Some(food) = self.goods.batches.get(batch) else {
                return Err("NO_BATCH");
            };
            if food.owner != Owner::Agent(id)
                || !(self.carried_by(id, food.holder) || self.accessible_cart(id, food.holder))
            {
                return Err("NOT_PERSONAL_CARRIED_FOOD");
            }
            if food.material != Material::Food
                || self.now - food.created_at >= food.spoil_after
                || quantity == 0
                || quantity > food.quantity
            {
                return Err("NO_EDIBLE_QUANTITY");
            }
            Ok(node)
        })();
        self.household.journal.add("transfer_attempts", 1);
        let node = match result {
            Ok(n) => n,
            Err(e) => {
                self.household
                    .journal
                    .add(&format!("transfer_failed:{e}"), 1);
                self.social_failed_transfer(id, dependent, e);
                return Err(e.into());
            }
        };
        self.cancel_action(id, "voluntary family care transfer");
        let out = self.goods.move_part(
            batch,
            quantity,
            Owner::Agent(dependent),
            Holder::Loose(node),
        );
        self.household.journal.add("transfers", 1);
        self.household.journal.add("food_transferred", quantity);
        self.log(Some(id), "CareTransferred", format!("dependent={dependent} quantity={quantity} from_batch={batch} to_batch={out} node={node} owner=Agent({dependent}) custody=Loose; recipient must eat normally"));
        self.household.memories[id].undertaking = None;
        self.household.journal.add("completed", 1);
        self.log(
            Some(id),
            "CareCompleted",
            format!("dependent={dependent}; ownership transferred, nourishment not guaranteed"),
        );
        self.social_food(id, Some(dependent), quantity, self.agents[id].nutrition);
        Ok(())
    }
    pub(crate) fn record_care_decision(&mut self, id: usize, o: &Observation, d: &Decision) {
        let Some(c) = &self.config.household else {
            return;
        };
        if !c.enabled || !c.care.iter().any(|r| r.person == id) {
            return;
        }
        let selected = match d.intent {
            Intent::GiveCareFood { dependent, .. } | Intent::TravelForCare { dependent, .. } => {
                Some(dependent)
            }
            _ => None,
        };
        let reason = evaluate(o, &self.agents[id].memory, &self.household.memories[id]).err();
        if let Some(e) = reason {
            self.household.journal.add(&format!("gate:{e}"), 1);
        }
        if selected.is_none() && reason.is_none() {
            let gate = match d.intent {
                Intent::Eat { .. } => "SELF_EATING",
                Intent::Sleep => "SLEEP",
                Intent::Wait if matches!(o.action, Action::Gather { .. }) => "ATOMIC_GATHER",
                _ => "EXISTING_POLICY_PRIORITY",
            };
            self.household.journal.add(&format!("priority:{gate}"), 1);
        }
        let current = self.household.memories[id].undertaking.clone();
        let mut events = vec![];
        if let Some(dep) = selected {
            if current.as_ref().is_none_or(|u| u.dependent != dep) {
                if current.is_some() {
                    events.push(("cancelled", "CareCancelled", "MORE_SEVERE_OBSERVED_NEED"));
                }
                self.household.memories[id].undertaking = Some(Undertaking {
                    dependent: dep,
                    created_at: self.now,
                    paused: false,
                });
                events.push(("goals", "CareCreated", "OBSERVED_NEED"));
            } else if current.as_ref().unwrap().paused {
                self.household.memories[id]
                    .undertaking
                    .as_mut()
                    .unwrap()
                    .paused = false;
                events.push(("resumed", "CareResumed", "ELIGIBLE_AGAIN"));
            }
            if matches!(d.intent, Intent::TravelForCare { .. }) {
                self.household.journal.add("travel_attempts", 1);
            }
        } else if let Some(u) = current {
            let s = self.household.memories[id].sightings.get(&u.dependent);
            let invalid = if self.agents[id].action == Action::Dead {
                Some("CAREGIVER_DEAD")
            } else if s.is_some_and(|s| !s.present || !s.alive) {
                Some("LOCAL_ABSENCE_OR_DEATH")
            } else {
                reason
            };
            if let Some(e) = invalid {
                self.household.memories[id].undertaking = None;
                self.household.journal.add(&format!("cancel:{e}"), 1);
                self.household.journal.add("cancelled", 1);
                self.log(
                    Some(id),
                    "CareCancelled",
                    format!("dependent={} reason={e}", u.dependent),
                );
            } else if !u.paused {
                self.household.memories[id]
                    .undertaking
                    .as_mut()
                    .unwrap()
                    .paused = true;
                let e = match d.intent {
                    Intent::Eat { .. } => "SELF_EATING",
                    Intent::Sleep => "SLEEP",
                    _ => "EXISTING_POLICY_PRIORITY",
                };
                events.push(("paused", "CarePaused", e));
            }
        }
        for (metric, kind, reason) in events {
            self.household.journal.add(metric, 1);
            self.log(
                Some(id),
                kind,
                format!("selected={selected:?} reason={reason}"),
            );
        }
    }
    pub(crate) fn integrate_household(&mut self, dt: Time) {
        let Some(c) = &self.config.household else {
            return;
        };
        let j = &mut self.household.journal;
        for a in &self.agents {
            if a.action == Action::Dead {
                continue;
            }
            *j.starvation_ms.entry(a.id).or_default() += (dt - a.nutrition).max(0) as u128;
            let activity = match a.action {
                Action::MilitaryDuty { .. } => "military_duty",
                Action::Reproduce { .. } => "reproduction",
                Action::Travel => "travel",
                Action::Queued { .. } => "queue",
                Action::Sleep { .. } => "sleep",
                Action::Farm { .. } => "farming",
                Action::BuildCart { .. } | Action::BuildCrossing { .. } => "construction",
                Action::Gather { .. } => "gather",
                _ => "idle",
            };
            *j.activity_ms.entry((a.id, activity.into())).or_default() += dt as u128;
            if self.household.memories[a.id]
                .undertaking
                .as_ref()
                .is_some_and(|u| !u.paused)
            {
                if a.action == Action::Travel {
                    j.care_travel_ms += dt as u128;
                }
                if matches!(a.action, Action::Queued { .. }) {
                    j.care_queue_ms += dt as u128;
                }
            }
            if let Location::Node(n) = a.location
                && j.last_nodes.insert(a.id, n).is_some_and(|old| old != n)
            {
                j.residence_changes += 1;
            }
        }
        for h in &c.households {
            for (i, &a) in h.members.iter().enumerate() {
                for &b in &h.members[i + 1..] {
                    if self.agents[a].action != Action::Dead
                        && self.agents[b].action != Action::Dead
                        && !(matches!(self.agents[a].location, Location::Node(_))
                            && self.agents[a].location == self.agents[b].location)
                    {
                        j.separation_ms += dt as u128;
                    }
                }
            }
        }
        // Observer-only, never a notification to a distant family member.
        for r in &c.care {
            if self.agents[r.person].action == Action::Dead
                && self.agents[r.dependent].action != Action::Dead
            {
                j.losses.insert((r.person, r.dependent));
            }
        }
    }
    pub(crate) fn household_death(&mut self, id: usize) {
        let Some(c) = &self.config.household else {
            return;
        };
        for r in &c.care {
            if r.person == id
                && self.agents[r.dependent].action != Action::Dead
                && self.agents[r.dependent].starvation < 24 * HOUR
            {
                self.household.journal.losses.insert((id, r.dependent));
            }
        }
        if let Some(u) = self.household.memories[id].undertaking.take() {
            self.household.journal.add("cancelled", 1);
            self.household.journal.add("cancel:CAREGIVER_DEAD", 1);
            self.log(
                Some(id),
                "CareCancelled",
                format!("dependent={} reason=CAREGIVER_DEAD", u.dependent),
            );
        }
    }
    pub(crate) fn audit_household(&self) -> Result<(), String> {
        if let Some(c) = &self.config.household {
            c.validate(self.agents.len(), self.config.nodes)?;
            if self.household.memories.len() != self.agents.len() {
                return Err("household personal memory count".into());
            }
            for (id, m) in self.household.memories.iter().enumerate() {
                if m.sightings.iter().any(|(&dep, s)| {
                    dep != s.person
                        || s.observed_at > self.now
                        || s.node >= self.config.nodes
                        || !c.care.iter().any(|r| r.person == id && r.dependent == dep)
                }) {
                    return Err("invalid personal care evidence".into());
                }
            }
        }
        Ok(())
    }
}
