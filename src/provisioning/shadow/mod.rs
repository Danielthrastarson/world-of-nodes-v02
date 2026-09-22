//! Research-only shadow action conformance. Nothing here submits live commands.
mod physics;
mod static_cases;
pub use static_cases::{institutional_rows, queue_counterexample, static_rows};

mod observer;
mod oracle;
mod plan;
use super::*;
pub use observer::HEADERS;
pub(crate) use observer::Journal;
impl World {
    pub fn enable_shadow_audit(&mut self, detailed: bool) {
        self.shadow.enabled = true;
        self.shadow.detailed = detailed;
    }
    pub fn take_shadow_rows(&mut self) -> std::collections::BTreeMap<&'static str, Vec<String>> {
        std::mem::take(&mut self.shadow.rows)
    }
    pub fn shadow_counts(&self) -> &std::collections::BTreeMap<String, u64> {
        &self.shadow.counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn audit_preserves_rng_state_events_and_decisions() {
        let mut plain = acceptance::fixture("known_source", 7, false).unwrap();
        let mut observed = plain.clone();
        observed.enable_shadow_audit(true);
        for h in 0..=6 {
            plain.advance_to(h * HOUR).unwrap();
            observed.advance_to(h * HOUR).unwrap();
            assert_eq!(plain.state_hash(), observed.state_hash());
            assert_eq!(plain.event_hash(), observed.event_hash());
            observed.take_shadow_rows();
            plain.take_acceptance_records();
            observed.take_acceptance_records();
        }
        assert!(
            observed
                .shadow_counts()
                .get("candidates")
                .copied()
                .unwrap_or(0)
                > 0
        );
    }
    #[test]
    fn known_wait_must_precede_physiological_movement() {
        assert!(queue_counterexample().contains("-2800000"));
    }
    #[test]
    fn independent_belief_tape_matches_kernel_without_other_actors() {
        let w = acceptance::fixture("known_source", 0, false).unwrap();
        let o = w.observe(0).unwrap();
        let mut m = w.agents[0].memory.clone();
        m.nodes.insert(0, o.local.clone().unwrap());
        let (_, a) = policy::audit_meals(&o, &m, &mut 0, false);
        let c = &a.candidates[0];
        let tape = plan::tape(c, &m).unwrap();
        let p = plan::predict(&o, &m, c, &tape);
        let k = oracle::belief(&o, &m, c, &tape);
        assert!(p.failure.is_none());
        assert_eq!(k.failure, "NONE");
        assert_eq!(p.body.elapsed, k.elapsed);
        assert_eq!(p.delivered, k.delivered);
    }
}

#[cfg(test)]
mod mismatch_tests {
    use super::*;
    #[test]
    fn waiting_order_can_create_a_false_alive_prediction() {
        let mut w = World::new(Config {
            starts: vec![0; 5],
            autonomous: false,
            initial_nutrition: 24 * HOUR,
            initial_fatigue: 0,
            ..Config::default()
        })
        .unwrap();
        w.agents[4].nutrition = 0;
        w.agents[4].fatigue = 10 * HOUR;
        w.agents[4].starvation = 176 * HOUR / 10;
        w.apply_intents(
            (0..4)
                .map(|id| {
                    (
                        id,
                        Intent::Travel {
                            edge: 0,
                            target: 1,
                            food: false,
                        },
                    )
                })
                .collect(),
        )
        .unwrap();
        let o = w.observe(4).unwrap();
        let mut m = PolicyMemory::default();
        let mut v = o.local.clone().unwrap();
        v.edges[0].queued = 4;
        m.nodes.insert(0, v);
        let r = policy::route(&o, &m, 1).unwrap();
        assert!(r.starvation < 24 * HOUR);
        let mut p = physics::Body {
            elapsed: 0,
            nutrition: 0,
            fatigue: o.fatigue,
            starvation: o.starvation,
        };
        p.elapse(2 * HOUR, false);
        p.work(2 * HOUR);
        assert!(!p.alive());
        w.apply_intents(vec![(
            4,
            Intent::Travel {
                edge: 0,
                target: 1,
                food: false,
            },
        )])
        .unwrap();
        w.advance_to(8 * HOUR).unwrap();
        assert_eq!(w.agents[4].action, Action::Dead);
        assert_eq!(w.agents[4].death_time, Some(p.elapsed));
    }
    #[test]
    fn expiry_interrupts_reserved_gather_without_refresh_or_material_loss() {
        let mut w = World::new(Config {
            starts: vec![0],
            food: vec![(0, 1, 0, 1000)],
            autonomous: false,
            ..Config::default()
        })
        .unwrap();
        w.apply_intents(vec![(
            0,
            Intent::Gather {
                batch: 0,
                communal: false,
            },
        )])
        .unwrap();
        assert_eq!(w.reserved(0), 1);
        w.advance_to(1000).unwrap();
        assert_eq!(w.reserved(0), 0);
        assert_eq!(w.goods.load(0), 0);
        assert_eq!(w.goods.batches[0].material, Material::Waste);
        assert!(w.goods.audit());
    }
    #[test]
    fn known_stock_reservation_is_not_available_to_a_second_gatherer() {
        let mut w = World::new(Config {
            starts: vec![0; 2],
            food: vec![(0, 1, 0, 48 * HOUR)],
            autonomous: false,
            ..Config::default()
        })
        .unwrap();
        w.apply_intents(vec![
            (
                0,
                Intent::Gather {
                    batch: 0,
                    communal: false,
                },
            ),
            (
                1,
                Intent::Gather {
                    batch: 0,
                    communal: false,
                },
            ),
        ])
        .unwrap();
        assert_eq!(w.reserved(0), 1);
        assert_eq!(w.agents[1].action, Action::Idle);
    }
}
