//! B3 accelerated reproductive mechanics. No sex-linked behavioral coefficients.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seed {
    pub birth_time: Time,
    pub sex: Sex,
    pub drive: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rules {
    pub adult_age: Time,
    pub fertile_until: Time,
    pub gestation: Time,
    pub undertaking: Time,
    pub interval: Time,
    pub conception_per_million: u64,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            adult_age: 24 * HOUR,
            fertile_until: 96 * HOUR,
            gestation: 8 * HOUR,
            undertaking: HOUR,
            interval: 32 * HOUR,
            conception_per_million: 250_000,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub people: Vec<Seed>,
    pub rules: Rules,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pregnancy {
    pub conception_time: Time,
    pub expected_birth_time: Time,
    pub other_parent: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Undertaking {
    pub partner: usize,
    pub started_at: Time,
    pub until: Time,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Biology {
    pub birth_time: Time,
    pub sex: Sex,
    pub drive: bool,
    pub rng: u64,
    pub next_attempt: Time,
    pub pregnancy: Option<Pregnancy>,
    pub undertaking: Option<Undertaking>,
    pub parents: Option<[usize; 2]>,
    pub generation: u32,
    pub matured: bool,
}
impl Biology {
    pub fn age(&self, now: Time) -> Time {
        now - self.birth_time
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct View {
    pub child: bool,
    pub fertile: bool,
    pub motivated: bool,
    pub available_at: Time,
    pub undertaking: Option<Undertaking>,
    pub partners: Vec<usize>,
    pub observed_at: Time,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub adult_ms: u128,
    pub child_ms: u128,
    pub reproduction_ms: u128,
    pub unsupported_ms: u128,
    pub peak: usize,
    pub delivery_delay_ms: u128,
}
impl Journal {
    pub fn count(&self, k: &str) -> u64 {
        self.counts.get(k).copied().unwrap_or(0)
    }
    pub(crate) fn add(&mut self, k: &str) {
        *self.counts.entry(k.into()).or_default() += 1;
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct State {
    pub people: Vec<Biology>,
    pub views: Vec<View>,
    pub consents: BTreeMap<usize, (usize, Time)>,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &super::Config) -> Result<Self, String> {
        let Some(l) = &c.lifecycle else {
            return Ok(Self::default());
        };
        let r = &l.rules;
        if l.people.len() != c.starts.len()
            || l.people.iter().any(|s| s.birth_time > 0)
            || r.adult_age <= 0
            || r.fertile_until <= r.adult_age
            || r.gestation <= 0
            || r.undertaking <= 0
            || r.interval <= r.undertaking
            || r.conception_per_million > 1_000_000
            || c.household.is_none()
        {
            return Err("invalid lifecycle configuration / household structure required".into());
        }
        let people = l
            .people
            .iter()
            .enumerate()
            .map(|(id, s)| Biology {
                birth_time: s.birth_time,
                sex: s.sex,
                drive: s.drive,
                rng: seed_rng(c.seed, id),
                next_attempt: 0,
                pregnancy: None,
                undertaking: None,
                parents: None,
                generation: 1,
                matured: -s.birth_time >= r.adult_age,
            })
            .collect();
        Ok(Self {
            people,
            views: vec![View::default(); l.people.len()],
            journal: Journal {
                peak: l.people.len(),
                ..Journal::default()
            },
            ..Self::default()
        })
    }
    pub fn next_boundary(&self, now: Time, mut next: Time, r: &Rules) -> Time {
        for b in &self.people {
            for t in [
                (!b.matured).then_some(b.birth_time + r.adult_age),
                b.undertaking.as_ref().map(|u| u.until),
                b.pregnancy.as_ref().map(|p| p.expected_birth_time),
            ]
            .into_iter()
            .flatten()
            {
                if t > now {
                    next = next.min(t)
                }
            }
        }
        next
    }
}
pub(crate) fn consider(o: &Observation, v: &View) -> Option<Decision> {
    if v.child
        || !v.fertile
        || !v.motivated
        || o.nutrition < 12 * HOUR
        || o.fatigue >= 16 * HOUR
        || o.job.is_some()
    {
        return None;
    }
    if v.undertaking.is_some() {
        return Some(Decision {
            intent: Intent::Wait,
            reason:
                "continue mutually accepted reproductive undertaking; survival/care may interrupt"
                    .into(),
        });
    }
    if v.available_at > o.now || o.action != Action::Idle {
        return None;
    }
    v.partners.first().map(|&partner|Decision{intent:Intent::Reproduce{partner},reason:"local Partner, own fertility/drive and current reserve permit voluntary undertaking; partner consent unknown".into()})
}

pub(crate) fn seed_rng(seed: u64, id: usize) -> u64 {
    let mut z = seed ^ (id as u64).wrapping_mul(0xd6e8feb86659fd93);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
