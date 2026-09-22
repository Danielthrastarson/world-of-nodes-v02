use super::physics::Body;
use crate::provisioning::*;
fn one(n: Time, f: Time) -> World {
    World::new(Config {
        starts: vec![0],
        autonomous: false,
        initial_nutrition: n,
        initial_fatigue: f,
        food: vec![(0, 20, 0, 48 * HOUR)],
        initial_request: 20,
        ..Config::default()
    })
    .unwrap()
}
fn body(w: &World) -> Body {
    let a = &w.agents[0];
    Body {
        elapsed: w.now,
        nutrition: a.nutrition,
        fatigue: a.fatigue,
        starvation: a.starvation,
    }
}
fn finish(w: &mut World) {
    while !matches!(w.agents[0].action, Action::Idle | Action::Dead) {
        w.advance_to(w.next_time(72 * HOUR)).unwrap();
    }
}
pub fn static_rows() -> Vec<String> {
    let mut rows = vec![];
    for n in [0, HOUR, 6 * HOUR, 6 * HOUR + 1, 12 * HOUR, 24 * HOUR] {
        for f in [0, 12 * HOUR - 1, 12 * HOUR, 16 * HOUR, 20 * HOUR, 24 * HOUR] {
            for (name, nominal, intent) in [
                (
                    "travel",
                    2 * HOUR,
                    Intent::Travel {
                        edge: 0,
                        target: 1,
                        food: false,
                    },
                ),
                (
                    "gather",
                    2000,
                    Intent::Gather {
                        batch: 0,
                        communal: false,
                    },
                ),
            ] {
                let mut w = one(n, f);
                let mut prediction = body(&w);
                prediction.work(nominal);
                w.apply_intents(vec![(0, intent)]).unwrap();
                finish(&mut w);
                let extra = if name == "travel" {
                    w.agents[0].location == Location::Node(1)
                } else {
                    w.goods.load(0) == 1
                        && w.goods.batches.iter().any(|b| {
                            b.quantity == 1
                                && b.owner == Owner::Agent(0)
                                && b.created_at == 0
                                && b.expires_at() == 48 * HOUR
                        })
                };
                rows.push(format!(
                    "{name},{n},{f},{},{},{},{},{},{}",
                    prediction == body(&w) && extra,
                    prediction.elapsed - w.now,
                    prediction.nutrition - w.agents[0].nutrition,
                    prediction.fatigue - w.agents[0].fatigue,
                    0,
                    0
                ));
            }
            let mut w = one(n, f);
            let mut p = body(&w);
            p.sleep_tick();
            w.apply_intents(vec![(0, Intent::Sleep)]).unwrap();
            w.advance_to(MINUTE).unwrap();
            rows.push(format!(
                "sleep,{n},{f},{},{},{},{},0,0",
                p == body(&w),
                p.elapsed - w.now,
                p.nutrition - w.agents[0].nutrition,
                p.fatigue - w.agents[0].fatigue
            ));
        }
    }
    for communal in [false, true] {
        for n in [0, 5 * HOUR, 8 * HOUR, 11 * HOUR + 1, 12 * HOUR] {
            let mut w = one(n, 4 * HOUR);
            let owner = if communal {
                Owner::Commons(0)
            } else {
                Owner::Agent(0)
            };
            let b = w.goods.move_part(0, 10, owner, Holder::Agent(0));
            let q = super::super::goods::headroom(n, owner).min(10);
            let mut p = body(&w);
            let ok = p.eat(q, communal);
            w.apply_intents(vec![(
                0,
                Intent::Eat {
                    batch: b,
                    quantity: q,
                },
            )])
            .unwrap();
            rows.push(format!(
                "eat_{communal},{n},0,{},0,{},0,{},0",
                p == body(&w) && w.goods.load(0) == 10 - if ok { q } else { 0 },
                p.nutrition - w.agents[0].nutrition,
                (10 - if ok { q } else { 0 }) as i64 - w.goods.load(0) as i64
            ));
        }
    }
    let mut w = one(24 * HOUR, 0);
    w.now = 47 * HOUR; // explicit static initial state, not a claimed prior history
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch: 0,
            communal: false,
        },
    )])
    .unwrap();
    finish(&mut w);
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(0) && b.quantity > 0)
        .unwrap()
        .id;
    w.goods.move_part(b, 1, Owner::Agent(0), Holder::Loose(0));
    w.advance_to(48 * HOUR).unwrap();
    rows.push(format!(
        "spoil_split,0,0,{},0,0,0,0,0",
        w.goods
            .batches
            .iter()
            .filter(|b| b.quantity > 0)
            .all(|b| b.material == Material::Waste)
            && w.goods.audit()
    ));
    rows
}
pub fn institutional_rows() -> Vec<String> {
    let mut out = vec![];
    let mut record = |name: &str, ok: bool| out.push(format!("{name},0,0,{ok},0,0,0,0,0"));
    let mut w = one(8 * HOUR, 4 * HOUR);
    w.apply_intents(vec![(
        0,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    )])
    .unwrap();
    record(
        "claim_accept",
        w.boards[0].claims[0].expires_at == 12 * HOUR
            && w.agents[0].job.as_ref().is_some_and(|j| j.target == 10)
            && w.goods.load(0) == 0,
    );
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch: 0,
            communal: true,
        },
    )])
    .unwrap();
    record("reservation", w.reserved(0) == 1 && w.goods.load(0) == 0);
    w.apply_intents(vec![(0, Intent::CancelAction)]).unwrap();
    record(
        "reservation_cancel",
        w.reserved(0) == 0 && w.goods.batches[0].quantity == 20,
    );
    w.apply_intents(vec![(
        0,
        Intent::Gather {
            batch: 0,
            communal: true,
        },
    )])
    .unwrap();
    finish(&mut w);
    let b = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Agent(0) && b.quantity > 0)
        .unwrap()
        .clone();
    record(
        "gather_communal",
        b.owner == Owner::Commons(0)
            && b.quantity == 1
            && b.created_at == 0
            && b.expires_at() == 48 * HOUR,
    );
    w.apply_intents(vec![(
        0,
        Intent::Deliver {
            batch: b.id,
            quantity: 1,
        },
    )])
    .unwrap();
    record(
        "deliver",
        w.metrics.delivered == 1
            && w.agents[0].job.is_none()
            && w.goods.batches.iter().any(|b| {
                b.holder == Holder::Commons(0)
                    && b.quantity == 1
                    && b.owner == Owner::Commons(0)
                    && b.expires_at() == 48 * HOUR
            }),
    );
    let batch = w
        .goods
        .batches
        .iter()
        .find(|b| b.holder == Holder::Commons(0) && b.quantity > 0)
        .unwrap()
        .id;
    let before = w.agents[0].nutrition;
    w.apply_intents(vec![(0, Intent::Eat { batch, quantity: 1 })])
        .unwrap();
    record(
        "withdraw_immediate",
        w.goods.load(0) == 0 && w.agents[0].nutrition == before + HOUR,
    );
    let b = w.goods.move_part(0, 2, Owner::Agent(99), Holder::Loose(0));
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 1,
        },
    )])
    .unwrap();
    record(
        "dead_owner_refusal",
        w.goods.batches[b].quantity == 2 && w.goods.load(0) == 0,
    );
    let b = w.goods.move_part(0, 2, Owner::None, Holder::Loose(0));
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: b,
            quantity: 1,
        },
    )])
    .unwrap();
    record(
        "pickup",
        w.goods.load(0) == 1
            && w.goods.batches.iter().any(|b| {
                b.quantity > 0
                    && b.holder == Holder::Agent(0)
                    && b.owner == Owner::Agent(0)
                    && b.expires_at() == 48 * HOUR
            }),
    );
    let cargo = w.goods.move_part(0, 3, Owner::Agent(0), Holder::Agent(0));
    w.goods.batches[cargo].material = Material::Waste;
    w.apply_intents(vec![(0, Intent::DiscardWaste)]).unwrap();
    record(
        "drop_waste",
        w.goods.load(0) == 0
            && w.goods.batches.iter().any(|b| {
                b.material == Material::Waste
                    && b.quantity == 4
                    && b.owner == Owner::Agent(0)
                    && b.holder == Holder::Loose(0)
            }),
    );
    let mut w = one(8 * HOUR, 4 * HOUR);
    w.apply_intents(vec![(
        0,
        Intent::Claim {
            board: 0,
            quantity: 10,
        },
    )])
    .unwrap();
    w.advance_to(12 * HOUR).unwrap();
    record(
        "lease_expiry",
        w.boards[0].claims.is_empty() && w.agents[0].job.is_some(),
    );
    // Different place, same claim protocol; loading never refreshes food.
    w.agents[0].location = Location::Node(1);
    let b = w.goods.move_part(0, 3, Owner::Commons(0), Holder::Loose(1));
    w.apply_intents(vec![(
        0,
        Intent::LoadCargo {
            batch: b,
            quantity: 3,
        },
    )])
    .unwrap();
    record(
        "load_cargo",
        w.goods.load(0) == 3
            && w.agents[0].job.as_ref().unwrap().acquired == 3
            && w.goods.batches.iter().any(|b| {
                b.holder == Holder::Agent(0) && b.quantity == 3 && b.expires_at() == 48 * HOUR
            }),
    );
    w.audit().unwrap();
    out
}

/// Known delay is identical in both models: isolates ordering, not missing traffic.
pub fn queue_counterexample() -> String {
    let mut w = World::new(Config {
        starts: vec![0; 5],
        autonomous: false,
        initial_nutrition: 24 * HOUR,
        initial_fatigue: 0,
        ..Config::default()
    })
    .unwrap();
    w.agents[4].nutrition = 8 * HOUR;
    w.agents[4].fatigue = 10 * HOUR;
    w.apply_intents(
        (0..4)
            .map(|id| {
                (
                    id,
                    Intent::Travel {
                        edge: 0,
                        target: 1,
                        food: false,
                    },
                )
            })
            .collect(),
    )
    .unwrap();
    let o = w.observe(4).unwrap();
    let mut m = PolicyMemory::default();
    let mut v = o.local.clone().unwrap();
    v.edges[0].queued = 4; // stipulated complete service-cycle evidence in this static fixture
    m.nodes.insert(0, v);
    let forecast = super::super::policy::route(&o, &m, 1).unwrap();
    let mut independent = Body {
        elapsed: 0,
        nutrition: o.nutrition,
        fatigue: o.fatigue,
        starvation: 0,
    };
    independent.elapse(2 * HOUR, false);
    independent.work(2 * HOUR);
    w.apply_intents(vec![(
        4,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    while !matches!(w.agents[4].action, Action::Idle | Action::Dead) {
        w.advance_to(w.next_time(24 * HOUR)).unwrap();
    }
    assert_eq!(independent.elapsed, w.now);
    assert_ne!(forecast.time, w.now);
    format!(
        "queue_before_movement,{},{},{},{},{}",
        forecast.time,
        independent.elapsed,
        w.now,
        forecast.time - w.now,
        forecast.nutrition - w.agents[4].nutrition
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn independent_primitives_match_kernel() {
        for r in super::static_rows()
            .into_iter()
            .chain(super::institutional_rows())
        {
            assert_eq!(r.split(',').nth(3), Some("true"), "{r}");
        }
    }
}
