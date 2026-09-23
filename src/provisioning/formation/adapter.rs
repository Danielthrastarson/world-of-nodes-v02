use super::*;
impl World {
    pub fn units(&self) -> &BTreeMap<usize, Unit> {
        &self.formation.units
    }
    pub fn unit_view(&self, id: usize) -> Option<&View> {
        self.formation.views.get(id)
    }
    pub fn formation_journal(&self) -> &Journal {
        &self.formation.journal
    }
    pub(crate) fn unit_enlist(&mut self, id: usize, unit: usize, muster: usize) {
        if !self.config.formation {
            return;
        }
        self.formation
            .units
            .entry(unit)
            .or_insert_with(|| Unit {
                id: unit,
                members: BTreeMap::new(),
                muster,
                desired_formation: DesiredFormation::GatherOnGraph,
                objective: None,
            })
            .members
            .insert(
                id,
                Member {
                    person: id,
                    enlisted_at: self.now,
                    assembled_at: None,
                    ended: None,
                },
            );
        self.formation.views[id] = View {
            unit: Some(unit),
            phase: "ENLISTED".into(),
            ..Default::default()
        };
    }
    pub(crate) fn unit_assemble(&mut self, id: usize) {
        if !self.config.formation {
            return;
        }
        let v = &mut self.formation.views[id];
        let unit = v.unit.expect("enlisted unit");
        v.active = true;
        v.phase = "ASSEMBLED".into();
        self.formation
            .units
            .get_mut(&unit)
            .unwrap()
            .members
            .get_mut(&id)
            .unwrap()
            .assembled_at = Some(self.now);
        self.log(
            Some(id),
            "UnitAssembled",
            format!("unit={unit}; no remote orders received"),
        );
    }
    pub(crate) fn unit_release(&mut self, id: usize, reason: &str) {
        if !self.config.formation {
            return;
        }
        let v = &mut self.formation.views[id];
        let Some(unit) = v.unit else { return };
        v.active = false;
        v.phase = if reason == "DEATH" {
            "DEAD"
        } else {
            "RELEASED"
        }
        .into();
        self.formation
            .units
            .get_mut(&unit)
            .unwrap()
            .members
            .get_mut(&id)
            .unwrap()
            .ended = Some((self.now, reason.into()));
        self.log(
            Some(id),
            "UnitServiceEnded",
            format!("unit={unit}; reason={reason}; roster retained"),
        );
    }
    pub(crate) fn has_unit_order(&self, id: usize) -> bool {
        self.formation
            .views
            .get(id)
            .is_some_and(|v| v.active && v.order.is_some())
    }
    pub(crate) fn duty_node(&self, id: usize, muster: usize) -> usize {
        self.formation
            .views
            .get(id)
            .filter(|v| v.active)
            .and_then(|v| v.order.as_ref())
            .map_or(muster, |o| o.target)
    }
    /// Research/local institutional entry: address one unit, delivering only to people present.
    /// The caller supplies the actual issue node; this is not a remote player command API.
    pub fn issue_unit_move(
        &mut self,
        unit: usize,
        issue_node: usize,
        target: usize,
    ) -> Result<usize, String> {
        if !self.config.formation || target >= self.config.nodes || issue_node >= self.config.nodes
        {
            return Err("invalid local unit command".into());
        }
        let u = self.formation.units.get(&unit).ok_or("unknown unit")?;
        let recipients: Vec<_> = u
            .members
            .keys()
            .copied()
            .filter(|&id| {
                self.military_role(id) == Some(unit)
                    && self.agents[id].location == Location::Node(issue_node)
            })
            .collect();
        if recipients.is_empty() {
            return Err("no assembled local recipients".into());
        }
        let serial = self.formation.next_order;
        self.formation.next_order += 1;
        let order = MoveOrder {
            serial,
            target,
            issued_at: self.now,
            issued_at_node: issue_node,
        };
        self.formation.units.get_mut(&unit).unwrap().objective = Some(order.clone());
        self.formation.journal.add("commands");
        self.log(None,"UnitMoveIssued",format!("unit={unit}; serial={serial}; node={issue_node}; target={target}; recipients={recipients:?}; local research command"));
        for &id in &recipients {
            let v = &mut self.formation.views[id];
            v.order = Some(order.clone());
            v.received_at = Some(self.now);
            v.departed_at = None;
            v.arrived_at = None;
            v.post_service_arrival = None;
            v.phase = "ORDER_RECEIVED".into();
            self.formation.journal.add("receipts");
            // Receipt updates the objective now; action selection occurs at the
            // next ordinary policy opportunity. A pending wake here would run
            // at an arbitrary caller checkpoint instead of a clock-defined time.
            self.log(Some(id),"UnitOrderReceived",format!("unit={unit}; serial={serial}; target={target}; no route/stock information supplied"));
        }
        self.audit()?;
        Ok(recipients.len())
    }
    pub(crate) fn record_unit_choice(&mut self, id: usize, d: &Decision) {
        if !self.has_unit_order(id) {
            return;
        }
        let target = self.formation.views[id].order.as_ref().unwrap().target;
        let phase = match d.intent {
            Intent::Travel {
                target: t,
                food: false,
                ..
            } if t == target => "WITH_UNIT",
            Intent::MilitaryDuty => "ARRIVED",
            Intent::Sleep => "PAUSED_FATIGUE",
            Intent::Eat { .. } => "PAUSED_EAT",
            Intent::GiveCareFood { .. } | Intent::TravelForCare { .. } => "PAUSED_CARE",
            Intent::Gather { .. } | Intent::Pickup { .. } | Intent::Travel { .. } => {
                "PAUSED_SELF_PROVISION"
            }
            Intent::Reproduce { .. } => "PAUSED_REPRODUCTION",
            Intent::Wait | Intent::CancelAction if d.reason.contains("PAUSED_") => {
                d.reason.split_once(": ").unwrap().1
            }
            Intent::Wait if matches!(self.agents[id].action, Action::Gather { .. }) => {
                "PAUSED_GATHER"
            }
            Intent::LeaveService => "VOLUNTARY_WITHDRAWAL",
            _ => "PAUSED_OTHER",
        }
        .to_string();
        if self.formation.views[id].phase != phase {
            self.formation.journal.add(&phase);
            self.formation.views[id].phase = phase.clone();
            self.log(
                Some(id),
                "UnitActionState",
                format!("{phase}; {}", d.reason),
            );
        }
    }
    /// Observation bookkeeping only. Never writes a route, inventory, physiology or position.
    pub(crate) fn refresh_formation(&mut self) {
        if !self.config.formation {
            return;
        }
        let mut events = vec![];
        for (id, v) in self.formation.views.iter_mut().enumerate() {
            let Some(o) = &v.order else { continue };
            let a = &self.agents[id];
            if v.active
                && v.departed_at.is_none()
                && a.action == Action::Travel
                && a.goal == Some((o.target, false))
            {
                v.departed_at = Some(self.now);
                events.push((id, "UnitDeparted", o.serial));
            }
            if a.location == Location::Node(o.target)
                && v.departed_at.is_some()
                && a.action != Action::Dead
            {
                if v.active && v.arrived_at.is_none() {
                    v.arrived_at = Some(self.now);
                    events.push((id, "UnitArrived", o.serial));
                } else if !v.active && v.arrived_at.is_none() && v.post_service_arrival.is_none() {
                    v.post_service_arrival = Some(self.now);
                    events.push((id, "UnitEndpointAfterRelease", o.serial));
                }
            }
        }
        for (id, kind, serial) in events {
            self.formation.journal.add(kind);
            self.log(
                Some(id),
                kind,
                format!("serial={serial}; physical state only"),
            );
        }
    }
    pub(crate) fn integrate_formation(&mut self, dt: Time) {
        if !self.config.formation {
            return;
        }
        for a in &self.agents {
            if a.action == Action::Dead {
                continue;
            }
            let v = &self.formation.views[a.id];
            let member = v.active;
            let j = &mut self.formation.journal;
            let deficit = (dt - a.nutrition).max(0) as u128;
            if member {
                j.member_starvation_ms += deficit;
                j.member_time_ms += dt as u128;
                j.member_fatigue_ms += a.fatigue as u128 * dt as u128;
                j.max_fatigue = j.max_fatigue.max(a.fatigue);
                if matches!(a.action, Action::Queued { .. }) {
                    j.member_queue_ms += dt as u128
                }
                if a.action == Action::Travel {
                    j.member_travel_ms += dt as u128
                }
                if v.phase.starts_with("PAUSED_") {
                    *j.pause_ms.entry(v.phase.clone()).or_default() += dt as u128
                }
            } else {
                j.civilian_starvation_ms += deficit;
                if matches!(a.action, Action::Queued { .. }) {
                    j.civilian_queue_ms += dt as u128
                }
                if a.action == Action::Travel {
                    j.civilian_travel_ms += dt as u128
                }
            }
        }
    }
    pub fn unit_snapshot(&self, unit: usize) -> Option<Snapshot> {
        let u = self.formation.units.get(&unit)?;
        let mut s = Snapshot {
            enlisted: u.members.len(),
            ..Default::default()
        };
        let mut nodes: BTreeMap<usize, usize> = BTreeMap::new();
        let mut positions = BTreeSet::new();
        let mut physical_clusters = BTreeMap::new();
        let mut times = vec![];
        for (&id, r) in &u.members {
            let a = &self.agents[id];
            let v = &self.formation.views[id];
            s.ever_assembled += usize::from(r.assembled_at.is_some());
            s.dead += usize::from(a.action == Action::Dead);
            s.released += usize::from(r.ended.is_some() && a.action != Action::Dead);
            if v.unit != Some(unit) {
                continue;
            }
            s.departed += usize::from(v.departed_at.is_some());
            s.arrived += usize::from(v.arrived_at.is_some());
            s.late_endpoint += usize::from(v.post_service_arrival.is_some());
            if let Some(t) = v.arrived_at {
                times.push(t)
            }
            if !v.active {
                continue;
            }
            s.active += 1;
            s.active_without_order += usize::from(v.order.is_none());
            s.paused_away += usize::from(
                v.phase.starts_with("PAUSED_")
                    && v.order
                        .as_ref()
                        .is_some_and(|o| a.location != Location::Node(o.target)),
            );
            *physical_clusters
                .entry(self.physical_position_key(id))
                .or_insert(0usize) += 1;
            s.at_muster += usize::from(a.location == Location::Node(u.muster));
            s.moving += usize::from(a.action == Action::Travel);
            s.queued += usize::from(matches!(a.action, Action::Queued { .. }));
            s.recovering += usize::from(matches!(a.action, Action::Sleep { .. }));
            if let Location::Node(n) = a.location {
                *nodes.entry(n).or_default() += 1;
                positions.insert(vec![n]);
            } else if let Location::Edge { from, to, .. } = a.location {
                positions.insert(vec![from, to]);
            }
            if let Some(o) = &u.objective {
                s.at_objective += usize::from(a.location == Location::Node(o.target));
                if let Location::Node(n) = a.location
                    && (n == o.target
                        || (0..self.config.edges.len())
                            .map(|id| self.effective_edge(id))
                            .any(|e| {
                                e.capacity > 0
                                    && ((e.a == n && e.b == o.target)
                                        || (e.b == n && e.a == o.target))
                            }))
                {
                    s.near_anchor_fraction += 1.0
                }
            }
        }
        let largest = nodes
            .iter()
            .max_by_key(|(n, q)| (**q, std::cmp::Reverse(**n)));
        if let Some((&n, &q)) = largest {
            s.modal_node = Some(n);
            s.modal_fraction = q as f64 / s.active as f64;
        }
        if s.active > 0 {
            let largest = physical_clusters.values().copied().max().unwrap_or(0);
            s.separated = s.active - largest;
            s.coincident_fraction = largest as f64 / s.active as f64;
            s.objective_fraction = s.at_objective as f64 / s.active as f64;
            s.near_anchor_fraction /= s.active as f64;
        }
        if !times.is_empty() {
            s.arrival_spread = Some(times.iter().max().unwrap() - times.iter().min().unwrap());
        }
        for a in &positions {
            for b in &positions {
                let hops = a
                    .iter()
                    .flat_map(|&x| b.iter().map(move |&y| (x, y)))
                    .filter_map(|(x, y)| self.graph_hops(x, y))
                    .min()
                    .unwrap_or(usize::MAX);
                s.max_endpoint_hops = s.max_endpoint_hops.max(hops);
            }
        }
        Some(s)
    }
    /// Exact rational progress in canonical edge direction. Research measurement only.
    fn physical_position_key(&self, id: usize) -> (u8, usize, u128, u128) {
        match self.agents[id].location {
            Location::Node(n) => (0, n, 0, 1),
            Location::Edge {
                edge,
                from,
                progress,
                ..
            } => {
                let total = self.traversal_nominal(id, edge) as u128 * SCALE;
                let p = if from == self.config.edges[edge].a {
                    progress
                } else {
                    total - progress
                };
                let (mut a, mut b) = (p, total);
                while b != 0 {
                    (a, b) = (b, a % b);
                }
                (1, edge, p / a, total / a)
            }
        }
    }
    fn graph_hops(&self, start: usize, to: usize) -> Option<usize> {
        let mut seen = BTreeSet::from([start]);
        let mut pending = std::collections::VecDeque::from([(start, 0)]);
        while let Some((n, d)) = pending.pop_front() {
            if n == to {
                return Some(d);
            }
            for id in 0..self.config.edges.len() {
                let e = self.effective_edge(id);
                if e.capacity == 0 {
                    continue;
                }
                let next = if e.a == n {
                    Some(e.b)
                } else if e.b == n {
                    Some(e.a)
                } else {
                    None
                };
                if let Some(x) = next
                    && seen.insert(x)
                {
                    pending.push_back((x, d + 1));
                }
            }
        }
        None
    }
    pub(crate) fn audit_formation(&self) -> Result<(), String> {
        if !self.config.formation {
            return Ok(());
        }
        if self.formation.views.len() != self.agents.len() {
            return Err("unit person cardinality".into());
        }
        for (&unit, u) in &self.formation.units {
            for (&id, r) in &u.members {
                if id != r.person || id >= self.agents.len() {
                    return Err("invalid unit member identity".into());
                }
                let v = &self.formation.views[id];
                // Multiple sequential historical units may retain the person; only the current view is active.
                if v.unit == Some(unit)
                    && v.active
                    && (self.military_role(id) != Some(unit)
                        || r.assembled_at.is_none()
                        || r.ended.is_some())
                {
                    return Err("unit active without physical assembly/service".into());
                }
                if r.assembled_at.is_some_and(|t| t < r.enlisted_at) {
                    return Err("assembly precedes enlistment".into());
                }
            }
        }
        Ok(())
    }
}
