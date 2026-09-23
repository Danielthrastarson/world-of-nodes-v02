//! Research-only matched scheduling control. Never called by autonomous agents.
use std::{
    env, fs,
    io::{BufWriter, Write},
};
use world_of_nodes::provisioning::*;
fn held(w: &World, id: usize, b: &Batch) -> bool {
    b.holder == Holder::Agent(id)
        || matches!(b.holder,Holder::Cart(c) if w.carts()[c].user==Some(id))
}
fn edible_quanta(nutrition: Time, communal: bool, quantity: u64) -> u64 {
    let ceiling = if communal { 12 * HOUR } else { 24 * HOUR };
    (((ceiling - nutrition).max(0) / HOUR) as u64).min(quantity)
}
fn run(edge: Time, demand: u64, cart: bool) -> Result<World, String> {
    let mut w = World::new(Config {
        autonomous: false,
        initial_request: demand,
        capital: Some(capital::Config {
            rules: capital::Rules {
                enabled: cart,
                ..capital::Rules::default()
            },
            wood: vec![(0, 500)],
        }),
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: edge,
                capacity: 4,
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: edge,
                capacity: 4,
            },
        ],
        ..Config::default()
    })?;
    let mut recovering = false;
    for tick in 0..=48 * 1800 {
        w.advance_to(tick * 2 * SECOND)?;
        if tick == 48 * 1800 {
            break;
        }
        let a = &w.agents()[0];
        if matches!(
            a.action,
            Action::Dead | Action::Travel | Action::Gather { .. } | Action::BuildCart { .. }
        ) {
            continue;
        }
        let Location::Node(node) = a.location else {
            unreachable!()
        };
        let edible = w.batches().iter().find(|b| {
            b.material == Material::Food
                && b.quantity > 0
                && b.expires_at() > w.now()
                && edible_quanta(a.nutrition, b.owner == Owner::Commons(0), b.quantity) > 0
                && ((held(&w, 0, b)) || b.holder == Holder::Commons(node))
        });
        let intent = if a.nutrition < 12 * HOUR
            && let Some(b) = edible
        {
            Intent::Eat {
                batch: b.id,
                quantity: edible_quanta(a.nutrition, b.owner == Owner::Commons(0), b.quantity),
            }
        } else if a.fatigue >= 22 * HOUR || (recovering && a.fatigue > 4 * HOUR) {
            recovering = true;
            Intent::Sleep
        } else {
            recovering = false;
            if cart && w.carts().is_empty() {
                let wood: u64 = w
                    .batches()
                    .iter()
                    .filter(|b| {
                        b.material == Material::Wood && b.owner == Owner::Agent(0) && held(&w, 0, b)
                    })
                    .map(|b| b.quantity)
                    .sum();
                if wood < 10 {
                    let b = w
                        .batches()
                        .iter()
                        .find(|b| {
                            b.material == Material::Wood
                                && b.holder == Holder::Deposit(0)
                                && b.quantity > 0
                        })
                        .unwrap();
                    Intent::Gather {
                        batch: b.id,
                        communal: false,
                    }
                } else {
                    Intent::BuildCart
                }
            } else if cart && w.carts()[0].user.is_none() {
                Intent::HitchCart { cart: 0 }
            } else if let Some(j) = &a.job {
                let cargo = w.batches().iter().find(|b| {
                    held(&w, 0, b)
                        && b.owner == Owner::Commons(0)
                        && b.material == Material::Food
                        && b.quantity > 0
                });
                if node == 0 && j.acquired > 0 {
                    if let Some(b) = cargo {
                        Intent::Deliver {
                            batch: b.id,
                            quantity: b.quantity,
                        }
                    } else {
                        Intent::CancelJob
                    }
                } else if node == 2 && j.acquired < j.target {
                    if let Some(b) = w.batches().iter().find(|b| {
                        b.holder == Holder::Deposit(2)
                            && b.material == Material::Food
                            && b.quantity > 0
                    }) {
                        Intent::Gather {
                            batch: b.id,
                            communal: true,
                        }
                    } else {
                        Intent::CancelJob
                    }
                } else {
                    let home = j.acquired >= j.target;
                    Intent::Travel {
                        edge: if node == 0 {
                            0
                        } else if node == 2 {
                            1
                        } else if home {
                            0
                        } else {
                            1
                        },
                        target: if home { 0 } else { 2 },
                        food: false,
                    }
                }
            } else if node == 0 {
                let q = w.boards()[0]
                    .unclaimed(w.now())
                    .min(w.carrier_capacity(0) - w.carrier_load(0));
                if q > 0 {
                    Intent::Claim {
                        board: 0,
                        quantity: q,
                    }
                } else {
                    Intent::Wait
                }
            } else {
                Intent::Travel {
                    edge: if node == 2 { 1 } else { 0 },
                    target: 0,
                    food: false,
                }
            }
        };
        w.apply_intents(vec![(0, intent)])?;
    }
    w.audit()?;
    Ok(w)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("capital_witness NEW_CSV")?;
    let mut out = BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?,
    );
    writeln!(
        out,
        "edge_h,demand,cart,delivered,consumed,starvation_h,focal_alive,wood_invested,labor_h,cart_food_quanta_edges,accounting_error,overtakes"
    )?;
    for edge in [HOUR / 4, HOUR / 2, 2 * HOUR] {
        for demand in [30, 600] {
            for cart in [false, true] {
                let w = run(edge, demand, cart)?;
                let c = w.capital_counters();
                writeln!(
                    out,
                    "{},{},{},{},{},{},{},{},{},{},0,{}",
                    edge as f64 / HOUR as f64,
                    demand,
                    cart,
                    w.metrics().delivered,
                    w.metrics().consumed,
                    w.metrics().deficit_ms as f64 / HOUR as f64,
                    w.agents()[0].action != Action::Dead,
                    c["wood_invested"],
                    c["labor_ms"] as f64 / HOUR as f64,
                    c["cart_food_quanta_edges_arrived"],
                    w.metrics().overtakes
                )?;
                out.flush()?;
            }
        }
    }
    Ok(())
}

#[test]
fn fractional_headroom_does_not_block_other_research_actions() {
    assert_eq!(edible_quanta(12 * HOUR - SECOND, true, 10), 0);
    assert_eq!(edible_quanta(11 * HOUR, true, 10), 1);
    assert_eq!(edible_quanta(5 * HOUR, true, 20), 7);
    assert_eq!(edible_quanta(5 * HOUR, true, 2), 2);
    assert_eq!(edible_quanta(24 * HOUR, false, 10), 0);
}
