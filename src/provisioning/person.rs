//! B0 common person behavior. Pure choice receives personal observations/beliefs only.
//! Agent is retained as the storage/debug name for historical replay compatibility.
use super::{
    Action, Agent, Decision, DiagnosticPolicy, HOUR, Intent, Job, Observation, PolicyMemory, Time,
    capital, farming, infrastructure, persistence, policy, reasoning,
};
mod adapter;
pub use super::Agent as Person;

/// Available domain evaluators, not occupations or separate brains.
#[derive(Clone, Copy, Debug, Default)]
pub struct Capabilities<'a> {
    pub information: Option<&'a super::information::Memory>,
    pub combat: Option<&'a super::combat::View>,
    pub formation: Option<&'a super::formation::View>,
    pub military: Option<&'a super::military::Memory>,
    pub life: Option<&'a super::lifecycle::View>,
    pub social: Option<&'a super::social::Memory>,
    pub care: Option<&'a super::household::Memory>,
    pub cultivation: Option<(&'a farming::Memory, &'a farming::Rules)>,
    pub capital: Option<(&'a capital::Memory, &'a capital::Rules)>,
    pub public_works: Option<(&'a infrastructure::Memory, bool)>,
}
impl Capabilities<'_> {
    pub fn any(&self) -> bool {
        self.information.is_some()
            || self.combat.is_some()
            || self.formation.is_some()
            || self.military.is_some()
            || self.life.is_some()
            || self.social.is_some()
            || self.care.is_some()
            || self.cultivation.is_some()
            || self.capital.is_some()
            || self.public_works.is_some()
    }
}
/// Derived from embodied state; not a second mutable hunger/fatigue system.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Needs {
    pub nutrition: Time,
    pub starvation: Time,
    pub fatigue: Time,
}
impl Needs {
    pub fn from_observation(o: &Observation) -> Self {
        Self {
            nutrition: o.nutrition,
            starvation: o.starvation,
            fatigue: o.fatigue,
        }
    }
    pub fn critical_hunger(self) -> bool {
        self.nutrition <= 6 * HOUR
    }
    pub fn critical_fatigue(self) -> bool {
        self.fatigue >= 22 * HOUR
    }
    pub fn self_provision_preferred(self) -> bool {
        self.nutrition < 12 * HOUR
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Soldier {
        unit: usize,
    },
    Sovereign {
        polity: usize,
    },
    Steward {
        polity: usize,
        office: usize,
        settlement: usize,
    },
}
/// Recognized rules, not permission to bypass kernel validation or query goods globally.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rights {
    pub commons_membership: Option<usize>,
    pub voluntary_goods_exchange: bool,
    pub personal_crop_claims: bool,
    pub personal_cart_use: bool,
}
#[derive(Debug)]
pub struct State<'a> {
    pub biology: Option<&'a super::lifecycle::Biology>,
    pub social: Option<&'a super::social::Memory>,
    pub household: Option<super::household::PersonalView<'a>>,
    pub person: &'a Person,
    pub needs: Needs,
    pub reasoning: reasoning::ReasoningState<'a>,
    pub capabilities: Capabilities<'a>,
    pub roles: Vec<Role>,
    pub rights: Rights,
    /// None for a dead person. Cargo retains its owner/custodian distinction.
    pub observation: Option<Observation>,
}
#[derive(Clone, Copy, Debug)]
pub struct Context<'a> {
    pub observation: &'a Observation,
    pub beliefs: &'a PolicyMemory,
    pub capabilities: Capabilities<'a>,
}
/// A real next-step plan, not a fabricated persistent sequence. The existing Job
/// remains the longer-lived undertaking; conditional forecasts are not commitments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionPlan {
    pub formed_at: Time,
    pub person: usize,
    pub prior_route_goal: Option<(usize, bool)>,
    pub undertaking: Option<Job>,
    pub proposed: Decision,
}
impl ActionPlan {
    pub fn new(o: &Observation, proposed: Decision) -> Self {
        Self {
            formed_at: o.now,
            person: o.agent,
            prior_route_goal: o.goal,
            undertaking: o.job.clone(),
            proposed,
        }
    }
    pub fn command(&self) -> &Intent {
        &self.proposed.intent
    }
}
/// Normal pure policy entry for every person, regardless of institutional role.
pub fn decide(context: Context<'_>, rng: &mut u64) -> Decision {
    policy::decide_with_capabilities(
        context.observation,
        context.beliefs,
        rng,
        policy::ForecastMode::Coherent,
        &mut policy::AcceptanceAudit::disabled(),
        false,
        context.capabilities,
    )
}
/// Historical research modes are adapters around the same embodied person.
/// Preserve their precedence exactly; economic capability runs historically use
/// coherent normal mechanics even if a diagnostic flag is also configured.
pub(crate) struct Selection {
    pub decision: Decision,
    pub audit: Option<policy::AcceptanceAudit>,
    pub used_persistence: bool,
}
pub(crate) fn select(
    context: Context<'_>,
    rng: &mut u64,
    variant: DiagnosticPolicy,
    acceptance_audit: bool,
    persistent: Option<(persistence::Arm, &mut persistence::State)>,
) -> Selection {
    let (o, m) = (context.observation, context.beliefs);
    let (decision, audit, used_persistence) = if context.capabilities.any() {
        (decide(context, rng), None, false)
    } else if let Some((arm, state)) = persistent {
        (policy::decide_persistent(o, m, rng, arm, state), None, true)
    } else if acceptance_audit {
        let (d, a) = policy::decide_audited(o, m, rng, variant);
        (d, Some(a), false)
    } else {
        (policy::decide_variant(o, m, rng, variant), None, false)
    };
    Selection {
        decision,
        audit,
        used_persistence,
    }
}
impl Agent {
    pub fn needs(&self) -> Needs {
        Needs {
            nutrition: self.nutrition,
            starvation: self.starvation,
            fatigue: self.fatigue,
        }
    }
    pub fn can_reconsider(&self) -> bool {
        !matches!(self.action, Action::Dead | Action::Travel)
    }
}
#[cfg(test)]
mod tests;

/// Exchange is another opportunity stage, preserving existing survival/commitment priority.
pub(crate) fn select_exchange(
    input: &super::barter::selection::Input,
    o: &Observation,
    fallback: Decision,
) -> super::barter::selection::Selection {
    super::barter::selection::choose(input, o, fallback)
}
