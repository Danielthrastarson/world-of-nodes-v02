//! Use-created terrain state. All traffic still uses the common physical scheduler.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub enabled: bool,
    pub ground_edges: Vec<usize>,
    pub recovery_ms: Time,
}
impl Config {
    pub fn ground(edges: Vec<usize>, enabled: bool) -> Self {
        Self {
            enabled,
            ground_edges: edges,
            recovery_ms: 6 * HOUR,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Surface {
    pub edge: usize,
    pub passages: u64,
    pub compaction: u64,
    pub recovery_count: u64,
    pub last_passage: Option<Time>,
    pub next_recovery: Option<Time>,
}
impl Surface {
    pub fn stage(&self) -> &'static str {
        match self.compaction {
            0 => "bare",
            1..=49 => "trail",
            _ => "worn_path",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trip {
    pub agent: usize,
    pub edge: usize,
    pub from: usize,
    pub to: usize,
    pub admitted: Time,
    pub arrived: Option<Time>,
    pub nominal_ms: Time,
    pub untreated_ms: Time,
    pub compaction_at_admission: u64,
    pub food: u64,
    pub load: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub surfaces: BTreeMap<usize, Surface>,
    pub trips: Vec<Trip>,
    pub active: BTreeMap<usize, usize>,
    pub queue_ms: BTreeMap<usize, u128>,
    pub occupancy_ms: BTreeMap<usize, u128>,
}
impl State {
    pub fn new(
        c: Option<&Config>,
        edges: &[Edge],
        crossings: Option<&infrastructure::Config>,
    ) -> Result<Self, String> {
        let Some(c) = c else {
            return Ok(Self::default());
        };
        if c.recovery_ms <= 0 {
            return Err("invalid surface recovery duration".into());
        }
        let mut surfaces = BTreeMap::new();
        for &edge in &c.ground_edges {
            if edges.get(edge).is_none_or(|e| e.capacity == 0)
                || surfaces.contains_key(&edge)
                || crossings.is_some_and(|x| x.sites.iter().any(|p| p.edge == edge))
            {
                return Err(
                    "path must be unique traversable ground, not a crossing project".into(),
                );
            }
            surfaces.insert(
                edge,
                Surface {
                    edge,
                    passages: 0,
                    compaction: 0,
                    recovery_count: 0,
                    last_passage: None,
                    next_recovery: None,
                },
            );
        }
        Ok(Self {
            surfaces,
            ..Self::default()
        })
    }
    pub fn next_boundary(&self, now: Time, target: Time) -> Time {
        self.surfaces
            .values()
            .filter_map(|s| s.next_recovery)
            .filter(|t| *t > now)
            .fold(target, Time::min)
    }
}
impl World {
    pub fn path_surfaces(&self) -> &BTreeMap<usize, Surface> {
        &self.paths.surfaces
    }
    pub fn path_trips(&self) -> &[Trip] {
        &self.paths.trips
    }
    pub fn path_edge_time(&self, edge: usize) -> (u128, u128) {
        (
            *self.paths.queue_ms.get(&edge).unwrap_or(&0),
            *self.paths.occupancy_ms.get(&edge).unwrap_or(&0),
        )
    }
    pub(super) fn path_duration(&self, edge: usize, nominal: Time) -> Time {
        let score = self.paths.surfaces.get(&edge).map_or(0, |s| s.compaction);
        ((nominal as u128 * (500 - score as u128)).div_ceil(500)) as Time
    }
    pub(super) fn path_departure(&mut self, id: usize, edge: usize, from: usize, to: usize) {
        if self.config.paths.is_none() {
            return;
        }
        let e = self.effective_edge(edge);
        let food = self
            .goods
            .batches
            .iter()
            .filter(|b| b.material == Material::Food && self.carried_by(id, b.holder))
            .map(|b| b.quantity)
            .sum();
        // Ground cannot overlap a crossing; non-ground trips already have their
        // authoritative crossing duration and receive no path reduction.
        let untreated = if self.paths.surfaces.contains_key(&edge) {
            self.config.edges[edge].nominal_ms
        } else {
            e.nominal_ms
        };
        let t = Trip {
            agent: id,
            edge,
            from,
            to,
            admitted: self.now,
            arrived: None,
            nominal_ms: e.nominal_ms,
            untreated_ms: untreated,
            compaction_at_admission: self.paths.surfaces.get(&edge).map_or(0, |s| s.compaction),
            food,
            load: self.carrier_load(id),
        };
        self.paths.active.insert(id, self.paths.trips.len());
        self.paths.trips.push(t);
    }
    pub(super) fn path_arrival(&mut self, id: usize) {
        let Some(t) = self.paths.active.remove(&id) else {
            return;
        };
        self.paths.trips[t].arrived = Some(self.now);
        let edge = self.paths.trips[t].edge;
        let Some(s) = self.paths.surfaces.get_mut(&edge) else {
            return;
        };
        s.passages += 1;
        s.last_passage = Some(self.now);
        let c = self.config.paths.as_ref().unwrap();
        if c.enabled {
            let before = s.compaction;
            s.compaction = (s.compaction + 1).min(100);
            s.next_recovery = Some(self.now.saturating_add(c.recovery_ms));
            if before != s.compaction {
                let after = s.compaction;
                self.log(Some(id),"PathTread",format!("edge={edge} completed_person_passage compaction={before}->{after}; capacity unchanged"));
            }
        }
    }
    pub(super) fn recover_paths(&mut self) {
        let Some(c) = &self.config.paths else {
            return;
        };
        if !c.enabled {
            return;
        }
        let mut events = vec![];
        for s in self.paths.surfaces.values_mut() {
            if let Some(t) = s.next_recovery
                && t <= self.now
            {
                let before = s.compaction;
                s.compaction = s.compaction.saturating_sub(1);
                s.recovery_count += 1;
                s.next_recovery = (s.compaction > 0).then(|| t.saturating_add(c.recovery_ms));
                events.push((s.edge, before, s.compaction));
            }
        }
        for (edge, before, after) in events {
            self.log(
                None,
                "PathRecovery",
                format!("edge={edge} disuse compaction={before}->{after}; no remote memory update"),
            );
        }
    }
    pub(super) fn integrate_paths(&mut self, dt: Time) {
        if self.config.paths.is_none() {
            return;
        }
        for a in &self.agents {
            match a.action {
                Action::Queued { edge, .. } => {
                    *self.paths.queue_ms.entry(edge).or_default() += dt as u128
                }
                Action::Travel => {
                    if let Location::Edge { edge, .. } = a.location {
                        *self.paths.occupancy_ms.entry(edge).or_default() += dt as u128;
                    }
                }
                _ => {}
            }
        }
    }
    pub(super) fn audit_paths(&self) -> Result<(), String> {
        let Some(c) = &self.config.paths else {
            return if self.paths == State::default() {
                Ok(())
            } else {
                Err("paths without configuration".into())
            };
        };
        for (&edge, s) in &self.paths.surfaces {
            if s.edge != edge
                || s.compaction > 100
                || s.compaction > s.passages
                || (!c.enabled && (s.compaction != 0 || s.next_recovery.is_some()))
                || self.effective_edge(edge).capacity != self.config.edges[edge].capacity
            {
                return Err("path surface/capacity invariant".into());
            }
            if s.next_recovery.is_some_and(|t| t <= self.now) {
                return Err("overdue surface recovery".into());
            }
        }
        for a in &self.agents {
            match self.paths.active.get(&a.id) {
                Some(t) => {
                    let t = self
                        .paths
                        .trips
                        .get(*t)
                        .ok_or("missing traversal contract")?;
                    if a.action != Action::Travel
                        || t.agent != a.id
                        || t.arrived.is_some()
                        || !matches!(a.location,Location::Edge{edge,..}if edge==t.edge)
                        || t.nominal_ms <= 0
                        || t.nominal_ms > t.untreated_ms
                    {
                        return Err("path traversal identity/locality".into());
                    }
                }
                None if a.action == Action::Travel => {
                    return Err("missing admission-time movement work".into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests;
