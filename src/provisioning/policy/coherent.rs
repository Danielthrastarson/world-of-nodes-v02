//! Opt-in mechanics correction. Only personal observations/memory enter this model.
//! No World, scheduler, remote stock or shadow oracle is available here.
use super::*;
use forecast::{Plan, TravelStep};

#[derive(Clone, Debug)]
struct State {
    elapsed: Time,
    nutrition: Time,
    fatigue: Time,
    starvation: Time,
}
impl State {
    fn new(o: &Observation) -> Self {
        Self {
            elapsed: 0,
            nutrition: o.nutrition,
            fatigue: o.fatigue,
            starvation: o.starvation,
        }
    }
    fn alive(&self) -> bool {
        self.starvation < 24 * HOUR
    }
    fn wait(&mut self, duration: Time) {
        let dt = duration
            .min(self.nutrition + (24 * HOUR - self.starvation).max(0))
            .max(0);
        self.starvation += (dt - self.nutrition).max(0);
        self.nutrition = (self.nutrition - dt).max(0);
        self.fatigue = (self.fatigue + dt).min(24 * HOUR);
        self.elapsed += dt;
    }
    fn work(&mut self, nominal: Time, limit: Time) {
        let mut left = nominal as u128 * SCALE;
        while left > 0 && self.alive() && self.elapsed < limit {
            let rate = performance(self.nutrition, self.fatigue);
            let dt = super::super::awake_threshold(self.nutrition, self.fatigue)
                .min(left.div_ceil(rate) as Time)
                .min(self.nutrition + 24 * HOUR - self.starvation)
                .min(limit - self.elapsed);
            if dt <= 0 {
                break;
            }
            left = left.saturating_sub(rate * dt as u128);
            self.wait(dt);
        }
    }
}

/// Remove only currently witnessed own membership, once. Never discount strangers.
fn evidence(o: &Observation, m: &PolicyMemory) -> (Observation, PolicyMemory) {
    let mut o = o.clone();
    let mut m = m.clone();
    if let (Location::Node(node), Action::Queued { edge, .. }) = (&o.location, &o.action)
        && let Some(v) = m.nodes.get_mut(node)
        && v.observed_at == o.now
        && let Some(e) = v.edges.iter_mut().find(|e| e.id == *edge)
    {
        e.queued = e.queued.saturating_sub(1);
        o.action = Action::Idle;
    }
    (o, m)
}

pub(super) fn route(o: &Observation, m: &PolicyMemory, target: usize) -> Option<Route> {
    let (o, m) = evidence(o, m);
    route_normalized(&o, &m, target)
}
fn route_normalized(o: &Observation, m: &PolicyMemory, target: usize) -> Option<Route> {
    let Location::Node(start) = o.location else {
        return None;
    };
    let mut edges: BTreeMap<usize, (EdgeView, Time)> = BTreeMap::new();
    for v in m.nodes.values() {
        for e in &v.edges {
            if edges.get(&e.id).is_none_or(|(_, t)| *t < v.observed_at) {
                edges.insert(e.id, (e.clone(), v.observed_at));
            }
        }
    }
    let mut states = BTreeMap::from([(start, (State::new(o), usize::MAX, vec![]))]);
    let mut visited = BTreeSet::new();
    loop {
        let (&node, (body, first, steps)) = states
            .iter()
            .filter(|(n, _)| !visited.contains(*n))
            .min_by_key(|(_, (b, _, _))| b.elapsed)?;
        let (body, first, steps) = (body.clone(), *first, steps.clone());
        if node == target {
            return Some(Route {
                first,
                time: body.elapsed,
                starvation: body.starvation,
                nutrition: body.nutrition,
                steps,
            });
        }
        visited.insert(node);
        for (e, observed_at) in edges.values() {
            if e.edge.capacity == 0 {
                continue;
            }
            let to = if e.edge.a == node {
                e.edge.b
            } else if e.edge.b == node {
                e.edge.a
            } else {
                continue;
            };
            if visited.contains(&to) {
                continue;
            }
            let mut b = body.clone();
            let delay = ((e.queued + e.occupied) / e.edge.capacity) as Time * e.edge.nominal_ms;
            b.wait(delay);
            let start_move = b.elapsed;
            b.work(e.edge.nominal_ms, Time::MAX);
            let mut path = steps.clone();
            path.push(TravelStep {
                from: node,
                to,
                edge: e.id,
                travel_ms: b.elapsed - start_move,
                queue_ms: delay,
                observed_at: *observed_at,
                nominal_ms: e.edge.nominal_ms,
            });
            if states
                .get(&to)
                .is_none_or(|(old, _, _)| b.elapsed < old.elapsed)
            {
                states.insert(to, (b, if node == start { e.id } else { first }, path));
            }
        }
    }
}

pub(super) fn sources(o: &Observation, m: &PolicyMemory) -> Vec<Source> {
    let mut out = vec![];
    for v in m.nodes.values() {
        let Some(r) = route(o, m, v.node) else {
            continue;
        };
        for f in &v.food {
            if f.quantity == 0 || !accessible(f, o.agent) {
                continue;
            }
            let mut body = State::new(o);
            body.wait(r.time);
            if matches!(f.holder, Holder::Deposit(_)) {
                body.work(2000, Time::MAX);
            }
            if f.expires_at <= o.now + body.elapsed {
                continue;
            }
            let age = o.now - v.observed_at;
            out.push(Source {
                node: v.node,
                food: f.clone(),
                age,
                confidence: (-(age as f64) / (12 * HOUR) as f64).exp(),
                route: Route {
                    time: body.elapsed,
                    nutrition: body.nutrition,
                    starvation: body.starvation,
                    ..r.clone()
                },
            });
        }
    }
    out.sort_by_key(|s| {
        (
            s.route.starvation >= 24 * HOUR,
            s.route.time,
            s.age,
            s.node,
            s.food.batch,
        )
    });
    out
}

struct Simulation<'a> {
    initial: &'a Observation,
    memory: &'a PolicyMemory,
    state: Observation,
    body: State,
    items: Vec<FoodView>,
    load: u64,
    plan: Plan,
    meals: bool,
}
impl Simulation<'_> {
    fn now(&self) -> Time {
        self.initial.now + self.body.elapsed
    }
    fn sync(&mut self) {
        self.state.now = self.now();
        self.state.nutrition = self.body.nutrition;
        self.state.fatigue = self.body.fatigue;
        self.state.starvation = self.body.starvation;
        self.state.action = Action::Idle;
        if !self.body.alive() {
            self.plan.failure = Some("CANNOT_COMPLETE_PLAN_ALIVE");
        }
    }
    fn cadence(&mut self) {
        if !self.meals {
            return;
        } // Legacy no-meal plan had no tick pauses.
        let dt = self.initial.decision_interval.max(1)
            - self.now().rem_euclid(self.initial.decision_interval.max(1));
        self.plan.steps.push(format!(
            "next_policy: node={:?} wait_ms={dt}",
            self.state.location
        ));
        self.body.wait(dt);
        self.sync();
    }
    fn eat(&mut self) {
        if !self.meals || self.body.nutrition >= 12 * HOUR || self.plan.failure.is_some() {
            return;
        }
        self.items.sort_by_key(|f| (f.expires_at, f.batch));
        for i in 0..self.items.len() {
            let f = self.items[i].clone();
            if f.expires_at <= self.now() {
                continue;
            }
            let q = f
                .quantity
                .min(super::super::goods::headroom(self.body.nutrition, f.owner));
            if q == 0 {
                continue;
            }
            let before = self.body.nutrition;
            self.items[i].quantity -= q;
            self.load -= q;
            self.body.nutrition += q as Time * HOUR;
            self.body.starvation = 0;
            self.plan.meal_quanta += q;
            self.plan.steps.push(format!("meal: t={} node={:?} batch={} owner={:?} q={} nutrition_before={} after={} expiry={} evidence=own_or_conditionally_acquired_remembered_batch", self.now(),self.state.location,f.batch,f.owner,q,before,self.body.nutrition,f.expires_at));
            self.sync();
            self.cadence();
            break;
        }
    }
    fn travel(&mut self, target: usize, label: &str) {
        let mut visited = BTreeSet::new();
        while self.state.location != Location::Node(target) && self.plan.failure.is_none() {
            if !visited.insert(format!("{:?}", self.state.location)) {
                self.plan.failure = Some("OTHER_EXPLICIT_REASON");
                break;
            }
            self.eat();
            if self.plan.failure.is_some() {
                break;
            }
            let Some(r) = route_normalized(&self.state, self.memory, target) else {
                self.plan.failure = Some("OTHER_EXPLICIT_REASON");
                break;
            };
            let Some(s) = r.steps.first() else {
                self.plan.failure = Some("OTHER_EXPLICIT_REASON");
                break;
            };
            let start = self.now();
            self.body.wait(s.queue_ms);
            let move_start = self.body.elapsed;
            self.body.work(s.nominal_ms, Time::MAX);
            self.sync();
            if self.body.alive() {
                self.state.location = Location::Node(s.to);
            }
            self.plan.steps.push(format!("{label}: edge={} {}->{} start={} queue_ms={} nominal_ms={} travel_ms={} total_ms={} evidence_at={} queue_order=before_travel conditional=true",s.edge,s.from,s.to,start,s.queue_ms,s.nominal_ms,self.body.elapsed-move_start,self.now()-start,s.observed_at));
        }
    }
}

pub(super) fn predict(
    o: &Observation,
    m: &PolicyMemory,
    board: usize,
    source: &Source,
    q: u64,
    meals: bool,
) -> Plan {
    let (initial, evidence) = evidence(o, m);
    let mut sim = Simulation {
        initial: &initial,
        memory: &evidence,
        state: initial.clone(),
        body: State::new(&initial),
        items: initial.own.clone(),
        load: initial.load,
        plan: Plan::default(),
        meals,
    };
    sim.travel(board, "to_board");
    if sim.plan.failure.is_none() {
        sim.plan.steps.push(format!(
            "claim: node={board} target={q} board_observed_at={} conditional=true",
            m.nodes[&board].observed_at
        ));
        if q == 0 {
            sim.plan.failure = Some("INSUFFICIENT_EXPECTED_QUANTITY");
        } else {
            sim.cadence();
            sim.travel(source.node, "to_source");
        }
    }
    sim.plan.source_starvation = sim.body.starvation;
    if !sim.body.alive() {
        sim.plan.failure = Some("CANNOT_REACH_SOURCE_ALIVE");
    }
    let mut available = source.food.quantity.min(source.food.available);
    while sim.plan.acquired < q && sim.plan.failure.is_none() {
        sim.eat();
        if sim.plan.failure.is_some() {
            break;
        }
        if available == 0 {
            sim.plan.failure = Some("INSUFFICIENT_EXPECTED_QUANTITY");
            break;
        }
        if sim.load >= o.capacity {
            sim.plan.failure = Some("CARRY_CAPACITY_ZERO");
            break;
        }
        let start = sim.now();
        // Record the attempted atomic action, even if expiry/death prevents completion.
        sim.plan.steps.push(format!("acquire: node={} source_batch={} source_observed_at={} q=1 original_expiry={} owner=Commons(0) conditional=true",source.node,source.food.batch,o.now-source.age,source.food.expires_at));
        if source.food.expires_at <= start {
            sim.plan.failure = Some("SOURCE_PREDICTED_SPOILED");
            break;
        }
        if matches!(source.food.holder, Holder::Deposit(_)) {
            sim.body.work(2000, source.food.expires_at - o.now);
        }
        sim.sync();
        if sim.plan.failure.is_some() {
            break;
        }
        if source.food.expires_at <= sim.now() {
            sim.plan.failure = Some("SOURCE_PREDICTED_SPOILED");
            break;
        }
        available -= 1;
        sim.plan.acquired += 1;
        sim.load += 1;
        if let Some(f) = sim.items.iter_mut().find(|f| f.batch == usize::MAX) {
            f.quantity += 1;
            f.available += 1;
        } else {
            sim.items.push(FoodView {
                batch: usize::MAX,
                quantity: 1,
                available: 1,
                owner: Owner::Commons(0),
                holder: Holder::Agent(o.agent),
                expires_at: source.food.expires_at,
            });
        }
        if !matches!(source.food.holder, Holder::Deposit(_)) {
            sim.cadence();
        }
    }
    sim.eat();
    if sim.plan.failure.is_none() {
        sim.travel(board, "return");
    }
    sim.plan.elapsed = sim.body.elapsed;
    sim.plan.nutrition = sim.body.nutrition;
    sim.plan.starvation = sim.body.starvation;
    if sim.plan.failure.is_none() {
        sim.plan.delivered = sim
            .items
            .iter()
            .filter(|f| f.owner == Owner::Commons(0) && f.expires_at > sim.now())
            .map(|f| f.quantity)
            .sum();
        if sim.plan.delivered == 0 {
            sim.plan.failure = Some("SOURCE_PREDICTED_SPOILED");
        }
    }
    sim.plan
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provisioning::{acceptance, shadow};
    fn input() -> (Observation, PolicyMemory) {
        let w = acceptance::fixture("known_source", 0, false).unwrap();
        let o = w.observe(0).unwrap();
        let mut m = w.agents[0].memory.clone();
        m.nodes.insert(0, o.local.clone().unwrap());
        (o, m)
    }
    #[test]
    fn queue_wait_changes_physiology_before_movement() {
        let (mut o, mut m) = input();
        o.nutrition = 8 * HOUR;
        o.fatigue = 10 * HOUR;
        m.nodes.get_mut(&0).unwrap().edges[0].queued = 4;
        let r = route(&o, &m, 1).unwrap();
        assert_eq!(r.time, 17_200_000);
        assert_eq!(r.nutrition, 8 * HOUR - r.time);
    }
    #[test]
    fn observed_self_is_removed_once_without_discounting_other_travelers() {
        let (mut o, mut m) = input();
        o.nutrition = 24 * HOUR;
        o.action = Action::Queued {
            edge: 0,
            to: 1,
            joined: 0,
        };
        m.nodes.get_mut(&0).unwrap().edges[0].queued = 4;
        let before = format!("{m:?}");
        let a = super::super::coherent_contract(&o, &m, false);
        assert_eq!(before, format!("{m:?}"));
        m.nodes.get_mut(&0).unwrap().edges[0].queued = 3;
        o.action = Action::Idle;
        let b = super::super::coherent_contract(&o, &m, false);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        // With no membership evidence, all four remembered travelers remain.
        m.nodes.get_mut(&0).unwrap().edges[0].queued = 4;
        let c = super::super::coherent_contract(&o, &m, false);
        assert!(
            c.candidates[0].plan.as_ref().unwrap().elapsed
                > b.candidates[0].plan.as_ref().unwrap().elapsed
        );
    }
    #[test]
    fn spoilage_interrupts_forecast_gather_at_original_deadline() {
        let (mut o, mut m) = input();
        o.nutrition = 24 * HOUR;
        m.nodes.get_mut(&2).unwrap().food[0].expires_at = 4 * HOUR + 4001;
        let a = super::super::coherent_contract(&o, &m, false);
        let c = &a.candidates[0];
        let p = c.plan.as_ref().unwrap();
        assert_eq!(p.elapsed, 4 * HOUR + 4001);
        assert_eq!(p.acquired, 2);
        assert_eq!(p.delivered, 0);
        let tape = shadow::plan::tape(c, &m).unwrap();
        let k = shadow::oracle::belief_endpoint(&o, &m, c, &tape);
        assert_eq!(k.elapsed, p.elapsed);
        assert_eq!(k.nutrition, p.nutrition);
    }
    #[test]
    fn corrected_policy_is_pure_and_preserves_risk_margin_and_no_meal_assumption() {
        let (o, m) = input();
        let before = format!("{o:?}{m:?}");
        let (_, a) = super::super::audit_coherent(&o, &m, &mut 0, false);
        assert_eq!(a.margin, 6 * HOUR);
        assert_eq!(a.outcome, "POST_PLAN_NUTRITION_MARGIN");
        assert_eq!(a.candidates[0].plan.as_ref().unwrap().meal_quanta, 0);
        assert_eq!(before, format!("{o:?}{m:?}"));
    }
}
