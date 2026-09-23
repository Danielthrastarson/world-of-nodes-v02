//! Research instruments only. No globally informed scheduler is callable by policy.
use super::*;

pub const COUNTERS: &[&str] = &[
    "at_job_priority_nutrition_margin",
    "at_job_priority_no_known_source",
    "at_job_priority_passes_gate",
    "gate_no_known_source",
    "gate_nutrition_margin",
    "gate_shelf_life",
    "gate_claims_suppress",
    "gate_passes_gate",
    "request_observers",
    "request_observations",
    "board_evaluations_without_source",
    "jobs_accepted",
    "source_sought",
    "jobs_reaching_f",
    "jobs_acquiring_cargo",
    "returns_started",
    "returns_completed",
    "jobs_delivered",
    "delivered_quanta",
    "carrier_cargo_consumed",
    "communal_cargo_spoiled_transit",
    "abandonments",
    "queue_joins",
    "queue_voluntary_exits",
    "repeated_job_edge_joins",
    "sleep_starts",
    "sleep_renewals",
    "sleep_interruptions",
    "decision_reversals",
    "claim_expiries",
    "replacement_attempts",
    "duplicate_attempts",
    "carrier_deaths",
    "predicted_fatal_return_decisions",
    "actual_fatal_returns",
    "commitment_churn",
    "return_edge_admissions",
    "first_delivery_ms",
];
#[derive(Clone, Debug, Default)]
pub struct JobRecord {
    pub id: u64,
    pub agent: usize,
    pub board: usize,
    pub accepted: Time,
    pub sought: bool,
    pub reached_f: bool,
    pub acquired: u64,
    pub cargo_consumed: u64,
    pub returned: bool,
    pub reached_destination: bool,
    pub delivered: u64,
    pub ended: bool,
    pub claim_expired: bool,
    pub reason: String,
    pub churn: u64,
    serving: bool,
    away: bool,
    pub queue_joins: BTreeMap<usize, u64>,
}
#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    pub counts: BTreeMap<String, u64>,
    pub reasons: BTreeMap<String, u64>,
    pub jobs: Vec<JobRecord>,
    active: BTreeMap<usize, usize>,
    observers: BTreeSet<(usize, usize)>,
    last_decision: BTreeMap<usize, String>,
    sleeping: BTreeSet<usize>,
    pub samples: Vec<String>,
    sampled: BTreeSet<(usize, String)>,
}
fn value(text: &str, key: &str) -> Option<u64> {
    text.split(&format!("{key}="))
        .nth(1)?
        .split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse()
        .ok()
}
impl Diagnostics {
    pub(crate) fn claim_expired(&mut self, agent: usize, board: usize, deadline: Time) {
        if let Some(j) =
            self.jobs.iter_mut().rev().find(|j| {
                j.agent == agent && j.board == board && j.accepted + 12 * HOUR == deadline
            })
        {
            j.claim_expired = true;
        }
    }
    pub(crate) fn cargo_eaten(&mut self, id: usize, q: u64) {
        self.add("carrier_cargo_consumed", q);
        if let Some(&i) = self.active.get(&id) {
            self.jobs[i].cargo_consumed += q;
        }
    }
    pub(crate) fn cargo_spoiled(&mut self, q: u64) {
        self.add("communal_cargo_spoiled_transit", q);
    }

    fn add(&mut self, key: &str, q: u64) {
        *self.counts.entry(key.into()).or_default() += q;
    }
    pub fn get(&self, key: &str) -> u64 {
        self.counts.get(key).copied().unwrap_or(0)
    }
    pub(crate) fn observe(&mut self, o: &Observation, m: &PolicyMemory, d: &Decision) {
        if let Some(b) = o.local.as_ref().and_then(|l| l.board.as_ref())
            && b.quantity > 0
        {
            self.add("request_observations", 1);
            if self.observers.insert((o.agent, b.node)) {
                self.add("request_observers", 1);
            }
            if !m.nodes.values().any(|n| {
                n.node != b.node
                    && n.food
                        .iter()
                        .any(|f| f.quantity > 0 && f.expires_at > o.now)
            }) {
                self.add("board_evaluations_without_source", 1);
            }
        }
        if matches!(o.action,Action::Sleep{until} if until<=o.now) && d.intent == Intent::Wait {
            self.sleeping.remove(&o.agent);
        }
        let category = match &d.intent {
            Intent::Travel { edge, .. } => format!("Travel:{edge}"),
            Intent::Gather { communal, .. } => format!("Gather:{communal}"),
            x => format!("{x:?}")
                .split([' ', '{'])
                .next()
                .unwrap()
                .to_string(),
        };
        if self
            .last_decision
            .insert(o.agent, category.clone())
            .is_some_and(|old| old != category)
        {
            self.add("decision_reversals", 1);
        }
        if let Some(&i) = self.active.get(&o.agent) {
            let serving = matches!(
                d.intent,
                Intent::Gather { communal: true, .. }
                    | Intent::LoadCargo { .. }
                    | Intent::Deliver { .. }
                    | Intent::Travel { .. }
            ) || (d.intent == Intent::Wait
                && matches!(o.action, Action::Queued { .. } | Action::Gather { .. }));
            let j = &mut self.jobs[i];
            if serving && j.away {
                j.churn += 1;
                j.away = false;
                self.add("commitment_churn", 1);
            }
            let j = &mut self.jobs[i];
            if !serving && j.serving {
                j.away = true;
            }
            j.serving = serving;
            if matches!(d.intent, Intent::Travel { food: true, .. }) && !j.sought {
                j.sought = true;
                self.add("source_sought", 1);
            }
            if d.intent == Intent::CancelJob
                && d.reason.contains("survival-infeasible")
                && o.job
                    .as_ref()
                    .is_some_and(|j| policy::goal_is_fatal(o, m, j.destination))
            {
                self.add("predicted_fatal_return_decisions", 1);
            }
        }
        for (gate, detail) in policy::audit_job_gate(o, m) {
            self.add(&format!("gate_{gate}"), 1);
            let at_priority = o.nutrition >= 12 * HOUR
                && o.load < o.capacity
                && o.fatigue < 22 * HOUR
                && !matches!(o.action, Action::Gather { .. })
                && !(matches!(o.action, Action::Sleep { .. }) && o.fatigue > 4 * HOUR);
            if at_priority {
                self.add(&format!("at_job_priority_{gate}"), 1);
                if self.sampled.insert((o.agent, format!("priority:{gate}"))) {
                    self.samples.push(format!(
                        "t={} agent={} AT_JOB_PRIORITY {} {}",
                        o.now, o.agent, gate, detail
                    ));
                }
            }

            *self.reasons.entry(format!("job_gate:{gate}")).or_default() += 1;
            if self.sampled.insert((o.agent, format!("gate:{gate}"))) {
                self.samples.push(format!(
                    "t={} agent={} GATE {} {}",
                    o.now, o.agent, gate, detail
                ));
            }
        }
        // Small inspectable, deterministic decision sample: first occurrence per actor/reason.
        let reason = d.reason.split(';').next().unwrap_or(&d.reason).to_string();
        if self.sampled.insert((o.agent, reason.clone())) {
            self.samples.push(format!("t={} agent={} at={:?} nutrition={} fatigue={} known_nodes={:?} job={:?} intent={:?} reason={}",
                o.now,o.agent,o.location,o.nutrition,o.fatigue,m.nodes.keys().collect::<Vec<_>>(),o.job,d.intent,d.reason));
        }
    }
    pub(crate) fn event(&mut self, e: &Event) {
        match e.kind.as_str() {
            "JobAccepted" => {
                let id = e.agent.unwrap();
                let board = value(&e.detail, "board").unwrap() as usize;
                let expired: Vec<_> = self
                    .jobs
                    .iter()
                    .filter(|j| j.board == board && j.claim_expired)
                    .collect();
                let replacement = !expired.is_empty();
                let duplicate = expired.iter().any(|j| !j.ended);
                if replacement {
                    self.add("replacement_attempts", 1);
                }
                if duplicate {
                    self.add("duplicate_attempts", 1);
                }
                self.active.insert(id, self.jobs.len());
                self.jobs.push(JobRecord {
                    id: e.id,
                    agent: id,
                    board,
                    accepted: e.at,
                    ..JobRecord::default()
                });
                self.add("jobs_accepted", 1);
            }
            "JobCancelled" => {
                self.add("abandonments", 1);
                let reason = e
                    .detail
                    .split("reason=")
                    .nth(1)
                    .unwrap_or("")
                    .split(';')
                    .next()
                    .unwrap_or("")
                    .to_string();
                *self.reasons.entry(reason.clone()).or_default() += 1;
                if let Some(i) = self.active.remove(&e.agent.unwrap()) {
                    let j = &mut self.jobs[i];
                    j.ended = true;
                    j.reason = reason.clone();
                    let carrier = j.acquired > 0;
                    let fatal = j.returned && !j.reached_destination;
                    if reason == "starvation death" {
                        if carrier {
                            self.add("carrier_deaths", 1);
                        }
                        if fatal {
                            self.add("actual_fatal_returns", 1);
                        }
                    }
                }
            }
            "Queued" => {
                self.add("queue_joins", 1);
                if let Some(&i) = self.active.get(&e.agent.unwrap()) {
                    let edge = value(&e.detail, "edge").unwrap() as usize;
                    let j = &mut self.jobs[i];
                    let n = j.queue_joins.entry(edge).or_default();
                    *n += 1;
                    let repeated = *n > 1;
                    let returning = value(&e.detail, "target") == Some(j.board as u64)
                        && j.acquired > j.cargo_consumed
                        && !j.returned;
                    if returning {
                        j.returned = true;
                        self.add("returns_started", 1);
                    }
                    if repeated {
                        self.add("repeated_job_edge_joins", 1);
                    }
                }
            }
            "Admitted" => {
                if let Some(&i) = self.active.get(&e.agent.unwrap())
                    && self.jobs[i].returned
                    && !self.jobs[i].reached_destination
                {
                    self.add("return_edge_admissions", 1);
                }
            }
            "Arrived" => {
                if let Some(&i) = self.active.get(&e.agent.unwrap()) {
                    let n = value(&e.detail, "node").unwrap() as usize;
                    let j = &mut self.jobs[i];
                    if n == 2 && !j.reached_f {
                        j.reached_f = true;
                        self.add("jobs_reaching_f", 1);
                    }
                    let j = &mut self.jobs[i];
                    if n == j.board && j.returned && !j.reached_destination {
                        j.reached_destination = true;
                        self.add("returns_completed", 1);
                    }
                }
            }
            "Gathered" | "CargoLoaded" => {
                if e.detail.contains("Commons(0)")
                    && let Some(&i) = self.active.get(&e.agent.unwrap())
                {
                    if self.jobs[i].acquired == 0 {
                        self.add("jobs_acquiring_cargo", 1);
                    }
                    self.jobs[i].acquired += value(&e.detail, "q").unwrap();
                }
            }
            "Delivered" => {
                let q = value(&e.detail, "q").unwrap();
                if self.get("delivered_quanta") == 0 {
                    self.counts.insert("first_delivery_ms".into(), e.at as u64);
                }
                self.add("delivered_quanta", q);
                self.add("jobs_delivered", 1);
                if let Some(i) = self.active.remove(&e.agent.unwrap()) {
                    self.jobs[i].delivered += q;
                    self.jobs[i].ended = true;
                    self.jobs[i].reason = "delivered".into();
                }
            }
            "Interrupted" => {
                if e.detail.contains("was=Queued") {
                    self.add("queue_voluntary_exits", 1);
                }
                if e.detail.contains("was=Sleep") && !e.detail.starts_with("sleep;") {
                    self.add("sleep_interruptions", 1);
                    self.sleeping.remove(&e.agent.unwrap());
                }
            }
            "SleepStarted" => {
                if self.sleeping.insert(e.agent.unwrap()) {
                    self.add("sleep_starts", 1);
                } else {
                    self.add("sleep_renewals", 1);
                }
            }
            "Decision" => {
                if !e.detail.contains("intent=Sleep") && !e.detail.contains("intent=Wait") {
                    self.sleeping.remove(&e.agent.unwrap());
                }
            }
            "LeaseExpired" => self.add("claim_expiries", value(&e.detail, "count").unwrap()),
            _ => {}
        }
    }
}

pub fn delivery_fixture(
    seed: u64,
    variant: DiagnosticPolicy,
    trace: bool,
) -> Result<World, String> {
    World::new(Config {
        seed,
        initial_request: 600,
        diagnostic_policy: variant,
        trace,
        ..Config::default()
    })
}

/// Global research controller parameters. Not available to autonomous decisions.
#[derive(Clone, Debug)]
pub struct Schedule {
    pub carriers: usize,
    pub residents: usize,
    pub meal_target_h: i64,
    pub rest_at_h: i64,
    pub rest_to_h: i64,
    pub cargo: u64,
    /// C residents eat normally, or only near starvation (survival != nourishment).
    pub rescue_only: bool,
}
impl Default for Schedule {
    fn default() -> Self {
        Self {
            carriers: 4,
            residents: 0,
            meal_target_h: 22,
            rest_at_h: 16,
            rest_to_h: 4,
            cargo: 10,
            rescue_only: false,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Witness {
    pub schedule: Schedule,
    pub delivered: u64,
    pub first_delivery: Option<Time>,
    pub residents_alive_72: usize,
    pub residents_without_starvation_48: usize,
    pub resident_starvation_ms: u128,
    pub event_hash: u64,
}
/// Executes each move, reservation, meal and transfer through the existing kernel.
/// Iterates exact engine event boundaries; no time, goods or physiology mutation.
pub fn schedule_witness(p: Schedule, trace: bool) -> Result<(Witness, World), String> {
    schedule_witness_in_world(
        p,
        Config {
            autonomous: false,
            initial_request: 600,
            trace,
            ..Config::default()
        },
    )
}
/// Same research controller on a registered line-world travel scale; never NPC policy.
pub fn schedule_witness_in_world(p: Schedule, config: Config) -> Result<(Witness, World), String> {
    if config.nodes != 3
        || config.starts != vec![0; 50]
        || config.boards != vec![0]
        || config.edges.len() != 2
        || (config.edges[0].a, config.edges[0].b) != (0, 1)
        || (config.edges[1].a, config.edges[1].b) != (1, 2)
        || config.food != vec![(2, 3000, 0, 48 * HOUR)]
        || config.carrying_capacity != 10
        || config.edges.iter().any(|e| e.capacity != 4)
    {
        return Err("research scheduler requires registered locked-stock line geometry".into());
    }
    let latest_launch = 48 * HOUR - 2 * config.edges.iter().map(|e| e.nominal_ms).sum::<Time>();
    if p.carriers + p.residents > 50 || p.cargo > 10 || p.cargo == 0 {
        return Err("invalid research allocation".into());
    }
    let mut w = World::new(Config {
        autonomous: false,
        ..config
    })?;
    let mut returning = [false; 50];
    let mut resting = [false; 50];
    let mut ever_starved = [false; 50];
    let mut last_rescue = [false; 50];
    let mut deficit = 0u128;
    let mut previous = 0;
    while w.now < 72 * HOUR {
        for (i, a) in w
            .agents
            .iter()
            .enumerate()
            .skip(p.carriers)
            .take(p.residents)
        {
            if w.now <= 48 * HOUR && a.starvation > 0 {
                ever_starved[i] = true;
            }
            if a.death_time.is_none_or(|t| t >= previous) {
                deficit += (w.now.min(a.death_time.unwrap_or(w.now)) - previous)
                    .min(a.starvation)
                    .max(0) as u128;
            }
        }
        previous = w.now;
        // Drain finite instantaneous actions; gather/sleep/travel still consume time.
        for _ in 0..8 {
            let mut intents = vec![];
            let mut available_c: u64 = w
                .goods
                .batches
                .iter()
                .filter(|b| b.holder == Holder::Commons(0) && b.material == Material::Food)
                .map(|b| b.quantity)
                .sum();
            for id in 0..50 {
                let a = &w.agents[id];
                if matches!(
                    a.action,
                    Action::Dead
                        | Action::Travel
                        | Action::Queued { .. }
                        | Action::Gather { .. }
                        | Action::Sleep { .. }
                ) {
                    continue;
                }
                let Location::Node(node) = a.location else {
                    unreachable!()
                };
                let own = w.goods.batches.iter().find(|b| {
                    b.holder == Holder::Agent(id) && b.material == Material::Food && b.quantity > 0
                });
                if id >= p.carriers {
                    if id < p.carriers + p.residents
                        && available_c > 0
                        && ((!p.rescue_only && a.nutrition < 11 * HOUR)
                            || (p.rescue_only
                                && (a.starvation >= 22 * HOUR
                                    || (w.now >= 47 * HOUR + MINUTE && !last_rescue[id]))))
                        && let Some(b) = w.goods.batches.iter().find(|b| {
                            b.holder == Holder::Commons(0)
                                && b.material == Material::Food
                                && b.quantity > 0
                        })
                    {
                        let q = goods::headroom(a.nutrition, Owner::Commons(0))
                            .min(b.quantity)
                            .min(available_c)
                            .min(if p.rescue_only { 1 } else { u64::MAX });
                        if q > 0 {
                            intents.push((
                                id,
                                Intent::Eat {
                                    batch: b.id,
                                    quantity: q,
                                },
                            ));
                            available_c -= q;
                            if p.rescue_only && w.now >= 47 * HOUR + MINUTE {
                                last_rescue[id] = true;
                            }
                            continue;
                        }
                    }
                    intents.push((id, Intent::Sleep));
                    continue;
                }
                if w.now >= 48 * HOUR {
                    intents.push((id, Intent::Sleep));
                    continue;
                }
                if node == 0
                    && returning[id]
                    && let Some(b) = own.filter(|b| b.owner == Owner::Commons(0))
                {
                    intents.push((
                        id,
                        Intent::Deliver {
                            batch: b.id,
                            quantity: b.quantity,
                        },
                    ));
                    returning[id] = false;
                    continue;
                }
                // Even at perfect speed a new C-F-C cycle now misses expiry.
                // This is global research scheduling, never an autonomous rule.
                if node == 0 && w.now >= latest_launch {
                    intents.push((id, Intent::Sleep));
                    continue;
                }
                if node == 0 && a.job.is_none() {
                    if w.boards[0].unclaimed(w.now) >= p.cargo {
                        intents.push((
                            id,
                            Intent::Claim {
                                board: 0,
                                quantity: p.cargo,
                            },
                        ));
                        continue;
                    }
                    // Do not create extra requests or change a physical constraint.
                    continue;
                }
                if node == 2 && !returning[id] {
                    // Personal meals use exact same harvest work and immutable-age goods.
                    if let Some(b) = own.filter(|b| b.owner == Owner::Agent(id)) {
                        let q = b.quantity.min(goods::headroom(a.nutrition, b.owner));
                        if q > 0 {
                            intents.push((
                                id,
                                Intent::Eat {
                                    batch: b.id,
                                    quantity: q,
                                },
                            ));
                            continue;
                        }
                    }
                    if a.nutrition < (p.meal_target_h * HOUR) && w.goods.load(id) < 10 {
                        intents.push((
                            id,
                            Intent::Gather {
                                batch: 0,
                                communal: false,
                            },
                        ));
                        continue;
                    }
                    if a.fatigue >= p.rest_at_h * HOUR {
                        resting[id] = true;
                    }
                    if resting[id] && a.fatigue > p.rest_to_h * HOUR {
                        intents.push((id, Intent::Sleep));
                        continue;
                    }
                    resting[id] = false;
                    if a.job.as_ref().is_some_and(|j| j.acquired < p.cargo) && w.goods.load(id) < 10
                    {
                        intents.push((
                            id,
                            Intent::Gather {
                                batch: 0,
                                communal: true,
                            },
                        ));
                        continue;
                    }
                    returning[id] = true;
                }
                let target = if returning[id] { 0 } else { 2 };
                let edge = if node == 0 || (node == 1 && returning[id]) {
                    0
                } else {
                    1
                };
                intents.push((
                    id,
                    Intent::Travel {
                        edge,
                        target,
                        food: false,
                    },
                ));
            }
            if intents.is_empty() {
                break;
            }
            let before = w.event_count;
            w.apply_intents(intents)?;
            if before == w.event_count {
                break;
            }
        }
        let next = w.next_time(72 * HOUR);
        w.advance_to(next)?;
    }
    for a in w.agents.iter().skip(p.carriers).take(p.residents) {
        if a.death_time.is_none_or(|t| t >= previous) {
            deficit += (w.now.min(a.death_time.unwrap_or(w.now)) - previous)
                .min(a.starvation)
                .max(0) as u128;
        }
    }
    let residents_alive_72 = w
        .agents
        .iter()
        .skip(p.carriers)
        .take(p.residents)
        .filter(|a| a.action != Action::Dead)
        .count();
    let residents_without_starvation_48 = (p.carriers..p.carriers + p.residents)
        .filter(|&i| !ever_starved[i])
        .count();
    let first_delivery =
        (w.metrics.delivered > 0).then_some(w.diagnostics.get("first_delivery_ms") as Time);
    let result = Witness {
        schedule: p,
        delivered: w.metrics.delivered,
        first_delivery,
        residents_alive_72,
        residents_without_starvation_48,
        resident_starvation_ms: deficit,
        event_hash: w.event_hash,
    };
    w.audit()?;
    Ok((result, w))
}
impl World {
    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }
    pub fn config_for_research(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn posted_request_is_local_and_survives_initial_refresh_without_food_knowledge() {
        let mut w = delivery_fixture(0, DiagnosticPolicy::Current, false).unwrap();
        w.advance_to(0).unwrap();
        assert_eq!(w.boards[0].quantity, 600);
        assert_eq!(w.diagnostics.get("request_observers"), 50);
        for a in &w.agents {
            assert_eq!(a.memory.nodes.keys().copied().collect::<Vec<_>>(), vec![0]);
            assert!(a.job.is_none());
        }
        assert_eq!(w.goods.batches[0].quantity, 3000);
    }
    #[test]
    fn reference_state_and_events_are_preserved_exactly() {
        let r = experiment::run("reference", 0, false).unwrap();
        assert_eq!(r.world.event_hash(), 0x89b7b642ce88e347);
        assert_eq!(r.world.state_hash(), 0xe801c9f7f2b1cd42);
    }
    fn queued_worker() -> (World, Observation, PolicyMemory) {
        let mut w = World::new(Config {
            autonomous: false,
            ..Config::default()
        })
        .unwrap();
        w.boards[0].quantity = 10;
        w.apply_intents(vec![(
            4,
            Intent::Claim {
                board: 0,
                quantity: 10,
            },
        )])
        .unwrap();
        w.apply_intents(
            (0..5)
                .map(|id| {
                    (
                        id,
                        Intent::Travel {
                            edge: 0,
                            target: 2,
                            food: true,
                        },
                    )
                })
                .collect(),
        )
        .unwrap();
        w.agents[4].fatigue = 22 * HOUR;
        let o = w.observe(4).unwrap();
        let m = PolicyMemory {
            nodes: (0..3).map(|n| (n, w.node_view(n))).collect(),
        };
        (w, o, m)
    }
    #[test]
    fn queue_commitment_keeps_fifo_place_but_does_not_change_admission() {
        let (mut w, o, m) = queued_worker();
        assert_eq!(policy::decide(&o, &m, &mut 0).intent, Intent::Sleep);
        let d = policy::decide_variant(&o, &m, &mut 0, DiagnosticPolicy::QueueCommitment);
        assert_eq!(d.intent, Intent::Wait);
        w.apply_intents(vec![(4, d.intent)]).unwrap();
        assert_eq!(w.agents[4].action, o.action);
        assert_eq!(w.metrics.overtakes, 0);
        w.audit().unwrap();
    }
    #[test]
    fn queue_commitment_yields_to_critical_local_food() {
        let (mut w, _, m) = queued_worker();
        w.goods.add(
            Material::Food,
            3,
            Owner::Commons(0),
            Holder::Commons(0),
            0,
            48 * HOUR,
        );
        w.agents[4].nutrition = HOUR;
        let o = w.observe(4).unwrap();
        assert!(matches!(
            policy::decide_variant(&o, &m, &mut 0, DiagnosticPolicy::QueueCommitment).intent,
            Intent::Eat { .. }
        ));
    }
    #[test]
    fn coherent_sleep_defers_noncritical_meal_but_interrupts_for_critical_meal() {
        let mut w = World::new(Config {
            autonomous: false,
            ..Config::default()
        })
        .unwrap();
        w.goods.add(
            Material::Food,
            4,
            Owner::Commons(0),
            Holder::Commons(0),
            0,
            48 * HOUR,
        );
        w.agents[0].nutrition = 10 * HOUR;
        w.agents[0].fatigue = 18 * HOUR;
        w.apply_intents(vec![(0, Intent::Sleep)]).unwrap();
        let o = w.observe(0).unwrap();
        let m = PolicyMemory::default();
        assert!(matches!(
            policy::decide(&o, &m, &mut 0).intent,
            Intent::Eat { .. }
        ));
        assert_eq!(
            policy::decide_variant(&o, &m, &mut 0, DiagnosticPolicy::SleepCommitment).intent,
            Intent::Sleep
        );
        w.agents[0].nutrition = 5 * HOUR;
        assert!(matches!(
            policy::decide_variant(
                &w.observe(0).unwrap(),
                &m,
                &mut 0,
                DiagnosticPolicy::SleepCommitment
            )
            .intent,
            Intent::Eat { .. }
        ));
    }
    #[test]
    fn lease_replacement_is_measured_without_remotely_cancelling_worker() {
        let mut w = World::new(Config {
            autonomous: false,
            initial_request: 10,
            ..Config::default()
        })
        .unwrap();
        w.apply_intents(vec![(
            0,
            Intent::Claim {
                board: 0,
                quantity: 10,
            },
        )])
        .unwrap();
        w.advance_to(12 * HOUR).unwrap();
        assert!(w.agents[0].job.is_some());
        w.apply_intents(vec![(
            1,
            Intent::Claim {
                board: 0,
                quantity: 10,
            },
        )])
        .unwrap();
        assert_eq!(w.diagnostics.get("claim_expiries"), 1);
        assert_eq!(w.diagnostics.get("replacement_attempts"), 1);
        assert_eq!(w.diagnostics.get("duplicate_attempts"), 1);
        assert_eq!(w.goods.batches[0].quantity, 3000);
        w.audit().unwrap();
    }
    #[test]
    fn counterfactual_delivers_through_real_physics_and_preserves_locked_parameters() {
        let (r, w) = schedule_witness(Schedule::default(), false).unwrap();
        assert!(r.delivered >= 100);
        assert!(r.first_delivery.unwrap() > 8 * HOUR);
        assert_eq!(w.config.edges, Config::default().edges);
        assert_eq!(w.config.food, Config::default().food);
        assert_eq!(w.config.starts, Config::default().starts);
        assert_eq!(w.config.carrying_capacity, 10);
        assert!(w.metrics.gathered >= r.delivered);
        assert_eq!(w.metrics.overtakes, 0);
        assert_eq!(
            w.goods.consumed
                + w.goods
                    .batches
                    .iter()
                    .map(|b| b.quantity as u128)
                    .sum::<u128>(),
            3000
        );
        w.audit().unwrap();
    }
}

/// Proven relaxations/certificates; see docs/DELIVERY_BOUNDS.md for assumptions.
/// Quantity is an upper bound, NOT a claim that the bound is attainable.
pub fn envelope_bounds() -> (Time, u64, u64, usize) {
    let first = 8 * HOUR + (6000 * SCALE).div_ceil(performance(6 * HOUR, 12 * HOUR)) as Time;
    let window = 48 * HOUR - 1 - 2 * HOUR - 2 * HOUR;
    let returns = (4 * window / (4 * HOUR)) as u64;
    (first, returns * 10, returns * 10 / 36, 47)
}

#[cfg(test)]
mod bounds_tests {
    use super::*;
    #[test]
    fn earliest_first_quantum_attains_the_relaxed_lower_bound() {
        let (r, w) = schedule_witness(
            Schedule {
                carriers: 1,
                cargo: 1,
                meal_target_h: 9,
                rest_at_h: 24,
                ..Schedule::default()
            },
            true,
        )
        .unwrap();
        assert_eq!(r.first_delivery, Some(envelope_bounds().0));
        assert!(
            w.events
                .iter()
                .filter(|e| e.kind == "Delivered")
                .all(|e| e.at < 48 * HOUR)
        );
        assert!(!w.events.iter().any(|e| e.kind == "Rejected"));
    }
    #[test]
    fn starvation_grace_support_bound_is_not_nutritional_success() {
        let (r, w) = schedule_witness(
            Schedule {
                carriers: 3,
                residents: 47,
                meal_target_h: 23,
                rest_at_h: 12,
                rescue_only: true,
                ..Schedule::default()
            },
            false,
        )
        .unwrap();
        assert_eq!(r.residents_alive_72, 47);
        assert_eq!(
            w.agents
                .iter()
                .filter(|a| a.action != Action::Dead && a.location == Location::Node(0))
                .count(),
            50
        );
        assert_eq!(r.residents_without_starvation_48, 0);
        assert!(
            w.agents[3..]
                .iter()
                .all(|a| a.consumed == 2 && a.nutrition == 0 && a.starvation > 23 * HOUR)
        );
        assert!(r.resident_starvation_ms > 2700 * HOUR as u128);
    }
}
