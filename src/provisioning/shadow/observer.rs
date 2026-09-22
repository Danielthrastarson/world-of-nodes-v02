use super::{physics::Body, plan};
use crate::provisioning::*;
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
struct Pending {
    at: Time,
    from: usize,
    to: usize,
    edge: usize,
    predicted_wait: Time,
    predicted: Body,
    frozen_wait: Option<Time>,
    frozen_at: Option<Time>,
    admitted: Option<Time>,
    nominal: Time,
    initial_n: Time,
    initial_f: Time,
    initial_s: Time,
    policy_time: Time,
    policy_nutrition: Time,
    policy_fatigue: Time,
}
#[derive(Clone, Debug)]
struct Primitive {
    kind: &'static str,
    at: Time,
    prediction: Body,
    load: u64,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct Journal {
    pub enabled: bool,
    pub detailed: bool,
    pub all_agents_oracle: bool,
    pub rows: BTreeMap<&'static str, Vec<String>>,
    pub counts: BTreeMap<String, u64>,
    pending: BTreeMap<usize, Pending>,
    oracle_keys: std::collections::BTreeSet<String>,
    staged: BTreeMap<usize, Pending>,
    primitives: BTreeMap<usize, Primitive>,
}
pub const HEADERS: &[(&str, &str)] = &[
    (
        "self_count_witness",
        "time_ms,agent,board,old_cargo,corrected_cargo,kernel_delivered,kernel_failure,elapsed_exact",
    ),
    (
        "route_feasibility",
        "time_ms,agent,target,forecast_alive,shadow_alive,classification",
    ),
    (
        "primitives",
        "time_ms,agent,action,status,time_error_ms,nutrition_error_ms,fatigue_error_ms,cargo_error",
    ),
    (
        "captures",
        "time_ms\tagent\tobservation\tmemory\tselected\tcandidates",
    ),
    (
        "plans",
        "time_ms,agent,candidate,source,board,complete_tape,forecast_gate,shadow_failure,elapsed_error_ms,nutrition_error_ms,fatigue_error_ms,cargo_error,classification",
    ),
    (
        "memory",
        "time_ms,agent,subject,node,age_ms,remembered,current,error",
    ),
    (
        "routes",
        "time_ms,agent,target,edge,from,to,observed_at_ms,remembered_queue,predicted_queue_ms,true_queue,true_opposite_queue,true_occupancy,ordering_error_ms,nutrition_error_ms,fatigue_error_ms",
    ),
    (
        "episodes",
        "time_ms,agent,end_ms,from,to,edge,status,predicted_queue_ms,frozen_queue_ms,realized_queue_ms,predicted_total_ms,frozen_total_ms,actual_total_ms,time_error_ms,nutrition_error_ms,fatigue_error_ms,queue_error_ms,category,policy_time_error_ms,policy_nutrition_error_ms,policy_fatigue_error_ms",
    ),
    (
        "self_count",
        "time_ms,agent,old_gate,corrected_gate,old_plan_ms,corrected_plan_ms,old_cargo,corrected_cargo,action_changed,queue_delay_difference_ms,hypothetical_legs",
    ),
    ("needs", "time_ms,agent,nutrition_ms,category,detail"),
    (
        "kernel_plans",
        "time_ms,agent,candidate,belief_failure,truth_failure,belief_ms,truth_ms,belief_cargo,truth_cargo,shadow_kernel_exact,interpretation",
    ),
];
impl Journal {
    fn count(&mut self, key: impl Into<String>) {
        *self.counts.entry(key.into()).or_default() += 1;
    }
    fn row(&mut self, kind: &'static str, row: String) {
        if self.detailed {
            self.rows.entry(kind).or_default().push(row);
        }
    }
    pub fn event(&mut self, e: &Event, agents: &[Agent], goods: &super::super::goods::Goods) {
        if !self.enabled {
            return;
        }
        let Some(id) = e.agent else { return };
        if self.primitives.get(&id).is_some_and(|p| {
            matches!(
                (p.kind, e.kind.as_str()),
                ("gather", "Gathered") | ("eat", "Consumed")
            )
        }) {
            self.finish_primitive(id, e.at, &agents[id], goods.load(id), "COMPLETED");
        } else if ["Interrupted", "Died", "Rejected"].contains(&e.kind.as_str())
            && self.primitives.contains_key(&id)
        {
            // Eat and gather can themselves cancel the prior queue. Do not confuse
            // that interruption with interruption of the newly submitted action.
            let p = &self.primitives[&id];
            if p.at < e.at || e.kind == "Rejected" {
                self.finish_primitive(id, e.at, &agents[id], goods.load(id), "CENSORED");
            }
        }
        if e.kind == "Queued" {
            if let Some(p) = self.staged.remove(&id) {
                self.pending.insert(id, p);
            }
            return;
        }
        let Some(p) = self.pending.get_mut(&id) else {
            return;
        };
        if e.kind == "Admitted" {
            p.admitted = Some(e.at);
            return;
        }
        if !["Arrived", "Interrupted", "Died", "Rejected"].contains(&e.kind.as_str()) {
            return;
        }
        let p = self.pending.remove(&id).unwrap();
        let a = &agents[id];
        let dt = e.at - p.at;
        let wait = p.admitted.unwrap_or(e.at) - p.at;
        let completed = e.kind == "Arrived";
        let category = if completed {
            // Compare the deterministic movement part conditional on the REAL wait.
            let mut b = Body {
                elapsed: 0,
                nutrition: p.initial_n,
                fatigue: p.initial_f,
                starvation: p.initial_s,
            };
            b.elapse(wait, false);
            b.work(p.nominal);
            if b.elapsed != dt || b.nutrition != a.nutrition || b.fatigue != a.fatigue {
                "PHYSICAL_MODEL_INCONSISTENCY"
            } else if p.frozen_wait.is_some_and(|v| v != wait) {
                "WORLD_CHANGED_AFTER_PREDICTION"
            } else if wait != p.predicted_wait {
                "UNAVAILABLE_INFORMATION"
            } else {
                "MATCH"
            }
        } else if wait > 0
            && (a.nutrition <= 6 * HOUR || a.fatigue >= 22 * HOUR || e.kind == "Died")
            && p.frozen_wait.is_some_and(|v| v >= wait)
        {
            "CAPACITY_COMPETITION"
        } else if e.kind == "Rejected" {
            "INSTITUTIONAL_RESTRICTION"
        } else {
            "RISK_COMMITMENT_PREFERENCE"
        };
        self.count(format!("episode:{category}"));
        self.count(format!("episode_status:{}", e.kind));
        let fmt = |v: Option<Time>| v.map(|n| n.to_string()).unwrap_or_default();
        self.row(
            "episodes",
            format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                p.at,
                id,
                e.at,
                p.from,
                p.to,
                p.edge,
                e.kind,
                p.predicted_wait,
                fmt(p.frozen_wait),
                wait,
                p.predicted.elapsed,
                fmt(p.frozen_at),
                if completed {
                    dt.to_string()
                } else {
                    String::new()
                },
                if completed {
                    (p.predicted.elapsed - dt).to_string()
                } else {
                    String::new()
                },
                if completed {
                    (p.predicted.nutrition - a.nutrition).to_string()
                } else {
                    String::new()
                },
                if completed {
                    (p.predicted.fatigue - a.fatigue).to_string()
                } else {
                    String::new()
                },
                if p.admitted.is_some() {
                    (p.predicted_wait - wait).to_string()
                } else {
                    String::new()
                },
                category,
                if completed {
                    (p.policy_time - dt).to_string()
                } else {
                    String::new()
                },
                if completed {
                    (p.policy_nutrition - a.nutrition).to_string()
                } else {
                    String::new()
                },
                if completed {
                    (p.policy_fatigue - a.fatigue).to_string()
                } else {
                    String::new()
                }
            ),
        );
    }
    pub fn observe(&mut self, w: &World, o: &Observation, d: &Decision, rng: u64) {
        self.count("decisions");
        if self
            .primitives
            .get(&o.agent)
            .is_some_and(|p| p.kind == "sleep" && o.now >= p.at + p.prediction.elapsed)
        {
            self.finish_primitive(o.agent, o.now, &w.agents[o.agent], o.load, "COMPLETED");
        }
        let mut b = Body {
            elapsed: 0,
            nutrition: o.nutrition,
            fatigue: o.fatigue,
            starvation: o.starvation,
        };
        let primitive = match d.intent {
            Intent::Gather { .. } if !matches!(o.action, Action::Gather { .. }) => {
                b.work(2000);
                Some(("gather", o.load + 1))
            }
            Intent::Eat { batch, quantity } => {
                let food = o
                    .own
                    .iter()
                    .chain(o.local.iter().flat_map(|v| &v.food))
                    .find(|f| f.batch == batch);
                food.and_then(|f| {
                    b.eat(quantity, f.owner == Owner::Commons(0)).then_some((
                        "eat",
                        o.load
                            - if f.holder == Holder::Agent(o.agent) {
                                quantity
                            } else {
                                0
                            },
                    ))
                })
            }
            Intent::Sleep if !matches!(o.action,Action::Sleep{until}if until>o.now) => {
                b.sleep_tick();
                Some(("sleep", o.load))
            }
            _ => None,
        };
        if let Some((kind, load)) = primitive {
            if self.primitives.contains_key(&o.agent) {
                self.finish_primitive(o.agent, o.now, &w.agents[o.agent], o.load, "CENSORED");
            }
            self.primitives.insert(
                o.agent,
                Primitive {
                    kind,
                    at: o.now,
                    prediction: b,
                    load,
                },
            );
        }

        let m = &w.agents[o.agent].memory;
        let meals = w.config.persistence.is_some()
            || w.config.diagnostic_policy == DiagnosticPolicy::MealAware;
        let corrected = w.config.persistence == Some(persistence::Arm::ForecastCorrected);
        let (counterdecision, audit) = if meals {
            policy::audit_meals(o, m, &mut rng.clone(), corrected)
        } else {
            policy::decide_audited(o, m, &mut rng.clone(), w.config.diagnostic_policy)
        };
        // This is a copy of policy inputs and RNG, never the live mutable stream.
        if w.config.persistence.is_none() {
            assert_eq!(&counterdecision, d);
        }
        if self.detailed {
            self.row(
                "captures",
                format!(
                    "{}\t{}\t{:?}\t{:?}\t{:?}\t{:?}",
                    o.now, o.agent, o, m, d, audit.candidates
                ),
            );
        }
        for (index, c) in audit.candidates.iter().enumerate() {
            self.count("candidates");
            let Some(tape) = plan::tape(c, m) else {
                self.count("candidate:no_complete_sequence");
                continue;
            };
            let p = c.plan.as_ref().unwrap();
            let r = plan::predict(o, m, c, &tape);
            let complete = tape.iter().any(|s| matches!(s, plan::Step::Deliver));
            let classification = if !complete {
                "PARTIAL_UNRESOLVED"
            } else if r.failure.is_some() {
                if c.violations.is_empty() {
                    "FALSE_FEASIBLE"
                } else {
                    "POLICY_REJECTED_AND_SHADOW_INFEASIBLE"
                }
            } else if p.starvation >= 24 * HOUR || p.source_starvation >= 24 * HOUR {
                "FALSE_INFEASIBLE"
            } else if c.violations.contains(&"POST_PLAN_NUTRITION_MARGIN") {
                "RISK_COMMITMENT_PREFERENCE"
            } else if !c.violations.is_empty() {
                "REJECTED_OTHER_GATE"
            } else {
                "POLICY_ACCEPTABLE_AND_SHADOW_FEASIBLE"
            };
            self.count(format!("plan:{classification}"));
            if complete
                && r.failure.is_none()
                && (p.elapsed != r.body.elapsed
                    || p.nutrition != r.body.nutrition
                    || p.delivered != r.delivered)
            {
                self.count("plan:arithmetic_difference");
            }
            self.row(
                "plans",
                format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    o.now,
                    o.agent,
                    index,
                    c.source.unwrap_or(usize::MAX),
                    c.board,
                    complete,
                    c.violations.first().copied().unwrap_or("PASS"),
                    r.failure.unwrap_or("NONE"),
                    p.elapsed - r.body.elapsed,
                    p.nutrition - r.body.nutrition,
                    (o.fatigue + p.elapsed).min(24 * HOUR) - r.body.fatigue,
                    p.delivered as i64 - r.delivered as i64,
                    classification
                ),
            );
            // Deterministic bounded kernel validation; full shadow tapes are never sampled.
            if self.detailed
                && complete
                && self.oracle_keys.len() < 160
                && (o.agent == 0 || self.all_agents_oracle)
                && self.oracle_keys.insert(if self.all_agents_oracle {
                    format!(
                        "{}:{:?}:{}:board={}:physical_mismatch={}",
                        audit.outcome,
                        o.location,
                        o.now / HOUR,
                        c.board,
                        r.failure.is_some()
                            && p.starvation < 24 * HOUR
                            && p.source_starvation < 24 * HOUR
                            && p.delivered > 0
                            && c.expiry.is_some_and(|t| t > o.now + p.elapsed)
                    )
                } else {
                    format!("{}:{:?}:{}", audit.outcome, o.location, o.now / HOUR)
                })
            {
                let belief = super::oracle::belief(o, m, c, &tape);
                let truth = super::oracle::truth(w, o.agent, c, &tape);
                self.row(
                    "kernel_plans",
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{}",
                        o.now,
                        o.agent,
                        index,
                        belief.failure,
                        truth.failure,
                        belief.elapsed,
                        truth.elapsed,
                        belief.delivered,
                        truth.delivered,
                        r.body.elapsed == belief.elapsed
                            && r.body.nutrition == belief.nutrition
                            && r.body.fatigue == belief.fatigue
                            && r.delivered == belief.delivered,
                        if belief.failure == "NONE" && truth.failure != "NONE" {
                            "HIDDEN_STATE_OR_CAPACITY"
                        } else {
                            "CONDITIONAL_COMPARISON"
                        }
                    ),
                );
            }
        }
        if matches!(o.action, Action::Queued { .. }) {
            let ((mut fixed, ca), (old_decision, old)) = if meals {
                (
                    policy::audit_meals(o, m, &mut rng.clone(), true),
                    policy::audit_meals(o, m, &mut rng.clone(), false),
                )
            } else {
                let mut evidence = m.clone();
                let mut initial = o.clone();
                if let (Location::Node(n), Action::Queued { edge, .. }) = (&o.location, &o.action)
                    && let Some(v) = evidence.nodes.get_mut(n)
                    && v.observed_at == o.now
                    && let Some(e) = v.edges.iter_mut().find(|e| e.id == *edge)
                {
                    e.queued = e.queued.saturating_sub(1);
                    initial.action = Action::Idle;
                }
                (
                    policy::decide_audited(
                        &initial,
                        &evidence,
                        &mut rng.clone(),
                        w.config.diagnostic_policy,
                    ),
                    policy::decide_audited(o, m, &mut rng.clone(), w.config.diagnostic_policy),
                )
            };
            // Normalizing evidence must not invent a leave/rejoin action.
            if let (Action::Queued { edge, .. }, Intent::Travel { edge: next, .. }) =
                (&o.action, &fixed.intent)
                && edge == next
            {
                fixed.intent = Intent::Wait;
            }

            for (p, q) in old.candidates.iter().zip(&ca.candidates) {
                let corrected_candidate = q.clone();
                if let (Some(p), Some(q)) = (&p.plan, &q.plan)
                    && (p.elapsed != q.elapsed
                        || p.delivered != q.delivered
                        || p.nutrition != q.nutrition)
                {
                    if self.detailed
                        && self.all_agents_oracle
                        && p.delivered == 0
                        && q.delivered > 0
                        && ca.outcome == "PASS"
                        && let Some(tape) = plan::tape(&corrected_candidate, m)
                    {
                        let kernel = super::oracle::belief(o, m, &corrected_candidate, &tape);
                        self.row(
                            "self_count_witness",
                            format!(
                                "{},{},{},{},{},{},{},{}",
                                o.now,
                                o.agent,
                                corrected_candidate.board,
                                p.delivered,
                                q.delivered,
                                kernel.delivered,
                                kernel.failure,
                                kernel.elapsed == q.elapsed
                            ),
                        );
                    }
                    self.count("self_count:affected_candidate");
                    if old.outcome != ca.outcome {
                        self.count("self_count:gate_changed");
                    }
                    if fixed.intent != old_decision.intent {
                        self.count("self_count:reactive_action_changed");
                    }
                    let old_legs: Vec<_> =
                        p.steps.iter().filter(|s| s.contains("queue_ms=")).collect();
                    let new_legs: Vec<_> =
                        q.steps.iter().filter(|s| s.contains("queue_ms=")).collect();
                    let queue_difference: Time = old_legs
                        .iter()
                        .map(|s| plan::value(s, "queue_ms").unwrap_or(0))
                        .sum::<Time>()
                        - new_legs
                            .iter()
                            .map(|s| plan::value(s, "queue_ms").unwrap_or(0))
                            .sum::<Time>();
                    let evidence = old_legs
                        .iter()
                        .zip(&new_legs)
                        .filter_map(|(p, q)| {
                            let old = plan::value(p, "queue_ms").unwrap_or(0);
                            let new = plan::value(q, "queue_ms").unwrap_or(0);
                            (old != new).then(|| {
                                format!(
                                    "{} edge={} {}->{}",
                                    p.split_whitespace().next().unwrap(),
                                    plan::value(p, "edge").unwrap(),
                                    old,
                                    new
                                )
                            })
                        })
                        .collect::<Vec<_>>()
                        .join("|");
                    self.row(
                        "self_count",
                        format!(
                            "{},{},{},{},{},{},{},{},{},{},{}",
                            o.now,
                            o.agent,
                            old.outcome,
                            ca.outcome,
                            p.elapsed,
                            q.elapsed,
                            p.delivered,
                            q.delivered,
                            fixed.intent != old_decision.intent,
                            queue_difference,
                            evidence
                        ),
                    );
                }
            }
        }
        self.memories(w, o, m);
        self.need(w, o, m, d);
        if let Intent::Travel { edge, target, .. } = d.intent {
            if self.pending.get(&o.agent).is_some_and(|p| p.edge == edge) {
                return;
            }
            let Location::Node(from) = o.location else {
                return;
            };
            let Some(route) = audited_route(w, o, m, target) else {
                return;
            };
            let Some(step) = route.steps.first() else {
                return;
            };
            // Some persistent variants select routes after evidence normalization. Store
            // the existing public route estimate explicitly; do not label it their goal forecast.
            let mut p = Body {
                elapsed: 0,
                nutrition: o.nutrition,
                fatigue: o.fatigue,
                starvation: o.starvation,
            };
            p.elapse(step.queue_ms, false);
            p.work(step.nominal_ms);
            let (frozen_wait, frozen_at) = if self.detailed {
                super::oracle::edge(w, o.agent, edge)
            } else {
                (None, None)
            };
            self.staged.insert(
                o.agent,
                Pending {
                    at: o.now,
                    from,
                    to: step.to,
                    edge,
                    predicted_wait: step.queue_ms,
                    predicted: p,
                    frozen_wait,
                    frozen_at,
                    admitted: None,
                    nominal: step.nominal_ms,
                    initial_n: o.nutrition,
                    initial_f: o.fatigue,
                    initial_s: o.starvation,
                    policy_time: step.travel_ms + step.queue_ms,
                    policy_nutrition: (o.nutrition - step.travel_ms - step.queue_ms).max(0),
                    policy_fatigue: (o.fatigue + step.travel_ms + step.queue_ms).min(24 * HOUR),
                },
            );
        }
    }
    fn finish_primitive(&mut self, id: usize, now: Time, a: &Agent, load: u64, status: &str) {
        let p = self.primitives.remove(&id).unwrap();
        let errors = (
            p.prediction.elapsed - (now - p.at),
            p.prediction.nutrition - a.nutrition,
            p.prediction.fatigue - a.fatigue,
            p.load as i64 - load as i64,
        );
        self.count(format!(
            "primitive:{}:{status}:{}",
            p.kind,
            errors == (0, 0, 0, 0)
        ));
        if status == "COMPLETED" {
            self.row(
                "primitives",
                format!(
                    "{},{},{},{},{},{},{},{}",
                    p.at, id, p.kind, status, errors.0, errors.1, errors.2, errors.3
                ),
            );
        }
    }
    fn memories(&mut self, w: &World, o: &Observation, m: &PolicyMemory) {
        for v in m.nodes.values() {
            let age = o.now - v.observed_at;
            assert!(age >= 0);
            for f in &v.food {
                let true_q = w
                    .goods
                    .batches
                    .get(f.batch)
                    .filter(|b| b.material == Material::Food && b.expires_at() > o.now)
                    .map_or(0, |b| b.quantity);
                self.row(
                    "memory",
                    format!(
                        "{},{},food_quantity,{},{},{},{},{}",
                        o.now,
                        o.agent,
                        v.node,
                        age,
                        f.quantity,
                        true_q,
                        f.quantity as i64 - true_q as i64
                    ),
                );
                if let Some(b) = w.goods.batches.get(f.batch) {
                    self.row(
                        "memory",
                        format!(
                            "{},{},food_freshness,{},{},{},{},{}",
                            o.now,
                            o.agent,
                            v.node,
                            age,
                            f.expires_at - o.now,
                            b.expires_at() - o.now,
                            f.expires_at - b.expires_at()
                        ),
                    );
                }
            }
            if let Some(b) = &v.board {
                let true_q = w
                    .boards
                    .iter()
                    .find(|b| b.node == v.node)
                    .map_or(0, |b| b.quantity);
                self.row(
                    "memory",
                    format!(
                        "{},{},request,{},{},{},{},{}",
                        o.now,
                        o.agent,
                        v.node,
                        age,
                        b.quantity,
                        true_q,
                        b.quantity as i64 - true_q as i64
                    ),
                );
            }
            for e in &v.edges {
                let local_q = w
                    .agents
                    .iter()
                    .filter(|a| {
                        a.location == Location::Node(v.node)
                            && matches!(a.action,Action::Queued{edge,..}if edge==e.id)
                    })
                    .count();
                self.row(
                    "memory",
                    format!(
                        "{},{},queue_same_bank,{},{},{},{},{}",
                        o.now,
                        o.agent,
                        v.node,
                        age,
                        e.queued,
                        local_q,
                        e.queued as i64 - local_q as i64
                    ),
                );
            }
        }
        for &target in m.nodes.keys() {
            let Some(r) = audited_route(w, o, m, target) else {
                continue;
            };
            let mut b = Body {
                elapsed: 0,
                nutrition: o.nutrition,
                fatigue: o.fatigue,
                starvation: o.starvation,
            };
            for s in &r.steps {
                let start = b.elapsed;
                let old_n = b.nutrition;
                let old_f = b.fatigue;
                b.elapse(s.queue_ms, false);
                b.work(s.nominal_ms);
                let views = m
                    .nodes
                    .values()
                    .filter(|v| v.observed_at == s.observed_at)
                    .flat_map(|v| &v.edges)
                    .find(|e| e.id == s.edge);
                let q = w
                    .agents
                    .iter()
                    .filter(|a| matches!(a.action,Action::Queued{edge,..}if edge==s.edge))
                    .count();
                let opposite = w
                    .agents
                    .iter()
                    .filter(|a| {
                        a.location == Location::Node(s.to)
                            && matches!(a.action,Action::Queued{edge,..}if edge==s.edge)
                    })
                    .count();
                let occupied = w
                    .agents
                    .iter()
                    .filter(|a| {
                        matches!(a.location,Location::Edge{edge,..}if edge==s.edge)
                            && a.action == Action::Travel
                    })
                    .count();
                let duration = s.travel_ms + s.queue_ms;
                self.row(
                    "routes",
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        o.now,
                        o.agent,
                        target,
                        s.edge,
                        s.from,
                        s.to,
                        s.observed_at,
                        views.map_or(0, |e| e.queued),
                        s.queue_ms,
                        q,
                        opposite,
                        occupied,
                        duration - (b.elapsed - start),
                        (old_n - duration).max(0) - b.nutrition,
                        (old_f + duration).min(24 * HOUR) - b.fatigue
                    ),
                );
                if duration != b.elapsed - start {
                    self.count("route:queue_order_difference");
                }
            }
            if !r.steps.is_empty() {
                let fa = r.starvation < 24 * HOUR;
                let sa = b.alive();
                let class = match (fa, sa) {
                    (true, true) => "CORRECT_FEASIBLE",
                    (false, false) => "CORRECT_INFEASIBLE",
                    (true, false) => "FALSE_FEASIBLE",
                    (false, true) => "FALSE_INFEASIBLE",
                };
                self.count(format!("route_feasibility:{class}"));
                self.row(
                    "route_feasibility",
                    format!("{},{},{},{},{},{}", o.now, o.agent, target, fa, sa, class),
                );
            }
        }
    }
    fn need(&mut self, w: &World, o: &Observation, m: &PolicyMemory, d: &Decision) {
        if o.nutrition >= 12 * HOUR {
            return;
        }
        let fresh: Vec<_> = w
            .goods
            .batches
            .iter()
            .filter(|b| b.quantity > 0 && b.material == Material::Food && b.expires_at() > o.now)
            .collect();
        let local = |b: &&Batch| {
            b.holder == Holder::Agent(o.agent)
                || matches!((o.location.clone(),b.holder),(Location::Node(n),Holder::Deposit(x)|Holder::Loose(x)|Holder::Commons(x))if n==x)
        };
        let permitted = |b: &&Batch| {
            b.owner == Owner::None
                || b.owner == Owner::Agent(o.agent)
                || (o.communal && b.owner == Owner::Commons(0))
        };
        let known: Vec<_> = m
            .nodes
            .values()
            .filter(|v| {
                v.food
                    .iter()
                    .any(|f| f.quantity > 0 && f.expires_at > o.now)
            })
            .collect();
        let (category, detail) = if fresh.is_empty() {
            if w.goods
                .batches
                .iter()
                .any(|b| b.quantity > 0 && b.material == Material::Waste)
            {
                ("SPOILED", "only waste remains")
            } else {
                ("PHYSICALLY_UNAVAILABLE", "no fresh food anywhere")
            }
        } else if fresh.iter().any(local) && !fresh.iter().any(|b| local(b) && permitted(b)) {
            (
                "INSTITUTIONALLY_FORBIDDEN",
                "local fresh goods fail ownership rule",
            )
        } else if matches!(
            d.intent,
            Intent::Eat { .. } | Intent::Gather { .. } | Intent::Pickup { .. }
        ) {
            (
                "ACCESS_IN_PROGRESS",
                "need is being acted on; not a failure",
            )
        } else if known.is_empty() {
            (
                "UNKNOWN",
                "fresh food exists but no fresh source remembered",
            )
        } else if known
            .iter()
            .all(|v| audited_route(w, o, m, v.node).is_none_or(|r| r.starvation >= 24 * HOUR))
        {
            (
                "KNOWN_BUT_TOO_FAR",
                "belief route predicts death or unavailable route",
            )
        } else if matches!(o.action, Action::Queued { .. }) {
            (
                "CAPACITY_BLOCKED",
                "currently waiting for a shared edge; not proof all food routes blocked",
            )
        } else {
            (
                "UNRESOLVED",
                "no proof of legal feasible food access or voluntary refusal",
            )
        };
        self.count(format!("need:{category}"));
        self.row(
            "needs",
            format!(
                "{},{},{},{},{}",
                o.now, o.agent, o.nutrition, category, detail
            ),
        );
    }
}

// The old estimator remains the audit target for every preserved arm. The new
// research arm must be compared with the estimator it actually uses.
fn audited_route(
    w: &World,
    o: &Observation,
    m: &PolicyMemory,
    target: usize,
) -> Option<policy::Route> {
    if matches!(
        w.config.diagnostic_policy,
        DiagnosticPolicy::Current
            | DiagnosticPolicy::CoherentForecast
            | DiagnosticPolicy::ConditionalMeals
    ) {
        policy::coherent::route(o, m, target)
    } else {
        policy::route(o, m, target)
    }
}
