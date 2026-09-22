//! Authoritative headless kernel. Public views are read-only research views,
//! not player observations. All quantities are integer material units; time is ms.

pub type AgentId = usize;
pub type NodeId = usize;
pub type EdgeId = usize;
pub type Stock = [u64; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resource {
    Food,
    Wood,
    Stone,
}
impl Resource {
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeSpec {
    pub x_mm: i64,
    pub y_mm: i64,
    pub deposits: Stock,
    pub loose: Stock,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeSpec {
    pub a: NodeId,
    pub b: NodeId,
    pub length_mm: u64,
    /// Simultaneous individual travelers, shared by both directions.
    pub capacity: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scenario {
    pub nodes: Vec<NodeSpec>,
    pub edges: Vec<EdgeSpec>,
    pub homes: Vec<NodeId>,
    pub carrying_capacity: u64,
    pub speed_mm_per_second: u64,
    pub gathering_ms_per_unit: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    spec: NodeSpec,
    reserved: Stock,
}
impl Node {
    pub fn deposits(&self) -> Stock {
        self.spec.deposits
    }
    pub fn loose(&self) -> Stock {
        self.spec.loose
    }
    pub fn reserved(&self) -> Stock {
        self.reserved
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Idle {
        node: NodeId,
    },
    Travel {
        edge: EdgeId,
        from: NodeId,
        to: NodeId,
        start_ms: u64,
        end_ms: u64,
        cause: usize,
    },
    Gather {
        node: NodeId,
        resource: Resource,
        amount: u64,
        end_ms: u64,
        cause: usize,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    action: Action,
    inventory: Stock,
}
impl Agent {
    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn inventory(&self) -> Stock {
        self.inventory
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Travel { edge: EdgeId },
    Gather { resource: Resource, amount: u64 },
    Pickup { resource: Resource, amount: u64 },
    Drop { resource: Resource, amount: u64 },
    CancelGather,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidScenario,
    UnknownAgent,
    Busy,
    NotLocal,
    Capacity,
    InsufficientStock,
    InvalidAmount,
    NotGathering,
    TimeOverflow,
    TimeReversal,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventKind {
    Accepted(Command),
    Rejected(Command, Error),
    Arrived,
    Gathered { resource: Resource, amount: u64 },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: usize,
    pub time_ms: u64,
    pub agent: AgentId,
    pub cause: Option<usize>,
    pub location: Action,
    pub kind: EventKind,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World {
    now_ms: u64,
    nodes: Vec<Node>,
    edges: Vec<EdgeSpec>,
    agents: Vec<Agent>,
    carrying_capacity: u64,
    speed: u64,
    gather_ms: u64,
    initial: [u128; 3],
    events: Vec<Event>,
}
impl World {
    pub fn new(s: Scenario) -> Result<Self, Error> {
        if s.nodes.is_empty()
            || s.carrying_capacity == 0
            || s.speed_mm_per_second == 0
            || s.gathering_ms_per_unit == 0
            || s.homes.iter().any(|&n| n >= s.nodes.len())
            || s.edges.iter().any(|e| {
                e.a >= s.nodes.len()
                    || e.b >= s.nodes.len()
                    || e.a == e.b
                    || e.capacity == 0
                    || e.length_mm == 0
                    || e.length_mm.checked_mul(1000).is_none()
            })
        {
            return Err(Error::InvalidScenario);
        }
        // A routed edge may be longer than the straight line, but not shorter.
        for e in &s.edges {
            let a = &s.nodes[e.a];
            let b = &s.nodes[e.b];
            let dx = (a.x_mm as i128 - b.x_mm as i128).unsigned_abs();
            let dy = (a.y_mm as i128 - b.y_mm as i128).unsigned_abs();
            let length = e.length_mm as u128;
            if dx > length || dy > length || dx * dx + dy * dy > length * length {
                return Err(Error::InvalidScenario);
            }
        }
        let mut world = Self {
            now_ms: 0,
            nodes: s
                .nodes
                .into_iter()
                .map(|spec| Node {
                    spec,
                    reserved: [0; 3],
                })
                .collect(),
            edges: s.edges,
            agents: s
                .homes
                .into_iter()
                .map(|node| Agent {
                    action: Action::Idle { node },
                    inventory: [0; 3],
                })
                .collect(),
            carrying_capacity: s.carrying_capacity,
            speed: s.speed_mm_per_second,
            gather_ms: s.gathering_ms_per_unit,
            initial: [0; 3],
            events: Vec::new(),
        };
        world.initial = world.totals();
        Ok(world)
    }
    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }
    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    pub fn initial_stock(&self) -> [u128; 3] {
        self.initial
    }
    pub fn totals(&self) -> [u128; 3] {
        let mut sums = [0u128; 3];
        for (r, sum) in sums.iter_mut().enumerate() {
            for n in &self.nodes {
                *sum += n.spec.deposits[r] as u128 + n.spec.loose[r] as u128;
            }
            for a in &self.agents {
                *sum += a.inventory[r] as u128;
            }
        }
        sums
    }
    fn log(&mut self, agent: AgentId, cause: Option<usize>, kind: EventKind) -> usize {
        let id = self.events.len();
        self.events.push(Event {
            id,
            time_ms: self.now_ms,
            agent,
            cause,
            location: self.agents[agent].action.clone(),
            kind,
        });
        id
    }
    /// Failed valid-agent commands are recorded, but never partially change physical state.
    pub fn command(&mut self, agent: AgentId, command: Command) -> Result<(), Error> {
        if agent >= self.agents.len() {
            return Err(Error::UnknownAgent);
        }
        let result = self.execute(agent, command);
        if let Err(error) = result {
            self.log(agent, None, EventKind::Rejected(command, error));
        }
        debug_assert!(self.audit().is_ok());
        result
    }
    fn execute(&mut self, agent: AgentId, command: Command) -> Result<(), Error> {
        if command == Command::CancelGather {
            if let Action::Gather {
                node,
                resource,
                amount,
                cause,
                ..
            } = self.agents[agent].action
            {
                self.nodes[node].reserved[resource.index()] -= amount;
                self.log(agent, Some(cause), EventKind::Accepted(command));
                self.agents[agent].action = Action::Idle { node };
                return Ok(());
            }
            return Err(Error::NotGathering);
        }
        let node = match self.agents[agent].action {
            Action::Idle { node } => node,
            _ => return Err(Error::Busy),
        };
        match command {
            Command::Travel { edge } => {
                let e = self.edges.get(edge).ok_or(Error::NotLocal)?;
                let to = if e.a == node {
                    e.b
                } else if e.b == node {
                    e.a
                } else {
                    return Err(Error::NotLocal);
                };
                let count = self
                    .agents
                    .iter()
                    .filter(
                        |a| matches!(a.action, Action::Travel { edge: other, .. } if other == edge),
                    )
                    .count();
                if count >= e.capacity {
                    return Err(Error::Capacity);
                }
                let duration = (e.length_mm * 1000).div_ceil(self.speed);
                let end_ms = self
                    .now_ms
                    .checked_add(duration)
                    .ok_or(Error::TimeOverflow)?;
                let cause = self.log(agent, None, EventKind::Accepted(command));
                self.agents[agent].action = Action::Travel {
                    edge,
                    from: node,
                    to,
                    start_ms: self.now_ms,
                    end_ms,
                    cause,
                };
            }
            Command::Gather { resource, amount } | Command::Pickup { resource, amount } => {
                if amount == 0 {
                    return Err(Error::InvalidAmount);
                }
                let r = resource.index();
                let load: u128 = self.agents[agent]
                    .inventory
                    .iter()
                    .map(|&v| v as u128)
                    .sum();
                if load + amount as u128 > self.carrying_capacity as u128 {
                    return Err(Error::Capacity);
                }
                if matches!(command, Command::Gather { .. }) {
                    if amount > self.nodes[node].spec.deposits[r] - self.nodes[node].reserved[r] {
                        return Err(Error::InsufficientStock);
                    }
                    let end_ms = amount
                        .checked_mul(self.gather_ms)
                        .and_then(|d| self.now_ms.checked_add(d))
                        .ok_or(Error::TimeOverflow)?;
                    let cause = self.log(agent, None, EventKind::Accepted(command));
                    self.nodes[node].reserved[r] += amount;
                    self.agents[agent].action = Action::Gather {
                        node,
                        resource,
                        amount,
                        end_ms,
                        cause,
                    };
                } else {
                    if amount > self.nodes[node].spec.loose[r] {
                        return Err(Error::InsufficientStock);
                    }
                    self.log(agent, None, EventKind::Accepted(command));
                    self.nodes[node].spec.loose[r] -= amount;
                    self.agents[agent].inventory[r] += amount;
                }
            }
            Command::Drop { resource, amount } => {
                let r = resource.index();
                if amount == 0 {
                    return Err(Error::InvalidAmount);
                }
                if amount > self.agents[agent].inventory[r] {
                    return Err(Error::InsufficientStock);
                }
                let stock = self.nodes[node].spec.loose[r]
                    .checked_add(amount)
                    .ok_or(Error::Capacity)?;
                self.log(agent, None, EventKind::Accepted(command));
                self.agents[agent].inventory[r] -= amount;
                self.nodes[node].spec.loose[r] = stock;
            }
            Command::CancelGather => unreachable!(),
        }
        Ok(())
    }
    /// Process completions at their own times, independent of caller frame/tick size.
    /// Simultaneous completions use stable agent-ID order. No policies run here.
    pub fn advance_to(&mut self, target_ms: u64) -> Result<(), Error> {
        if target_ms < self.now_ms {
            return Err(Error::TimeReversal);
        }
        loop {
            let next = self
                .agents
                .iter()
                .enumerate()
                .filter_map(|(id, a)| match a.action {
                    Action::Travel { end_ms, .. } | Action::Gather { end_ms, .. }
                        if end_ms <= target_ms =>
                    {
                        Some((end_ms, id))
                    }
                    _ => None,
                })
                .min();
            let Some((time, agent)) = next else {
                break;
            };
            self.now_ms = time;
            match self.agents[agent].action.clone() {
                Action::Travel { to, cause, .. } => {
                    self.agents[agent].action = Action::Idle { node: to };
                    self.log(agent, Some(cause), EventKind::Arrived);
                }
                Action::Gather {
                    node,
                    resource,
                    amount,
                    cause,
                    ..
                } => {
                    let r = resource.index();
                    self.nodes[node].reserved[r] -= amount;
                    self.nodes[node].spec.deposits[r] -= amount;
                    self.agents[agent].inventory[r] += amount;
                    self.agents[agent].action = Action::Idle { node };
                    self.log(agent, Some(cause), EventKind::Gathered { resource, amount });
                }
                Action::Idle { .. } => unreachable!(),
            }
            debug_assert!(self.audit().is_ok());
        }
        self.now_ms = target_ms;
        Ok(())
    }
    pub fn audit(&self) -> Result<(), &'static str> {
        if self.totals() != self.initial {
            return Err("material balance changed");
        }
        let mut reservations = vec![[0u128; 3]; self.nodes.len()];
        let mut occupancy = vec![0; self.edges.len()];
        for a in &self.agents {
            let load: u128 = a.inventory.iter().map(|&v| v as u128).sum();
            let pending = match a.action {
                Action::Gather {
                    node,
                    resource,
                    amount,
                    end_ms,
                    ..
                } => {
                    if end_ms < self.now_ms {
                        return Err("overdue gathering");
                    }
                    reservations[node][resource.index()] += amount as u128;
                    amount as u128
                }
                Action::Travel { edge, end_ms, .. } => {
                    if end_ms < self.now_ms {
                        return Err("overdue travel");
                    }
                    occupancy[edge] += 1;
                    0
                }
                Action::Idle { .. } => 0,
            };
            if load + pending > self.carrying_capacity as u128 {
                return Err("carrying capacity exceeded");
            }
        }
        for (n, reserved) in self.nodes.iter().zip(reservations) {
            for (r, &value) in reserved.iter().enumerate() {
                if value != n.reserved[r] as u128 || n.reserved[r] > n.spec.deposits[r] {
                    return Err("reservation mismatch");
                }
            }
        }
        if occupancy
            .iter()
            .zip(&self.edges)
            .any(|(&n, e)| n > e.capacity)
        {
            return Err("edge capacity exceeded");
        }
        Ok(())
    }
}

/// Seeded finite deposits; edges are terrain traversal links, not constructed roads.
/// No houses, fields, or prescribed settlement layout are generated.
pub fn reference_scenario(seed: u64, distance_mm: u64, carrying_capacity: u64) -> Scenario {
    let mut rng = seed;
    let mut next = || {
        rng = rng.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    Scenario {
        nodes: vec![
            NodeSpec {
                x_mm: 0,
                y_mm: 0,
                deposits: [0; 3],
                loose: [500, 0, 0],
            },
            NodeSpec {
                x_mm: 1000,
                y_mm: 0,
                deposits: [0, 200 + next() % 101, 0],
                loose: [0; 3],
            },
            NodeSpec {
                x_mm: 0,
                y_mm: 1000,
                deposits: [0, 0, 200 + next() % 101],
                loose: [0; 3],
            },
        ],
        edges: vec![
            EdgeSpec {
                a: 0,
                b: 1,
                length_mm: distance_mm,
                capacity: 4,
            },
            EdgeSpec {
                a: 0,
                b: 2,
                length_mm: distance_mm,
                capacity: 4,
            },
        ],
        homes: vec![0; 50],
        carrying_capacity,
        speed_mm_per_second: 1000,
        gathering_ms_per_unit: 2000,
    }
}

/// Versioned local provisioning experiment.
pub mod provisioning;
