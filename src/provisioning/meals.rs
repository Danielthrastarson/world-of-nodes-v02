//! Observational instrumentation for conditional intermediate meals. No commands.
use super::*;
use policy::forecast::AcceptanceAudit;
use shadow::plan::value;

#[derive(Clone, Debug, Default)]
pub(crate) struct Journal {
    enabled: bool,
    detailed: bool,
    offline: bool,
    counts: BTreeMap<String, u64>,
    rows: Vec<String>,
    decisions: BTreeMap<(Time, usize), String>,
    board_bound: BTreeSet<usize>,
    sources: BTreeSet<usize>,
    seen: BTreeSet<(usize, String)>,
}
impl Journal {
    fn count(&mut self, k: impl Into<String>) {
        *self.counts.entry(k.into()).or_default() += 1;
    }
    pub(crate) fn event(&mut self, e: &Event, agents: &[Agent]) {
        if !self.enabled {
            return;
        }
        let Some(id) = e.agent else {
            return;
        };
        if e.kind == "Arrived" {
            if agents[id].location == Location::Node(2) && self.sources.insert(id) {
                self.count("unique_f_arrivals");
            }
            if self.board_bound.contains(&id) {
                if agents[id].location == Location::Node(1) {
                    self.count("board_x_arrivals");
                }
                if agents[id].location == Location::Node(0) {
                    self.count("board_c_arrivals");
                    self.board_bound.remove(&id);
                }
            }
        }
        if e.kind == "Queued" && self.board_bound.contains(&id) {
            self.count("board_queue_joins");
            if agents[id].location == Location::Node(2) {
                self.count("board_f_to_x_attempts");
            }
            if agents[id].location == Location::Node(1) {
                self.count("board_x_to_c_attempts");
            }
        }
    }
    pub(crate) fn observe(
        &mut self,
        o: &Observation,
        m: &PolicyMemory,
        live: &Decision,
        rng: u64,
        mode: DiagnosticPolicy,
    ) {
        if !self.enabled {
            return;
        }
        self.count("decisions");
        self.decisions
            .insert((o.now, o.agent), format!("{:?}", live.intent));
        let (_, audit) = policy::decide_audited(o, m, &mut rng.clone(), mode);
        if !self.offline {
            self.record(o, m, live, &audit);
        }
        if self.offline {
            let (old, oa) = policy::audit_meals(o, m, &mut rng.clone(), false);
            let (new, na) =
                policy::decide_audited(o, m, &mut rng.clone(), DiagnosticPolicy::ConditionalMeals);
            if old.intent != new.intent {
                self.count("offline_intent_changed");
            }
            if oa.evaluated && na.evaluated {
                self.count("offline_matched_evaluations");
                self.count(format!("offline_old_gate:{}", oa.outcome));
                self.count(format!("offline_new_gate:{}", na.outcome));
                if oa.outcome != na.outcome {
                    self.count("offline_primary_gate_changed");
                }
            }
            for c in &na.candidates {
                if let Some(p) = oa
                    .candidates
                    .iter()
                    .find(|p| p.board == c.board && p.source == c.source)
                {
                    self.count("offline_matched_candidates");
                    let a = p.violations.first().copied().unwrap_or("PASS");
                    let b = c.violations.first().copied().unwrap_or("PASS");
                    if a != b {
                        self.count("offline_candidate_gate_changed");
                    }
                    if a != "PASS" && b == "PASS" {
                        self.count("offline_rejection_to_pass");
                    }
                    if a == "PASS" && b != "PASS" {
                        self.count("offline_pass_to_rejection");
                    }
                    if let (Some(p), Some(q)) = (&p.plan, &c.plan)
                        && (p.elapsed != q.elapsed
                            || p.nutrition != q.nutrition
                            || p.delivered != q.delivered
                            || p.acquired != q.acquired)
                    {
                        self.count("offline_numerical_change");
                    }
                }
            }
            if self.detailed && oa.evaluated {
                self.rows.push(format!(
                    "OFFLINE\t{}\t{}\told={:?}\tnew={:?}\told_audit={oa:?}\tnew_audit={na:?}",
                    o.now, o.agent, old.intent, new.intent
                ));
            }
        }
    }
    fn record(&mut self, o: &Observation, m: &PolicyMemory, live: &Decision, a: &AcceptanceAudit) {
        if a.requests > 0 {
            self.count("decisions_with_remembered_request");
            if m.nodes
                .values()
                .any(|v| v.food.iter().any(|f| f.quantity > 0))
            {
                self.count("decisions_with_source_memory");
            }
        }
        if a.evaluated {
            self.count("acceptance_evaluations");
            self.count(format!("gate:{}", a.outcome));
        }
        for c in &a.candidates {
            self.count("candidate_evaluations");
            self.count(format!(
                "candidate_gate:{}",
                c.violations.first().copied().unwrap_or("PASS")
            ));
            if let Some(p) = &c.plan {
                if p.failure.is_none()
                    && p.starvation < 24 * HOUR
                    && p.source_starvation < 24 * HOUR
                    && p.delivered > 0
                    && c.expiry.is_some_and(|e| o.now + p.elapsed < e)
                {
                    self.count("physically_executable_candidates");
                }
                if c.violations.is_empty() {
                    self.count("pass_candidates");
                    self.counts
                        .entry("pass_meal_quanta".into())
                        .and_modify(|v| *v += p.meal_quanta)
                        .or_insert(p.meal_quanta);
                    for step in &p.steps {
                        if step.starts_with("meal:") {
                            self.count("pass_meals");
                            assert!(value(step, "q").unwrap() > 0);
                        }
                    }
                    if self.detailed {
                        self.rows.push(format!("PASS\t{}\t{}\tfinal_nutrition={}\tdeliverable={}\tmeal_quanta={}\tcandidate={c:?}",o.now,o.agent,p.nutrition,p.delivered,p.meal_quanta));
                    }
                }
            }
        }
        let board = live.reason.starts_with("visit remembered board")
            || live
                .reason
                .starts_with("remain FIFO; visit remembered board");
        if board {
            self.count("board_directed_decisions");
            if self.board_bound.insert(o.agent) {
                self.count("board_intention_episodes");
            }
        } else if !matches!(live.intent, Intent::Wait) {
            if self.board_bound.contains(&o.agent) && self.detailed {
                self.rows.push(format!("BOARD_EXIT\t{}\t{}\tnutrition={}\tintent={:?}\treason={}\taudit={a:?}\tobservation={o:?}\tmemory={m:?}",o.now,o.agent,o.nutrition,live.intent,live.reason));
            }
            self.board_bound.remove(&o.agent);
        }
        let reason = live
            .reason
            .split(';')
            .next()
            .unwrap_or(&live.reason)
            .to_string();
        if self.detailed && (board || self.seen.insert((o.agent, reason))) {
            self.rows.push(format!("DECISION\t{}\t{}\tat={:?}\tnutrition={}\tfatigue={}\tintent={:?}\treason={}\tobservation={o:?}\tmemory={m:?}",o.now,o.agent,o.location,o.nutrition,o.fatigue,live.intent,live.reason));
        }
    }
}
impl World {
    pub fn enable_meal_research(&mut self, detailed: bool, offline: bool) {
        self.meals.enabled = true;
        self.meals.detailed = detailed;
        self.meals.offline = offline;
    }
    pub fn meal_research_counts(&self) -> &BTreeMap<String, u64> {
        &self.meals.counts
    }
    pub fn take_meal_research_rows(&mut self) -> Vec<String> {
        std::mem::take(&mut self.meals.rows)
    }
    pub fn take_meal_research_decisions(&mut self) -> BTreeMap<(Time, usize), String> {
        std::mem::take(&mut self.meals.decisions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn consuming_all_cargo_is_quantity_failure_not_spoilage() {
        let w = acceptance::fixture("known_source", 0, false).unwrap();
        let mut o = w.observe(0).unwrap();
        o.local.as_mut().unwrap().board.as_mut().unwrap().quantity = 1;
        let mut m = w.agents[0].memory.clone();
        m.nodes.insert(0, o.local.clone().unwrap());
        let a = policy::conditional_contract(&o, &m);
        let p = a.candidates[0].plan.as_ref().unwrap();
        assert_eq!(p.acquired, 1);
        assert_eq!(p.meal_quanta, 1);
        assert_eq!(p.delivered, 0);
        assert!(o.now + p.elapsed < 48 * HOUR);
        assert_eq!(p.failure, Some("INSUFFICIENT_EXPECTED_QUANTITY"));
    }
}
