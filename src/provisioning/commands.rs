//! F13B: accepted office duties, physical command transport, and bounded obligations.
//! An envelope is information. It does not confer goods, a route, labor or success.
use super::*;
mod adapter;
#[cfg(test)]
mod tests;

pub type MessageId = u64;
pub type CommandId = MessageId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Objective {
    CommissionCrossing {
        project: usize,
        settlement: usize,
    },
    /// An invalid request for the explicit negative scope test; never executable.
    Unsupported {
        label: String,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub id: CommandId,
    pub issuer: usize,
    pub polity: usize,
    pub recipient: usize,
    pub office: usize,
    pub objective: Objective,
    pub issued_at: Time,
    pub expires_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Revocation {
    pub id: MessageId,
    pub issuer: usize,
    pub recipient: usize,
    pub office: usize,
    pub command: CommandId,
    pub issued_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Envelope {
    Order(Order),
    Revoke(Revocation),
}
impl Envelope {
    pub fn id(&self) -> MessageId {
        match self {
            Self::Order(o) => o.id,
            Self::Revoke(r) => r.id,
        }
    }
    pub fn issuer(&self) -> usize {
        match self {
            Self::Order(o) => o.issuer,
            Self::Revoke(r) => r.issuer,
        }
    }
    pub fn recipient(&self) -> usize {
        match self {
            Self::Order(o) => o.recipient,
            Self::Revoke(r) => r.recipient,
        }
    }
    pub fn issued_at(&self) -> Time {
        match self {
            Self::Order(o) => o.issued_at,
            Self::Revoke(r) => r.issued_at,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub envelope: Envelope,
    pub sender: usize,
    pub received_at: Time,
    pub node: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Received,
    Active,
    Paused,
    Completed,
    Failed,
    Expired,
    Refused,
    Revoked,
}
impl Status {
    pub fn terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Expired | Self::Refused | Self::Revoked
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleObligation {
    pub command: CommandId,
    pub source_issuer: usize,
    pub recipient_office: usize,
    pub objective: Objective,
    pub issued_at: Time,
    pub received_at: Time,
    pub expires_at: Time,
    pub status: Status,
    pub reason: String,
    pub changed_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Delivery {
    pub message: MessageId,
    pub recipient: usize,
    pub rendezvous: usize,
    pub created_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnownSite {
    pub project: usize,
    pub settlement: usize,
    pub observed_at: Time,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    /// Physical appointment offers; no automatic acceptance by the holder.
    pub offered_offices: BTreeMap<usize, authority::Office>,
    /// Copies held by the accepting person and appointing sovereign after contact.
    pub accepted_offices: BTreeMap<usize, authority::Office>,
    pub envelopes: BTreeMap<MessageId, Receipt>,
    pub obligations: BTreeMap<CommandId, RoleObligation>,
    pub invalid: BTreeMap<MessageId, String>,
    pub received_revocations: BTreeSet<CommandId>,
    pub known_sites: BTreeMap<usize, KnownSite>,
    /// Attribution of the current generic travel goal to its role undertaking.
    pub pursuing: Option<CommandId>,
    pub delivery: Option<Delivery>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transmission {
    pub message: MessageId,
    pub sender: usize,
    pub recipient: usize,
    pub node: usize,
    pub issued_at: Time,
    pub received_at: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub person: usize,
    pub command: CommandId,
    pub at: Time,
    pub status: Status,
    pub reason: String,
}
/// Research observations, excluded from Person inputs and authoritative hash.
#[derive(Clone, Debug, Default)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub transmissions: Vec<Transmission>,
    pub transitions: Vec<Transition>,
}
impl Journal {
    pub fn add(&mut self, key: &str) {
        *self.counts.entry(key.into()).or_default() += 1;
    }
    pub fn count(&self, key: &str) -> u64 {
        self.counts.get(key).copied().unwrap_or(0)
    }
}
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub memories: Vec<Memory>,
    pub next_message: MessageId,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &Config) -> Self {
        Self {
            memories: if c.commands {
                vec![Memory::default(); c.starts.len()]
            } else {
                vec![]
            },
            ..Self::default()
        }
    }
}

/// Acceptance is a distinct ordinary choice, never an implication of being appointed.
pub(crate) fn consider_acceptance(
    o: &Observation,
    _m: &PolicyMemory,
    memory: &Memory,
) -> Option<Decision> {
    if o.nutrition < 12 * HOUR || o.fatigue >= 18 * HOUR || o.action != Action::Idle {
        return None;
    }
    let local = o.local.as_ref()?;
    let office = memory
        .offered_offices
        .values()
        .find(|office| office.holder == o.agent && local.people.contains(&office.appointed_by))?;
    Some(Decision {
        intent: Intent::AcceptOffice { office: office.id },
        reason: "voluntarily accept scoped steward duty while safely idle and issuer present"
            .into(),
    })
}

/// Pure role arbitration. It has only this Person's received obligations and beliefs.
/// The caller keeps immediate survival above this stage and optional work below it.
pub(crate) fn consider(o: &Observation, m: &PolicyMemory, memory: &Memory) -> Option<Decision> {
    let needs = person::Needs::from_observation(o);
    if needs.critical_hunger() || needs.critical_fatigue() {
        return None;
    }
    let obligation = memory
        .obligations
        .values()
        .find(|x| !x.status.terminal() && x.expires_at > o.now)?;
    let Objective::CommissionCrossing { settlement, .. } = obligation.objective else {
        return None;
    };
    let choose = |intent, why: &str| Decision {
        intent,
        reason: format!("role obligation {}: {why}", obligation.command),
    };
    let local = o.local.as_ref()?;
    if local.node == settlement {
        return Some(choose(
            Intent::FulfilOrder {
                command: obligation.command,
            },
            "attempt supported local institutional act",
        ));
    }
    let route = policy::coherent::route(o, m, settlement)?;
    if route.starvation >= 24 * HOUR {
        return None;
    }
    Some(choose(
        Intent::Travel {
            edge: route.first,
            target: settlement,
            food: false,
        },
        "travel through remembered geography; no forced arrival",
    ))
}

/// Typed selection attribution. Explanatory strings never grant authority or
/// determine whether a Person is following an institutional undertaking.
pub(crate) fn selected_role_command(
    o: &Observation,
    m: &PolicyMemory,
    memory: &Memory,
    selected: &Decision,
) -> Option<CommandId> {
    let candidate = consider(o, m, memory)?;
    if candidate.intent != selected.intent {
        return None;
    }
    memory
        .obligations
        .values()
        .find(|x| !x.status.terminal() && x.expires_at > o.now)
        .map(|x| x.command)
}

pub(crate) fn protects_choice(
    o: &Observation,
    m: &PolicyMemory,
    memory: &Memory,
    selected: &Decision,
) -> bool {
    [
        consider(o, m, memory),
        consider_delivery(o, m, memory),
        consider_acceptance(o, m, memory),
    ]
    .iter()
    .flatten()
    .any(|candidate| candidate.intent == selected.intent)
}

/// Existing embodied carrier undertaking, with a fixed rendezvous and no tracking.
pub(crate) fn consider_delivery(
    o: &Observation,
    m: &PolicyMemory,
    memory: &Memory,
) -> Option<Decision> {
    let u = memory.delivery.as_ref()?;
    let choose = |intent, why: &str| Decision {
        intent,
        reason: format!("command {} delivery: {why}", u.message),
    };
    if !memory.envelopes.contains_key(&u.message) {
        return Some(choose(Intent::CancelCommandDelivery, "MESSAGE_NOT_HELD"));
    }
    let local = o.local.as_ref()?;
    if local.people.contains(&u.recipient) {
        return Some(choose(
            Intent::ShareCommand {
                message: u.message,
                recipient: u.recipient,
            },
            "recipient observed locally",
        ));
    }
    if local.node == u.rendezvous {
        return Some(choose(
            Intent::CancelCommandDelivery,
            "RENDEZVOUS_RECIPIENT_ABSENT",
        ));
    }
    let pause = if matches!(o.action, Action::Queued { .. }) {
        Intent::CancelAction
    } else {
        Intent::Wait
    };
    if person::Needs::from_observation(o).critical_hunger() {
        return Some(choose(pause, "PAUSED_CRITICAL_HUNGER"));
    }
    let Some(route) = policy::coherent::route(o, m, u.rendezvous) else {
        return Some(choose(pause, "PAUSED_NO_KNOWN_ROUTE"));
    };
    if route.starvation >= 24 * HOUR {
        return Some(choose(pause, "PAUSED_PREDICTED_DEATH"));
    }
    Some(choose(
        Intent::Travel {
            edge: route.first,
            target: u.rendezvous,
            food: false,
        },
        "remembered route to fixed rendezvous",
    ))
}
