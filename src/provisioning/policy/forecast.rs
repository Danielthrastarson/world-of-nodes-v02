//! Acceptance-only prediction. Inputs are observations and timestamped personal memory.
//! No World, scheduler, global stock or future queue query is available here.
use super::*;
use crate::provisioning::goods::headroom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ForecastMode {
    Legacy(Time),
    Meals,
    MealsSelfExcluded,
    Coherent,
    CoherentMeals,
}
impl ForecastMode {
    pub fn coherent(self) -> bool {
        matches!(self, Self::Coherent | Self::CoherentMeals)
    }
    pub fn margin(self) -> Time {
        match self {
            Self::Legacy(n) => n,
            Self::Meals | Self::MealsSelfExcluded | Self::Coherent | Self::CoherentMeals => {
                6 * HOUR
            }
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct TravelStep {
    pub from: usize,
    pub to: usize,
    pub edge: usize,
    pub travel_ms: Time,
    pub queue_ms: Time,
    pub observed_at: Time,
    pub nominal_ms: Time,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct Plan {
    pub elapsed: Time,
    pub nutrition: Time,
    pub starvation: Time,
    pub source_starvation: Time,
    pub meal_quanta: u64,
    pub delivered: u64,
    pub acquired: u64,
    pub steps: Vec<String>,
    pub failure: Option<&'static str>,
    pub shadow: Option<(Time, Time, Time, &'static str)>,
}
#[derive(Clone)]
pub(crate) struct Candidate {
    pub board: usize,
    pub source: Option<usize>,
    pub observed_at: Option<Time>,
    pub quantity: u64,
    pub expiry: Option<Time>,
    pub confidence: f64,
    pub violations: Vec<&'static str>,
    pub plan: Option<Plan>,
}
impl std::fmt::Debug for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Candidate")
            .field("board", &self.board)
            .field("source", &self.source)
            .field("observed_at", &self.observed_at)
            .field("quantity", &self.quantity)
            .field("expiry", &self.expiry)
            .field("confidence", &self.confidence)
            .field("violations", &self.violations)
            .field("plan", &self.plan)
            .finish()
    }
}
#[derive(Clone, Debug, Default)]
pub(crate) struct AcceptanceAudit {
    pub enabled: bool,
    pub requests: usize,
    pub local: bool,
    pub evaluated: bool,
    pub margin: Time,
    pub outcome: String,
    pub detail: String,
    pub candidates: Vec<Candidate>,
}
impl AcceptanceAudit {
    pub fn disabled() -> Self {
        Self::default()
    }
    pub fn new(o: &Observation, m: &PolicyMemory, mode: ForecastMode) -> Self {
        Self {
            enabled: true,
            requests: m
                .nodes
                .values()
                .filter(|v| v.board.as_ref().is_some_and(|b| b.quantity > 0))
                .count(),
            local: o
                .local
                .as_ref()
                .and_then(|v| v.board.as_ref())
                .is_some_and(|b| b.quantity > 0),
            margin: mode.margin(),
            ..Self::default()
        }
    }
    pub fn finish(&mut self, o: &Observation, d: &Decision) {
        if !self.enabled || self.requests == 0 {
            return;
        }
        self.detail = d.reason.clone();
        if !self.evaluated {
            self.outcome = if matches!(d.intent, Intent::Sleep) {
                "FATIGUE_CONSTRAINT"
            } else if o.load >= o.capacity && !matches!(d.intent, Intent::Eat { .. }) {
                "CARRY_CAPACITY_ZERO"
            } else {
                "OTHER_EXPLICIT_REASON"
            }
            .into();
        } else if self.candidates.iter().any(|c| c.violations.is_empty()) {
            self.outcome = "PASS".into();
        } else {
            self.outcome = self
                .candidates
                .iter()
                .filter_map(|c| c.violations.first())
                .next()
                .copied()
                .unwrap_or("NO_KNOWN_SOURCE")
                .into();
        }
    }
}
fn reject(a: &mut AcceptanceAudit, board: usize, source: Option<usize>, gate: &'static str) {
    if a.enabled {
        a.candidates.push(Candidate {
            board,
            source,
            observed_at: None,
            quantity: 0,
            expiry: None,
            confidence: 0.,
            violations: vec![gate],
            plan: None,
        });
    }
}
fn violations(p: &Plan, expiry: Time, now: Time, margin: Time) -> Vec<&'static str> {
    let mut out = vec![];
    if let Some(g) = p.failure {
        out.push(g);
    }
    if p.source_starvation >= 24 * HOUR && !out.contains(&"CANNOT_REACH_SOURCE_ALIVE") {
        out.push("CANNOT_REACH_SOURCE_ALIVE");
    }
    if p.starvation >= 24 * HOUR && !out.contains(&"CANNOT_COMPLETE_PLAN_ALIVE") {
        out.push("CANNOT_COMPLETE_PLAN_ALIVE");
    }
    if p.nutrition < margin {
        out.push("POST_PLAN_NUTRITION_MARGIN");
    }
    if expiry <= now + p.elapsed && !out.contains(&"SOURCE_PREDICTED_SPOILED") {
        out.push("SOURCE_PREDICTED_SPOILED");
    }
    if p.delivered == 0 && !out.contains(&"INSUFFICIENT_EXPECTED_QUANTITY") {
        out.push("INSUFFICIENT_EXPECTED_QUANTITY");
    }
    out
}
fn trace_leg(p: &mut Plan, label: &str, r: &Route) {
    for s in &r.steps {
        p.steps.push(format!("{label}: edge={} {}->{} travel_ms={} queue_ms={} evidence_at={} queue_order=legacy_after_travel",s.edge,s.from,s.to,s.travel_ms,s.queue_ms,s.observed_at));
    }
}
/// This arithmetic is intentionally identical to P1, including its approximations.
fn legacy(
    o: &Observation,
    m: &PolicyMemory,
    board: usize,
    s: &Source,
    to_board: &Route,
    q: u64,
) -> Option<Plan> {
    let mut p = Plan::default();
    trace_leg(&mut p, "to_board", to_board);
    let mut predicted = o.clone();
    predicted.location = Location::Node(board);
    predicted.action = Action::Idle;
    (predicted.nutrition, predicted.starvation) = after(o, to_board.time);
    predicted.fatigue = (o.fatigue + to_board.time).min(24 * HOUR);
    let outbound = route(&predicted, m, s.node)?;
    trace_leg(&mut p, "to_source", &outbound);
    let work = ((o.capacity - o.load) as u128 * 2000 * SCALE).div_ceil(performance(
        outbound.nutrition,
        (predicted.fatigue + outbound.time).min(24 * HOUR),
    )) as Time;
    let elapsed = to_board.time + outbound.time + work;
    p.source_starvation = after(o, to_board.time + outbound.time).1;
    p.steps.push(format!("gather: node={} work_ms={work} legacy_free_slots={} claim_q={q} meals=0 source_observed_at={} expiry={}",s.node,o.capacity-o.load,o.now-s.age,s.food.expires_at));
    predicted.location = Location::Node(s.node);
    (predicted.nutrition, predicted.starvation) = after(o, elapsed);
    predicted.fatigue = (o.fatigue + elapsed).min(24 * HOUR);
    let inbound = route(&predicted, m, board)?;
    trace_leg(&mut p, "return", &inbound);
    p.elapsed = elapsed + inbound.time;
    (p.nutrition, p.starvation) = after(o, p.elapsed);
    p.delivered = q;
    p.acquired = q;
    Some(p)
}
/// Same candidate scoring and board visit/claim requirement as P1.
pub(super) fn evaluate(
    o: &Observation,
    m: &PolicyMemory,
    known: &[Source],
    mode: ForecastMode,
    a: &mut AcceptanceAudit,
) -> Option<(f64, usize, u64, Route)> {
    a.evaluated = true;
    let mut best: Option<(f64, usize, u64, Route)> = None;
    for view in m.nodes.values() {
        let Some(board) = &view.board else {
            continue;
        };
        if board.quantity == 0 {
            continue;
        }
        let demand = board.unclaimed(o.now);
        if demand == 0 {
            reject(a, board.node, None, "REQUEST_ALREADY_CLAIMED");
            continue;
        }
        let Some(to_board) = route_for(o, m, board.node, mode) else {
            reject(a, board.node, None, "OTHER_EXPLICIT_REASON");
            continue;
        };
        let mut considered = false;
        for source in known {
            if source.node == board.node {
                continue;
            }
            considered = true;
            let q = demand
                .min(o.capacity - o.load)
                .min(source.food.quantity)
                .min(if mode.coherent() {
                    source.food.available
                } else {
                    u64::MAX
                });
            let maybe = match mode {
                ForecastMode::Coherent | ForecastMode::CoherentMeals => Some(coherent::predict(
                    o,
                    m,
                    board.node,
                    source,
                    q,
                    mode == ForecastMode::CoherentMeals,
                )),
                ForecastMode::Legacy(_) => legacy(o, m, board.node, source, &to_board, q),
                ForecastMode::Meals | ForecastMode::MealsSelfExcluded => {
                    let corrected = mode == ForecastMode::MealsSelfExcluded;
                    let mut p = physical(o, m, board.node, source, q, true, corrected);
                    let shadow = physical(o, m, board.node, source, q, false, corrected);
                    let gates = violations(&shadow, source.food.expires_at, o.now, 6 * HOUR);
                    p.shadow = Some((
                        shadow.elapsed,
                        shadow.nutrition,
                        shadow.starvation,
                        gates.first().copied().unwrap_or("PASS"),
                    ));
                    Some(p)
                }
            };
            let Some(p) = maybe else {
                reject(a, board.node, Some(source.node), "OTHER_EXPLICIT_REASON");
                continue;
            };
            let gates = violations(&p, source.food.expires_at, o.now, mode.margin());
            let allowed = gates.is_empty();
            if a.enabled {
                a.candidates.push(Candidate {
                    board: board.node,
                    source: Some(source.node),
                    observed_at: Some(o.now - source.age),
                    quantity: source.food.quantity,
                    expiry: Some(source.food.expires_at),
                    confidence: source.confidence,
                    violations: gates,
                    plan: Some(p.clone()),
                });
            }
            if !allowed {
                continue;
            }
            let remaining =
                (source.food.expires_at - o.now - p.elapsed) as f64 / (48 * HOUR) as f64;
            let urgency = 1.0 + board.quantity.min(600) as f64 / 600.0;
            let score = p.delivered as f64 / p.elapsed.max(1) as f64
                * urgency
                * source.confidence
                * remaining.min(1.0);
            if best.as_ref().is_none_or(|(v, _, _, _)| score > *v) {
                best = Some((score, board.node, q, to_board.clone()));
            }
        }
        if !considered && a.enabled {
            let mut found = false;
            for v in m.nodes.values().filter(|v| v.node != board.node) {
                for f in &v.food {
                    found = true;
                    let mut partial = None;
                    let gate = if f.quantity == 0 {
                        "INSUFFICIENT_EXPECTED_QUANTITY"
                    } else if !accessible(f, o.agent) {
                        "OTHER_EXPLICIT_REASON"
                    } else if let Some(r) = route_for(o, m, v.node, mode) {
                        let work = if matches!(f.holder, Holder::Deposit(_)) {
                            (2000 * SCALE).div_ceil(performance(
                                r.nutrition,
                                (o.fatigue + r.time).min(24 * HOUR),
                            )) as Time
                        } else {
                            0
                        };
                        partial = Some(if mode.coherent() {
                            coherent::prefilter(o, &r, f, v.observed_at)
                        } else {
                            Plan {
                                elapsed: r.time + work,
                                nutrition: (r.nutrition - work).max(0),
                                starvation: r.starvation + (work - r.nutrition).max(0),
                                source_starvation: r.starvation,
                                steps: vec![format!(
                                    "source_prefilter_only: route={:?} gather_one_ms={work} expiry={} observed_at={}; full job not evaluated",
                                    r.steps, f.expires_at, v.observed_at
                                )],
                                ..Plan::default()
                            }
                        });
                        if f.expires_at <= o.now + partial.as_ref().unwrap().elapsed {
                            "SOURCE_PREDICTED_SPOILED"
                        } else {
                            "OTHER_EXPLICIT_REASON"
                        }
                    } else {
                        "OTHER_EXPLICIT_REASON"
                    };
                    a.candidates.push(Candidate {
                        board: board.node,
                        source: Some(v.node),
                        observed_at: Some(v.observed_at),
                        quantity: f.quantity,
                        expiry: Some(f.expires_at),
                        confidence: (-((o.now - v.observed_at) as f64) / (12 * HOUR) as f64).exp(),
                        violations: vec![gate],
                        plan: partial,
                    });
                }
            }
            if !found {
                reject(a, board.node, None, "NO_KNOWN_SOURCE");
            }
        }
    }
    best
}

struct Sim<'a> {
    initial: &'a Observation,
    memory: &'a PolicyMemory,
    state: Observation,
    items: Vec<FoodView>,
    waste: u64,
    plan: Plan,
    meals: bool,
    cargo_left: u64,
}
impl Sim<'_> {
    fn now(&self) -> Time {
        self.state.now
    }
    fn elapse(&mut self, dt: Time) {
        let dt = dt.min(self.state.nutrition + (24 * HOUR - self.state.starvation).max(0));
        let zero = (dt - self.state.nutrition).max(0);
        self.state.starvation += zero;
        self.state.nutrition = (self.state.nutrition - dt).max(0);
        self.state.fatigue = (self.state.fatigue + dt).min(24 * HOUR);
        self.state.now += dt;
        if self.state.starvation >= 24 * HOUR {
            self.plan.failure = Some("CANNOT_COMPLETE_PLAN_ALIVE");
        }
    }
    fn work(&mut self, nominal: Time) {
        let mut left = nominal as u128 * SCALE;
        while left > 0 && self.plan.failure.is_none() {
            let rate = performance(self.state.nutrition, self.state.fatigue);
            let dt = super::super::awake_threshold(self.state.nutrition, self.state.fatigue)
                .min(left.div_ceil(rate) as Time)
                .min(if self.state.nutrition == 0 {
                    (24 * HOUR - self.state.starvation).max(1)
                } else {
                    Time::MAX
                })
                .max(1);
            left = left.saturating_sub(rate * dt as u128);
            self.elapse(dt);
        }
    }
    fn load(&self) -> u64 {
        self.waste + self.items.iter().map(|f| f.quantity).sum::<u64>()
    }
    fn pause_after_instant(&mut self) {
        let cadence = self.initial.decision_interval.max(1);
        let dt = cadence - self.now().rem_euclid(cadence);
        self.plan.steps.push(format!(
            "next_policy: node={:?} wait_ms={dt}",
            self.state.location
        ));
        self.elapse(dt);
    }
    fn eat(&mut self) {
        if !self.meals || self.state.nutrition >= 12 * HOUR || self.plan.failure.is_some() {
            return;
        }
        self.items.sort_by_key(|f| (f.expires_at, f.batch));
        for i in 0..self.items.len() {
            let f = self.items[i].clone();
            if f.expires_at <= self.now() {
                continue;
            }
            let q = f.quantity.min(headroom(self.state.nutrition, f.owner));
            if q == 0 {
                continue;
            }
            let before = self.state.nutrition;
            self.items[i].quantity -= q;
            if f.owner == Owner::Commons(0) {
                self.cargo_left = self.cargo_left.saturating_sub(q);
            }
            self.state.nutrition += q as Time * HOUR;
            self.state.starvation = 0;
            self.plan.meal_quanta += q;
            self.plan.steps.push(format!("meal: t={} node={:?} batch={} owner={:?} q={} nutrition_before={} after={} expiry={} load_after={} cargo_remaining={} evidence=own_inventory_or_conditionally_acquired_remembered_batch",self.now(),self.state.location,f.batch,f.owner,q,before,self.state.nutrition,f.expires_at,self.load(),self.cargo_left));
            self.pause_after_instant();
            break;
        }
    }
    fn travel(&mut self, target: usize, label: &str) {
        let mut seen = BTreeSet::new();
        loop {
            let Location::Node(node) = self.state.location else {
                unreachable!()
            };
            if node == target || self.plan.failure.is_some() {
                break;
            }
            if !seen.insert(node) {
                self.plan.failure = Some("OTHER_EXPLICIT_REASON");
                break;
            }
            self.eat();
            let Some(r) = route(&self.state, self.memory, target) else {
                self.plan.failure = Some("OTHER_EXPLICIT_REASON");
                break;
            };
            let Some(s) = r.steps.first() else {
                self.plan.failure = Some("OTHER_EXPLICIT_REASON");
                break;
            };
            let start = self.now();
            let n = self.state.nutrition;
            let f = self.state.fatigue;
            // Same remembered queue evidence, with actual waiting before movement.
            self.elapse(s.queue_ms);
            self.work(s.nominal_ms);
            if self.plan.failure.is_none() {
                self.state.location = Location::Node(s.to);
            }
            self.state.action = Action::Idle;
            self.plan.steps.push(format!("{label}: edge={} {}->{} start={} queue_ms={} nominal_ms={} total_ms={} n_before={} n_after={} fatigue_before={} after={} evidence_at={} queue_order=before_travel conditional=true",s.edge,s.from,s.to,start,s.queue_ms,s.nominal_ms,self.now()-start,n,self.state.nutrition,f,self.state.fatigue,s.observed_at));
        }
    }
}
fn physical(
    o: &Observation,
    m: &PolicyMemory,
    board: usize,
    s: &Source,
    target: u64,
    meals: bool,
    exclude_self: bool,
) -> Plan {
    // Only self membership witnessed in the actual local observation is removed.
    // Keep the copied evidence constant on every hypothetical leg; never infer
    // future departures or subtract the same membership twice.
    let mut evidence = m.clone();
    let mut initial = o.clone();
    let mut normalization = None;
    if exclude_self
        && let (Location::Node(node), Action::Queued { edge, .. }) = (&o.location, &o.action)
        && let Some(view) = evidence.nodes.get_mut(node)
        && view.observed_at == o.now
        && let Some(e) = view.edges.iter_mut().find(|e| e.id == *edge)
    {
        normalization = Some(format!(
            "self_exclusion: node={node} edge={edge} observed_at={} queued_before={} subtraction=1 repeated_subtraction=false",
            view.observed_at, e.queued
        ));
        e.queued = e.queued.saturating_sub(1);
        initial.action = Action::Idle;
    }
    let (o, m) = if exclude_self {
        (&initial, &evidence)
    } else {
        (o, m)
    };
    let own: u64 = o.own.iter().map(|f| f.quantity).sum();
    let mut sim = Sim {
        initial: o,
        memory: m,
        state: o.clone(),
        items: o.own.clone(),
        waste: o.load.saturating_sub(own),
        plan: Plan::default(),
        meals,
        cargo_left: o
            .own
            .iter()
            .filter(|f| f.owner == Owner::Commons(0) && f.expires_at > o.now)
            .map(|f| f.quantity)
            .sum(),
    };
    if let Some(note) = normalization {
        sim.plan.steps.push(note);
    }
    sim.travel(board, "to_board");
    if sim.plan.failure.is_none() {
        sim.plan.steps.push(format!("claim: node={board} target={target} board_observed_at={} unclaimed_then={} conditional=true",m.nodes[&board].observed_at,m.nodes[&board].board.as_ref().unwrap().unclaimed(o.now)));
        sim.pause_after_instant();
        sim.travel(s.node, "to_source");
    }
    sim.plan.source_starvation = sim.state.starvation;
    if sim.state.starvation >= 24 * HOUR {
        sim.plan.failure = Some("CANNOT_REACH_SOURCE_ALIVE");
    }
    let mut available = s.food.quantity.min(s.food.available);
    let mut acquired = 0;
    while acquired < target && sim.plan.failure.is_none() {
        sim.eat();
        if available == 0 {
            sim.plan.failure = Some("INSUFFICIENT_EXPECTED_QUANTITY");
            break;
        }
        if sim.load() >= o.capacity {
            sim.plan.failure = Some("CARRY_CAPACITY_ZERO");
            break;
        }
        let start = sim.now();
        if s.food.expires_at <= start {
            sim.plan.failure = Some("SOURCE_PREDICTED_SPOILED");
            break;
        }
        if matches!(s.food.holder, Holder::Deposit(_)) {
            sim.work(2000);
        }
        if sim.plan.failure.is_some() {
            break;
        }
        if s.food.expires_at <= sim.now() {
            sim.plan.failure = Some("SOURCE_PREDICTED_SPOILED");
            break;
        }
        available -= 1;
        acquired += 1;
        sim.cargo_left += 1;
        // Synthetic descendant batch identifier is only inside this copied forecast.
        let batch = usize::MAX;
        if let Some(f) = sim.items.iter_mut().find(|f| f.batch == batch) {
            f.quantity += 1;
            f.available += 1;
        } else {
            sim.items.push(FoodView {
                batch,
                quantity: 1,
                available: 1,
                owner: Owner::Commons(0),
                holder: Holder::Agent(o.agent),
                expires_at: s.food.expires_at,
            });
        }
        sim.plan.steps.push(format!("acquire: node={} source_batch={} source_observed_at={} believed_quantity={} believed_available={} work_ms={} q=1 acquired={} load={} original_expiry={} owner=Commons(0) conditional=true",s.node,s.food.batch,o.now-s.age,s.food.quantity,s.food.available,sim.now()-start,acquired,sim.load(),s.food.expires_at));
        if !matches!(s.food.holder, Holder::Deposit(_)) {
            sim.pause_after_instant();
        }
    }
    sim.eat();
    if sim.plan.failure.is_none() {
        sim.travel(board, "return");
    }
    sim.plan.elapsed = sim.now() - o.now;
    sim.plan.nutrition = sim.state.nutrition;
    sim.plan.starvation = sim.state.starvation;
    sim.plan.acquired = acquired;
    sim.plan.delivered = sim.cargo_left;
    sim.plan
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provisioning::{
        Action, Config, DiagnosticPolicy, Intent, MINUTE, World, acceptance,
    };
    fn inputs() -> (World, Observation, PolicyMemory) {
        let w = acceptance::fixture("known_source", 0, false).unwrap();
        let o = w.observe(0).unwrap();
        let mut m = w.agents[0].memory.clone();
        m.nodes.insert(0, o.local.clone().unwrap());
        (w, o, m)
    }
    #[test]
    fn self_exclusion_is_once_per_observed_membership_on_every_hypothetical_leg() {
        let (w, mut o, mut m) = inputs();
        o.now = 5 * HOUR;
        o.location = Location::Node(2);
        o.nutrition = 18 * HOUR;
        o.fatigue = 9 * HOUR;
        o.action = Action::Queued {
            edge: 1,
            to: 1,
            joined: o.now,
        };
        let mut v = w.node_view(2);
        v.observed_at = o.now;
        v.edges[0].queued = 4;
        m.nodes.insert(2, v.clone());
        o.local = Some(v);
        let original = format!("{m:?}");
        let source = sources(&o, &m).into_iter().find(|s| s.node == 2).unwrap();
        let old = physical(&o, &m, 0, &source, 10, true, false);
        let corrected = physical(&o, &m, 0, &source, 10, true, true);
        assert!(
            old.steps
                .iter()
                .any(|s| s.contains("edge=1") && s.contains("queue_ms=7200000"))
        );
        assert!(
            !corrected
                .steps
                .iter()
                .any(|s| s.contains("queue_ms=7200000"))
        );
        assert!(
            old.steps
                .iter()
                .find(|s| s.starts_with("to_board:"))
                .unwrap()
                .contains("queue_ms=0")
        );
        assert!(
            corrected
                .steps
                .iter()
                .find(|s| s.starts_with("to_board:"))
                .unwrap()
                .contains("queue_ms=0")
        );
        assert_eq!(format!("{m:?}"), original);
        // Explicit copied evidence with three OTHER applicants is the same plan.
        m.nodes.get_mut(&2).unwrap().edges[0].queued = 3;
        o.action = Action::Idle;
        let manual = physical(&o, &m, 0, &source, 10, true, false);
        assert_eq!(corrected.elapsed, manual.elapsed);
        assert_eq!(corrected.delivered, manual.delivered);
        assert_eq!(corrected.nutrition, manual.nutrition);
        assert!(corrected.elapsed < old.elapsed);
    }
    #[test]
    fn self_exclusion_does_not_discount_queues_without_membership_evidence() {
        let (_, o, mut m) = inputs();
        m.nodes.get_mut(&2).unwrap().edges[0].queued = 4;
        let source = sources(&o, &m).into_iter().find(|s| s.node == 2).unwrap();
        assert_eq!(
            format!("{:?}", physical(&o, &m, 0, &source, 10, true, false)),
            format!("{:?}", physical(&o, &m, 0, &source, 10, true, true))
        );
    }
    #[test]
    fn zero_margin_changes_only_the_no_meal_threshold_and_never_allows_fatal_plan() {
        let (_, o, m) = inputs();
        let (_, a) = super::super::decide_audited(&o, &m, &mut 0, DiagnosticPolicy::Current);
        let (d, b) = super::super::decide_audited(&o, &m, &mut 0, DiagnosticPolicy::ZeroMargin);
        let p = a.candidates[0].plan.as_ref().unwrap();
        let q = b.candidates[0].plan.as_ref().unwrap();
        assert_eq!(format!("{p:?}"), format!("{q:?}"));
        assert_eq!(a.outcome, "POST_PLAN_NUTRITION_MARGIN");
        assert_eq!(b.outcome, "PASS");
        assert!(matches!(d.intent, Intent::Claim { board: 0, .. }));
        assert_eq!(q.meal_quanta, 0);
        let mut far = m.clone();
        for v in far.nodes.values_mut() {
            for e in &mut v.edges {
                e.edge.nominal_ms = 12 * HOUR;
            }
        }
        let (d, a) = super::super::decide_audited(&o, &far, &mut 0, DiagnosticPolicy::ZeroMargin);
        assert!(!matches!(d.intent, Intent::Claim { .. }));
        assert_ne!(a.outcome, "PASS");
    }
    #[test]
    fn meals_reduce_cargo_obey_ceiling_and_require_only_remembered_evidence() {
        let (w, o, m) = inputs();
        let (_, a) = super::super::decide_audited(&o, &m, &mut 0, DiagnosticPolicy::MealAware);
        let p = a.candidates[0].plan.as_ref().unwrap();
        assert_eq!(a.outcome, "PASS");
        assert_eq!(p.acquired, 10);
        assert_eq!(p.acquired, p.delivered + p.meal_quanta);
        assert!(p.nutrition >= 6 * HOUR);
        assert_ne!(p.shadow.unwrap().3, "PASS");
        let mut changed = World::new(Config {
            food: vec![(2, 0, 0, HOUR)],
            initial_request: 600,
            ..w.config.clone()
        })
        .unwrap();
        changed.agents[0].memory = m.clone();
        let other = changed.observe(0).unwrap();
        let (d1, a1) = super::super::decide_audited(&o, &m, &mut 7, DiagnosticPolicy::MealAware);
        let (d2, a2) =
            super::super::decide_audited(&other, &m, &mut 7, DiagnosticPolicy::MealAware);
        assert_eq!(d1, d2);
        assert_eq!(format!("{a1:?}"), format!("{a2:?}"));
    }
    fn finish_action(w: &mut World) {
        while !matches!(w.agents[0].action, Action::Idle | Action::Dead) {
            let t = w.next_time(48 * HOUR);
            w.advance_to(t).unwrap();
        }
    }
    fn actual_meal(w: &mut World) {
        if w.agents[0].nutrition >= 12 * HOUR {
            return;
        }
        let food = w
            .goods
            .batches
            .iter()
            .find(|f| {
                f.holder == Holder::Agent(0)
                    && f.quantity > 0
                    && f.material == crate::provisioning::Material::Food
            })
            .cloned();
        if let Some(f) = food {
            let q = f.quantity.min(headroom(w.agents[0].nutrition, f.owner));
            if q > 0 {
                w.apply_intents(vec![(
                    0,
                    Intent::Eat {
                        batch: f.id,
                        quantity: q,
                    },
                )])
                .unwrap();
                w.advance_to((w.now / MINUTE + 1) * MINUTE).unwrap();
            }
        }
    }
    #[test]
    fn meal_forecast_replays_with_exact_physical_time_ownership_and_nutrition() {
        let (mut w, o, m) = inputs();
        w.config.autonomous = false;
        let (_, a) = super::super::decide_audited(&o, &m, &mut 0, DiagnosticPolicy::MealAware);
        let p = a.candidates[0].plan.as_ref().unwrap();
        w.apply_intents(vec![(
            0,
            Intent::Claim {
                board: 0,
                quantity: 10,
            },
        )])
        .unwrap();
        w.advance_to(MINUTE).unwrap();
        for edge in [0, 1] {
            actual_meal(&mut w);
            w.apply_intents(vec![(
                0,
                Intent::Travel {
                    edge,
                    target: 2,
                    food: true,
                },
            )])
            .unwrap();
            finish_action(&mut w);
        }
        for _ in 0..10 {
            actual_meal(&mut w);
            w.apply_intents(vec![(
                0,
                Intent::Gather {
                    batch: 0,
                    communal: true,
                },
            )])
            .unwrap();
            finish_action(&mut w);
        }
        for edge in [1, 0] {
            actual_meal(&mut w);
            w.apply_intents(vec![(
                0,
                Intent::Travel {
                    edge,
                    target: 0,
                    food: false,
                },
            )])
            .unwrap();
            finish_action(&mut w);
        }
        assert_eq!(w.now, p.elapsed);
        assert_eq!(w.agents[0].nutrition, p.nutrition);
        assert_eq!(w.agents[0].starvation, p.starvation);
        assert_eq!(w.goods.load(0), p.delivered);
        assert_eq!(w.agents[0].consumed, p.meal_quanta);
        w.audit().unwrap();
    }
    #[test]
    fn predicted_source_expiry_and_remembered_reservations_are_not_ignored() {
        let (_, o, mut m) = inputs();
        m.nodes.get_mut(&2).unwrap().food[0].expires_at = HOUR;
        let (_, a) = super::super::decide_audited(&o, &m, &mut 0, DiagnosticPolicy::MealAware);
        assert_eq!(a.outcome, "SOURCE_PREDICTED_SPOILED");
        m.nodes.get_mut(&2).unwrap().food[0].expires_at = 48 * HOUR;
        m.nodes.get_mut(&2).unwrap().food[0].available = 0;
        let (_, a) = super::super::decide_audited(&o, &m, &mut 0, DiagnosticPolicy::MealAware);
        assert_eq!(a.outcome, "INSUFFICIENT_EXPECTED_QUANTITY");
    }
}
