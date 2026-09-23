//! B2 sparse directed history and lazy affect. Appraisal uses witnessed evidence only.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub influence: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self { influence: true }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Relationship {
    pub trust: i32,
    pub attachment: i32,
    pub grievance: i32,
    pub obligation: i32,
    pub last_update: Time,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pulse {
    pub height: Time,
    pub at: Time,
}
impl Pulse {
    pub fn remaining(&self, now: Time) -> Time {
        (self.height - (now - self.at).max(0)).max(0)
    }
    pub fn value(&self, now: Time) -> i32 {
        (self.remaining(now) / HOUR) as i32
    }
    fn raise(&mut self, amount: i32, now: Time) {
        if amount > 0 {
            self.height = (self.remaining(now) + amount as Time * HOUR).min(100 * HOUR);
            self.at = now;
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Affect {
    pub anger: Pulse,
    pub fear: Pulse,
    pub grief: Pulse,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pub relationships: BTreeMap<usize, Relationship>,
    pub affect: Affect,
    pub learned_deaths: BTreeSet<usize>,
    pub in_danger: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Occurrence {
    Support {
        quantity: u64,
        giver_nutrition: Time,
    },
    UnmetExpectation {
        quantity: u64,
        alternative_believed: bool,
    },
    UnavailableAttempt,
    DeathLearned,
    OwnStarvationDanger,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evidence {
    pub at: Time,
    pub observer: usize,
    pub actor: usize,
    pub node: usize,
    pub source_event: u64,
    pub occurrence: Occurrence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Appraisal {
    pub evidence: Evidence,
    pub before: Relationship,
    pub after: Relationship,
    pub affect_before: Affect,
    pub affect_after: Affect,
}
/// No kernel query, no access to an actor's private goals or alternatives.
pub fn appraise(m: &mut Memory, e: Evidence) -> Appraisal {
    let before = m.relationships.get(&e.actor).cloned().unwrap_or_default();
    let affect_before = m.affect.clone();
    let mut after = before.clone();
    match e.occurrence {
        Occurrence::Support {
            quantity,
            giver_nutrition,
        } => {
            let multiplier = if giver_nutrition <= 6 * HOUR {
                3
            } else if giver_nutrition <= 12 * HOUR {
                2
            } else {
                1
            };
            let strength = (quantity.min(100) as i32 * multiplier).min(100);
            after.trust += strength;
            after.attachment += (strength + 1) / 2;
            after.obligation += strength;
        }
        Occurrence::UnmetExpectation {
            quantity,
            alternative_believed: true,
        } => {
            let loss = (quantity.min(100) as i32 * (1 + before.trust.max(0) / 20)).min(100);
            after.trust -= loss;
            after.grievance += loss;
            m.affect.anger.raise(2 * loss, e.at);
        }
        Occurrence::UnmetExpectation {
            alternative_believed: false,
            ..
        }
        | Occurrence::UnavailableAttempt => {
            after.trust -= 1;
        }
        Occurrence::DeathLearned => {
            if m.learned_deaths.insert(e.actor) {
                m.affect.grief.raise(before.attachment / 2, e.at);
            }
        }
        Occurrence::OwnStarvationDanger => m.affect.fear.raise(12, e.at),
    }
    after.trust = after.trust.clamp(-100, 100);
    after.attachment = after.attachment.clamp(0, 100);
    after.grievance = after.grievance.clamp(0, 100);
    after.obligation = after.obligation.clamp(0, 100);
    after.last_update = e.at;
    if e.actor != e.observer {
        m.relationships.insert(e.actor, after.clone());
    }
    Appraisal {
        evidence: e,
        before,
        after,
        affect_before,
        affect_after: m.affect.clone(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assessment {
    pub dependent: usize,
    pub need: i32,
    pub trust: i32,
    pub attachment: i32,
    pub obligation: i32,
    pub food: i32,
    pub travel: i32,
    pub grievance: i32,
    pub anger: i32,
    pub fear: i32,
    pub grief: i32,
}
impl Assessment {
    pub fn pressure(&self) -> i32 {
        self.need + self.trust + self.attachment + self.obligation
    }
    pub fn cost(&self) -> i32 {
        self.food + self.travel + self.grievance + self.anger + self.fear + self.grief
    }
    pub fn permits(&self) -> bool {
        self.pressure() > self.cost()
    }
}
pub(crate) fn assess(
    o: &Observation,
    m: &PolicyMemory,
    care: &household::Memory,
    social: &Memory,
    d: &Decision,
) -> Option<Assessment> {
    let (dependent, quantity, travel) = match d.intent {
        Intent::GiveCareFood {
            dependent,
            quantity,
            ..
        } => (dependent, quantity, 0),
        Intent::TravelForCare {
            dependent, target, ..
        } => {
            let r = policy::coherent::route(o, m, target)?;
            let q = o
                .own
                .iter()
                .filter(|f| f.owner == Owner::Agent(o.agent) && f.expires_at > o.now + r.time)
                .min_by_key(|f| (f.expires_at, f.batch))?
                .quantity
                .min(6);
            (dependent, q, ((r.time + HOUR - 1) / HOUR) as i32)
        }
        _ => return None,
    };
    let s = care.sightings.get(&dependent)?;
    let r = social
        .relationships
        .get(&dependent)
        .cloned()
        .unwrap_or_default();
    Some(Assessment {
        dependent,
        need: ((12 * HOUR - s.nutrition) / (2 * HOUR)) as i32,
        trust: r.trust / 6,
        attachment: r.attachment / 10,
        obligation: r.obligation / 6,
        food: quantity as i32,
        travel,
        grievance: r.grievance / 6,
        anger: social.affect.anger.value(o.now) / 6,
        fear: social.affect.fear.value(o.now) / 6,
        grief: social.affect.grief.value(o.now) / 6,
    })
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Journal {
    pub appraisals: Vec<Appraisal>,
    pub counts: BTreeMap<String, u64>,
}
impl Journal {
    pub fn count(&self, key: &str) -> u64 {
        self.counts.get(key).copied().unwrap_or(0)
    }
    pub(crate) fn add(&mut self, key: &str) {
        *self.counts.entry(key.into()).or_default() += 1;
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub memories: Vec<Memory>,
    /// Sparse inverse indexes, not per-tick pair enumeration.
    pub known_by: BTreeMap<usize, BTreeSet<usize>>,
    pub expected_by: BTreeMap<usize, BTreeSet<usize>>,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &super::Config) -> Self {
        if c.social.is_none() {
            return Self::default();
        }
        let mut s = Self {
            memories: vec![Memory::default(); c.starts.len()],
            ..Self::default()
        };
        if let Some(h) = &c.household {
            for r in &h.kin {
                s.connect(r.from, r.to);
                s.connect(r.to, r.from);
            }
            for r in &h.care {
                s.connect(r.person, r.dependent);
                s.connect(r.dependent, r.person);
                s.expected_by
                    .entry(r.person)
                    .or_default()
                    .insert(r.dependent);
            }
        }
        s
    }
    fn connect(&mut self, observer: usize, actor: usize) {
        self.memories[observer]
            .relationships
            .entry(actor)
            .or_insert(Relationship {
                attachment: 20,
                ..Relationship::default()
            });
        self.known_by.entry(actor).or_default().insert(observer);
    }
}
