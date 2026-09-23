//! F11B organization only. Positions, movement and inventories remain on Persons.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesiredFormation {
    GatherOnGraph,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoveOrder {
    pub serial: u64,
    pub target: usize,
    pub issued_at: Time,
    pub issued_at_node: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member {
    pub person: usize,
    pub enlisted_at: Time,
    pub assembled_at: Option<Time>,
    pub ended: Option<(Time, String)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit {
    pub id: usize,
    pub members: BTreeMap<usize, Member>,
    pub muster: usize,
    pub desired_formation: DesiredFormation,
    pub objective: Option<MoveOrder>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct View {
    pub unit: Option<usize>,
    pub active: bool,
    pub order: Option<MoveOrder>,
    pub received_at: Option<Time>,
    pub departed_at: Option<Time>,
    pub arrived_at: Option<Time>,
    pub post_service_arrival: Option<Time>,
    pub phase: String,
}
#[derive(Clone, Debug, Default)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub pause_ms: BTreeMap<String, u128>,
    pub member_queue_ms: u128,
    pub civilian_queue_ms: u128,
    pub member_travel_ms: u128,
    pub civilian_travel_ms: u128,
    pub member_starvation_ms: u128,
    pub civilian_starvation_ms: u128,
    pub member_food: u64,
    pub member_fatigue_ms: u128,
    pub max_fatigue: Time,
    pub member_time_ms: u128,
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
    pub units: BTreeMap<usize, Unit>,
    pub views: Vec<View>,
    pub journal: Journal,
    pub next_order: u64,
}
impl State {
    pub fn new(c: &Config) -> Result<Self, String> {
        if c.formation {
            if c.military.is_none() {
                return Err("unit organization requires military institution".into());
            }
            Ok(Self {
                views: vec![View::default(); c.starts.len()],
                ..Default::default()
            })
        } else {
            Ok(Self::default())
        }
    }
}
/// No unit-wide/current-world data enters choice: only the personally received order.
pub(crate) fn consider(o: &Observation, m: &PolicyMemory, v: &View) -> Option<Decision> {
    if !v.active {
        return None;
    }
    let order = v.order.as_ref()?;
    let choose = |intent, why: &str| Decision {
        intent,
        reason: format!("unit order {}: {why}", order.serial),
    };
    if o.location == Location::Node(order.target) {
        return Some(choose(
            Intent::MilitaryDuty,
            "at personally received objective",
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
    let Some(r) = policy::coherent::route(o, m, order.target) else {
        return Some(choose(pause, "PAUSED_NO_KNOWN_ROUTE"));
    };
    if r.starvation >= 24 * HOUR {
        return Some(choose(pause, "PAUSED_PREDICTED_DEATH"));
    }
    Some(choose(
        Intent::Travel {
            edge: r.first,
            target: order.target,
            food: false,
        },
        "execute remembered movement undertaking",
    ))
}
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub enlisted: usize,
    pub ever_assembled: usize,
    pub active: usize,
    pub at_muster: usize,
    pub moving: usize,
    pub queued: usize,
    pub recovering: usize,
    pub at_objective: usize,
    pub departed: usize,
    pub arrived: usize,
    pub late_endpoint: usize,
    pub released: usize,
    pub dead: usize,
    pub separated: usize,
    pub modal_node: Option<usize>,
    pub modal_fraction: f64,
    pub coincident_fraction: f64,
    pub paused_away: usize,
    pub active_without_order: usize,
    pub objective_fraction: f64,
    pub near_anchor_fraction: f64,
    pub max_endpoint_hops: usize,
    pub arrival_spread: Option<Time>,
}
