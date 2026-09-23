//! Provisioning model P1. The legacy kernel remains available for historical reproduction.
pub mod acceptance;
pub mod authority;
pub mod barter;
pub mod capital;
pub mod coherence;
pub mod combat;
pub mod congestion;
pub mod decomposition;
pub mod experiment;
pub mod farming;
pub mod formation;
pub mod geography;
mod goods;
pub mod household;
pub mod information;
pub mod infrastructure;
pub mod lifecycle;
pub mod meals;
pub mod military;
pub mod paths;
pub mod persistence;
pub mod person;
mod policy;
pub mod reasoning;
mod runtime;
pub mod settlement_research;
pub mod shadow;
pub mod social;
use goods::Goods;
pub use goods::{Batch, Holder, Material, Owner};
pub use person::Person;
pub use policy::{Decision, Intent, PolicyMemory};
use runtime::awake_threshold;
use std::collections::{BTreeMap, BTreeSet};
pub type Time = i64;
pub const SECOND: Time = 1000;
pub const MINUTE: Time = 60 * SECOND;
pub const HOUR: Time = 60 * MINUTE;
pub const SCALE: u128 = 1_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
    pub nominal_ms: Time,
    pub capacity: usize,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiagnosticPolicy {
    #[default]
    Current,
    /// Explicit historical estimator; never the normal policy.
    LegacyForecast,
    /// Research only: corrected mechanics with conditional legal meals.
    ConditionalMeals,
    QueueCommitment,
    SleepCommitment,
    JobCommitment,
    MealAware,
    ZeroMargin,
    ThreeHourMargin,
    CoherentForecast,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub information: bool,
    pub combat: Option<combat::Config>,
    pub formation: bool,
    pub military: Option<military::Config>,
    pub lifecycle: Option<lifecycle::Config>,
    pub social: Option<social::Config>,
    pub household: Option<household::Config>,
    pub authority: Option<authority::Config>,
    pub barter: bool,
    /// F9B local market institution. Empty retains historical direct barter.
    pub market_nodes: Vec<usize>,
    pub paths: Option<paths::Config>,
    pub infrastructure: Option<infrastructure::Config>,
    pub capital: Option<capital::Config>,
    pub farming: Option<farming::Config>,
    pub persistence: Option<persistence::Arm>,
    pub diagnostic_policy: DiagnosticPolicy,
    pub initial_request: u64,
    pub acceptance_audit: bool,
    pub seed: u64,
    pub nodes: usize,
    pub edges: Vec<Edge>,
    pub starts: Vec<usize>,
    pub food: Vec<(usize, u64, Time, Time)>,
    pub boards: Vec<usize>,
    pub communal: bool,
    pub prior_memory: bool,
    pub policy_ms: Time,
    pub carrying_capacity: u64,
    pub initial_nutrition: Time,
    pub initial_fatigue: Time,
    pub reverse_evaluation: bool,
    pub trace: bool,
    pub autonomous: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            information: false,
            combat: None,
            formation: false,
            military: None,
            lifecycle: None,
            social: None,
            household: None,
            authority: None,
            barter: false,
            market_nodes: vec![],
            paths: None,
            infrastructure: None,
            capital: None,
            farming: None,
            persistence: None,
            diagnostic_policy: DiagnosticPolicy::Current,
            initial_request: 0,
            acceptance_audit: false,
            seed: 0,
            nodes: 3,
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
                Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
            ],
            starts: vec![0; 50],
            food: vec![(2, 3000, 0, 48 * HOUR)],
            boards: vec![0],
            communal: true,
            prior_memory: false,
            policy_ms: MINUTE,
            carrying_capacity: 10,
            initial_nutrition: 12 * HOUR,
            initial_fatigue: 4 * HOUR,
            reverse_evaluation: false,
            trace: false,
            autonomous: true,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Location {
    Node(usize),
    Edge {
        edge: usize,
        from: usize,
        to: usize,
        progress: u128,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Strike {
        target: usize,
        remaining: u128,
    },
    RaidStock {
        batch: usize,
        remaining: u128,
    },
    MilitaryDuty {
        unit: usize,
    },
    Reproduce {
        partner: usize,
        until: Time,
    },
    BuildCrossing {
        project: usize,
    },
    BuildCart {
        project: usize,
    },
    Farm {
        plot: usize,
    },
    Idle,
    Queued {
        edge: usize,
        to: usize,
        joined: Time,
    },
    Travel,
    Gather {
        batch: usize,
        remaining: u128,
    },
    Sleep {
        until: Time,
    },
    Dead,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Job {
    pub request: usize,
    pub destination: usize,
    pub target: u64,
    pub acquired: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    pub id: usize,
    pub location: Location,
    pub action: Action,
    pub nutrition: Time,
    pub fatigue: Time,
    pub starvation: Time,
    pub job: Option<Job>,
    pub memory: PolicyMemory,
    pub rng: u64,
    pub goal: Option<(usize, bool)>,
    pub last_event: Option<u64>,
    pub death_time: Option<Time>,
    pub consumed: u64,
    pub travel_since_food: u128,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    pub agent: usize,
    pub quantity: u64,
    pub expires_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub node: usize,
    pub quantity: u64,
    pub created_at: Time,
    pub refreshed_at: Time,
    pub claims: Vec<Claim>,
}
impl Board {
    pub fn unclaimed(&self, now: Time) -> u64 {
        self.quantity.saturating_sub(
            self.claims
                .iter()
                .filter(|c| c.expires_at > now)
                .map(|c| c.quantity)
                .sum(),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoodView {
    pub batch: usize,
    pub quantity: u64,
    pub available: u64,
    pub owner: Owner,
    pub holder: Holder,
    pub expires_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeView {
    pub id: usize,
    pub edge: Edge,
    pub queued: usize,
    pub occupied: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeView {
    pub node: usize,
    pub observed_at: Time,
    pub food: Vec<FoodView>,
    pub edges: Vec<EdgeView>,
    pub board: Option<Board>,
    pub people: Vec<usize>,
}
#[derive(Clone, Debug)]
pub struct Observation {
    pub body_factor: u128,
    pub raider: bool,
    pub now: Time,
    pub agent: usize,
    pub location: Location,
    pub action: Action,
    pub nutrition: Time,
    pub fatigue: Time,
    pub starvation: Time,
    pub own: Vec<FoodView>,
    pub load: u64,
    pub capacity: u64,
    pub local: Option<NodeView>,
    pub job: Option<Job>,
    pub goal: Option<(usize, bool)>,
    pub communal: bool,
    pub decision_interval: Time,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Metrics {
    pub deaths: u64,
    pub gathered: u64,
    pub delivered: u64,
    pub consumed: u64,
    pub spoiled: u64,
    pub spoiled_transit: u64,
    pub spoiled_commons: u64,
    pub carried_at_death: u64,
    pub stranded: u64,
    pub request_ms: u128,
    pub request_quantum_ms: u128,
    pub jobs: u64,
    pub cancellations: u64,
    pub stale_failures: u64,
    pub infeasible_abandons: u64,
    pub queue_ms: u128,
    pub loaded_queue_ms: u128,
    pub unloaded_queue_ms: u128,
    pub overtakes: u64,
    pub idle_ms: u128,
    pub sleep_ms: u128,
    pub deficit_ms: u128,
    pub food_travel_mm: u128,
    pub food_trips: u64,
    pub lease_expiries: u64,
    pub survivors: BTreeMap<Time, usize>,
    pub physiology_ms: u128,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: u64,
    pub at: Time,
    pub agent: Option<usize>,
    pub cause: Option<u64>,
    pub kind: String,
    pub detail: String,
}
#[derive(Clone, Debug)]
pub struct World {
    pub(crate) information: information::State,
    pub(crate) combat: combat::State,
    pub(crate) formation: formation::State,
    pub(crate) military: military::State,
    pub(crate) lifecycle: lifecycle::State,
    pub(crate) social: social::State,
    pub(crate) household: household::State,
    pub(crate) authority: authority::State,
    pub(crate) barter: barter::State,
    pub(crate) settlement_research: Option<settlement_research::Journal>,
    pub(crate) paths: paths::State,
    pub(crate) infrastructure: infrastructure::State,
    pub(crate) capital: capital::State,
    pub(crate) farm: farming::State,
    pub(crate) config: Config,
    pub(crate) now: Time,
    pub(crate) agents: Vec<Agent>,
    pub(crate) goods: Goods,
    pub(crate) boards: Vec<Board>,
    pub(crate) metrics: Metrics,
    pub(crate) events: Vec<Event>,
    pub(crate) event_count: u64,
    pub(crate) event_hash: u64,
    pub(crate) diagnostics: decomposition::Diagnostics,
    pub(crate) acceptance: acceptance::Journal,
    pub(crate) persistence: persistence::Journal,
    pub(crate) transport_research: congestion::Instrument,
    pub(crate) shadow: shadow::Journal,
    pub(crate) coherence: coherence::Journal,
    pub(crate) meals: meals::Journal,
    request_backlog: u64,
    gather_communal: BTreeMap<usize, bool>,
    initialized: bool,
    next_policy: Time,
    wake: BTreeSet<usize>,
}
pub fn performance(n: Time, f: Time) -> u128 {
    let hunger = if n > 6 * HOUR {
        1000
    } else if n > 0 {
        800
    } else {
        500
    };
    let fatigue = if f < 12 * HOUR {
        1000
    } else if f < 16 * HOUR {
        900
    } else if f < 20 * HOUR {
        750
    } else {
        500
    };
    hunger * fatigue
}
impl World {
    pub fn new(config: Config) -> Result<Self, String> {
        if (!config.market_nodes.is_empty() && !config.barter)
            || config.market_nodes.iter().any(|n| *n >= config.nodes)
            || config.nodes == 0
            || config.policy_ms <= 0
            || config.carrying_capacity == 0
            || !(0..=24 * HOUR).contains(&config.initial_nutrition)
            || !(0..=24 * HOUR).contains(&config.initial_fatigue)
            || config.starts.iter().any(|&n| n >= config.nodes)
            || config.boards.iter().any(|&n| n >= config.nodes)
            || config.edges.iter().any(|e| {
                e.a >= config.nodes || e.b >= config.nodes || e.a == e.b || e.nominal_ms <= 0
            })
            || config
                .food
                .iter()
                .any(|&(n, _, created, life)| n >= config.nodes || created > 0 || life <= 0)
        {
            return Err("invalid provisioning configuration".into());
        }
        if (config.military.is_some()
            || config.lifecycle.is_some()
            || config.social.is_some()
            || config.household.is_some()
            || config.farming.is_some()
            || config.capital.is_some()
            || config.infrastructure.is_some())
            && (config.persistence.is_some()
                || config.diagnostic_policy != DiagnosticPolicy::Current
                || config.acceptance_audit)
        {
            return Err("farming cannot silently override a historical/research policy".into());
        }
        let agents = config
            .starts
            .iter()
            .enumerate()
            .map(|(id, &node)| Agent {
                id,
                location: Location::Node(node),
                action: Action::Idle,
                nutrition: config.initial_nutrition,
                fatigue: config.initial_fatigue,
                starvation: 0,
                job: None,
                memory: PolicyMemory::default(),
                rng: config
                    .seed
                    .wrapping_add((id as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)),
                goal: None,
                last_event: None,
                death_time: None,
                consumed: 0,
                travel_since_food: 0,
            })
            .collect();
        let mut goods = Goods::default();
        for &(node, q, created, life) in &config.food {
            goods.add(
                Material::Food,
                q,
                Owner::None,
                Holder::Deposit(node),
                created,
                life,
            );
        }
        let boards = if config.communal {
            config
                .boards
                .iter()
                .map(|&node| Board {
                    node,
                    quantity: config.initial_request,
                    created_at: 0,
                    refreshed_at: 0,
                    claims: vec![],
                })
                .collect()
        } else {
            vec![]
        };
        if config.infrastructure.is_some() && !config.communal {
            return Err("F6 public project requires existing community".into());
        }
        let paths = paths::State::new(
            config.paths.as_ref(),
            &config.edges,
            config.infrastructure.as_ref(),
        )?;
        let infrastructure = infrastructure::State::new(
            config.infrastructure.as_ref(),
            &config.edges,
            config.starts.len(),
            config.nodes,
        )?;
        if let Some(c) = &config.infrastructure {
            for &(node, q) in &c.wood {
                goods.add(
                    Material::Wood,
                    q,
                    Owner::None,
                    Holder::Deposit(node),
                    0,
                    Time::MAX,
                );
            }
        }
        let capital =
            capital::State::new(config.capital.as_ref(), config.nodes, config.starts.len())?;
        if let Some(c) = &config.capital {
            for &(node, q) in &c.wood {
                goods.add(
                    Material::Wood,
                    q,
                    Owner::None,
                    Holder::Deposit(node),
                    0,
                    Time::MAX,
                );
            }
        }
        let farm = farming::State::new(config.farming.as_ref(), config.nodes, config.starts.len())?;
        let authority = authority::State::new(&config)?;
        let household = household::State::new(&config)?;
        let social = social::State::new(&config);
        let lifecycle = lifecycle::State::new(&config)?;
        let military = military::State::new(&config)?;
        let formation = formation::State::new(&config)?;
        let combat = combat::State::new(&config)?;
        let information = information::State::new(&config);
        let mut w = Self {
            information,
            combat,
            formation,
            military,
            lifecycle,
            social,
            household,
            authority,
            barter: barter::State::default(),
            settlement_research: None,
            paths,
            infrastructure,
            capital,
            farm,
            shadow: shadow::Journal::default(),
            coherence: coherence::Journal::default(),
            meals: meals::Journal::default(),
            request_backlog: config.initial_request,
            diagnostics: decomposition::Diagnostics::default(),
            acceptance: acceptance::Journal::default(),
            persistence: persistence::Journal::default(),
            transport_research: congestion::Instrument::default(),
            config,
            now: 0,
            agents,
            goods,
            boards,
            metrics: Metrics::default(),
            events: vec![],
            event_count: 0,
            event_hash: 0xcbf29ce484222325,
            gather_communal: BTreeMap::new(),
            initialized: false,
            next_policy: 0,
            wake: BTreeSet::new(),
        };
        if w.config.prior_memory {
            // Explicit information-only experimental intervention, timestamped at t=0.
            let views: Vec<_> = (0..w.config.nodes).map(|n| w.node_view(n)).collect();
            for a in &mut w.agents {
                for v in &views {
                    a.memory.nodes.insert(v.node, v.clone());
                }
            }
        }
        if w.config.prior_memory && w.config.infrastructure.is_some() {
            // Explicit experimental initial memories, never a live remote update.
            for id in 0..w.agents.len() {
                for node in 0..w.config.nodes {
                    let view = w.infrastructure_view(node, id);
                    w.infrastructure.memories[id].insert(node, view);
                }
            }
        }
        if w.config.lifecycle.is_some() {
            for id in 0..w.agents.len() {
                w.observe_lifecycle(id);
            }
        }
        w.audit()?;
        Ok(w)
    }
    pub fn now(&self) -> Time {
        self.now
    }
    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }
    pub fn batches(&self) -> &[Batch] {
        &self.goods.batches
    }
    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    pub fn event_hash(&self) -> u64 {
        self.event_hash
    }
    pub fn boards(&self) -> &[Board] {
        &self.boards
    }
    fn reserved(&self, batch: usize) -> u64 {
        self.agents
            .iter()
            .filter(|a| matches!(a.action,Action::Gather{batch:b,..} if b==batch))
            .count() as u64
    }
    fn food_view(&self, b: &Batch) -> FoodView {
        FoodView {
            batch: b.id,
            quantity: b.quantity,
            available: b.quantity.saturating_sub(self.reserved(b.id)),
            owner: b.owner,
            holder: b.holder,
            expires_at: b.expires_at(),
        }
    }
    fn node_view(&self, node: usize) -> NodeView {
        NodeView{node,observed_at:self.now,food:self.goods.batches.iter().filter(|b|b.quantity>0&&b.material==Material::Food&&matches!(b.holder,Holder::Deposit(n)|Holder::Loose(n)|Holder::Commons(n) if n==node)).map(|b|self.food_view(b)).collect(),
            edges:self.config.edges.iter().enumerate().filter(|(_,e)|e.a==node||e.b==node).map(|(id,_)|EdgeView{id,edge:self.effective_edge(id),queued:self.agents.iter().filter(|a|a.location==Location::Node(node)&&matches!(a.action,Action::Queued{edge,..} if edge==id)).count(),occupied:0}).collect(),
            board:self.boards.iter().find(|b|b.node==node).cloned(),people:self.agents.iter().filter(|a|a.location==Location::Node(node)&&a.action!=Action::Dead).map(|a|a.id).collect()}
    }
    pub fn observe(&self, id: usize) -> Option<Observation> {
        let a = self.agents.get(id)?;
        if a.action == Action::Dead {
            return None;
        }
        Some(Observation {
            body_factor: self.body_factor(id),
            raider: self.is_raider(id),
            now: self.now,
            agent: id,
            location: a.location.clone(),
            action: a.action.clone(),
            nutrition: a.nutrition,
            fatigue: a.fatigue,
            starvation: a.starvation,
            own: self
                .goods
                .batches
                .iter()
                .filter(|b| {
                    (self.carried_by(id, b.holder) || self.accessible_cart(id, b.holder))
                        && b.quantity > 0
                        && b.material == Material::Food
                })
                .map(|b| self.food_view(b))
                .collect(),
            load: self.carrier_load(id),
            capacity: self.carrier_capacity(id),
            local: if let Location::Node(n) = a.location {
                Some(self.node_view(n))
            } else {
                None
            },
            job: a.job.clone(),
            goal: a.goal,
            communal: self.config.communal,
            decision_interval: self.config.policy_ms,
        })
    }
    fn log(&mut self, agent: Option<usize>, kind: &str, detail: String) {
        let cause = agent.and_then(|id| self.agents[id].last_event);
        let e = Event {
            id: self.event_count,
            at: self.now,
            agent,
            cause,
            kind: kind.into(),
            detail,
        };
        self.market_event(&e);
        self.shadow.event(&e, &self.agents, &self.goods);
        self.coherence.event(&e, &self.agents);
        self.meals.event(&e, &self.agents, &self.goods.batches);
        if settlement_research::observed_event(kind)
            && let Some(mut journal) = self.settlement_research.take()
        {
            journal.event(&e, self);
            self.settlement_research = Some(journal);
        }
        self.diagnostics.event(&e);
        if self.config.persistence.is_some() {
            self.persistence.event(&e, &self.agents);
        }
        for byte in format!("{e:?}").bytes() {
            self.event_hash ^= byte as u64;
            self.event_hash = self.event_hash.wrapping_mul(0x100000001b3);
        }
        if let Some(id) = agent {
            self.agents[id].last_event = Some(e.id);
        }
        self.event_count += 1;
        if self.config.trace {
            self.events.push(e);
        }
    }
    pub fn audit(&self) -> Result<(), String> {
        self.audit_formation()?;
        self.audit_combat()?;
        self.audit_information()?;
        self.audit_military()?;
        self.audit_lifecycle()?;
        self.audit_social()?;
        self.audit_household()?;
        self.audit_authority()?;
        self.audit_barter()?;
        self.audit_farming()?;
        self.audit_capital()?;
        self.audit_infrastructure()?;
        self.audit_paths()?;
        if !self.goods.audit_capital(
            self.farm.produced as u128,
            self.capital.embedded_wood() + self.infrastructure.embedded_wood(),
        ) {
            return Err("material accounting error".into());
        }
        for a in &self.agents {
            if self.goods.load(a.id) > self.config.carrying_capacity {
                return Err("load limit".into());
            }
            if matches!(a.action, Action::Gather { .. })
                && self.carrier_load(a.id) >= self.carrier_capacity(a.id)
            {
                return Err("reserved load limit".into());
            }
            if matches!(a.action, Action::Travel) != matches!(a.location, Location::Edge { .. })
                && a.action != Action::Dead
            {
                return Err("location/action exclusivity".into());
            }
            if a.action == Action::Dead && (self.goods.load(a.id) != 0 || a.job.is_some()) {
                return Err("dead actor retained custody/job".into());
            }
            if !(0..=24 * HOUR).contains(&a.nutrition) || !(0..=24 * HOUR).contains(&a.fatigue) {
                return Err("physiology bounds".into());
            }
        }
        for b in &self.goods.batches {
            if self.reserved(b.id) > b.quantity {
                return Err("double reservation".into());
            }
        }
        for edge in 0..self.config.edges.len() {
            let e = self.effective_edge(edge);
            if self
                .agents
                .iter()
                .filter(|a| {
                    a.action == Action::Travel
                        && matches!(a.location,Location::Edge{edge:x,..}if x==edge)
                })
                .count()
                > e.capacity
            {
                return Err("edge occupancy".into());
            }
        }
        if self.metrics.overtakes != 0 {
            return Err("FIFO overtake".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
