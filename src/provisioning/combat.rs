//! F11C opt-in, graph-local contact. Bodies belong to Persons; no unit HP.
use super::*;
mod adapter;
pub mod research;
#[cfg(test)]
mod tests;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raid {
    pub person: usize,
    pub target: usize,
    pub home: usize,
    pub begins: Time,
    pub quota: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub nodes: Vec<usize>,
    pub raids: Vec<Raid>,
    pub strike_work: Time,
    pub theft_work: Time,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Body {
    pub injuries: u8,
}
impl Body {
    pub fn factor(&self) -> u128 {
        match self.injuries {
            0 => SCALE,
            1 => SCALE / 2,
            _ => 0,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct View {
    pub injuries: u8,
    pub raid: Option<Raid>,
    pub stolen: u64,
    pub protect: Option<usize>,
    pub serving: bool,
    pub enemies: Vec<usize>,
}
#[derive(Clone, Debug, Default)]
pub struct Journal {
    pub counts: BTreeMap<String, u64>,
    pub engaged: BTreeSet<usize>,
    pub first_contact: Option<Time>,
    pub last_contact: Option<Time>,
    pub combat_ms: u128,
    pub theft_ms: u128,
    pub stolen: u64,
}
impl Journal {
    pub fn count(&self, k: &str) -> u64 {
        self.counts.get(k).copied().unwrap_or(0)
    }
    pub fn add(&mut self, k: &str) {
        *self.counts.entry(k.into()).or_default() += 1;
    }
}
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub bodies: Vec<Body>,
    pub views: Vec<View>,
    pub journal: Journal,
}
impl State {
    pub fn new(c: &super::Config) -> Result<Self, String> {
        let Some(cfg) = &c.combat else {
            return Ok(Self::default());
        };
        if cfg.strike_work <= 0
            || cfg.theft_work <= 0
            || cfg.nodes.iter().any(|n| *n >= c.nodes)
            || cfg.raids.iter().any(|r| {
                r.person >= c.starts.len()
                    || r.target >= c.nodes
                    || r.home >= c.nodes
                    || r.quota == 0
            })
            || cfg
                .raids
                .iter()
                .map(|r| r.person)
                .collect::<BTreeSet<_>>()
                .len()
                != cfg.raids.len()
        {
            return Err("invalid contact config".into());
        }
        let mut views = vec![View::default(); c.starts.len()];
        for r in &cfg.raids {
            views[r.person].raid = Some(r.clone());
        }
        Ok(Self {
            bodies: vec![Body::default(); c.starts.len()],
            views,
            ..Default::default()
        })
    }
}
pub(crate) fn consider(
    o: &Observation,
    m: &PolicyMemory,
    v: &View,
    withdrawing: bool,
) -> Option<Decision> {
    let choose = |intent, reason: &str| Decision {
        intent,
        reason: format!("contact: {reason}"),
    };
    if v.injuries >= 2 {
        return Some(choose(
            Intent::Wait,
            "incapacitated; care and accessible eating remain physical",
        ));
    }
    // A personally received withdrawal lets the ordinary formation undertaking
    // attempt movement. It does not erase hostile contact or prevent enemy hits.
    if withdrawing && v.serving && v.raid.is_none() {
        return None;
    }
    let active =
        v.raid.as_ref().is_some_and(|r| o.now >= r.begins) || (v.protect.is_some() && v.serving);
    if active && !v.enemies.is_empty() {
        if matches!(o.action,Action::Strike{target,..} if v.enemies.contains(&target)) {
            return Some(choose(Intent::Wait, "continue timed local strike"));
        }
        return Some(choose(
            Intent::Strike {
                target: v.enemies[o.agent % v.enemies.len()],
            },
            "locally observed opposing capable person",
        ));
    }
    if let Some(r) = &v.raid {
        if o.now < r.begins {
            return Some(choose(Intent::Wait, "seeded raid undertaking not yet due"));
        }
        let target = if v.stolen >= r.quota || o.load >= o.capacity {
            r.home
        } else {
            r.target
        };
        if o.location == Location::Node(target) {
            if target == r.home && (v.stolen >= r.quota || o.load >= o.capacity) {
                return Some(choose(
                    Intent::Wait,
                    "raid complete; physical cargo remains here",
                ));
            }
            if let Action::RaidStock { .. } = o.action {
                return Some(choose(Intent::Wait, "continue timed theft"));
            }
            if let Some(f) = o.local.as_ref().and_then(|n| {
                n.food.iter().find(|f| {
                    f.owner == Owner::Commons(0)
                        && matches!(f.holder,Holder::Commons(x) if x==r.target)
                        && f.quantity > 0
                        && f.expires_at > o.now
                })
            }) {
                return Some(choose(
                    Intent::RaidStock { batch: f.batch },
                    "local stock; one quantum; timed unlawful custody transfer",
                ));
            }
            return Some(choose(Intent::Wait, "no locally edible target stock"));
        }
        if let Some(route) = policy::coherent::route(o, m, target)
            && route.starvation < 24 * HOUR
        {
            return Some(choose(
                Intent::Travel {
                    edge: route.first,
                    target,
                    food: false,
                },
                "seeded objective through personally known route",
            ));
        }
        return Some(choose(
            Intent::Wait,
            "unknown or survival-infeasible raid route",
        ));
    }
    None
}
