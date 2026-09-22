//! Pure policy: only observation, personal memory, and a private random stream.
//! No World reference or global stock queries are available here.
use super::{
    Action, EdgeView, FoodView, HOUR, Holder, Location, NodeView, Observation, Owner, SCALE, Time,
    performance,
};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) mod coherent;
pub(crate) mod forecast;
mod intention;
pub(crate) use forecast::{AcceptanceAudit, ForecastMode};
pub(crate) use intention::decide_persistent;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PolicyMemory {
    pub nodes: BTreeMap<usize, NodeView>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    LoadCargo {
        batch: usize,
        quantity: u64,
    },
    Wait,
    Sleep,
    Eat {
        batch: usize,
        quantity: u64,
    },
    Gather {
        batch: usize,
        communal: bool,
    },
    Pickup {
        batch: usize,
        quantity: u64,
    },
    Travel {
        edge: usize,
        target: usize,
        food: bool,
    },
    Claim {
        board: usize,
        quantity: u64,
    },
    Deliver {
        batch: usize,
        quantity: u64,
    },
    CancelJob,
    CancelAction,
    DiscardWaste,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    pub intent: Intent,
    pub reason: String,
}
fn choice(intent: Intent, reason: impl Into<String>) -> Decision {
    Decision {
        intent,
        reason: reason.into(),
    }
}
fn accessible(f: &FoodView, id: usize) -> bool {
    matches!(f.owner, Owner::None | Owner::Commons(0)) || f.owner == Owner::Agent(id)
}
fn random(rng: &mut u64) -> u64 {
    *rng = rng.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *rng;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
#[derive(Clone, Debug)]
pub(crate) struct Route {
    pub(crate) first: usize,
    pub(crate) time: Time,
    pub(crate) starvation: Time,
    pub(crate) nutrition: Time,
    pub(crate) steps: Vec<forecast::TravelStep>,
}
fn after(o: &Observation, elapsed: Time) -> (Time, Time) {
    (
        (o.nutrition - elapsed).max(0),
        if elapsed >= o.nutrition {
            o.starvation + (elapsed - o.nutrition).max(0)
        } else {
            0
        },
    )
}
// Estimate from remembered edge occupancy and current personal state; no future truth.
pub(crate) fn route(o: &Observation, m: &PolicyMemory, target: usize) -> Option<Route> {
    let Location::Node(start) = o.location else {
        return None;
    };
    if start == target {
        return Some(Route {
            first: usize::MAX,
            time: 0,
            starvation: o.starvation,
            nutrition: o.nutrition,
            steps: vec![],
        });
    }
    let mut edges: BTreeMap<usize, (EdgeView, Time)> = BTreeMap::new();
    for v in m.nodes.values() {
        for e in &v.edges {
            if edges.get(&e.id).is_none_or(|(_, t)| *t < v.observed_at) {
                edges.insert(e.id, (e.clone(), v.observed_at));
            }
        }
    }
    let mut dist = BTreeMap::from([(start, (0i64, usize::MAX))]);
    let mut paths = BTreeMap::from([(start, vec![])]);
    let mut visited = BTreeSet::new();
    loop {
        let (&node, &(time, first)) = dist
            .iter()
            .filter(|(n, _)| !visited.contains(*n))
            .min_by_key(|(_, v)| v.0)?;
        if node == target {
            let (nutrition, starvation) = after(o, time);
            return Some(Route {
                first,
                time,
                starvation,
                nutrition,
                steps: paths[&node].clone(),
            });
        }
        visited.insert(node);
        for (e, observed_at) in edges.values() {
            if e.edge.capacity == 0 {
                continue;
            }
            let to = if e.edge.a == node {
                e.edge.b
            } else if e.edge.b == node {
                e.edge.a
            } else {
                continue;
            };
            let (n, _) = after(o, time);
            let f = (o.fatigue + time).min(24 * HOUR);
            let mut remaining = e.edge.nominal_ms as u128 * SCALE;
            let mut trip = 0;
            let mut pn = n;
            let mut pf = f;
            // Piecewise physiological prediction, accounting for speed changes during traversal.
            while remaining > 0 {
                let rate = performance(pn, pf);
                let threshold = super::awake_threshold(pn, pf)
                    .min((remaining.div_ceil(rate)) as Time)
                    .max(1);
                remaining = remaining.saturating_sub(rate * threshold as u128);
                trip += threshold;
                pn = (pn - threshold).max(0);
                pf = (pf + threshold).min(24 * HOUR);
            }
            let already_queued = matches!(o.action,Action::Queued{edge,..} if edge==e.id);
            let ahead = if already_queued {
                e.queued.saturating_sub(1)
            } else {
                e.queued
            };
            // Occupancy predicts a nominal service cycle, not exact remaining time.
            let delay = ((ahead + e.occupied) / e.edge.capacity) as Time * e.edge.nominal_ms;
            let candidate = time.saturating_add(trip).saturating_add(delay);
            let first_edge = if node == start { e.id } else { first };
            if dist.get(&to).is_none_or(|x| candidate < x.0) {
                dist.insert(to, (candidate, first_edge));
                let mut path = paths[&node].clone();
                path.push(forecast::TravelStep {
                    from: node,
                    to,
                    edge: e.id,
                    travel_ms: trip,
                    queue_ms: delay,
                    observed_at: *observed_at,
                    nominal_ms: e.edge.nominal_ms,
                });
                paths.insert(to, path);
            }
        }
    }
}
#[derive(Clone, Debug)]
struct Source {
    node: usize,
    food: FoodView,
    route: Route,
    age: Time,
    confidence: f64,
}
fn sources(o: &Observation, m: &PolicyMemory) -> Vec<Source> {
    let mut out = vec![];
    for v in m.nodes.values() {
        let Some(r) = route(o, m, v.node) else {
            continue;
        };
        for f in &v.food {
            if f.quantity == 0 || !accessible(f, o.agent) {
                continue;
            }
            let work = if matches!(f.holder, Holder::Deposit(_)) {
                (2000 * SCALE).div_ceil(performance(
                    r.nutrition,
                    (o.fatigue + r.time).min(24 * HOUR),
                )) as Time
            } else {
                0
            };
            let time = r.time.saturating_add(work);
            let (nutrition, starvation) = after(o, time);
            if f.expires_at <= o.now + time {
                continue;
            }
            let age = o.now - v.observed_at;
            out.push(Source {
                node: v.node,
                food: f.clone(),
                route: Route {
                    time,
                    nutrition,
                    starvation,
                    ..r.clone()
                },
                age,
                confidence: (-(age as f64) / (12 * HOUR) as f64).exp(),
            });
        }
    }
    out.sort_by_key(|s| {
        (
            s.route.starvation >= 24 * HOUR,
            s.route.time,
            s.age,
            s.node,
            s.food.batch,
        )
    });
    out
}
fn toward(o: &Observation, r: &Route, target: usize, food: bool, why: &str) -> Decision {
    if matches!(o.action,Action::Queued{edge,..} if edge==r.first) {
        return choice(
            Intent::Wait,
            format!(
                "remain FIFO; {why}; predicted_ms={} arrival_starvation_ms={}",
                r.time, r.starvation
            ),
        );
    }
    choice(
        Intent::Travel {
            edge: r.first,
            target,
            food,
        },
        format!(
            "{why}; predicted_ms={} arrival_nutrition_ms={} arrival_starvation_ms={}",
            r.time, r.nutrition, r.starvation
        ),
    )
}
fn eat(o: &Observation) -> Option<Decision> {
    if o.nutrition >= 12 * HOUR {
        return None;
    }
    let mut food = o.own.clone();
    if let Some(local) = &o.local {
        food.extend(
            local
                .food
                .iter()
                .filter(|f| !matches!(f.holder, Holder::Deposit(_)))
                .cloned(),
        );
    }
    food.sort_by_key(|f| (f.expires_at, f.batch));
    for f in food {
        if accessible(&f, o.agent) && f.expires_at > o.now {
            let q = super::goods::headroom(o.nutrition, f.owner).min(f.available);
            if q > 0 {
                return Some(choice(
                    Intent::Eat {
                        batch: f.batch,
                        quantity: q,
                    },
                    "locally accessible food; exact ownership/ration ceiling",
                ));
            }
        }
    }
    None
}
fn acquire(o: &Observation, s: &Source, communal: bool) -> Decision {
    if !matches!(o.location,Location::Node(n) if n==s.node) {
        return toward(
            o,
            &s.route,
            s.node,
            true,
            &format!(
                "seek remembered food; source={} observed_at={} believed_quantity={} expires_at={}",
                s.node,
                o.now - s.age,
                s.food.quantity,
                s.food.expires_at
            ),
        );
    }
    if o.load >= o.capacity {
        return choice(
            Intent::DiscardWaste,
            "carrying capacity blocks acquisition; discard only own-carried waste",
        );
    }
    if s.food.available == 0 {
        return choice(Intent::Wait, "local source fully reserved");
    }
    if matches!(s.food.holder, Holder::Deposit(_)) {
        choice(
            Intent::Gather {
                batch: s.food.batch,
                communal,
            },
            format!("one quantum from observed deposit; communal={communal}"),
        )
    } else if communal {
        choice(
            Intent::LoadCargo {
                batch: s.food.batch,
                quantity: 1,
            },
            "load physical goods for accepted communal job",
        )
    } else if matches!(s.food.owner, Owner::Commons(_)) {
        choice(
            Intent::Wait,
            "communal stock is for local rationing; no personal pickup",
        )
    } else {
        choice(
            Intent::Pickup {
                batch: s.food.batch,
                quantity: 1,
            },
            "local personal/open-access loose food",
        )
    }
}
pub fn decide(o: &Observation, m: &PolicyMemory, rng: &mut u64) -> Decision {
    decide_impl(
        o,
        m,
        rng,
        ForecastMode::Legacy(6 * HOUR),
        &mut AcceptanceAudit::disabled(),
        false,
    )
}
fn decide_impl(
    o: &Observation,
    m: &PolicyMemory,
    rng: &mut u64,
    mode: ForecastMode,
    audit: &mut AcceptanceAudit,
    skip_self_provision: bool,
) -> Decision {
    if matches!(o.action, Action::Dead | Action::Travel) {
        return choice(Intent::Wait, "atomic traversal or dead");
    }
    if let Some(d) = eat(o) {
        return d;
    }
    let known = sources_for(o, m, mode);
    let feasible = known.iter().find(|s| s.route.starvation < 24 * HOUR);
    if o.nutrition <= 6 * HOUR
        && let Some(s) = feasible
    {
        return acquire(o, s, false);
    }
    if o.fatigue >= 22 * HOUR
        || (matches!(o.action, Action::Sleep { .. })
            && o.fatigue > 4 * HOUR
            && o.nutrition > 6 * HOUR)
    {
        return choice(Intent::Sleep, "critical fatigue or ongoing recovery");
    }
    if matches!(o.action, Action::Gather { .. }) {
        return choice(Intent::Wait, "finish one-quantum atomic gathering");
    }
    // A discovered local source provides an opportunity, not a compulsory return home.
    if !skip_self_provision
        && o.nutrition < 12 * HOUR
        && let Some(s) = feasible
    {
        return acquire(o, s, false);
    }
    if let Some(job) = &o.job {
        if let Some(cargo) = o
            .own
            .iter()
            .find(|f| f.owner == Owner::Commons(0) && f.quantity > 0)
        {
            let Some(r) = route_for(o, m, job.destination, mode) else {
                return choice(Intent::CancelJob, "delivery route unavailable in memory");
            };
            if r.starvation >= 24 * HOUR || cargo.expires_at <= o.now + r.time {
                return choice(
                    Intent::CancelJob,
                    "delivery survival-infeasible or cargo predicted spoiled",
                );
            }
            if matches!(o.location,Location::Node(n) if n==job.destination) {
                return choice(
                    Intent::Deliver {
                        batch: cargo.batch,
                        quantity: cargo.quantity,
                    },
                    "physical communal delivery",
                );
            }
            if job.acquired >= job.target || o.load >= o.capacity || known.is_empty() {
                return toward(o, &r, job.destination, false, "deliver accepted job");
            }
        }
        if job.acquired < job.target
            && o.load < o.capacity
            && let Some(s) = known
                .iter()
                .find(|s| s.route.starvation < 24 * HOUR && s.node != job.destination)
        {
            return acquire(o, s, true);
        }
        if !o.own.iter().any(|f| f.owner == Owner::Commons(0)) {
            return choice(
                Intent::CancelJob,
                "accepted job cannot acquire expected cargo",
            );
        }
    }
    // Voluntary jobs require personally known demand and source, with a survival margin.
    if o.communal && o.nutrition > 6 * HOUR && o.job.is_none() && o.load < o.capacity {
        let best = forecast::evaluate(o, m, &known, mode, audit);
        if let Some((score, node, q, r)) = best {
            if matches!(o.location,Location::Node(n) if n==node) {
                return choice(
                    Intent::Claim {
                        board: node,
                        quantity: q,
                    },
                    format!("voluntary known request; score={score:.9}"),
                );
            }
            return toward(
                o,
                &r,
                node,
                false,
                "visit remembered board to claim locally",
            );
        }
    }
    // A personal six-quantum buffer is a disclosed policy heuristic, not a worker quota.
    let personal: u64 = o
        .own
        .iter()
        .filter(|f| f.owner == Owner::Agent(o.agent))
        .map(|f| f.quantity)
        .sum();
    if personal < 6.min(o.capacity)
        && o.load < o.capacity
        && let Some(s) = feasible
    {
        return acquire(o, s, false);
    }
    let needs_food = o.nutrition < 12 * HOUR || personal == 0 || o.job.is_some();
    if needs_food {
        let mut unknown = BTreeSet::new();
        for v in m.nodes.values() {
            for e in &v.edges {
                for n in [e.edge.a, e.edge.b] {
                    if !m.nodes.contains_key(&n) && e.edge.capacity > 0 {
                        unknown.insert(n);
                    }
                }
            }
        }
        let mut options: Vec<_> = unknown
            .into_iter()
            .filter_map(|n| route_for(o, m, n, mode).map(|r| (n, r)))
            .collect();
        options.sort_by_key(|(_, r)| (r.starvation >= 24 * HOUR, r.time));
        if let Some((_, first)) = options.first() {
            let key = (first.starvation >= 24 * HOUR, first.time);
            let equal = options
                .iter()
                .take_while(|(_, r)| (r.starvation >= 24 * HOUR, r.time) == key)
                .count();
            let (target, r) = &options[(random(rng) as usize) % equal];
            return toward(
                o,
                r,
                *target,
                false,
                "explore unknown location; no resource target",
            );
        }
        if let Some(s) = known.first() {
            return acquire(o, s, false);
        }
    }
    choice(
        Intent::Wait,
        "no locally justified work; remaining near resources is allowed",
    )
}

pub(crate) fn goal_is_fatal(o: &Observation, m: &PolicyMemory, target: usize) -> bool {
    route(o, m, target).is_none_or(|r| r.starvation >= 24 * HOUR)
}

/// Research variants remain observation-only. Current/Job are intentionally
/// equivalent: P1 already preserves jobs across ordinary evaluations.
pub fn decide_variant(
    o: &Observation,
    m: &PolicyMemory,
    rng: &mut u64,
    variant: super::DiagnosticPolicy,
) -> Decision {
    use super::DiagnosticPolicy;
    if variant == DiagnosticPolicy::SleepCommitment
        && matches!(o.action, Action::Sleep { .. })
        && o.fatigue > 4 * HOUR
    {
        let critical = o.nutrition <= 6 * HOUR
            && (eat(o).is_some() || sources(o, m).iter().any(|s| s.route.starvation < 24 * HOUR));
        if !critical {
            return choice(Intent::Sleep, "diagnostic coherent sleep recovery");
        }
    }
    if variant == DiagnosticPolicy::QueueCommitment
        && o.job.is_some()
        && matches!(o.action, Action::Queued { .. })
    {
        let critical = o.nutrition <= 6 * HOUR
            && (eat(o).is_some() || sources(o, m).iter().any(|s| s.route.starvation < 24 * HOUR));
        let impossible = o.goal.is_some_and(|(target, food)| {
            goal_is_fatal(o, m, target)
                || (food
                    && m.nodes.get(&target).is_some_and(|v| {
                        !v.food
                            .iter()
                            .any(|f| f.quantity > 0 && f.expires_at > o.now)
                    }))
        });
        let expired_cargo = o.job.as_ref().is_some_and(|j| {
            route(o, m, j.destination).is_some_and(|r| {
                !o.own.is_empty()
                    && o.own
                        .iter()
                        .filter(|f| f.owner == Owner::Commons(0))
                        .any(|f| f.expires_at <= o.now + r.time)
            })
        });
        if !critical && !impossible && !expired_cargo {
            return choice(Intent::Wait, "diagnostic accepted-job FIFO commitment");
        }
    }
    match variant {
        DiagnosticPolicy::CoherentForecast => decide_impl(
            o,
            m,
            rng,
            ForecastMode::Coherent,
            &mut AcceptanceAudit::disabled(),
            false,
        ),
        DiagnosticPolicy::MealAware => decide_impl(
            o,
            m,
            rng,
            ForecastMode::Meals,
            &mut AcceptanceAudit::disabled(),
            false,
        ),
        DiagnosticPolicy::ZeroMargin => decide_impl(
            o,
            m,
            rng,
            ForecastMode::Legacy(0),
            &mut AcceptanceAudit::disabled(),
            false,
        ),
        DiagnosticPolicy::ThreeHourMargin => decide_impl(
            o,
            m,
            rng,
            ForecastMode::Legacy(3 * HOUR),
            &mut AcceptanceAudit::disabled(),
            false,
        ),
        _ => decide(o, m, rng),
    }
}

pub(crate) fn decide_audited(
    o: &Observation,
    m: &PolicyMemory,
    rng: &mut u64,
    variant: super::DiagnosticPolicy,
) -> (Decision, AcceptanceAudit) {
    let mode = match variant {
        super::DiagnosticPolicy::CoherentForecast => ForecastMode::Coherent,
        super::DiagnosticPolicy::MealAware => ForecastMode::Meals,
        super::DiagnosticPolicy::ZeroMargin => ForecastMode::Legacy(0),
        super::DiagnosticPolicy::ThreeHourMargin => ForecastMode::Legacy(3 * HOUR),
        _ => ForecastMode::Legacy(6 * HOUR),
    };
    let mut audit = AcceptanceAudit::new(o, m, mode);
    let d = decide_impl(o, m, rng, mode, &mut audit, false);
    audit.finish(o, &d);
    (d, audit)
}

pub(crate) fn audit_meals(
    o: &Observation,
    m: &PolicyMemory,
    rng: &mut u64,
    corrected: bool,
) -> (Decision, AcceptanceAudit) {
    let mode = if corrected {
        ForecastMode::MealsSelfExcluded
    } else {
        ForecastMode::Meals
    };
    let mut audit = AcceptanceAudit::new(o, m, mode);
    let d = decide_impl(o, m, rng, mode, &mut audit, false);
    audit.finish(o, &d);
    (d, audit)
}

/// Read-only audit of the *existing* voluntary-job gate, using the same route
/// estimator and known sources as decide. Never grants information or changes an intent.
pub(crate) fn audit_job_gate(o: &Observation, m: &PolicyMemory) -> Vec<(String, String)> {
    if !o.communal || o.job.is_some() {
        return vec![];
    }
    let known = sources(o, m);
    let mut out = vec![];
    for view in m.nodes.values() {
        let Some(b) = &view.board else { continue };
        if b.quantity == 0 {
            continue;
        }
        if b.unclaimed(o.now) == 0 {
            out.push((
                "claims_suppress".into(),
                format!(
                    "board={} recorded={} observed={}",
                    b.node, b.quantity, view.observed_at
                ),
            ));
            continue;
        }
        if known.iter().all(|s| s.node == b.node) {
            out.push((
                "no_known_source".into(),
                format!("board={} observed={}", b.node, view.observed_at),
            ));
            continue;
        }
        if o.nutrition <= 6 * HOUR || o.load >= o.capacity {
            continue;
        }
        let Some(to_board) = route(o, m, b.node) else {
            continue;
        };
        for source in &known {
            if source.node == b.node {
                continue;
            }
            let mut predicted = o.clone();
            predicted.location = Location::Node(b.node);
            predicted.action = Action::Idle;
            (predicted.nutrition, predicted.starvation) = after(o, to_board.time);
            predicted.fatigue = (o.fatigue + to_board.time).min(24 * HOUR);
            let Some(outbound) = route(&predicted, m, source.node) else {
                continue;
            };
            let work = ((o.capacity - o.load) as u128 * 2000 * SCALE).div_ceil(performance(
                outbound.nutrition,
                (predicted.fatigue + outbound.time).min(24 * HOUR),
            )) as Time;
            let elapsed = to_board.time + outbound.time + work;
            predicted.location = Location::Node(source.node);
            (predicted.nutrition, predicted.starvation) = after(o, elapsed);
            predicted.fatigue = (o.fatigue + elapsed).min(24 * HOUR);
            let Some(inbound) = route(&predicted, m, b.node) else {
                continue;
            };
            let total = elapsed + inbound.time;
            let (n, s) = after(o, total);
            let gate = if s > 0 || n < 6 * HOUR {
                "nutrition_margin"
            } else if source.food.expires_at <= o.now + total {
                "shelf_life"
            } else {
                "passes_gate"
            };
            out.push((gate.into(),format!("board={} source={} nutrition={} to_board={} outward={} work={} inward={} total={} predicted_nutrition={} predicted_starvation={} expiry={}",b.node,source.node,o.nutrition,to_board.time,outbound.time,work,inbound.time,total,n,s,source.food.expires_at)));
        }
    }
    out
}

fn route_for(
    o: &Observation,
    m: &PolicyMemory,
    target: usize,
    mode: ForecastMode,
) -> Option<Route> {
    if mode.coherent() {
        coherent::route(o, m, target)
    } else {
        route(o, m, target)
    }
}
fn sources_for(o: &Observation, m: &PolicyMemory, mode: ForecastMode) -> Vec<Source> {
    if mode.coherent() {
        coherent::sources(o, m)
    } else {
        sources(o, m)
    }
}
pub(crate) fn audit_coherent(
    o: &Observation,
    m: &PolicyMemory,
    rng: &mut u64,
    meals: bool,
) -> (Decision, AcceptanceAudit) {
    let mode = if meals {
        ForecastMode::CoherentMeals
    } else {
        ForecastMode::Coherent
    };
    let mut a = AcceptanceAudit::new(o, m, mode);
    let d = decide_impl(o, m, rng, mode, &mut a, false);
    a.finish(o, &d);
    (d, a)
}

pub(crate) fn coherent_contract(o: &Observation, m: &PolicyMemory, meals: bool) -> AcceptanceAudit {
    let mode = if meals {
        ForecastMode::CoherentMeals
    } else {
        ForecastMode::Coherent
    };
    let mut a = AcceptanceAudit::new(o, m, mode);
    let known = sources_for(o, m, mode);
    forecast::evaluate(o, m, &known, mode, &mut a);
    a
}
