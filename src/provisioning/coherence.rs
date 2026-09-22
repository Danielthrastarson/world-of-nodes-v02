//! Research controls for forecast mechanics. Never callable from autonomous policy.
use super::*;
use policy::forecast::Candidate;
use shadow::{oracle, plan};

pub fn static_validation() -> Result<Vec<String>, String> {
    let w = acceptance::fixture("known_source", 0, false)?;
    let base = w.observe(0).ok_or("missing agent")?;
    let mut memory = w.agents[0].memory.clone();
    memory.nodes.insert(0, base.local.clone().unwrap());
    let mut rows = vec![];
    for n in [0, 6, 8, 12, 24] {
        for f in [4, 10, 12, 16, 20, 22] {
            for queued in [0, 4, 12] {
                for (name, quantity, available, demand, loose, expiry) in [
                    ("deposit", 3000, 3000, 600, false, 48 * HOUR),
                    ("request_one", 3000, 3000, 1, false, 48 * HOUR),
                    ("loose", 3000, 3000, 600, true, 48 * HOUR),
                    ("reserved", 1, 0, 600, false, 48 * HOUR),
                    ("partial_reserved", 10, 3, 600, false, 48 * HOUR),
                    ("expiry", 3000, 3000, 600, false, 9 * HOUR),
                ] {
                    let mut o = base.clone();
                    o.nutrition = n * HOUR;
                    o.fatigue = f * HOUR;
                    let mut m = memory.clone();
                    let food = &mut m.nodes.get_mut(&2).unwrap().food[0];
                    food.quantity = quantity;
                    food.available = available;
                    food.expires_at = expiry;
                    if loose {
                        food.holder = Holder::Loose(2);
                    }
                    m.nodes
                        .get_mut(&0)
                        .unwrap()
                        .board
                        .as_mut()
                        .unwrap()
                        .quantity = demand;
                    m.nodes.get_mut(&0).unwrap().edges[0].queued = queued;
                    // Evaluate candidate even when the unchanged priority hierarchy would
                    // choose immediate survival first; this is a static model contract.
                    let audit = policy::coherent_contract(&o, &m, false);
                    for c in &audit.candidates {
                        if let Some(row) = validate_candidate(&o, &m, c)? {
                            rows.push(format!("{name},{n},{f},{queued},{row}"));
                        }
                    }
                }
            }
        }
    }
    Ok(rows)
}
fn validate_candidate(
    o: &Observation,
    m: &PolicyMemory,
    c: &Candidate,
) -> Result<Option<String>, String> {
    let Some(tape) = plan::tape(c, m) else {
        return Ok(None);
    };
    let p = c.plan.as_ref().unwrap();
    let k = oracle::belief_endpoint(o, m, c, &tape);
    let fatigue = (o.fatigue + p.elapsed).min(24 * HOUR);
    let exact = p.elapsed == k.elapsed
        && p.nutrition == k.nutrition
        && fatigue == k.fatigue
        && p.starvation == k.starvation
        && p.delivered == k.delivered;
    if !exact {
        return Err(format!(
            "static mismatch n={} f={} t={} source={:?}: plan={p:?} kernel={k:?}",
            o.nutrition, o.fatigue, o.now, c.source
        ));
    }
    Ok(Some(format!(
        "{},{},{},{},{},{},{},{}",
        p.elapsed,
        k.elapsed,
        p.nutrition,
        k.nutrition,
        fatigue,
        k.fatigue,
        p.delivered,
        k.delivered
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corrected_forecast_matches_static_kernel_matrix() {
        let rows = static_validation().unwrap();
        assert!(rows.len() > 400, "{}", rows.len());
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Journal {
    pub enabled: bool,
    pub detailed: bool,
    pub counts: BTreeMap<String, u64>,
    pub rows: BTreeMap<&'static str, Vec<String>>,
    kernel_keys: BTreeSet<(usize, usize, Time, String)>,
    board_bound: BTreeSet<usize>,
    migrated: BTreeSet<usize>,
    pub decisions: BTreeMap<(Time, usize), String>,
}
pub const HEADERS: &[(&str, &str)] = &[
    (
        "comparisons",
        "time_ms,agent,board,source,old_gate,new_gate,old_feasible,new_feasible,old_ms,new_ms,old_nutrition,new_nutrition,old_fatigue,new_fatigue,old_cargo,new_cargo,self_only_changed,all_changed",
    ),
    (
        "decisions",
        "time_ms\tagent\tlive\told\tcorrected\told_gate\tnew_gate",
    ),
    (
        "kernel",
        "time_ms,agent,board,source,exact,forecast_ms,kernel_ms,forecast_cargo,kernel_cargo,kernel_failure",
    ),
    (
        "trace",
        "time_ms\tagent\tobservation\tmemory\told_audit\tcorrected_audit",
    ),
];
fn gate(c: &Candidate) -> &str {
    c.violations.first().copied().unwrap_or("PASS")
}
fn feasible(c: &Candidate, now: Time) -> Option<bool> {
    c.plan
        .as_ref()
        .filter(|p| {
            !p.steps
                .iter()
                .any(|s| s.starts_with("source_prefilter_only"))
        })
        .map(|p| {
            p.failure.is_none()
                && p.starvation < 24 * HOUR
                && p.source_starvation < 24 * HOUR
                && p.delivered > 0
                && c.expiry.is_some_and(|e| e > now + p.elapsed)
        })
}
fn differs(a: &Candidate, b: &Candidate) -> bool {
    match (&a.plan, &b.plan) {
        (Some(a), Some(b)) => {
            a.elapsed != b.elapsed
                || a.nutrition != b.nutrition
                || a.starvation != b.starvation
                || a.delivered != b.delivered
                || a.acquired != b.acquired
                || a.failure != b.failure
        }
        (None, None) => false,
        _ => true,
    }
}
impl Journal {
    fn count(&mut self, k: impl Into<String>) {
        *self.counts.entry(k.into()).or_default() += 1;
    }
    fn row(&mut self, k: &'static str, v: String) {
        if self.detailed {
            self.rows.entry(k).or_default().push(v);
        }
    }
    pub(crate) fn event(&mut self, e: &Event, agents: &[Agent]) {
        if !self.enabled {
            return;
        }
        let Some(id) = e.agent else {
            return;
        };
        if e.kind == "Queued" && self.board_bound.contains(&id) {
            self.count("board_queue_joins");
        }
        if e.kind == "Arrived" {
            if agents[id].location == Location::Node(2) && self.migrated.insert(id) {
                self.count("unique_agents_reaching_f");
            }
            if self.board_bound.contains(&id) {
                if agents[id].location == Location::Node(1) {
                    self.count("board_bound_x_arrivals");
                }
                if agents[id].location == Location::Node(0) {
                    self.count("board_arrivals");
                    self.board_bound.remove(&id);
                }
            }
        }
        if e.kind == "Died" {
            self.board_bound.remove(&id);
        }
    }
    pub(crate) fn observe(&mut self, w: &World, o: &Observation, live: &Decision, rng: u64) {
        self.count("decision_points");
        let m = &w.agents[o.agent].memory;
        let meals = w.config.persistence.is_some()
            || w.config.diagnostic_policy == DiagnosticPolicy::MealAware;
        let (old, oa) = if meals {
            policy::audit_meals(
                o,
                m,
                &mut rng.clone(),
                w.config.persistence == Some(persistence::Arm::ForecastCorrected),
            )
        } else {
            policy::decide_audited(o, m, &mut rng.clone(), DiagnosticPolicy::Current)
        };
        let (new, na) = policy::audit_coherent(o, m, &mut rng.clone(), meals);
        let self_only = if meals {
            Some(policy::audit_meals(o, m, &mut rng.clone(), true))
        } else {
            None
        };
        let changed = old.intent != new.intent;
        if changed {
            self.count("intent_changed_on_same_inputs");
        }
        if old.reason != new.reason {
            self.count("reason_changed_on_same_inputs");
        }
        if self_only
            .as_ref()
            .is_some_and(|(d, _)| d.intent != old.intent)
        {
            self.count("self_only_intent_changed");
            if changed {
                self.count("self_only_intent_change_retained_by_all");
            }
        }
        let actual = if w.config.diagnostic_policy == DiagnosticPolicy::CoherentForecast {
            &na
        } else {
            &oa
        };
        if actual.evaluated {
            self.count("live_acceptance_evaluations");
            self.count(format!("live_gate:{}", actual.outcome));
        }
        self.count(format!(
            "new_gate:{}:{}",
            if na.evaluated {
                "EVALUATION"
            } else {
                "DEFERRED"
            },
            na.outcome
        ));
        if live.reason.starts_with("visit remembered board") {
            self.count("board_directed_intents");
            self.board_bound.insert(o.agent);
        } else if live
            .reason
            .starts_with("remain FIFO; visit remembered board")
        {
            self.board_bound.insert(o.agent);
        } else if !matches!(live.intent, Intent::Wait) {
            self.board_bound.remove(&o.agent);
        }
        self.decisions
            .insert((o.now, o.agent), format!("{:?}", live.intent));
        if self.detailed {
            self.row(
                "decisions",
                format!(
                    "{}\t{}\t{:?}\t{:?}\t{:?}\t{}\t{}",
                    o.now, o.agent, live.intent, old.intent, new.intent, oa.outcome, na.outcome
                ),
            );
        }
        for c in &na.candidates {
            self.count("new_candidate_evaluations");
            self.count(format!("new_candidate_gate:{}", gate(c)));
            let prev = oa
                .candidates
                .iter()
                .find(|p| p.board == c.board && p.source == c.source);
            if let Some(p) = prev {
                self.count("matched_candidates");
                if gate(p) != gate(c) {
                    self.count("primary_gate_changed");
                }
                if !p.violations.is_empty() && c.violations.is_empty() {
                    self.count("rejection_to_pass");
                }
                if p.violations.is_empty() && !c.violations.is_empty() {
                    self.count("pass_to_rejection");
                }
                if let (Some(a), Some(b)) = (feasible(p, o.now), feasible(c, o.now)) {
                    self.count("matched_physical_classifications");
                    if a != b {
                        self.count("physical_feasibility_changed");
                    }
                }
                let changed = differs(p, c);
                if changed {
                    self.count("candidate_changed");
                }
                let self_changed = self_only
                    .as_ref()
                    .and_then(|(_, a)| {
                        a.candidates
                            .iter()
                            .find(|v| v.board == p.board && v.source == p.source)
                    })
                    .is_some_and(|v| differs(p, v));
                if self_changed {
                    self.count("self_only_candidate_changed");
                    if changed {
                        self.count("self_only_candidate_change_retained_by_all");
                    }
                }
                if let (Some(a), Some(b)) = (&p.plan, &c.plan) {
                    for (label, x, y) in [
                        ("time", a.elapsed, b.elapsed),
                        ("nutrition", a.nutrition, b.nutrition),
                        (
                            "fatigue",
                            (o.fatigue + a.elapsed).min(24 * HOUR),
                            (o.fatigue + b.elapsed).min(24 * HOUR),
                        ),
                        ("cargo", a.delivered as i64, b.delivered as i64),
                    ] {
                        if x != y {
                            self.count(format!("changed:{label}"));
                        }
                    }
                    self.row(
                        "comparisons",
                        format!(
                            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                            o.now,
                            o.agent,
                            c.board,
                            c.source.map_or(String::new(), |n| n.to_string()),
                            gate(p),
                            gate(c),
                            feasible(p, o.now).map_or(String::new(), |v| v.to_string()),
                            feasible(c, o.now).map_or(String::new(), |v| v.to_string()),
                            a.elapsed,
                            b.elapsed,
                            a.nutrition,
                            b.nutrition,
                            (o.fatigue + a.elapsed).min(24 * HOUR),
                            (o.fatigue + b.elapsed).min(24 * HOUR),
                            a.delivered,
                            b.delivered,
                            self_changed,
                            changed
                        ),
                    );
                }
            } else {
                self.count("candidate_newly_evaluated");
            }
            let Some(tape) = plan::tape(c, m) else {
                self.count("new_partial_prefilter");
                continue;
            };
            let r = plan::predict(o, m, c, &tape);
            let predicted = feasible(c, o.now).unwrap_or(false);
            let physical = r.failure.is_none() && r.delivered > 0;
            let class = match (predicted, physical) {
                (true, true) => "CORRECT_FEASIBLE",
                (false, false) => "CORRECT_INFEASIBLE",
                (true, false) => "FALSE_FEASIBLE",
                (false, true) => "FALSE_INFEASIBLE",
            };
            self.count(format!("new_physical:{class}"));
            let p = c.plan.as_ref().unwrap();
            if p.elapsed != r.body.elapsed
                || p.nutrition != r.body.nutrition
                || p.delivered != r.delivered
            {
                self.count("new_shadow_terminal_difference");
            }
            if self.detailed
                && self.kernel_keys.insert((
                    c.board,
                    c.source.unwrap_or(usize::MAX),
                    o.now / HOUR,
                    gate(c).into(),
                ))
            {
                let k = oracle::belief_endpoint(o, m, c, &tape);
                let exact = p.elapsed == k.elapsed
                    && p.nutrition == k.nutrition
                    && p.starvation == k.starvation
                    && (o.fatigue + p.elapsed).min(24 * HOUR) == k.fatigue
                    && p.delivered == k.delivered;
                self.count(if exact {
                    "kernel_exact"
                } else {
                    "kernel_mismatch"
                });
                self.row(
                    "kernel",
                    format!(
                        "{},{},{},{},{},{},{},{},{},{}",
                        o.now,
                        o.agent,
                        c.board,
                        c.source.unwrap_or(usize::MAX),
                        exact,
                        p.elapsed,
                        k.elapsed,
                        p.delivered,
                        k.delivered,
                        k.failure
                    ),
                );
                self.row(
                    "trace",
                    format!(
                        "{}\t{}\t{:?}\t{:?}\t{:?}\t{:?}",
                        o.now, o.agent, o, m, oa, na
                    ),
                );
            }
        }
    }
}
impl World {
    pub fn enable_coherence_audit(&mut self, detailed: bool) {
        self.coherence.enabled = true;
        self.coherence.detailed = detailed;
    }
    pub fn coherence_counts(&self) -> &BTreeMap<String, u64> {
        &self.coherence.counts
    }
    pub fn take_coherence_rows(&mut self) -> BTreeMap<&'static str, Vec<String>> {
        std::mem::take(&mut self.coherence.rows)
    }
    pub fn take_coherence_decisions(&mut self) -> BTreeMap<(Time, usize), String> {
        std::mem::take(&mut self.coherence.decisions)
    }
}
