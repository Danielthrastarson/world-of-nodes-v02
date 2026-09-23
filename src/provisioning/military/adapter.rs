use super::*;
impl World {
    pub fn military_memory(&self, id: usize) -> Option<&Memory> {
        self.military.memories.get(id)
    }
    pub fn military_journal(&self) -> &Journal {
        &self.military.journal
    }
    pub fn military_role(&self, id: usize) -> Option<usize> {
        self.military
            .memories
            .get(id)?
            .undertaking
            .as_ref()
            .filter(|u| u.arrived_at.is_some())
            .map(|u| u.request.id)
    }
    pub(crate) fn observe_military(&mut self, id: usize) {
        let Some(c) = &self.config.military else {
            return;
        };
        if !c.enabled {
            return;
        }
        let Location::Node(node) = self.agents[id].location else {
            return;
        };
        for r in c
            .requests
            .iter()
            .filter(|r| r.posting == node && r.opens <= self.now)
        {
            // Admission is visible at the muster only, never remotely updated at posting.
            let present = if node == r.muster {
                self.military
                    .memories
                    .iter()
                    .filter(|m| {
                        m.undertaking
                            .as_ref()
                            .is_some_and(|u| u.request.id == r.id && u.arrived_at.is_some())
                    })
                    .count()
            } else {
                0
            };
            self.military.memories[id].known.insert(
                r.id,
                Seen {
                    request: r.clone(),
                    observed_at: self.now,
                    present,
                },
            );
            self.military.journal.add("observations", id);
        }
    }
    pub(crate) fn volunteer(&mut self, id: usize, request: usize) -> Result<(), String> {
        let v = self
            .military
            .memories
            .get(id)
            .ok_or("no military institution")?;
        if v.undertaking.is_some() || v.attempted.contains(&request) {
            return Err("already undertaking/attempted".into());
        }
        let s = v.known.get(&request).ok_or("NO_LOCAL_INFORMATION")?.clone();
        let o = self.observe(id).ok_or("not alive")?;
        gate(&o, &self.agents[id].memory, &s)?;
        let previous = format!("{:?}", self.agents[id].action);
        self.cancel_action(id, "voluntary service investment");
        self.agents[id].goal = None;
        self.military.memories[id].attempted.insert(request);
        self.military.memories[id].undertaking = Some(Undertaking {
            request: s.request,
            volunteered_at: self.now,
            arrived_at: None,
            until: None,
        });
        self.unit_enlist(
            id,
            request,
            self.military.memories[id]
                .undertaking
                .as_ref()
                .unwrap()
                .request
                .muster,
        );
        self.military.journal.add("volunteers", id);
        self.log(
            Some(id),
            "Volunteered",
            format!("request={request}; previous={previous}; no role or reservation yet"),
        );
        self.wake.insert(id);
        Ok(())
    }
    pub(crate) fn join_muster(&mut self, id: usize) -> Result<(), String> {
        let u = self
            .military
            .memories
            .get(id)
            .and_then(|m| m.undertaking.as_ref())
            .ok_or("no voluntary undertaking")?
            .clone();
        if u.arrived_at.is_some() {
            return Err("already joined".into());
        }
        if self.agents[id].location != Location::Node(u.request.muster) {
            return Err("muster requires physical presence".into());
        }
        let count = self
            .military
            .memories
            .iter()
            .filter(|m| {
                m.undertaking
                    .as_ref()
                    .is_some_and(|v| v.request.id == u.request.id && v.arrived_at.is_some())
            })
            .count();
        if self.now + u.request.duration > u.request.expires || count >= u.request.attendance {
            self.leave_service(id, "LOCAL_ADMISSION_CLOSED");
            return Ok(());
        }
        self.cancel_action(id, "physical muster arrival");
        let slot = self.military.memories[id].undertaking.as_mut().unwrap();
        slot.arrived_at = Some(self.now);
        slot.until = Some(self.now + slot.request.duration);
        self.agents[id].goal = None;
        self.agents[id].action = Action::MilitaryDuty { unit: u.request.id };
        self.unit_assemble(id);
        self.military.journal.add("arrivals", id);
        self.log(
            Some(id),
            "MusterArrived",
            format!(
                "unit={}; travel_ms={}; same Person; until={}",
                u.request.id,
                self.now - u.volunteered_at,
                self.now + u.request.duration
            ),
        );
        Ok(())
    }
    pub(crate) fn begin_duty(&mut self, id: usize) -> Result<(), String> {
        let u = self
            .military
            .memories
            .get(id)
            .and_then(|m| m.undertaking.as_ref())
            .ok_or("not serving")?;
        if u.arrived_at.is_none()
            || self.agents[id].location != Location::Node(self.duty_node(id, u.request.muster))
            || u.until.is_none_or(|t| t <= self.now)
        {
            return Err("no current local service".into());
        }
        let unit = u.request.id;
        if self.agents[id].action != (Action::MilitaryDuty { unit }) {
            self.cancel_action(id, "resume voluntary duty");
            self.agents[id].action = Action::MilitaryDuty { unit };
            self.log(Some(id), "DutyResumed", format!("unit={unit}"));
        }
        Ok(())
    }
    pub(crate) fn leave_service(&mut self, id: usize, reason: &str) {
        let Some(v) = self.military.memories.get_mut(id) else {
            return;
        };
        if let Some(u) = v.undertaking.take() {
            // Ending an office/role must not interrupt unrelated embodied recovery.
            if matches!(self.agents[id].action, Action::MilitaryDuty { .. })
                || ((u.arrived_at.is_none() || self.has_unit_order(id))
                    && matches!(self.agents[id].action, Action::Queued { .. }))
            {
                self.cancel_action(id, "voluntary service ended");
            }
            self.agents[id].goal = None;
            self.unit_release(id, reason);
            self.military.journal.add(reason, id);
            self.log(
                Some(id),
                "ServiceEnded",
                format!(
                    "unit={}; active={}; reason={reason}",
                    u.request.id,
                    u.arrived_at.is_some()
                ),
            );
            self.wake.insert(id);
        }
    }
    /// Action arbitration stays common. Eating/sleep pause; a contrary chosen goal withdraws.
    pub(crate) fn military_action(&mut self, id: usize, i: &Intent) {
        if self.has_unit_order(id) && !matches!(i, Intent::LeaveService) {
            return;
        }
        let Some(u) = self
            .military
            .memories
            .get(id)
            .and_then(|v| v.undertaking.as_ref())
        else {
            return;
        };
        let allowed = matches!(
            i,
            Intent::Wait
                | Intent::Eat { .. }
                | Intent::Sleep
                | Intent::JoinMuster
                | Intent::MilitaryDuty
                | Intent::LeaveService
        ) || matches!(i,Intent::Travel{target,food:false,..} if *target==u.request.muster&&u.arrived_at.is_none());
        if !allowed {
            self.leave_service(id, "PRIORITY_OVERRIDE")
        }
    }
    pub(crate) fn finish_military(&mut self) {
        let ends: Vec<_> = self
            .military
            .memories
            .iter()
            .enumerate()
            .filter_map(|(id, m)| {
                m.undertaking.as_ref().and_then(|u| {
                    if self.agents[id].action == Action::Dead {
                        Some((id, "DEATH"))
                    } else if u.until.is_some_and(|t| t <= self.now) {
                        Some((id, "TERM_COMPLETED"))
                    } else if u.request.expires <= self.now {
                        Some((id, "REQUEST_EXPIRED"))
                    } else {
                        None
                    }
                })
            })
            .collect();
        for (id, why) in ends {
            self.leave_service(id, why)
        }
    }
    pub(crate) fn record_military_choice(&mut self, id: usize, o: &Observation, d: &Decision) {
        if self.config.military.is_none() {
            return;
        }
        let v = &self.military.memories[id];
        let pending: Vec<_> = v
            .known
            .values()
            .filter(|s| !v.attempted.contains(&s.request.id) && o.now < s.request.expires)
            .map(|s| {
                let reason = if self.is_child(id) {
                    "CHILD"
                } else if matches!(d.intent,Intent::Volunteer{request}if request==s.request.id) {
                    "PASS"
                } else {
                    gate(o, &self.agents[id].memory, s)
                        .err()
                        .unwrap_or("HIGHER_PRIORITY")
                };
                (s.request.id, reason)
            })
            .collect();
        for (r, why) in pending {
            self.military.journal.add("evaluations", id);
            self.military.journal.add(why, id);
            // Journal is research-only; exact refusal traces are sparse by gate/person/request.
            if self.military.journal.count(why) <= 16 {
                self.log(
                    Some(id),
                    "ServiceEvaluation",
                    format!("request={r}; gate={why}; selected={:?}", d.intent),
                )
            }
        }
    }
    pub(crate) fn integrate_military(&mut self, dt: Time) {
        if self.config.military.is_none() {
            return;
        }
        for a in &self.agents {
            if a.action == Action::Dead {
                continue;
            }
            let u = self.military.memories[a.id].undertaking.as_ref();
            let active = u.is_some_and(|u| u.arrived_at.is_some());
            let j = &mut self.military.journal;
            let deficit = (dt - a.nutrition).max(0) as u128;
            if active {
                j.soldier_starvation_ms += deficit
            } else {
                j.civilian_starvation_ms += deficit
            }
            if active {
                j.active_ms += dt as u128
            }
            if matches!(a.action, Action::MilitaryDuty { .. }) {
                j.duty_ms += dt as u128
            }
            if u.is_some() {
                if matches!(a.action, Action::Travel) {
                    j.travel_ms += dt as u128;
                    if self.capital.carts.iter().any(|c| c.user == Some(a.id)) {
                        j.cart_travel_ms += dt as u128
                    }
                }
                if matches!(a.action, Action::Queued { .. }) {
                    j.queue_ms += dt as u128
                }
            }
        }
    }
    pub(crate) fn audit_military(&self) -> Result<(), String> {
        if self.config.military.is_none() {
            return Ok(());
        }
        if self.military.memories.len() != self.agents.len() {
            return Err("military memory cardinality".into());
        }
        for a in &self.agents {
            let u = &self.military.memories[a.id].undertaking;
            if a.action == Action::Dead && u.is_some() {
                return Err("dead military role".into());
            }
            if let Action::MilitaryDuty { unit } = a.action
                && (!u.as_ref().is_some_and(|u| {
                    u.request.id == unit
                        && u.arrived_at.is_some()
                        && (a.location == Location::Node(u.request.muster)
                            || a.location == Location::Node(self.duty_node(a.id, u.request.muster))
                            || self
                                .formation
                                .views
                                .get(a.id)
                                .and_then(|v| v.order.as_ref())
                                .is_some_and(|o| a.location == Location::Node(o.issued_at_node)))
                }) || self.is_child(a.id))
            {
                return Err("invalid duty".into());
            }
        }
        Ok(())
    }
}
