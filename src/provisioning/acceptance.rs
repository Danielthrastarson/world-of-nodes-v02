//! Research fixtures and journals; observations passed to decisions remain local.
use super::*;

pub const GATES: &[&str] = &[
    "NO_KNOWN_SOURCE",
    "SOURCE_MEMORY_TOO_STALE",
    "SOURCE_PREDICTED_SPOILED",
    "INSUFFICIENT_EXPECTED_QUANTITY",
    "CANNOT_REACH_SOURCE_ALIVE",
    "CANNOT_COMPLETE_PLAN_ALIVE",
    "POST_PLAN_NUTRITION_MARGIN",
    "FATIGUE_CONSTRAINT",
    "CARRY_CAPACITY_ZERO",
    "REQUEST_ALREADY_CLAIMED",
    "OTHER_EXPLICIT_REASON",
    "PASS",
];
#[derive(Clone, Debug, Default)]
pub(crate) struct Journal {
    pub counts: BTreeMap<(String, String, String), u64>,
    pub zero_ms: BTreeMap<usize, u128>,
    pub records: Vec<String>,
    pub shadow_passes_with_meals: u64,
    pub forecast_candidates: u64,
}
impl Journal {
    pub fn record(&mut self, o: &Observation, d: &Decision, a: policy::AcceptanceAudit) {
        if a.requests == 0 {
            return;
        }
        let stage = if a.evaluated {
            "EVALUATION"
        } else {
            "DEFERRED"
        };
        for scope in if a.local {
            vec!["ALL", "LOCAL"]
        } else {
            vec!["ALL"]
        } {
            *self
                .counts
                .entry((scope.into(), stage.into(), a.outcome.clone()))
                .or_default() += 1;
        }
        for c in &a.candidates {
            self.forecast_candidates += 1;
            if c.violations.is_empty()
                && c.plan
                    .as_ref()
                    .and_then(|p| p.shadow)
                    .is_some_and(|s| s.3 == "PASS")
            {
                self.shadow_passes_with_meals += 1;
            }
        }
        self.records.push(format!(
            "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}",
            o.now,
            o.agent,
            o.location,
            a.local,
            stage,
            a.outcome,
            a.margin,
            o.nutrition,
            o.fatigue,
            o.starvation,
            d.intent,
            a
        ));
    }
}
pub const JOURNAL_HEADER: &str = "time_ms\tagent\tlocation\tlocal_request\tstage\tprimary_gate\tmargin_ms\tnutrition_ms\tfatigue_ms\tstarvation_ms\tintent\taudit";
pub fn fixture(name: &str, seed: u64, trace: bool) -> Result<World, String> {
    let variant = match name {
        "current" | "known_source" => DiagnosticPolicy::LegacyForecast,
        "meal_aware" => DiagnosticPolicy::MealAware,
        "zero_margin" => DiagnosticPolicy::ZeroMargin,
        "three_hour_margin" => DiagnosticPolicy::ThreeHourMargin,
        _ => return Err(format!("unknown acceptance fixture {name}")),
    };
    let mut w = World::new(Config {
        seed,
        trace,
        initial_request: 600,
        acceptance_audit: true,
        diagnostic_policy: variant,
        ..Config::default()
    })?;
    if name == "known_source" {
        // Stipulated prior belief, not a read of future global state or simulated history.
        let remembered = NodeView {
            node: 2,
            observed_at: -HOUR,
            food: vec![FoodView {
                batch: 0,
                quantity: 3000,
                available: 3000,
                owner: Owner::None,
                holder: Holder::Deposit(2),
                expires_at: 48 * HOUR,
            }],
            edges: vec![EdgeView {
                id: 1,
                edge: Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
                queued: 0,
                occupied: 0,
            }],
            board: None,
            people: vec![],
        };
        for agent in &mut w.agents {
            agent.memory.nodes.insert(2, remembered.clone());
        }
    }
    Ok(w)
}
impl World {
    pub fn take_acceptance_records(&mut self) -> Vec<String> {
        std::mem::take(&mut self.acceptance.records)
    }
    pub fn acceptance_counts(&self) -> &BTreeMap<(String, String, String), u64> {
        &self.acceptance.counts
    }
    pub fn zero_nutrition_ms(&self, id: usize) -> u128 {
        self.acceptance.zero_ms.get(&id).copied().unwrap_or(0)
    }
    pub fn meal_shadow_passes(&self) -> u64 {
        self.acceptance.shadow_passes_with_meals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_prior_is_timestamped_local_memory_not_fresh_world_information() {
        let a = fixture("current", 0, false).unwrap();
        let b = fixture("known_source", 0, false).unwrap();
        assert!(a.agents[0].memory.nodes.is_empty());
        assert_eq!(
            b.agents[0].memory.nodes.keys().copied().collect::<Vec<_>>(),
            vec![2]
        );
        let v = &b.agents[0].memory.nodes[&2];
        assert_eq!(v.observed_at, -HOUR);
        assert_eq!(v.food[0].quantity, 3000);
        assert_eq!(v.food[0].expires_at, 48 * HOUR);
        assert_eq!(a.goods, b.goods);
        assert_eq!(a.config, b.config);
        assert_eq!(b.observe(0).unwrap().local.unwrap().node, 0);
    }
    #[test]
    fn hourly_journaling_preserves_control_hashes_and_accounts_for_zero_nutrition() {
        let mut w = fixture("current", 0, false).unwrap();
        for h in 0..=72 {
            w.advance_to(h * HOUR).unwrap();
            w.take_acceptance_records();
        }
        assert_eq!(w.event_hash(), 0x056700f14dd3919c);
        assert_eq!(w.state_hash(), 0xa73b33d1ecabd5d1);
        assert_eq!(
            w.acceptance.zero_ms.values().sum::<u128>(),
            w.metrics.deficit_ms
        );
        assert!(
            w.acceptance
                .counts
                .keys()
                .all(|(_, _, g)| GATES.contains(&g.as_str()))
        );
    }
}
