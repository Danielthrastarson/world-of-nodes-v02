//! B1 structural kinship and voluntary directed care. No household inventory/brain.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Household {
    pub id: usize,
    pub members: Vec<usize>,
    pub home: Option<usize>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kin {
    Parent,
    Child,
    Partner,
    Sibling,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub from: usize,
    pub to: usize,
    pub kind: Kin,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Caregiver {
    pub person: usize,
    pub dependent: usize,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub households: Vec<Household>,
    pub kin: Vec<Relation>,
    pub care: Vec<Caregiver>,
    /// Research control retains the same social structure without care motivation.
    pub enabled: bool,
}
impl Config {
    pub fn relations(&self, person: usize) -> Vec<Relation> {
        let mut out = BTreeSet::new();
        for r in &self.kin {
            if r.from == person {
                out.insert((r.to, r.kind));
            }
            if r.to == person {
                out.insert((
                    r.from,
                    match r.kind {
                        Kin::Parent => Kin::Child,
                        Kin::Child => Kin::Parent,
                        k => k,
                    },
                ));
            }
        }
        out.into_iter()
            .map(|(to, kind)| Relation {
                from: person,
                to,
                kind,
            })
            .collect()
    }
    fn validate(&self, people: usize, nodes: usize) -> Result<(), String> {
        let mut members = BTreeSet::new();
        let mut households = BTreeSet::new();
        for h in &self.households {
            if !households.insert(h.id)
                || h.home.is_some_and(|n| n >= nodes)
                || h.members.iter().any(|&a| a >= people || !members.insert(a))
            {
                return Err("invalid/duplicate household membership".into());
            }
        }
        if self
            .kin
            .iter()
            .any(|r| r.from >= people || r.to >= people || r.from == r.to)
            || self
                .care
                .iter()
                .any(|r| r.person >= people || r.dependent >= people || r.person == r.dependent)
            || self
                .care
                .iter()
                .map(|r| (r.person, r.dependent))
                .collect::<BTreeSet<_>>()
                .len()
                != self.care.len()
        {
            return Err("invalid kinship/care relation".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sighting {
    pub person: usize,
    pub node: usize,
    pub observed_at: Time,
    pub nutrition: Time,
    pub starvation: Time,
    pub present: bool,
    pub alive: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Undertaking {
    pub dependent: usize,
    pub created_at: Time,
    pub paused: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pub sightings: BTreeMap<usize, Sighting>,
    pub undertaking: Option<Undertaking>,
}
#[derive(Clone, Debug)]
pub struct PersonalView<'a> {
    pub households: Vec<&'a Household>,
    pub kin: Vec<Relation>,
    pub dependents: Vec<usize>,
    pub memory: &'a Memory,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub starvation_ms: BTreeMap<usize, u128>,
    pub activity_ms: BTreeMap<(usize, String), u128>,
    pub care_travel_ms: u128,
    pub care_queue_ms: u128,
    pub separation_ms: u128,
    pub losses: BTreeSet<(usize, usize)>,
    pub residence_changes: u64,
    last_nodes: BTreeMap<usize, usize>,
}
impl Journal {
    pub fn count(&self, key: &str) -> u64 {
        self.counts.get(key).copied().unwrap_or(0)
    }
    pub(crate) fn add(&mut self, key: &str, q: u64) {
        *self.counts.entry(key.into()).or_default() += q;
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub memories: Vec<Memory>,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &super::Config) -> Result<Self, String> {
        let Some(h) = &c.household else {
            return Ok(Self::default());
        };
        h.validate(c.starts.len(), c.nodes)?;
        Ok(Self {
            memories: vec![Memory::default(); c.starts.len()],
            ..Self::default()
        })
    }
}

/// Belief-only choice. No World, foreign inventory or live remote person state.
pub(crate) fn consider(o: &Observation, m: &PolicyMemory, social: &Memory) -> Option<Decision> {
    evaluate(o, m, social).ok()
}
pub(crate) fn evaluate(
    o: &Observation,
    m: &PolicyMemory,
    social: &Memory,
) -> Result<Decision, &'static str> {
    if o.job.is_some() {
        return Err("EXISTING_JOB");
    }
    let mut seen: Vec<_> = social
        .sightings
        .values()
        .filter(|s| s.present && s.alive && s.nutrition <= 6 * HOUR)
        .collect();
    seen.sort_by_key(|s| (s.nutrition, s.observed_at, s.person));
    if seen.is_empty() {
        return Err("NO_OBSERVED_SEVERE_NEED");
    }
    let mut why = "NO_PERSONAL_CARGO";
    for s in seen {
        if o.now - s.observed_at > 12 * HOUR {
            why = "STALE_NEED";
            continue;
        }
        let Some(r) = policy::coherent::route(o, m, s.node) else {
            why = "NO_KNOWN_ROUTE";
            continue;
        };
        if r.nutrition < 6 * HOUR {
            why = "SELF_SURVIVAL_BUFFER";
            continue;
        }
        if s.starvation + (o.now + r.time - s.observed_at - s.nutrition).max(0) >= 24 * HOUR {
            why = "DEPENDENT_PREDICTED_DEAD";
            continue;
        }
        let local_owned = o
            .local
            .as_ref()
            .filter(|v| v.node == s.node)
            .map_or(0, |v| {
                v.food
                    .iter()
                    .filter(|f| {
                        f.owner == Owner::Agent(s.person)
                            && f.expires_at > o.now
                            && !matches!(f.holder, Holder::Deposit(_))
                    })
                    .map(|f| f.quantity)
                    .sum::<u64>()
            });
        let need = ((12 * HOUR - s.nutrition) / HOUR) as u64;
        if local_owned >= need {
            why = "LOCAL_FOOD_ALREADY_PROVIDED";
            continue;
        }
        let mut food: Vec<_> = o
            .own
            .iter()
            .filter(|f| f.owner == Owner::Agent(o.agent) && f.quantity > 0)
            .collect();
        food.sort_by_key(|f| (f.expires_at, f.batch));
        for f in food {
            if f.expires_at <= o.now + r.time {
                why = "CARGO_PREDICTED_SPOILED";
                continue;
            }
            let intent = if o.location == Location::Node(s.node) {
                Intent::GiveCareFood {
                    dependent: s.person,
                    batch: f.batch,
                    quantity: f.quantity.min(6).min(need - local_owned),
                }
            } else {
                Intent::TravelForCare {
                    dependent: s.person,
                    edge: r.first,
                    target: s.node,
                }
            };
            return Ok(Decision {
                intent,
                reason: format!(
                    "directed care; dependent={} observed_at={} observed_nutrition={} target={} predicted_travel={} own_arrival_nutrition={}",
                    s.person, s.observed_at, s.nutrition, s.node, r.time, r.nutrition
                ),
            });
        }
    }
    Err(why)
}
