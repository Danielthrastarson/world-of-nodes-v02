//! F10: institutional records and local validation, never an autonomous planner.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub polity_id: usize,
    pub sovereign: usize,
    pub settlements: Vec<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polity {
    pub id: usize,
    pub sovereign: usize,
    pub commons: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OfficeStatus {
    Appointed,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Office {
    pub id: usize,
    pub polity: usize,
    pub holder: usize,
    pub settlement: usize,
    pub status: OfficeStatus,
    pub appointed_at: Time,
    pub appointed_by: usize,
    pub appointment_node: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commission {
    pub project: usize,
    pub office: usize,
    pub polity: usize,
    pub issuer: usize,
    pub node: usize,
    pub issued_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalView {
    pub observed_at: Time,
    pub node: usize,
    pub present_offices: Vec<Office>,
    pub posted_commissions: Vec<Commission>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub at: Time,
    pub office: usize,
    pub active: bool,
    pub reason: &'static str,
}
/// Observer-only metrics: neither policy inputs nor institutional authority.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Journal {
    pub active_ms: BTreeMap<usize, u128>,
    pub transitions: Vec<Transition>,
    pub travel_ms: BTreeMap<usize, u128>,
    pub queue_ms: BTreeMap<usize, u128>,
    active: BTreeSet<usize>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub polity: Option<Polity>,
    pub offices: Vec<Office>,
    pub commissions: Vec<Commission>,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &super::Config) -> Result<Self, String> {
        let Some(a) = &c.authority else {
            return Ok(Self::default());
        };
        if !c.communal
            || a.sovereign >= c.starts.len()
            || a.settlements.is_empty()
            || a.settlements.iter().any(|n| *n >= c.nodes)
            || a.settlements.iter().collect::<BTreeSet<_>>().len() != a.settlements.len()
            || c.infrastructure
                .as_ref()
                .is_some_and(|i| i.sites.iter().any(|s| !a.settlements.contains(&s.node)))
        {
            return Err("invalid F10 polity/settlement configuration".into());
        }
        Ok(Self {
            polity: Some(Polity {
                id: a.polity_id,
                sovereign: a.sovereign,
                commons: 0,
            }),
            ..Self::default()
        })
    }
}
impl World {
    /// Authoritative server/research record. Not a knowledge query for NPCs.
    pub fn polity(&self) -> Option<&Polity> {
        self.authority.polity.as_ref()
    }
    pub fn offices(&self) -> &[Office] {
        &self.authority.offices
    }
    pub fn commissions(&self) -> &[Commission] {
        &self.authority.commissions
    }
    /// Assignment communicated at the physical appointment, not remote office intelligence.
    /// These immutable personal records remain available while travelling.
    pub fn known_appointments(&self, id: usize) -> Vec<Office> {
        if self.agents.get(id).is_none_or(|a| a.action == Action::Dead) {
            return vec![];
        }
        self.offices()
            .iter()
            .filter(|o| o.holder == id)
            .cloned()
            .collect()
    }
    pub fn authority_journal(&self) -> &Journal {
        &self.authority.journal
    }
    pub fn office_active(&self, office: usize) -> bool {
        self.authority.offices.get(office).is_some_and(|o| {
            let a = &self.agents[o.holder];
            o.status == OfficeStatus::Appointed
                && a.action != Action::Dead
                && a.location == Location::Node(o.settlement)
        })
    }
    /// Derived true footprint. Never fed to autonomous policy or a player map.
    pub fn administered_settlements(&self) -> BTreeSet<usize> {
        self.offices()
            .iter()
            .filter(|o| self.office_active(o.id))
            .map(|o| o.settlement)
            .collect()
    }
    /// Only local institutional observation. No remote holder state or postings.
    pub fn observe_authority(&self, id: usize) -> Option<LocalView> {
        let a = self.agents.get(id)?;
        if a.action == Action::Dead || self.polity().is_none() {
            return None;
        }
        let Location::Node(node) = a.location else {
            return None;
        };
        Some(LocalView {
            observed_at: self.now,
            node,
            present_offices: self
                .offices()
                .iter()
                .filter(|o| o.settlement == node && self.office_active(o.id))
                .cloned()
                .collect(),
            posted_commissions: self
                .commissions()
                .iter()
                .filter(|c| c.node == node)
                .cloned()
                .collect(),
        })
    }
    pub(super) fn appoint_steward(
        &mut self,
        issuer: usize,
        candidate: usize,
        settlement: usize,
    ) -> Result<(), String> {
        if self.is_child(issuer) || candidate < self.agents.len() && self.is_child(candidate) {
            return Err("office requires adult capability".into());
        }
        let p = self.polity().ok_or("no polity")?;
        let a = self.agents.get(candidate).ok_or("unknown candidate")?;
        if issuer != p.sovereign || self.agents[issuer].action == Action::Dead {
            return Err("living sovereign required".into());
        }
        let Location::Node(node) = self.agents[issuer].location else {
            return Err("appointment requires a node".into());
        };
        if a.action == Action::Dead || a.location != Location::Node(node) {
            return Err("living candidate must be physically colocated".into());
        }
        if !self
            .config
            .authority
            .as_ref()
            .unwrap()
            .settlements
            .contains(&settlement)
        {
            return Err("unregistered settlement".into());
        }
        if self
            .offices()
            .iter()
            .any(|o| o.holder == candidate && o.settlement == settlement)
        {
            return Err("appointment already recorded".into());
        }
        let office = Office {
            id: self.offices().len(),
            polity: p.id,
            holder: candidate,
            settlement,
            status: OfficeStatus::Appointed,
            appointed_at: self.now,
            appointed_by: issuer,
            appointment_node: node,
        };
        // Appointment conveys an institutional identity, not resources, a route, or a goal.
        self.authority.offices.push(office.clone());
        self.cancel_action(issuer, "local appointment");
        self.log(Some(issuer), "StewardAppointed", format!("{office:?}"));
        Ok(())
    }
    pub(super) fn commission_crossing(
        &mut self,
        issuer: usize,
        office: usize,
        project: usize,
    ) -> Result<(), String> {
        let o = self.offices().get(office).ok_or("unknown office")?;
        if o.holder != issuer || !self.office_active(office) {
            return Err("active local steward required".into());
        }
        let p = self
            .infrastructure
            .projects
            .get(project)
            .ok_or("unknown local project")?;
        if p.site.node != o.settlement
            || p.completed_at.is_some()
            || !self
                .config
                .infrastructure
                .as_ref()
                .is_some_and(|i| i.enabled)
        {
            return Err("project must be available at administered settlement".into());
        }
        if self.commissions().iter().any(|c| c.project == project) {
            return Err("already commissioned".into());
        }
        let c = Commission {
            project,
            office,
            polity: o.polity,
            issuer,
            node: o.settlement,
            issued_at: self.now,
        };
        self.authority.commissions.push(c.clone());
        self.cancel_action(issuer, "local commission");
        self.log(
            Some(issuer),
            "CrossingCommissioned",
            format!("{c:?}; no resources or workers assigned"),
        );
        Ok(())
    }
    pub(super) fn crossing_authorized(&self, project: usize) -> bool {
        self.config.authority.is_none() || self.commissions().iter().any(|c| c.project == project)
    }
    pub(super) fn integrate_authority(&mut self, dt: Time) {
        if self.config.authority.is_none() {
            return;
        }
        for n in self.administered_settlements() {
            *self.authority.journal.active_ms.entry(n).or_default() += dt as u128;
        }
        let holders: BTreeSet<_> = self.offices().iter().map(|o| o.holder).collect();
        for id in holders {
            let map = match self.agents[id].action {
                Action::Travel => &mut self.authority.journal.travel_ms,
                Action::Queued { .. } => &mut self.authority.journal.queue_ms,
                _ => continue,
            };
            *map.entry(id).or_default() += dt as u128;
        }
    }
    pub(super) fn refresh_authority(&mut self) {
        if self.config.authority.is_none() {
            return;
        }
        let active: BTreeSet<_> = self
            .offices()
            .iter()
            .filter(|o| self.office_active(o.id))
            .map(|o| o.id)
            .collect();
        for &office in active.symmetric_difference(&self.authority.journal.active) {
            let is_active = active.contains(&office);
            let dead = self.agents[self.offices()[office].holder].action == Action::Dead;
            self.authority.journal.transitions.push(Transition {
                at: self.now,
                office,
                active: is_active,
                reason: if is_active {
                    "PRESENT"
                } else if dead {
                    "HOLDER_DIED"
                } else {
                    "HOLDER_DEPARTED"
                },
            });
        }
        self.authority.journal.active = active;
    }
    pub(super) fn audit_authority(&self) -> Result<(), String> {
        let Some(c) = &self.config.authority else {
            return if self.authority == State::default() {
                Ok(())
            } else {
                Err("authority without institution".into())
            };
        };
        if self.polity()
            != Some(&Polity {
                id: c.polity_id,
                sovereign: c.sovereign,
                commons: 0,
            })
        {
            return Err("polity identity changed".into());
        }
        for (id, o) in self.offices().iter().enumerate() {
            if o.id != id
                || o.holder >= self.agents.len()
                || o.polity != c.polity_id
                || o.appointed_by != c.sovereign
                || !c.settlements.contains(&o.settlement)
                || o.appointed_at > self.now
                || o.appointment_node >= self.config.nodes
            {
                return Err("invalid office".into());
            }
        }
        let mut projects = BTreeSet::new();
        for a in self.commissions() {
            let Some(o) = self.offices().get(a.office) else {
                return Err("commission without office".into());
            };
            if self
                .infrastructure
                .projects
                .get(a.project)
                .is_none_or(|p| p.site.node != a.node)
                || o.holder != a.issuer
                || o.polity != a.polity
                || o.settlement != a.node
                || a.issued_at < o.appointed_at
                || a.issued_at > self.now
                || !projects.insert(a.project)
            {
                return Err("invalid commission".into());
            }
        }
        for p in &self.infrastructure.projects {
            if (p.wood > 0 || p.started_at.is_some() || p.completed_at.is_some())
                && !self.crossing_authorized(p.id)
            {
                return Err("unauthorized public project execution".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests;
