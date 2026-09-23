//! F1 research inputs. Map roles never enter autonomous policy.
use super::*;
pub const MAPS: &[&str] = &[
    "line",
    "branch",
    "alternative_routes",
    "dead_end",
    "bottleneck",
    "unequal_sources",
];
pub fn configuration(name: &str, seed: u64, trace: bool) -> Result<Config, String> {
    let mut c = Config {
        seed,
        trace,
        initial_request: 600,
        ..Config::default()
    };
    let links: Vec<(usize, usize, usize)> = match name {
        "line" => vec![(0, 1, 4), (1, 2, 4)],
        "branch" | "unequal_sources" | "dead_end" => vec![(0, 1, 4), (1, 2, 4), (0, 3, 4)],
        "alternative_routes" => vec![(0, 1, 4), (1, 3, 4), (0, 2, 4), (2, 3, 4)],
        "bottleneck" => vec![(0, 1, 4), (1, 2, 1), (2, 3, 4)],
        _ => return Err(format!("unknown topology {name}")),
    };
    c.nodes = if name == "line" { 3 } else { 4 };
    c.edges = links
        .into_iter()
        .map(|(a, b, capacity)| Edge {
            a,
            b,
            capacity,
            nominal_ms: 2 * HOUR,
        })
        .collect();
    c.food = match name {
        "line" => vec![(2, 3000, 0, 48 * HOUR)],
        "branch" => vec![(2, 1500, 0, 48 * HOUR), (3, 1500, 0, 48 * HOUR)],
        "unequal_sources" => vec![(2, 2700, 0, 48 * HOUR), (3, 300, 0, 48 * HOUR)],
        _ => vec![(3, 3000, 0, 48 * HOUR)],
    };
    Ok(c)
}
/// Pure renaming; preserve connectivity, stocks and physical parameters.
pub fn reversed_labels(mut c: Config) -> Config {
    let n = c.nodes;
    for e in &mut c.edges {
        e.a = n - 1 - e.a;
        e.b = n - 1 - e.b;
    }
    c.edges.reverse();
    let edge_count = c.edges.len();
    if let Some(paths) = &mut c.paths {
        for edge in &mut paths.ground_edges {
            *edge = edge_count - 1 - *edge;
        }
    }
    if let Some(infrastructure) = &mut c.infrastructure {
        for site in &mut infrastructure.sites {
            site.edge = edge_count - 1 - site.edge;
            site.node = n - 1 - site.node;
        }
        for (node, _) in &mut infrastructure.wood {
            *node = n - 1 - *node;
        }
    }
    if let Some(capital) = &mut c.capital {
        for (node, _) in &mut capital.wood {
            *node = n - 1 - *node;
        }
    }
    if let Some(farm) = &mut c.farming {
        for node in &mut farm.units {
            *node = n - 1 - *node;
        }
    }
    for s in &mut c.starts {
        *s = n - 1 - *s;
    }
    for b in &mut c.boards {
        *b = n - 1 - *b;
    }
    for f in &mut c.food {
        f.0 = n - 1 - f.0;
    }
    c
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relabeling_preserves_the_physical_request_and_unique_route() {
        let c = configuration("line", 0, false).unwrap();
        let mut a = World::new(c.clone()).unwrap();
        let mut b = World::new(reversed_labels(c)).unwrap();
        for h in 0..=72 {
            a.advance_to(h * HOUR).unwrap();
            b.advance_to(h * HOUR).unwrap();
            assert_eq!(
                a.boards[0].quantity, b.boards[0].quantity,
                "request must follow its configured board, hour={h}"
            );
            assert_eq!(a.metrics, b.metrics, "physical metrics hour={h}");
            for (x, y) in a.agents.iter().zip(&b.agents) {
                assert_eq!(
                    (x.nutrition, x.fatigue, x.starvation, x.consumed),
                    (y.nutrition, y.fatigue, y.starvation, y.consumed),
                    "agent={} hour={h}",
                    x.id
                );
                let expected = match x.location {
                    Location::Node(n) => Location::Node(2 - n),
                    Location::Edge {
                        edge,
                        from,
                        to,
                        progress,
                    } => Location::Edge {
                        edge: 1 - edge,
                        from: 2 - from,
                        to: 2 - to,
                        progress,
                    },
                };
                assert_eq!(expected, y.location);
            }
            a.audit().unwrap();
            b.audit().unwrap();
        }
    }
    #[test]
    fn hidden_food_relocation_cannot_guide_initial_exploration() {
        for name in MAPS {
            let c = configuration(name, 9, false).unwrap();
            let mut changed = c.clone();
            changed.food = vec![(1, 3000, 0, 48 * HOUR)];
            let a = World::new(c).unwrap();
            let b = World::new(changed).unwrap();
            for id in 0..50 {
                let oa = a.observe(id).unwrap();
                let ob = b.observe(id).unwrap();
                assert_eq!(format!("{oa:?}"), format!("{ob:?}"));
                let mut ma = a.agents[id].memory.clone();
                let mut mb = b.agents[id].memory.clone();
                ma.nodes.insert(0, oa.local.clone().unwrap());
                mb.nodes.insert(0, ob.local.clone().unwrap());
                let mut ra = a.agents[id].rng;
                let mut rb = b.agents[id].rng;
                let da = policy::decide(&oa, &ma, &mut ra);
                let db = policy::decide(&ob, &mb, &mut rb);
                assert_eq!(da, db);
                assert!(da.reason.contains("explore unknown"));
                assert!(matches!(da.intent, Intent::Travel { food: false, .. }));
            }
        }
    }
    #[test]
    fn memory_and_food_goals_only_follow_physical_observations() {
        for name in MAPS {
            let mut w = World::new(configuration(name, 0, true).unwrap()).unwrap();
            w.advance_to(18 * HOUR).unwrap();
            let mut visited: BTreeSet<(usize, usize)> = (0..50).map(|id| (id, 0)).collect();
            for e in w.events() {
                if e.kind == "Arrived" {
                    let n = shadow::plan::value(&e.detail, "node").unwrap() as usize;
                    visited.insert((e.agent.unwrap(), n));
                }
            }
            for a in &w.agents {
                assert!(
                    a.memory.nodes.keys().all(|n| visited.contains(&(a.id, *n))),
                    "{name} agent{} remote knowledge",
                    a.id
                );
                if let Some(o) = w.observe(a.id) {
                    let d = policy::decide(&o, &a.memory, &mut a.rng.clone());
                    if let Intent::Travel {
                        target, food: true, ..
                    } = d.intent
                    {
                        assert!(
                            a.memory
                                .nodes
                                .get(&target)
                                .is_some_and(|v| !v.food.is_empty())
                        );
                    }
                }
            }
            w.audit().unwrap();
        }
    }
}
