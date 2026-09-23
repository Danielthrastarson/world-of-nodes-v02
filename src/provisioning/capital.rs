//! Personal capital. Physics and local belief-based evaluation are separate below.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rules {
    pub enabled: bool,
    pub wood_cost: u64,
    pub work_ms: Time,
    pub cargo_capacity: u64,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            enabled: true,
            wood_cost: 10,
            work_ms: 4 * HOUR,
            cargo_capacity: 20,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub rules: Rules,
    pub wood: Vec<(usize, u64)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: usize,
    pub investor: usize,
    pub node: usize,
    pub wood: u64,
    pub remaining: u128,
    pub started_at: Time,
    pub labor_ms: u128,
    pub cart: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trip {
    pub started: Time,
    pub arrived: Option<Time>,
    pub from: usize,
    pub to: usize,
    pub cargo: u64,
    pub food: u64,
    pub food_arrived: u64,
    pub backpack_food: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cart {
    pub id: usize,
    pub owner: Owner,
    pub location: Location,
    pub user: Option<usize>,
    pub capacity: u64,
    pub wood: u64,
    pub completed_at: Time,
    pub first_use: Option<Time>,
    pub idle_ms: u128,
    pub trips: Vec<Trip>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartView {
    pub id: usize,
    pub owner: Owner,
    pub location: Location,
    pub user: Option<usize>,
    pub capacity: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wood {
    pub batch: usize,
    pub available: u64,
    pub owner: Owner,
    pub holder: Holder,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct View {
    pub observed_at: Time,
    pub wood: Vec<Wood>,
    pub carts: Vec<CartView>,
    pub projects: Vec<Project>,
}
pub type Memory = BTreeMap<usize, View>;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub projects: Vec<Project>,
    pub carts: Vec<Cart>,
    pub memories: Vec<Memory>,
    pub considered: BTreeSet<usize>,
    pub gates: BTreeMap<String, u64>,
    pub last_gate: BTreeMap<usize, String>,
    pub backpack_food_edges: u64,
}
impl State {
    pub fn new(c: Option<&Config>, nodes: usize, agents: usize) -> Result<Self, String> {
        let Some(c) = c else {
            return Ok(Self::default());
        };
        if c.wood.iter().any(|(n, _)| *n >= nodes)
            || c.rules.wood_cost == 0
            || c.rules.work_ms <= 0
            || c.rules.cargo_capacity == 0
        {
            return Err("invalid capital config".into());
        }
        Ok(Self {
            memories: vec![Memory::new(); agents],
            ..Self::default()
        })
    }
    pub fn embedded_wood(&self) -> u128 {
        self.projects
            .iter()
            .filter(|p| p.cart.is_none())
            .map(|p| p.wood as u128)
            .sum::<u128>()
            + self.carts.iter().map(|c| c.wood as u128).sum::<u128>()
    }
}
impl World {
    pub fn carts(&self) -> &[Cart] {
        &self.capital.carts
    }
    pub fn cart_projects(&self) -> &[Project] {
        &self.capital.projects
    }
    pub fn cart_memory(&self, id: usize) -> Option<&Memory> {
        self.capital.memories.get(id)
    }
    pub fn cart_load(&self, cart: usize) -> u64 {
        self.goods
            .batches
            .iter()
            .filter(|b| b.holder == Holder::Cart(cart))
            .map(|b| b.quantity)
            .sum()
    }
    pub(super) fn active_cart(&self, id: usize) -> Option<&Cart> {
        self.capital.carts.iter().find(|c| c.user == Some(id))
    }
    pub(super) fn carried_by(&self, id: usize, holder: Holder) -> bool {
        holder == Holder::Agent(id)
            || matches!(holder,Holder::Cart(c) if self.capital.carts.get(c).is_some_and(|c|c.user==Some(id)))
    }
    pub(super) fn accessible_cart(&self, id: usize, holder: Holder) -> bool {
        matches!(holder,Holder::Cart(c) if self.capital.carts.get(c).is_some_and(|c|c.owner==Owner::Agent(id)&&c.location==self.agents[id].location&&matches!(c.location,Location::Node(_))))
    }
    pub fn carrier_load(&self, id: usize) -> u64 {
        self.goods
            .batches
            .iter()
            .filter(|b| self.carried_by(id, b.holder))
            .map(|b| b.quantity)
            .sum()
    }
    pub fn carrier_capacity(&self, id: usize) -> u64 {
        self.config.carrying_capacity + self.active_cart(id).map_or(0, |c| c.capacity)
    }
    pub(super) fn load_carrier(&mut self, id: usize, batch: usize, q: u64, owner: Owner) {
        let bag = q.min(self.config.carrying_capacity - self.goods.load(id));
        if bag > 0 {
            self.goods.move_part(batch, bag, owner, Holder::Agent(id));
        }
        if q > bag {
            let c = self.active_cart(id).expect("validated cart capacity").id;
            self.goods.move_part(batch, q - bag, owner, Holder::Cart(c));
        }
    }
    pub(super) fn observe_capital(&mut self, id: usize) {
        if self.config.capital.is_none() {
            return;
        }
        let Location::Node(n) = self.agents[id].location else {
            return;
        };
        let wood = self
            .goods
            .batches
            .iter()
            .filter(|b| {
                b.material == Material::Wood
                    && b.quantity > 0
                    && (matches!(b.holder,Holder::Deposit(x)|Holder::Loose(x) if x==n)
                        || self.carried_by(id, b.holder))
            })
            .map(|b| Wood {
                batch: b.id,
                available: b.quantity.saturating_sub(self.reserved(b.id)),
                owner: b.owner,
                holder: b.holder,
            })
            .collect();
        self.capital.memories[id].insert(
            n,
            View {
                observed_at: self.now,
                wood,
                carts: self
                    .capital
                    .carts
                    .iter()
                    .filter(|c| c.location == Location::Node(n))
                    .map(|c| CartView {
                        id: c.id,
                        owner: c.owner,
                        location: c.location.clone(),
                        user: c.user,
                        capacity: c.capacity,
                    })
                    .collect(),
                projects: self
                    .capital
                    .projects
                    .iter()
                    .filter(|p| p.node == n && p.cart.is_none())
                    .cloned()
                    .collect(),
            },
        );
    }
    pub(super) fn begin_cart(&mut self, id: usize) -> Result<(), String> {
        let r = self
            .config
            .capital
            .as_ref()
            .ok_or("capital unavailable")?
            .rules
            .clone();
        if !r.enabled {
            return Err("cart construction disabled".into());
        }
        let Location::Node(node) = self.agents[id].location else {
            return Err("nonlocal construction".into());
        };
        if matches!(self.agents[id].action, Action::BuildCart { .. }) {
            return Ok(());
        }
        let existing = self
            .capital
            .projects
            .iter()
            .find(|p| p.investor == id && p.node == node && p.cart.is_none())
            .map(|p| p.id);
        let project = if let Some(p) = existing {
            p
        } else {
            let inputs: Vec<_> = self
                .goods
                .batches
                .iter()
                .filter(|b| {
                    b.material == Material::Wood
                        && b.owner == Owner::Agent(id)
                        && b.quantity > 0
                        && (self.carried_by(id, b.holder) || b.holder == Holder::Loose(node))
                })
                .map(|b| (b.id, b.quantity))
                .collect();
            if inputs.iter().map(|(_, q)| q).sum::<u64>() < r.wood_cost {
                return Err("personal local wood insufficient".into());
            }
            let mut left = r.wood_cost;
            for (b, q) in inputs {
                let take = q.min(left);
                self.goods.batches[b].quantity -= take;
                left -= take;
                if left == 0 {
                    break;
                }
            }
            let p = self.capital.projects.len();
            self.capital.projects.push(Project {
                id: p,
                investor: id,
                node,
                wood: r.wood_cost,
                remaining: r.work_ms as u128 * SCALE,
                started_at: self.now,
                labor_ms: 0,
                cart: None,
            });
            self.log(
                Some(id),
                "CartStarted",
                format!(
                    "project={p} node={node} wood={} finished_cart_owner=not_yet_created",
                    r.wood_cost
                ),
            );
            p
        };
        self.cancel_action(id, "construct personal capital");
        self.agents[id].action = Action::BuildCart { project };
        Ok(())
    }
    pub(super) fn finish_carts(&mut self) {
        for p in 0..self.capital.projects.len() {
            let x = self.capital.projects[p].clone();
            if x.cart.is_none()
                && x.remaining == 0
                && self.agents[x.investor].action == (Action::BuildCart { project: p })
            {
                let id = self.capital.carts.len();
                let capacity = self.config.capital.as_ref().unwrap().rules.cargo_capacity;
                self.capital.carts.push(Cart {
                    id,
                    owner: Owner::Agent(x.investor),
                    location: Location::Node(x.node),
                    user: None,
                    capacity,
                    wood: x.wood,
                    completed_at: self.now,
                    first_use: None,
                    idle_ms: 0,
                    trips: vec![],
                });
                self.capital.projects[p].cart = Some(id);
                self.agents[x.investor].action = Action::Idle;
                self.wake.insert(x.investor);
                self.log(
                    Some(x.investor),
                    "CartCompleted",
                    format!(
                        "cart={id} project={p} owner=Agent({}) capacity={capacity}",
                        x.investor
                    ),
                );
            }
        }
    }
    pub(super) fn hitch_cart(&mut self, id: usize, cart: usize) -> Result<(), String> {
        let c = self.capital.carts.get(cart).ok_or("unknown cart")?;
        if c.owner != Owner::Agent(id)
            || c.location != self.agents[id].location
            || !matches!(c.location, Location::Node(_))
            || c.user.is_some()
            || self.active_cart(id).is_some()
        {
            return Err("cart ownership/location/exclusive user precondition".into());
        }
        self.cancel_action(id, "hitch owned cart");
        self.capital.carts[cart].user = Some(id);
        self.log(Some(id), "CartHitched", format!("cart={cart}"));
        Ok(())
    }
    pub(super) fn park_cart(&mut self, id: usize) -> Result<(), String> {
        let c = self.active_cart(id).ok_or("no attached cart")?.id;
        self.cancel_action(id, "park owned cart");
        self.capital.carts[c].user = None;
        self.log(Some(id), "CartParked", format!("cart={c}"));
        Ok(())
    }
    pub(super) fn sync_carts(&mut self) {
        for c in &mut self.capital.carts {
            if let Some(id) = c.user {
                c.location = self.agents[id].location.clone();
            }
        }
    }
    pub(super) fn integrate_capital(&mut self, dt: Time) {
        for c in &mut self.capital.carts {
            if c.user
                .is_none_or(|id| self.agents[id].action != Action::Travel)
            {
                c.idle_ms += dt as u128;
            }
        }
    }
    pub(super) fn capital_departure(&mut self, id: usize, from: usize, to: usize) {
        if self.config.capital.is_none() {
            return;
        }
        let bag = self
            .goods
            .batches
            .iter()
            .filter(|b| b.holder == Holder::Agent(id) && b.material == Material::Food)
            .map(|b| b.quantity)
            .sum();
        self.capital.backpack_food_edges += bag;
        if let Some(c) = self.active_cart(id) {
            let cart = c.id;
            let cargo = self.cart_load(cart);
            let food = self
                .goods
                .batches
                .iter()
                .filter(|b| b.holder == Holder::Cart(cart) && b.material == Material::Food)
                .map(|b| b.quantity)
                .sum();
            let c = &mut self.capital.carts[cart];
            c.first_use.get_or_insert(self.now);
            c.trips.push(Trip {
                started: self.now,
                arrived: None,
                from,
                to,
                cargo,
                food,
                food_arrived: 0,
                backpack_food: bag,
            });
        }
    }
    pub(super) fn capital_arrival(&mut self, id: usize) {
        if let Some(c) = self.active_cart(id) {
            let cart = c.id;
            let food = self
                .goods
                .batches
                .iter()
                .filter(|b| {
                    b.holder == Holder::Cart(cart)
                        && b.material == Material::Food
                        && b.expires_at() > self.now
                })
                .map(|b| b.quantity)
                .sum();
            if let Some(t) = self.capital.carts[cart].trips.last_mut() {
                t.arrived = Some(self.now);
                t.food_arrived = food;
            }
        }
    }
    pub(super) fn capital_death(&mut self, id: usize) {
        if let Some(c) = self.active_cart(id) {
            let c = c.id;
            let food = self
                .goods
                .batches
                .iter()
                .filter(|b| b.holder == Holder::Cart(c) && b.material == Material::Food)
                .map(|b| b.quantity)
                .sum::<u64>();
            self.metrics.carried_at_death += food;
            if matches!(self.capital.carts[c].location, Location::Edge { .. }) {
                self.metrics.stranded += food;
            }
            self.capital.carts[c].user = None;
            self.log(
                Some(id),
                "CartStranded",
                format!(
                    "cart={c} cargo_food={food}; ownership unchanged location={:?}",
                    self.capital.carts[c].location
                ),
            );
        }
    }
    pub(super) fn audit_capital(&self) -> Result<(), String> {
        if self.config.capital.is_none() {
            return if self.capital == State::default() {
                Ok(())
            } else {
                Err("capital without config".into())
            };
        }
        let mut users = BTreeSet::new();
        for (id, c) in self.capital.carts.iter().enumerate() {
            if c.id != id
                || !matches!(c.owner,Owner::Agent(x) if x<self.agents.len())
                || self.cart_load(id) > c.capacity
            {
                return Err("cart identity/owner/load".into());
            }
            if let Some(u) = c.user
                && (!users.insert(u)
                    || c.owner != Owner::Agent(u)
                    || self.agents[u].action == Action::Dead
                    || c.location != self.agents[u].location)
            {
                return Err("cart user/locality/exclusivity".into());
            }
        }
        for b in &self.goods.batches {
            if let Holder::Cart(c) = b.holder
                && c >= self.capital.carts.len()
            {
                return Err("cargo in nonexistent cart".into());
            }
        }
        for a in &self.agents {
            if let Action::BuildCart { project } = a.action {
                let p = self
                    .capital
                    .projects
                    .get(project)
                    .ok_or("missing construction")?;
                if p.investor != a.id || a.location != Location::Node(p.node) || p.cart.is_some() {
                    return Err("construction exclusivity/locality".into());
                }
            }
        }
        for p in &self.capital.projects {
            if p.wood != self.config.capital.as_ref().unwrap().rules.wood_cost {
                return Err("construction input ledger".into());
            }
            if let Some(c) = p.cart {
                let c = self.capital.carts.get(c).ok_or("missing completed cart")?;
                if p.remaining != 0 || c.owner != Owner::Agent(p.investor) || c.wood != p.wood {
                    return Err("cart transformation ledger".into());
                }
            }
        }
        Ok(())
    }
    pub fn capital_counters(&self) -> BTreeMap<String, u128> {
        let s = &self.capital;
        let mut out:BTreeMap<_,_>=[("agents_considered",s.considered.len() as u128),("started",s.projects.len() as u128),("completed",s.carts.len() as u128),("wood_invested",s.embedded_wood()),("labor_ms",s.projects.iter().map(|p|p.labor_ms).sum()),("idle_cart_ms",s.carts.iter().map(|c|c.idle_ms).sum()),("dead_owner_carts",s.carts.iter().filter(|c|matches!(c.owner,Owner::Agent(id) if self.agents[id].action==Action::Dead)).count() as u128),("dead_investor_projects",s.projects.iter().filter(|p|p.cart.is_none()&&self.agents[p.investor].action==Action::Dead).count() as u128),("cart_trips_started",s.carts.iter().map(|c|c.trips.len() as u128).sum()),("cart_trips_completed",s.carts.iter().flat_map(|c|&c.trips).filter(|t|t.arrived.is_some()).count() as u128),("cart_food_quanta_edges_departed",s.carts.iter().flat_map(|c|&c.trips).map(|t|t.food as u128).sum()),("cart_food_quanta_edges_arrived",s.carts.iter().flat_map(|c|&c.trips).map(|t|t.food_arrived as u128).sum()),("backpack_food_quanta_edges_departed",s.backpack_food_edges as u128)].into_iter().map(|(k,v)|(k.into(),v)).collect();
        for (k, v) in &s.gates {
            out.insert(format!("gate_{k}"), *v as u128);
        }
        out
    }
    pub(super) fn record_capital_evaluation(&mut self, id: usize, o: &Observation) {
        let Some(c) = &self.config.capital else {
            return;
        };
        if !c.rules.enabled {
            return;
        }
        let e = evaluate(
            o,
            &self.agents[id].memory,
            &self.capital.memories[id],
            &c.rules,
        );
        *self.capital.gates.entry(e.gate.clone()).or_default() += 1;
        if e.demand > 0 {
            self.capital.considered.insert(id);
        }
        let text = format!(
            "{} demand={} without_ms={} with_ms={} work_ms={} wood_missing={}",
            e.gate, e.demand, e.without_ms, e.with_ms, e.work_ms, e.missing
        );
        if self.capital.last_gate.get(&id) != Some(&text) {
            self.capital.last_gate.insert(id, text.clone());
            self.log(Some(id), "CapitalForecast", text);
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct Evaluation {
    pub gate: String,
    pub demand: u64,
    pub without_ms: Time,
    pub with_ms: Time,
    pub work_ms: Time,
    pub missing: u64,
    pub intent: Option<Intent>,
}
fn result(gate: &str) -> Evaluation {
    Evaluation {
        gate: gate.into(),
        demand: 0,
        without_ms: 0,
        with_ms: 0,
        work_ms: 0,
        missing: 0,
        intent: None,
    }
}
/// Only personal observations and known mechanics; no World or hidden configuration.
pub(crate) fn evaluate(
    o: &Observation,
    m: &PolicyMemory,
    capital: &Memory,
    r: &Rules,
) -> Evaluation {
    if !r.enabled {
        return result("UNAVAILABLE");
    }
    let Location::Node(node) = o.location else {
        return result("TRAVERSING");
    };
    let Some(local) = capital.get(&node) else {
        return result("NO_LOCAL_OBSERVATION");
    };
    if let Some(c) = local
        .carts
        .iter()
        .find(|c| c.owner == Owner::Agent(o.agent))
    {
        let mut e = result("OWNED_CAPITAL");
        if c.user.is_none() {
            e.intent = Some(Intent::HitchCart { cart: c.id });
        }
        return e;
    }
    if capital
        .values()
        .flat_map(|v| &v.carts)
        .any(|c| c.owner == Owner::Agent(o.agent))
    {
        return result("OWNED_ELSEWHERE");
    }
    let mut e = result("NO_KNOWN_DEMAND");
    let mut candidate = None;
    for v in m.nodes.values() {
        let Some(board) = &v.board else {
            continue;
        };
        let demand = board.unclaimed(o.now);
        if demand == 0 {
            continue;
        }
        for source in m.nodes.values().filter(|s| s.node != board.node) {
            let mut start = o.clone();
            start.location = Location::Node(board.node);
            let Some(out) = policy::coherent::route(&start, m, source.node) else {
                continue;
            };
            start.location = Location::Node(source.node);
            start.nutrition = out.nutrition;
            start.fatigue = (o.fatigue + out.time).min(24 * HOUR);
            start.starvation = out.starvation;
            let Some(back) = policy::coherent::route(&start, m, board.node) else {
                continue;
            };
            let round = out.time.saturating_add(back.time);
            if round == 0 {
                continue;
            }
            for f in &source.food {
                if !(f.owner == Owner::None
                    || f.owner == Owner::Commons(0)
                    || f.owner == Owner::Agent(o.agent))
                {
                    continue;
                }
                let q = demand.min(f.available);
                if q == 0 || f.expires_at <= o.now + round {
                    continue;
                }
                let horizon = ((f.expires_at - o.now) / round) as u64;
                let q = q.min(horizon.saturating_mul(o.capacity));
                if q == 0 {
                    continue;
                }
                let without = (q.div_ceil(o.capacity) as Time).saturating_mul(round);
                let with =
                    (q.div_ceil(o.capacity + r.cargo_capacity) as Time).saturating_mul(round);
                if candidate
                    .as_ref()
                    .is_none_or(|(gain, _, _, _)| without - with > *gain)
                {
                    candidate = Some((without - with, q, without, with));
                }
            }
        }
    }
    let Some((saving, q, without, with)) = candidate else {
        return e;
    };
    e.demand = q;
    e.without_ms = without;
    e.with_ms = with;
    let project = local
        .projects
        .iter()
        .find(|p| p.investor == o.agent && p.cart.is_none());
    let own_wood: u64 = local
        .wood
        .iter()
        .filter(|w| w.owner == Owner::Agent(o.agent))
        .map(|w| w.available)
        .sum();
    e.missing = if project.is_some() {
        0
    } else {
        r.wood_cost.saturating_sub(own_wood)
    };
    let available: u64 = local
        .wood
        .iter()
        .filter(|w| w.owner == Owner::None)
        .map(|w| w.available)
        .sum();
    if available < e.missing {
        e.gate = "INSUFFICIENT_LOCAL_WOOD".into();
        return e;
    }
    let remaining = project.map_or(r.work_ms as u128 * SCALE, |p| p.remaining);
    let Some((work, n, _)) = policy::coherent::capital_work(o, e.missing, remaining) else {
        e.gate = "CONSTRUCTION_FATAL".into();
        return e;
    };
    e.work_ms = work;
    if n < 6 * HOUR {
        e.gate = "SIX_HOUR_MARGIN".into();
        return e;
    }
    if saving <= work {
        e.gate = "NO_TIME_PAYBACK".into();
        return e;
    }
    e.gate = "INVEST".into();
    e.intent = if e.missing == 0 {
        Some(Intent::BuildCart)
    } else if o.load < o.capacity {
        local
            .wood
            .iter()
            .find(|w| {
                w.available > 0 && w.owner == Owner::None && matches!(w.holder, Holder::Deposit(_))
            })
            .map(|w| Intent::Gather {
                batch: w.batch,
                communal: false,
            })
    } else {
        local
            .wood
            .iter()
            .find(|w| w.owner == Owner::Agent(o.agent) && w.holder == Holder::Agent(o.agent))
            .map(|w| Intent::Drop {
                batch: w.batch,
                quantity: w.available,
            })
    };
    if e.intent.is_none() {
        e.gate = "INPUT_ACCESS_BLOCKED".into();
    }
    e
}
pub(crate) fn consider(
    o: &Observation,
    m: &PolicyMemory,
    capital: &Memory,
    r: &Rules,
) -> Option<Decision> {
    let e = evaluate(o, m, capital, r);
    e.intent.map(|intent|Decision{intent,reason:format!("personal capital {}; conditional demand={} without_ms={} with_ms={} work_ms={} wood_missing={}",e.gate,e.demand,e.without_ms,e.with_ms,e.work_ms,e.missing)})
}
#[cfg(test)]
mod tests;
