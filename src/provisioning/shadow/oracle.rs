//! Research-only kernel witnesses. These never feed information back to prediction.
use super::plan::Step;
use crate::provisioning::policy::forecast::Candidate;
use crate::provisioning::*;
#[derive(Debug)]
pub(crate) struct Outcome {
    pub elapsed: Time,
    pub delivered: u64,
    pub failure: String,
    pub nutrition: Time,
    pub fatigue: Time,
    pub starvation: Time,
}
fn fork(w: &World) -> World {
    // Deliberately do not clone potentially huge audit/trace journals.
    let mut f = World::new(Config {
        autonomous: false,
        trace: false,
        ..w.config.clone()
    })
    .unwrap();
    f.now = w.now;
    f.agents = w.agents.clone();
    f.goods = w.goods.clone();
    f.boards = w.boards.clone();
    f.gather_communal = w.gather_communal.clone();
    f.request_backlog = w.request_backlog;
    f.initialized = true;
    f.config.persistence = None;
    f.config.acceptance_audit = false;
    f
}
fn finish(w: &mut World, id: usize, deadline: Time) {
    while !matches!(w.agents[id].action, Action::Idle | Action::Dead) && w.now < deadline {
        w.advance_to(w.next_time(deadline)).unwrap();
    }
}
pub(crate) fn edge(w: &World, id: usize, edge: usize) -> (Option<Time>, Option<Time>) {
    let mut f = fork(w);
    let start = f.now;
    f.config.trace = true;
    f.apply_intents(vec![(
        id,
        Intent::Travel {
            edge,
            target: match w.agents[id].location {
                Location::Node(n) => {
                    if w.config.edges[edge].a == n {
                        w.config.edges[edge].b
                    } else {
                        w.config.edges[edge].a
                    }
                }
                _ => return (None, None),
            },
            food: false,
        },
    )])
    .unwrap();
    finish(&mut f, id, start + 72 * HOUR);
    let admission = f
        .events
        .iter()
        .find(|e| e.agent == Some(id) && e.kind == "Admitted")
        .map(|e| e.at - start);
    let arrival = f
        .events
        .iter()
        .find(|e| e.agent == Some(id) && e.kind == "Arrived")
        .map(|e| e.at - start);
    (admission, arrival)
}
fn execute(
    mut w: World,
    id: usize,
    c: &Candidate,
    tape: &[Step],
    belief_queue: bool,
    stop_at_death: bool,
) -> Outcome {
    let start = w.now;
    let mut failure = "NONE".to_string();
    w.config.trace = true;
    for step in tape {
        let before = w.events.len();
        match *step {
            Step::Travel {
                edge, to, queue, ..
            } => {
                if belief_queue {
                    let dt = if stop_at_death {
                        queue.min(
                            w.agents[id].nutrition + (24 * HOUR - w.agents[id].starvation).max(0),
                        )
                    } else {
                        queue
                    };
                    w.advance_to(w.now + dt).unwrap();
                    if stop_at_death && w.agents[id].action == Action::Dead {
                        failure = "DEATH".into();
                        break;
                    }
                }
                w.apply_intents(vec![(
                    id,
                    Intent::Travel {
                        edge,
                        target: to,
                        food: false,
                    },
                )])
                .unwrap();
                finish(&mut w, id, start + 144 * HOUR);
            }
            Step::Wait(dt) => {
                let dt = if stop_at_death {
                    dt.min(w.agents[id].nutrition + (24 * HOUR - w.agents[id].starvation).max(0))
                } else {
                    dt
                };
                w.advance_to(w.now + dt).unwrap();
            }
            Step::Claim { board, quantity } => {
                w.apply_intents(vec![(id, Intent::Claim { board, quantity })])
                    .unwrap();
            }
            Step::Gather {
                batch,
                quantity,
                deposit,
            } => {
                for _ in 0..quantity {
                    let intent = if deposit {
                        Intent::Gather {
                            batch,
                            communal: true,
                        }
                    } else {
                        Intent::LoadCargo { batch, quantity: 1 }
                    };
                    w.apply_intents(vec![(id, intent)]).unwrap();
                    finish(&mut w, id, start + 144 * HOUR);
                    if w.events[before..]
                        .iter()
                        .any(|e| e.agent == Some(id) && e.kind == "Rejected")
                    {
                        break;
                    }
                }
            }
            Step::Eat {
                quantity,
                owner,
                expiry,
            } => {
                let food = w
                    .goods
                    .batches
                    .iter()
                    .find(|b| {
                        b.holder == Holder::Agent(id)
                            && b.owner == owner
                            && b.expires_at() == expiry
                            && b.quantity >= quantity
                            && b.material == Material::Food
                    })
                    .map(|b| b.id);
                let Some(batch) = food else {
                    failure = "MEAL_UNAVAILABLE".into();
                    break;
                };
                w.apply_intents(vec![(id, Intent::Eat { batch, quantity })])
                    .unwrap();
            }
            Step::Deliver => {
                let cargo = w
                    .goods
                    .batches
                    .iter()
                    .find(|b| {
                        b.holder == Holder::Agent(id)
                            && b.owner == Owner::Commons(0)
                            && b.quantity > 0
                            && b.material == Material::Food
                    })
                    .cloned();
                let Some(b) = cargo else {
                    failure = "NO_EDIBLE_CARGO".into();
                    break;
                };
                w.apply_intents(vec![(
                    id,
                    Intent::Deliver {
                        batch: b.id,
                        quantity: b.quantity,
                    },
                )])
                .unwrap();
            }
        }
        if let Some(e) = w.events[before..]
            .iter()
            .find(|e| e.agent == Some(id) && e.kind == "Rejected")
        {
            failure = e.detail.replace(',', ";");
            break;
        }
        if w.agents[id].action == Action::Dead {
            failure = "DEATH".into();
            break;
        }
    }
    if failure == "NONE" && !matches!(w.agents[id].location,Location::Node(n)if n==c.board) {
        failure = "NOT_AT_DESTINATION".into();
    }
    w.audit().unwrap();
    Outcome {
        elapsed: w.now - start,
        delivered: w.metrics.delivered,
        failure,
        nutrition: w.agents[id].nutrition,
        fatigue: w.agents[id].fatigue,
        starvation: w.agents[id].starvation,
    }
}
pub(crate) fn truth(w: &World, id: usize, c: &Candidate, tape: &[Step]) -> Outcome {
    execute(fork(w), id, c, tape, false, false)
}
pub(crate) fn belief(o: &Observation, m: &PolicyMemory, c: &Candidate, tape: &[Step]) -> Outcome {
    belief_impl(o, m, c, tape, false)
}
pub(crate) fn belief_endpoint(
    o: &Observation,
    m: &PolicyMemory,
    c: &Candidate,
    tape: &[Step],
) -> Outcome {
    belief_impl(o, m, c, tape, true)
}
fn belief_impl(
    o: &Observation,
    m: &PolicyMemory,
    c: &Candidate,
    tape: &[Step],
    stop_at_death: bool,
) -> Outcome {
    let max_node = m
        .nodes
        .values()
        .flat_map(|v| v.edges.iter().flat_map(|e| [e.edge.a, e.edge.b]))
        .max()
        .unwrap_or(0);
    let max_edge = m
        .nodes
        .values()
        .flat_map(|v| v.edges.iter().map(|e| e.id))
        .max()
        .unwrap_or(0);
    let mut edges = vec![
        Edge {
            a: 0,
            b: 1,
            nominal_ms: HOUR,
            capacity: 0
        };
        max_edge + 1
    ];
    for v in m.nodes.values() {
        for e in &v.edges {
            edges[e.id] = e.edge.clone();
        }
    }
    let node = if let Location::Node(n) = o.location {
        n
    } else {
        0
    };
    let mut w = World::new(Config {
        nodes: (max_node + 1).max(2),
        starts: vec![node; o.agent + 1],
        edges,
        food: vec![],
        boards: vec![],
        communal: o.communal,
        carrying_capacity: o.capacity,
        autonomous: false,
        ..Config::default()
    })
    .unwrap();
    w.now = o.now;
    w.initialized = true;
    for a in &mut w.agents {
        a.action = Action::Dead;
    }
    let a = &mut w.agents[o.agent];
    a.action = Action::Idle;
    a.nutrition = o.nutrition;
    a.fatigue = o.fatigue;
    a.starvation = o.starvation;
    a.job = o.job.clone();
    w.boards = m.nodes.values().filter_map(|v| v.board.clone()).collect();
    let mut foods: BTreeMap<usize, FoodView> = m
        .nodes
        .values()
        .flat_map(|v| v.food.iter().map(|f| (f.batch, f.clone())))
        .collect();
    for f in &o.own {
        foods.insert(f.batch, f.clone());
    }
    let max = foods.keys().max().copied().unwrap_or(0);
    for id in 0..=max {
        if let Some(f) = foods.get(&id) {
            // Frozen hypothesis: only observed unreserved food is obtainable. This
            // removes unavailable quantity, not a claim about actual world material.
            w.goods.add(
                Material::Food,
                if f.holder == Holder::Agent(o.agent) {
                    f.quantity
                } else {
                    f.available
                },
                f.owner,
                f.holder,
                f.expires_at - 48 * HOUR,
                48 * HOUR,
            );
        } else {
            w.goods.add(
                Material::Food,
                0,
                Owner::None,
                Holder::Loose(node),
                0,
                48 * HOUR,
            );
        }
    }
    let own: u64 = o.own.iter().map(|f| f.quantity).sum();
    if o.load > own {
        w.goods.add(
            Material::Waste,
            o.load - own,
            Owner::Agent(o.agent),
            Holder::Agent(o.agent),
            0,
            48 * HOUR,
        );
    }
    // Explicit absolute deadlines. Expired remembered batches are waste before actions.
    for b in &mut w.goods.batches {
        if b.expires_at() <= o.now {
            b.material = Material::Waste;
        }
    }
    execute(w, o.agent, c, tape, true, stop_at_death)
}
