//! F12: information is a dated personal observation carried by an embodied Person.
//! Reports are evidence, never commands, permissions, or a live view of remote truth.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;

pub type ReportId = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub id: ReportId,
    pub author: usize,
    pub observation: NodeView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub report: Report,
    pub sender: usize,
    pub received_at: Time,
}

/// A controlled initial purpose in F12, not a new profession or remote recipient tracker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Undertaking {
    pub report: ReportId,
    pub recipient: usize,
    pub rendezvous: usize,
    pub created_at: Time,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pub reports: BTreeMap<ReportId, Receipt>,
    /// Active spatial beliefs imported from reports. A direct local observation
    /// clears this marker without destroying the report/receipt history.
    pub reported_nodes: BTreeMap<usize, ReportId>,
    pub delivery: Option<Undertaking>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handoff {
    pub report: ReportId,
    pub author: usize,
    pub sender: usize,
    pub recipient: usize,
    pub node: usize,
    pub observed_at: Time,
    pub received_at: Time,
    pub spatial_belief_updated: bool,
}

/// Observer-only counters. The policy cannot access this journal or other memories.
#[derive(Clone, Debug, Default)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub handoffs: Vec<Handoff>,
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
    pub next_report: ReportId,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &Config) -> Self {
        Self {
            memories: if c.information {
                vec![Memory::default(); c.starts.len()]
            } else {
                vec![]
            },
            ..Self::default()
        }
    }
}

/// Common Person capability: execute one already accepted, bounded delivery purpose.
/// No World, global mailbox, remote location, or remote viability query is available.
pub(crate) fn consider(o: &Observation, m: &PolicyMemory, memory: &Memory) -> Option<Decision> {
    let u = memory.delivery.as_ref()?;
    let choose = |intent, why: &str| Decision {
        intent,
        reason: format!("report {} delivery: {why}", u.report),
    };
    if !memory.reports.contains_key(&u.report) {
        return Some(choose(Intent::CancelReportDelivery, "REPORT_NOT_HELD"));
    }
    let local = o.local.as_ref()?;
    if local.people.contains(&u.recipient) {
        return Some(choose(
            Intent::ShareReport {
                report: u.report,
                recipient: u.recipient,
            },
            "recipient physically observed here",
        ));
    }
    if local.node == u.rendezvous {
        return Some(choose(
            Intent::CancelReportDelivery,
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
        "follow remembered route to fixed rendezvous",
    ))
}
