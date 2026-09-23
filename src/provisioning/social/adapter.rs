//! World truth ends at physical observation. No distant updates or private motives.
use super::*;
impl World {
    pub fn social_memory(&self, id: usize) -> Option<&Memory> {
        self.social.memories.get(id)
    }
    pub fn social_journal(&self) -> &Journal {
        &self.social.journal
    }
    fn receive_social(&mut self, e: Evidence) {
        if self.config.social.is_none() || self.agents[e.observer].action == Action::Dead {
            return;
        }
        let id = e.observer;
        let actor = e.actor;
        let a = appraise(&mut self.social.memories[id], e);
        if actor != id {
            self.social.known_by.entry(actor).or_default().insert(id);
        }
        self.social.journal.add("observed_evidence");
        for (kind, before, after) in [
            ("anger", &a.affect_before.anger, &a.affect_after.anger),
            ("fear", &a.affect_before.fear, &a.affect_after.fear),
            ("grief", &a.affect_before.grief, &a.affect_after.grief),
        ] {
            if after.remaining(self.now) > before.remaining(self.now) {
                self.social.journal.add(&format!("{kind}_episodes"));
            }
        }
        self.log(Some(id), "SocialAppraisal", format!("{a:?}"));
        self.social.journal.appraisals.push(a);
    }
    pub(crate) fn social_food(
        &mut self,
        actor: usize,
        recipient: Option<usize>,
        quantity: u64,
        actor_nutrition: Time,
    ) {
        if self.config.social.is_none() {
            return;
        }
        let Location::Node(node) = self.agents[actor].location else {
            return;
        };
        let mut witnesses = self
            .social
            .expected_by
            .get(&actor)
            .cloned()
            .unwrap_or_default();
        if let Some(r) = recipient {
            witnesses.insert(r);
        }
        let source_event = self.event_count.saturating_sub(1);
        for id in witnesses {
            if id == actor
                || self.agents[id].action == Action::Dead
                || self.agents[id].location != Location::Node(node)
            {
                continue;
            }
            let occurrence = if recipient == Some(id) {
                Occurrence::Support {
                    quantity,
                    giver_nutrition: actor_nutrition,
                }
            } else if self.agents[id].nutrition <= 6 * HOUR {
                Occurrence::UnmetExpectation {
                    quantity,
                    alternative_believed: actor_nutrition > 6 * HOUR,
                }
            } else {
                continue;
            };
            self.receive_social(Evidence {
                at: self.now,
                observer: id,
                actor,
                node,
                source_event,
                occurrence,
            });
        }
    }
    pub(crate) fn social_failed_transfer(&mut self, actor: usize, recipient: usize, reason: &str) {
        if self.config.social.is_none() || recipient >= self.agents.len() {
            return;
        }
        let Location::Node(node) = self.agents[actor].location else {
            return;
        };
        if self.agents[recipient].location != Location::Node(node)
            || self.agents[recipient].action == Action::Dead
            || self.agents[recipient].nutrition > 6 * HOUR
            || !self
                .social
                .expected_by
                .get(&actor)
                .is_some_and(|s| s.contains(&recipient))
        {
            return;
        }
        // A visible unsuccessful attempt is not evidence of private willingness/stock elsewhere.
        self.log(
            Some(actor),
            "CareAttemptUnavailable",
            format!("recipient={recipient} physical_rejection={reason}"),
        );
        self.receive_social(Evidence {
            at: self.now,
            observer: recipient,
            actor,
            node,
            source_event: self.event_count - 1,
            occurrence: Occurrence::UnavailableAttempt,
        });
    }
    pub(crate) fn observe_social(&mut self, id: usize) {
        if self.config.social.is_none() || self.agents[id].action == Action::Dead {
            return;
        }
        let danger = self.agents[id].nutrition == 0 && self.agents[id].starvation >= 18 * HOUR;
        let before = self.social.memories[id].in_danger;
        self.social.memories[id].in_danger = danger;
        if danger && !before {
            // Own physiology is known even in transit; normal policy observation occurs at legal stops.
            let node = match self.agents[id].location {
                Location::Node(n) => n,
                Location::Edge { from, .. } => from,
            };
            self.log(
                Some(id),
                "OwnDangerObserved",
                format!("nutrition=0 starvation_ms={}", self.agents[id].starvation),
            );
            self.receive_social(Evidence {
                at: self.now,
                observer: id,
                actor: id,
                node,
                source_event: self.event_count - 1,
                occurrence: Occurrence::OwnStarvationDanger,
            });
        }
        let Location::Node(node) = self.agents[id].location else {
            return;
        };
        let learned: Vec<_> = self.social.memories[id]
            .relationships
            .keys()
            .copied()
            .filter(|&other| {
                !self.social.memories[id].learned_deaths.contains(&other)
                    && self.agents[other].action == Action::Dead
                    && self.agents[other].location == Location::Node(node)
            })
            .collect();
        for actor in learned {
            self.log(
                Some(id),
                "DeathObserved",
                format!("person={actor} node={node}"),
            );
            self.receive_social(Evidence {
                at: self.now,
                observer: id,
                actor,
                node,
                source_event: self.event_count - 1,
                occurrence: Occurrence::DeathLearned,
            });
        }
    }
    pub(crate) fn social_death(&mut self, id: usize) {
        if self.config.social.is_none() {
            return;
        }
        let witnesses = self.social.known_by.get(&id).cloned().unwrap_or_default();
        let Location::Node(node) = self.agents[id].location else {
            return;
        };
        for observer in witnesses {
            if self.agents[observer].location == Location::Node(node)
                && self.agents[observer].action != Action::Dead
                && self.agents[observer].starvation < 24 * HOUR
            {
                self.observe_social(observer);
            }
        }
    }
    pub(crate) fn record_social_choice(&mut self, id: usize, o: &Observation, d: &Decision) {
        if !self.config.social.as_ref().is_some_and(|c| c.influence)
            || !self.config.household.as_ref().is_some_and(|c| c.enabled)
        {
            return;
        }
        if matches!(d.intent, Intent::Eat { .. } | Intent::Sleep)
            || matches!(o.action, Action::Gather { .. })
        {
            return;
        }
        let care = &self.household.memories[id];
        if let Ok(candidate) = household::evaluate(o, &self.agents[id].memory, care)
            && let Some(a) = assess(
                o,
                &self.agents[id].memory,
                care,
                &self.social.memories[id],
                &candidate,
            )
        {
            self.social.journal.add("care_assessments");
            if !a.permits() {
                self.social.journal.add("care_refused");
            }
            if matches!(
                d.intent,
                Intent::GiveCareFood { .. } | Intent::TravelForCare { .. }
            ) {
                self.social.journal.add("care_selected");
            }
            self.log(
                Some(id),
                "SocialChoice",
                format!(
                    "assessment={a:?} pressure={} cost={} permits={} selected={:?}",
                    a.pressure(),
                    a.cost(),
                    a.permits(),
                    d.intent
                ),
            );
        }
    }
    pub(crate) fn audit_social(&self) -> Result<(), String> {
        if self.config.social.is_none() {
            return Ok(());
        }
        if self.social.memories.len() != self.agents.len() {
            return Err("social memory cardinality".into());
        }
        for (id, m) in self.social.memories.iter().enumerate() {
            for (&other, r) in &m.relationships {
                if other >= self.agents.len()
                    || other == id
                    || !(-100..=100).contains(&r.trust)
                    || [r.attachment, r.grievance, r.obligation]
                        .iter()
                        .any(|v| !(0..=100).contains(v))
                    || r.last_update > self.now
                    || !self
                        .social
                        .known_by
                        .get(&other)
                        .is_some_and(|s| s.contains(&id))
                {
                    return Err("invalid sparse relationship".into());
                }
            }
            for p in [&m.affect.anger, &m.affect.fear, &m.affect.grief] {
                if p.at > self.now || !(0..=100 * HOUR).contains(&p.height) {
                    return Err("invalid affect pulse".into());
                }
            }
        }
        Ok(())
    }
}
