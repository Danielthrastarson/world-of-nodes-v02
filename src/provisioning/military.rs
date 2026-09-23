//! Voluntary service over existing Persons. No military physics, property or supplies.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Objective {
    DefendSettlement,
    ProtectCrossing,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub id: usize,
    pub posting: usize,
    pub muster: usize,
    pub attendance: usize,
    pub opens: Time,
    pub expires: Time,
    pub duration: Time,
    pub objective: Objective,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub requests: Vec<Request>,
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seen {
    pub request: Request,
    pub observed_at: Time,
    pub present: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Undertaking {
    pub request: Request,
    pub volunteered_at: Time,
    pub arrived_at: Option<Time>,
    pub until: Option<Time>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pub known: BTreeMap<usize, Seen>,
    pub undertaking: Option<Undertaking>,
    pub attempted: BTreeSet<usize>,
}
#[derive(Clone, Debug, Default)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub people: BTreeMap<String, BTreeSet<usize>>,
    pub active_ms: u128,
    pub duty_ms: u128,
    pub travel_ms: u128,
    pub queue_ms: u128,
    pub soldier_starvation_ms: u128,
    pub civilian_starvation_ms: u128,
    pub food: u64,
    pub cart_travel_ms: u128,
}
impl Journal {
    pub fn add(&mut self, key: &str, person: usize) {
        *self.counts.entry(key.into()).or_default() += 1;
        self.people.entry(key.into()).or_default().insert(person);
    }
    pub fn count(&self, key: &str) -> u64 {
        self.counts.get(key).copied().unwrap_or(0)
    }
}
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub memories: Vec<Memory>,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &super::Config) -> Result<Self, String> {
        if let Some(k) = &c.military {
            let mut ids = BTreeSet::new();
            for r in &k.requests {
                if !ids.insert(r.id)
                    || r.posting >= c.nodes
                    || r.muster >= c.nodes
                    || r.attendance == 0
                    || r.opens < 0
                    || r.expires <= r.opens
                    || r.duration <= 0
                    || r.duration > r.expires - r.opens
                {
                    return Err("invalid muster request".into());
                }
            }
            Ok(Self {
                memories: vec![Memory::default(); c.starts.len()],
                ..Self::default()
            })
        } else {
            Ok(Self::default())
        }
    }
}
/// All inputs are personal observations or remembered local notices. No World.
pub(crate) fn gate(o: &Observation, m: &PolicyMemory, s: &Seen) -> Result<Time, &'static str> {
    let r = &s.request;
    if o.now < r.opens || o.now >= r.expires {
        return Err("REQUEST_EXPIRED");
    }
    if s.present >= r.attendance {
        return Err("OBSERVED_FULL");
    }
    if o.job.is_some() {
        return Err("EXISTING_JOB");
    }
    let route = policy::coherent::route(o, m, r.muster).ok_or("NO_KNOWN_ROUTE")?;
    if route.starvation >= 24 * HOUR {
        return Err("CANNOT_REACH_ALIVE");
    }
    if o.now + route.time + r.duration > r.expires {
        return Err("INSUFFICIENT_TIME");
    }
    if route.nutrition - r.duration < 6 * HOUR {
        return Err("SIX_HOUR_MARGIN");
    }
    if o.fatigue + route.time + r.duration >= 22 * HOUR {
        return Err("FATIGUE");
    }
    Ok(route.time)
}
pub(crate) fn consider(o: &Observation, m: &PolicyMemory, v: &Memory) -> Option<Decision> {
    let d = |intent, reason: &str| Decision {
        intent,
        reason: reason.into(),
    };
    if let Some(u) = &v.undertaking {
        if o.now >= u.request.expires || u.until.is_some_and(|t| o.now >= t) {
            return Some(d(Intent::LeaveService, "service term/request ended"));
        }
        if u.arrived_at.is_some() {
            if o.location != Location::Node(u.request.muster) || o.nutrition <= 6 * HOUR {
                return Some(d(
                    Intent::LeaveService,
                    "service withdrawn: presence/survival",
                ));
            }
            return Some(d(Intent::MilitaryDuty, "voluntary announced service term"));
        }
        let Some(r) = policy::coherent::route(o, m, u.request.muster) else {
            return Some(d(
                Intent::LeaveService,
                "muster route no longer known feasible",
            ));
        };
        if r.nutrition - u.request.duration < 6 * HOUR
            || o.now + r.time + u.request.duration > u.request.expires
        {
            return Some(d(
                Intent::LeaveService,
                "muster forecast no longer within survival/time margin",
            ));
        }
        if o.location == Location::Node(u.request.muster) {
            return Some(d(
                Intent::JoinMuster,
                "physical muster arrival; local admission required",
            ));
        }
        return Some(d(
            Intent::Travel {
                edge: r.first,
                target: u.request.muster,
                food: false,
            },
            "travel for remembered voluntary muster",
        ));
    }
    v.known
        .values()
        .filter(|s| !v.attempted.contains(&s.request.id))
        .find(|s| gate(o, m, s).is_ok())
        .map(|s| {
            d(
                Intent::Volunteer {
                    request: s.request.id,
                },
                "known security request; bounded safe voluntary service opportunity",
            )
        })
}
