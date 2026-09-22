//! Interprets the recorded candidate tape using belief data only. No World import.
use super::physics::Body;
use crate::provisioning::policy::forecast::Candidate;
use crate::provisioning::{FoodView, Holder, Location, Observation, Owner, PolicyMemory, Time};
#[derive(Clone, Debug)]
pub enum Step {
    Travel {
        edge: usize,
        to: usize,
        nominal: Time,
        queue: Time,
    },
    Claim {
        board: usize,
        quantity: u64,
    },
    Gather {
        batch: usize,
        quantity: u64,
        deposit: bool,
    },
    Eat {
        quantity: u64,
        owner: Owner,
        expiry: Time,
    },
    Wait(Time),
    Deliver,
}
pub fn value(s: &str, key: &str) -> Option<i64> {
    s.split_whitespace().find_map(|v| {
        v.strip_prefix(&format!("{key}="))
            .and_then(|v| v.trim_end_matches(';').parse().ok())
    })
}
pub fn tape(c: &Candidate, m: &PolicyMemory) -> Option<Vec<Step>> {
    let p = c.plan.as_ref()?;
    if p.steps
        .iter()
        .any(|s| s.starts_with("source_prefilter_only"))
    {
        return None;
    }
    let source = c.source?;
    let source_food = m
        .nodes
        .get(&source)?
        .food
        .iter()
        .find(|f| Some(f.expires_at) == c.expiry)?;
    let legacy = p.steps.iter().any(|s| s.starts_with("gather:"));
    let mut result = vec![];
    let mut claimed = false;
    for s in &p.steps {
        if s.starts_with("to_board:") || s.starts_with("to_source:") || s.starts_with("return:") {
            if legacy && !claimed && !s.starts_with("to_board:") {
                result.push(Step::Claim {
                    board: c.board,
                    quantity: p.acquired,
                });
                claimed = true;
            }
            let route = s.split_whitespace().find(|t| t.contains("->"))?;
            let to = route.split("->").nth(1)?.parse().ok()?;
            result.push(Step::Travel {
                edge: value(s, "edge")? as usize,
                to,
                nominal: value(s, "nominal_ms").unwrap_or_else(|| {
                    let edge = value(s, "edge").unwrap() as usize;
                    m.nodes
                        .values()
                        .flat_map(|v| &v.edges)
                        .find(|e| e.id == edge)
                        .unwrap()
                        .edge
                        .nominal_ms
                }),
                queue: value(s, "queue_ms")?,
            });
        } else if s.starts_with("claim:") {
            result.push(Step::Claim {
                board: c.board,
                quantity: value(s, "target")? as u64,
            });
            claimed = true;
        } else if s.starts_with("gather:") {
            if !claimed {
                result.push(Step::Claim {
                    board: c.board,
                    quantity: p.acquired,
                });
                claimed = true;
            }
            result.push(Step::Gather {
                batch: source_food.batch,
                quantity: p.acquired,
                deposit: matches!(source_food.holder, Holder::Deposit(_)),
            });
        } else if s.starts_with("acquire:") {
            result.push(Step::Gather {
                batch: source_food.batch,
                quantity: 1,
                deposit: matches!(source_food.holder, Holder::Deposit(_)),
            });
        } else if s.starts_with("meal:") {
            let owner = if s.contains("owner=Commons(0)") {
                Owner::Commons(0)
            } else {
                let id = s
                    .split("owner=Agent(")
                    .nth(1)?
                    .split(')')
                    .next()?
                    .parse()
                    .ok()?;
                Owner::Agent(id)
            };
            result.push(Step::Eat {
                quantity: value(s, "q")? as u64,
                owner,
                expiry: value(s, "expiry")?,
            });
        } else if s.starts_with("next_policy:") {
            result.push(Step::Wait(value(s, "wait_ms")?));
        }
    }
    // A failed partial forecast is not evidence of an executable complete plan.
    if p.failure.is_none() {
        result.push(Step::Deliver);
    }
    Some(result)
}
#[derive(Clone, Debug)]
pub struct ResultState {
    pub body: Body,
    pub delivered: u64,
    pub load: u64,
    pub failure: Option<&'static str>,
}
pub fn predict(o: &Observation, m: &PolicyMemory, c: &Candidate, tape: &[Step]) -> ResultState {
    let mut r = ResultState {
        body: Body {
            elapsed: 0,
            nutrition: o.nutrition,
            fatigue: o.fatigue,
            starvation: o.starvation,
        },
        delivered: 0,
        load: o.load,
        failure: None,
    };
    let mut items = o.own.clone();
    let mut node = match o.location {
        Location::Node(n) => n,
        _ => {
            r.failure = Some("AT_EDGE");
            return r;
        }
    };
    let mut claimed = false;
    let mut acquired = 0;
    let mut target = 0;
    let mut available: std::collections::BTreeMap<usize, u64> = m
        .nodes
        .values()
        .flat_map(|v| &v.food)
        .map(|f| (f.batch, f.available))
        .collect();
    for step in tape {
        match *step {
            Step::Travel {
                to, nominal, queue, ..
            } => {
                r.body.elapse(queue, false);
                r.body.work(nominal);
                node = to;
            }
            Step::Wait(dt) => r.body.elapse(dt, false),
            Step::Claim { board, quantity } => {
                if node != board
                    || quantity == 0
                    || quantity > o.capacity - r.load
                    || o.job.is_some()
                    || !o.communal
                {
                    r.failure = Some("CLAIM_PRECONDITION");
                    break;
                }
                let Some(b) = m.nodes.get(&board).and_then(|v| v.board.as_ref()) else {
                    r.failure = Some("NO_BOARD");
                    break;
                };
                if quantity > b.unclaimed(o.now + r.body.elapsed) {
                    r.failure = Some("CLAIM_SUPPRESSED");
                    break;
                }
                claimed = true;
                target = quantity;
            }
            Step::Gather {
                batch,
                quantity,
                deposit,
            } => {
                let Some(f) = m
                    .nodes
                    .get(&node)
                    .and_then(|v| v.food.iter().find(|f| f.batch == batch))
                else {
                    r.failure = Some("NONLOCAL_SOURCE");
                    break;
                };
                for _ in 0..quantity {
                    if !claimed || acquired >= target {
                        r.failure = Some("NO_UNMET_JOB");
                        break;
                    }
                    if r.load >= o.capacity {
                        r.failure = Some("CARRY_CAPACITY");
                        break;
                    }
                    if available.get(&batch).copied().unwrap_or(0) == 0 {
                        r.failure = Some("SOURCE_UNAVAILABLE");
                        break;
                    }
                    if f.owner != Owner::None
                        && f.owner != Owner::Commons(0)
                        && f.owner != Owner::Agent(o.agent)
                    {
                        r.failure = Some("OWNERSHIP");
                        break;
                    }
                    if f.expires_at <= o.now + r.body.elapsed {
                        r.failure = Some("SPOILED");
                        break;
                    }
                    if deposit {
                        r.body.work(2000);
                    }
                    if !r.body.alive() {
                        break;
                    }
                    if f.expires_at <= o.now + r.body.elapsed {
                        r.failure = Some("SPOILED");
                        break;
                    }
                    *available.get_mut(&batch).unwrap() -= 1;
                    acquired += 1;
                    r.load += 1;
                    items.push(FoodView {
                        quantity: 1,
                        available: 1,
                        owner: Owner::Commons(0),
                        holder: Holder::Agent(o.agent),
                        ..f.clone()
                    });
                }
            }
            Step::Eat {
                quantity,
                owner,
                expiry,
            } => {
                let available: u64 = items
                    .iter()
                    .filter(|f| {
                        f.owner == owner
                            && f.expires_at == expiry
                            && expiry > o.now + r.body.elapsed
                    })
                    .map(|f| f.quantity)
                    .sum();
                if available < quantity || !r.body.eat(quantity, owner == Owner::Commons(0)) {
                    r.failure = Some("MEAL_PRECONDITION");
                    break;
                }
                let mut left = quantity;
                for f in &mut items {
                    if f.owner == owner && f.expires_at == expiry {
                        let q = left.min(f.quantity);
                        f.quantity -= q;
                        left -= q;
                    }
                }
                r.load -= quantity;
            }
            Step::Deliver => {
                if !claimed || node != c.board {
                    r.failure = Some("DELIVERY_PRECONDITION");
                    break;
                }
                r.delivered = items
                    .iter()
                    .filter(|f| {
                        f.owner == Owner::Commons(0) && f.expires_at > o.now + r.body.elapsed
                    })
                    .map(|f| f.quantity)
                    .sum();
                r.load -= r.delivered;
                if r.delivered == 0 {
                    r.failure = Some("NO_EDIBLE_CARGO");
                }
            }
        }
        if !r.body.alive() {
            r.failure = Some("DEATH");
        }
        if r.failure.is_some() {
            break;
        }
    }
    r
}
