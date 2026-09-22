//! Small, read-only vocabulary over existing state; no arbitration or new commitments.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Goal {
    ReachNode(usize),
    AccessFoodAt(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Undertaking {
    Provision {
        request: usize,
        destination: usize,
        target: u64,
        acquired: u64,
    },
}
/// Beliefs are remembered observations, not a reference to World. An undertaking
/// may survive an eating/sleeping action. A route goal is not a guaranteed claim.
#[derive(Debug)]
pub struct ReasoningState<'a> {
    pub beliefs: &'a PolicyMemory,
    pub goal: Option<Goal>,
    pub undertaking: Option<Undertaking>,
    pub action: &'a Action,
}
impl Agent {
    pub fn reasoning_state(&self) -> ReasoningState<'_> {
        ReasoningState {
            beliefs: &self.memory,
            goal: self.goal.map(|(n, food)| {
                if food {
                    Goal::AccessFoodAt(n)
                } else {
                    Goal::ReachNode(n)
                }
            }),
            undertaking: self.job.as_ref().map(|j| Undertaking::Provision {
                request: j.request,
                destination: j.destination,
                target: j.target,
                acquired: j.acquired,
            }),
            action: &self.action,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn undertaking_is_distinct_from_atomic_action_and_claim_lease() {
        let mut w = World::new(Config {
            autonomous: false,
            initial_request: 600,
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
        let undertaking = w.agents[0].reasoning_state().undertaking;
        assert!(undertaking.is_some());
        w.apply_intents(vec![(0, Intent::Sleep)]).unwrap();
        assert!(matches!(
            w.agents[0].reasoning_state().action,
            Action::Sleep { .. }
        ));
        assert_eq!(undertaking, w.agents[0].reasoning_state().undertaking);
        w.advance_to(MINUTE).unwrap();
        w.apply_intents(vec![(0, Intent::CancelJob)]).unwrap();
        assert!(w.agents[0].reasoning_state().undertaking.is_none());
        w.audit().unwrap();
    }
    #[test]
    fn same_personal_history_and_rng_produce_same_decision() {
        let mut w = acceptance::fixture("known_source", 7, false).unwrap();
        let o = w.observe(0).unwrap();
        let mut m = w.agents[0].memory.clone();
        m.nodes.insert(0, o.local.clone().unwrap());
        for mode in [
            DiagnosticPolicy::Current,
            DiagnosticPolicy::ConditionalMeals,
        ] {
            let mut r1 = 81;
            let mut r2 = 81;
            let a = policy::decide_variant(&o, &m, &mut r1, mode);
            // Change hidden physical truth without refreshing the captured personal inputs.
            w.goods.batches[0].quantity = 0;
            let b = policy::decide_variant(&o, &m, &mut r2, mode);
            assert_eq!(a, b);
            assert_eq!(r1, r2);
        }
    }
}
