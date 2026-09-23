//! Communal fixed infrastructure. Policy inputs below are personal observations only.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    pub edge: usize,
    pub node: usize,
    pub wood_cost: u64,
    pub work_ms: Time,
    pub opened_nominal_ms: Time,
    pub opened_capacity: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub enabled: bool,
    pub sites: Vec<Site>,
    pub wood: Vec<(usize, u64)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: usize,
    pub site: Site,
    pub owner: Owner,
    pub wood: u64,
    pub remaining: u128,
    pub started_at: Option<Time>,
    pub completed_at: Option<Time>,
    pub completed_by: Option<usize>,
    pub donors: BTreeMap<usize, u64>,
    pub labor_ms: BTreeMap<usize, u128>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct View {
    pub observed_at: Time,
    pub projects: Vec<(Project, Option<usize>)>,
    pub wood: Vec<capital::Wood>,
}
pub type Memory = BTreeMap<usize, View>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trip {
    pub agent: usize,
    pub edge: usize,
    pub from: usize,
    pub to: usize,
    pub admitted: Time,
    pub arrived: Option<Time>,
    pub nominal_ms: Time,
    pub improved: bool,
    pub food: u64,
    pub wood: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub projects: Vec<Project>,
    pub memories: Vec<Memory>,
    pub trips: Vec<Trip>,
    pub active_trips: BTreeMap<usize, usize>,
    pub attempts: u64,
    pub work_starts: u64,
    pub queue_ms: BTreeMap<usize, u128>,
    pub occupancy_ms: BTreeMap<usize, u128>,
}
impl State {
    pub fn new(
        c: Option<&Config>,
        edges: &[Edge],
        agents: usize,
        nodes: usize,
    ) -> Result<Self, String> {
        let Some(c) = c else {
            return Ok(Self::default());
        };
        let mut used = BTreeSet::new();
        if c.wood.iter().any(|(n, _)| *n >= nodes) {
            return Err("invalid crossing wood location".into());
        }
        let mut projects = vec![];
        for (id, site) in c.sites.iter().enumerate() {
            let e = edges.get(site.edge).ok_or("unknown crossing edge")?;
            if !used.insert(site.edge)
                || (site.node != e.a && site.node != e.b)
                || site.wood_cost == 0
                || site.work_ms <= 0
                || site.opened_nominal_ms <= 0
                || site.opened_nominal_ms > e.nominal_ms
                || site.opened_capacity == 0
                || (e.capacity > 0 && site.opened_capacity != e.capacity)
            {
                return Err("invalid crossing site/recipe/effect".into());
            }
            projects.push(Project {
                id,
                site: site.clone(),
                owner: Owner::Commons(0),
                wood: 0,
                remaining: site.work_ms as u128 * SCALE,
                started_at: None,
                completed_at: None,
                completed_by: None,
                donors: BTreeMap::new(),
                labor_ms: BTreeMap::new(),
            });
        }
        Ok(Self {
            projects,
            memories: vec![Memory::new(); agents],
            ..Self::default()
        })
    }
    pub fn embedded_wood(&self) -> u128 {
        self.projects.iter().map(|p| p.wood as u128).sum()
    }
}
impl World {
    pub fn crossing_projects(&self) -> &[Project] {
        &self.infrastructure.projects
    }
    pub fn crossing_trips(&self) -> &[Trip] {
        &self.infrastructure.trips
    }
    pub fn crossing_memory(&self, id: usize) -> Option<&Memory> {
        self.infrastructure.memories.get(id)
    }
    pub fn effective_edge(&self, edge: usize) -> Edge {
        let mut e = self.config.edges[edge].clone();
        if let Some(p) = self
            .infrastructure
            .projects
            .iter()
            .find(|p| p.site.edge == edge && p.completed_at.is_some())
        {
            e.nominal_ms = p.site.opened_nominal_ms;
            e.capacity = p.site.opened_capacity;
        }
        e.nominal_ms = self.path_duration(edge, e.nominal_ms);
        e
    }
    pub(super) fn traversal_nominal(&self, id: usize, edge: usize) -> Time {
        if let Some(t) = self.paths.active.get(&id) {
            return self.paths.trips[*t].nominal_ms;
        }
        self.infrastructure
            .active_trips
            .get(&id)
            .map_or(self.config.edges[edge].nominal_ms, |t| {
                self.infrastructure.trips[*t].nominal_ms
            })
    }
    pub(super) fn infrastructure_view(&self, node: usize, id: usize) -> View {
        View {
            observed_at: self.now,
            projects: self
                .infrastructure
                .projects
                .iter()
                .filter(|p| p.site.node == node)
                .map(|p| {
                    let worker = self
                        .agents
                        .iter()
                        .find(|a| a.action == Action::BuildCrossing { project: p.id })
                        .map(|a| a.id);
                    (p.clone(), worker)
                })
                .collect(),
            wood: self
                .goods
                .batches
                .iter()
                .filter(|b| {
                    b.material == Material::Wood
                        && b.quantity > 0
                        && (matches!(b.holder,Holder::Deposit(n)|Holder::Loose(n) if n==node)
                            || self.carried_by(id, b.holder))
                })
                .map(|b| capital::Wood {
                    batch: b.id,
                    available: b.quantity.saturating_sub(self.reserved(b.id)),
                    owner: b.owner,
                    holder: b.holder,
                })
                .collect(),
        }
    }
    pub(super) fn observe_infrastructure(&mut self, id: usize) {
        if self.config.infrastructure.is_none() {
            return;
        }
        if let Location::Node(n) = self.agents[id].location {
            let view = self.infrastructure_view(n, id);
            self.infrastructure.memories[id].insert(n, view);
        }
    }
    pub(super) fn donate_crossing(
        &mut self,
        id: usize,
        project: usize,
        batch: usize,
        quantity: u64,
    ) -> Result<(), String> {
        if !self
            .config
            .infrastructure
            .as_ref()
            .is_some_and(|c| c.enabled)
        {
            return Err("crossing unavailable".into());
        }
        let p = self
            .infrastructure
            .projects
            .get(project)
            .ok_or("unknown project")?;
        let b = self.goods.batches.get(batch).ok_or("unknown material")?;
        if self.agents[id].location != Location::Node(p.site.node)
            || p.completed_at.is_some()
            || b.owner != Owner::Agent(id)
            || b.material != Material::Wood
            || !self.carried_by(id, b.holder)
            || quantity == 0
            || quantity > b.quantity
            || quantity > p.site.wood_cost - p.wood
        {
            return Err("local voluntary material donation precondition".into());
        }
        self.cancel_action(id, "commit personal wood to communal crossing");
        self.goods.batches[batch].quantity -= quantity;
        let p = &mut self.infrastructure.projects[project];
        p.wood += quantity;
        *p.donors.entry(id).or_default() += quantity;
        p.started_at.get_or_insert(self.now);
        self.log(
            Some(id),
            "CrossingDonation",
            format!("project={project} q={quantity} owner=Commons(0) physical_embedded_wood"),
        );
        Ok(())
    }
    pub(super) fn begin_crossing(&mut self, id: usize, project: usize) -> Result<(), String> {
        if !self
            .config
            .infrastructure
            .as_ref()
            .is_some_and(|c| c.enabled)
        {
            return Err("crossing unavailable".into());
        }
        self.infrastructure.attempts += 1;
        let p = self
            .infrastructure
            .projects
            .get(project)
            .ok_or("unknown crossing")?;
        if self.agents[id].location != Location::Node(p.site.node)
            || p.completed_at.is_some()
            || p.wood != p.site.wood_cost
            || self
                .agents
                .iter()
                .any(|a| a.id != id && a.action == Action::BuildCrossing { project })
        {
            return Err("local construction input/work-position precondition".into());
        }
        if self.agents[id].action == (Action::BuildCrossing { project }) {
            return Ok(());
        }
        self.cancel_action(id, "work communal crossing");
        self.agents[id].action = Action::BuildCrossing { project };
        self.infrastructure.work_starts += 1;
        self.log(
            Some(id),
            "CrossingWorkStarted",
            format!("project={project}; labor confers no personal title"),
        );
        Ok(())
    }
    pub(super) fn finish_crossings(&mut self) {
        for id in 0..self.agents.len() {
            if let Action::BuildCrossing { project } = self.agents[id].action {
                let p = &mut self.infrastructure.projects[project];
                if p.remaining == 0 && p.completed_at.is_none() {
                    p.completed_at = Some(self.now);
                    p.completed_by = Some(id);
                    self.agents[id].action = Action::Idle;
                    self.wake.insert(id);
                    self.log(Some(id),"CrossingCompleted",format!("project={project} owner=Commons(0); adjacent observation required; existing traversals unchanged"));
                }
            }
        }
    }
    pub(super) fn infrastructure_departure(
        &mut self,
        id: usize,
        edge: usize,
        from: usize,
        to: usize,
    ) {
        if self.config.infrastructure.is_none() {
            return;
        }
        let e = self.effective_edge(edge);
        let quantity = |mat| {
            self.goods
                .batches
                .iter()
                .filter(|b| b.material == mat && self.carried_by(id, b.holder))
                .map(|b| b.quantity)
                .sum()
        };
        let t = Trip {
            agent: id,
            edge,
            from,
            to,
            admitted: self.now,
            arrived: None,
            nominal_ms: e.nominal_ms,
            improved: self
                .infrastructure
                .projects
                .iter()
                .any(|p| p.site.edge == edge && p.completed_at.is_some()),
            food: quantity(Material::Food),
            wood: quantity(Material::Wood),
        };
        self.infrastructure
            .active_trips
            .insert(id, self.infrastructure.trips.len());
        self.infrastructure.trips.push(t);
    }
    pub(super) fn infrastructure_arrival(&mut self, id: usize) {
        if let Some(t) = self.infrastructure.active_trips.remove(&id) {
            self.infrastructure.trips[t].arrived = Some(self.now);
        }
    }
    pub(super) fn audit_infrastructure(&self) -> Result<(), String> {
        if self.config.infrastructure.is_none() {
            return if self.infrastructure == State::default() {
                Ok(())
            } else {
                Err("infrastructure without config".into())
            };
        }
        for (id, p) in self.infrastructure.projects.iter().enumerate() {
            if p.id != id
                || p.owner != Owner::Commons(0)
                || p.wood > p.site.wood_cost
                || p.donors.values().sum::<u64>() != p.wood
                || (p.completed_at.is_some()
                    && (p.remaining != 0 || p.wood != p.site.wood_cost || p.completed_by.is_none()))
            {
                return Err("crossing transformation/ownership ledger".into());
            }
            let workers: Vec<_> = self
                .agents
                .iter()
                .filter(|a| a.action == Action::BuildCrossing { project: id })
                .collect();
            if workers.len() > 1
                || workers
                    .iter()
                    .any(|a| a.location != Location::Node(p.site.node))
            {
                return Err("crossing labor locality/exclusivity".into());
            }
        }
        for a in &self.agents {
            if let Action::BuildCrossing { project } = a.action {
                let p = self
                    .infrastructure
                    .projects
                    .get(project)
                    .ok_or("unknown active project")?;
                if p.completed_at.is_some() || p.wood != p.site.wood_cost {
                    return Err("crossing work without inputs".into());
                }
            }
            if a.action == Action::Travel && !self.infrastructure.active_trips.contains_key(&a.id) {
                return Err("travel without admission contract".into());
            }
        }
        Ok(())
    }
    pub fn infrastructure_edge_time(&self, edge: usize) -> (u128, u128) {
        (
            *self.infrastructure.queue_ms.get(&edge).unwrap_or(&0),
            *self.infrastructure.occupancy_ms.get(&edge).unwrap_or(&0),
        )
    }
    pub fn infrastructure_counters(&self) -> BTreeMap<String, u128> {
        let s = &self.infrastructure;
        [
            ("attempts", s.attempts as u128),
            ("work_starts", s.work_starts as u128),
            (
                "started",
                s.projects.iter().filter(|p| p.started_at.is_some()).count() as u128,
            ),
            (
                "completed",
                s.projects
                    .iter()
                    .filter(|p| p.completed_at.is_some())
                    .count() as u128,
            ),
            ("wood", s.embedded_wood()),
            (
                "labor_ms",
                s.projects.iter().flat_map(|p| p.labor_ms.values()).sum(),
            ),
            (
                "improved_trips",
                s.trips
                    .iter()
                    .filter(|t| t.improved && t.arrived.is_some())
                    .count() as u128,
            ),
            (
                "improved_users",
                s.trips
                    .iter()
                    .filter(|t| t.improved && t.arrived.is_some())
                    .map(|t| t.agent)
                    .collect::<BTreeSet<_>>()
                    .len() as u128,
            ),
            (
                "improved_food_flow",
                s.trips
                    .iter()
                    .filter(|t| t.improved && t.arrived.is_some())
                    .map(|t| t.food as u128)
                    .sum(),
            ),
            (
                "wood_transport",
                s.trips
                    .iter()
                    .filter(|t| t.arrived.is_some())
                    .map(|t| t.wood as u128)
                    .sum(),
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v))
        .collect()
    }
}

/// A conditional investment heuristic. No hidden state or configuration enters.
pub(crate) fn consider(
    o: &Observation,
    m: &PolicyMemory,
    mem: &Memory,
    enabled: bool,
) -> Option<Decision> {
    if !enabled {
        return None;
    }
    let Location::Node(node) = o.location else {
        return None;
    };
    let route = |o: &Observation, m: &PolicyMemory, target| policy::coherent::route(o, m, target);
    for view in mem.values() {
        for (p, worker) in &view.projects {
            if p.completed_at.is_some() {
                continue;
            }
            let Some(to_site) = route(o, m, p.site.node) else {
                continue;
            };
            if to_site.starvation >= 24 * HOUR {
                continue;
            }
            let mut at = o.clone();
            at.location = Location::Node(p.site.node);
            at.nutrition = to_site.nutrition;
            at.fatigue = (o.fatigue + to_site.time).min(24 * HOUR);
            at.starvation = to_site.starvation;
            let mut improved = m.clone();
            for v in improved.nodes.values_mut() {
                for e in &mut v.edges {
                    if e.id == p.site.edge {
                        e.edge.nominal_ms = p.site.opened_nominal_ms;
                        e.edge.capacity = p.site.opened_capacity;
                    }
                }
            }
            let Some((remaining, _, _)) = policy::coherent::capital_work(&at, 0, p.remaining)
            else {
                continue;
            };
            let mut worthwhile = false;
            for source in m.nodes.values() {
                let Some(after) = route(&at, &improved, source.node) else {
                    continue;
                };
                if after.starvation >= 24 * HOUR {
                    continue;
                }
                let q: u64 = source
                    .food
                    .iter()
                    .filter(|f| {
                        f.expires_at > o.now + to_site.time + remaining + after.time
                            && (f.owner == Owner::None
                                || f.owner == Owner::Commons(0)
                                || f.owner == Owner::Agent(o.agent))
                    })
                    .map(|f| f.available)
                    .sum();
                if q == 0 {
                    continue;
                }
                match route(&at, m, source.node) {
                    None => worthwhile = true,
                    Some(before) => {
                        let demand: u64 = m
                            .nodes
                            .values()
                            .filter_map(|n| n.board.as_ref())
                            .map(|b| b.unclaimed(o.now))
                            .sum();
                        let trips = q.min(demand).div_ceil(o.capacity);
                        if before
                            .time
                            .saturating_sub(after.time)
                            .saturating_mul(2)
                            .saturating_mul(trips as Time)
                            > remaining
                        {
                            worthwhile = true;
                        }
                    }
                }
            }
            if !worthwhile {
                continue;
            }
            let own: Vec<_> = mem
                .get(&node)?
                .wood
                .iter()
                .filter(|w| {
                    w.owner == Owner::Agent(o.agent)
                        && matches!(w.holder, Holder::Agent(_) | Holder::Cart(_))
                })
                .collect();
            let missing = p.site.wood_cost - p.wood;
            let intent = if missing > 0 {
                if let Some(w) = own.first() {
                    if node == p.site.node {
                        Intent::ContributeCrossing {
                            project: p.id,
                            batch: w.batch,
                            quantity: w.available.min(missing),
                        }
                    } else {
                        Intent::Travel {
                            edge: to_site.first,
                            target: p.site.node,
                            food: false,
                        }
                    }
                } else {
                    let mut sources: Vec<_> = mem
                        .iter()
                        .filter_map(|(n, v)| {
                            let w = v.wood.iter().find(|w| {
                                w.available > 0
                                    && (w.owner == Owner::None || w.owner == Owner::Agent(o.agent))
                            })?;
                            let r = route(o, m, *n)?;
                            (r.starvation < 24 * HOUR).then_some((*n, w, r))
                        })
                        .collect();
                    sources.sort_by_key(|(_, _, r)| r.time);
                    let Some((n, w, r)) = sources.first() else {
                        continue;
                    };
                    if *n != node {
                        Intent::Travel {
                            edge: r.first,
                            target: *n,
                            food: false,
                        }
                    } else if o.load >= o.capacity {
                        continue;
                    } else if matches!(w.holder, Holder::Deposit(_)) {
                        Intent::Gather {
                            batch: w.batch,
                            communal: false,
                        }
                    } else {
                        Intent::Pickup {
                            batch: w.batch,
                            quantity: 1,
                        }
                    }
                }
            } else {
                if worker.is_some_and(|id| id != o.agent) {
                    continue;
                }
                if node != p.site.node {
                    Intent::Travel {
                        edge: to_site.first,
                        target: p.site.node,
                        food: false,
                    }
                } else {
                    let chunk = p.remaining.min(HOUR as u128 * SCALE);
                    let (_, nutrition, _) = policy::coherent::capital_work(o, 0, chunk)?;
                    if nutrition < 6 * HOUR {
                        continue;
                    }
                    Intent::BuildCrossing { project: p.id }
                }
            };
            return Some(Decision {
                intent,
                reason: format!(
                    "voluntary communal crossing project={}; conditional own food access/remembered transport savings; no guaranteed outcome",
                    p.id
                ),
            });
        }
    }
    None
}
#[cfg(test)]
mod tests;
