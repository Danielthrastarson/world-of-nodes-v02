//! Explicit initial-condition controls; never imported by autonomous selection.
use super::*;
pub const CASES: &[&str] = &[
    "colocated",
    "self_conflict",
    "multiple",
    "near_known",
    "far_known",
    "unknown",
    "caregiver_death",
    "nonkin",
    "farming",
    "farming_distant",
    "construction",
];
pub fn fixture(case: &str, enabled: bool, seed: u64) -> Result<World, String> {
    if !CASES.contains(&case) {
        return Err("unregistered household fixture".into());
    }
    let distant = ["near_known", "far_known", "unknown", "farming_distant"].contains(&case);
    let count = if case == "multiple" { 3 } else { 2 };
    let mut c = super::super::Config {
        seed,
        trace: seed == 0,
        nodes: if distant { 2 } else { 1 },
        starts: if distant { vec![0, 1] } else { vec![0; count] },
        edges: if distant {
            vec![Edge {
                a: 0,
                b: 1,
                capacity: 4,
                nominal_ms: if case == "far_known" { 10 * HOUR } else { HOUR },
            }]
        } else {
            vec![]
        },
        food: vec![],
        boards: vec![],
        communal: false,
        initial_nutrition: 18 * HOUR,
        ..super::super::Config::default()
    };
    if case.starts_with("farming") {
        c.farming = Some(farming::Config {
            units: (0..c.nodes).flat_map(|n| [n, n]).collect(),
            rules: farming::Rules::default(),
        });
    }
    if case == "construction" {
        // F6 closed-world parameters, with explicitly seeded caregiver/dependent state.
        c.nodes = 3;
        c.starts = vec![0; 50];
        c.communal = true;
        c.boards = vec![0];
        c.initial_request = 600;
        c.initial_nutrition = 12 * HOUR;
        c.prior_memory = true;
        c.edges = vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR / 4,
                capacity: 4,
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: HOUR / 4,
                capacity: 0,
            },
        ];
        c.food = vec![(1, 300, 0, 48 * HOUR), (2, 2700, 0, 48 * HOUR)];
        c.infrastructure = Some(infrastructure::Config {
            enabled: true,
            wood: vec![(0, 100)],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 20,
                work_ms: 8 * HOUR,
                opened_nominal_ms: HOUR / 4,
                opened_capacity: 4,
            }],
        });
    }
    c.household = Some(Config {
        enabled,
        households: if case == "nonkin" {
            vec![]
        } else {
            vec![Household {
                id: 7,
                members: (0..count).collect(),
                home: None,
            }]
        },
        kin: if case == "nonkin" {
            vec![]
        } else {
            (1..count)
                .map(|to| Relation {
                    from: 0,
                    to,
                    kind: Kin::Parent,
                })
                .collect()
        },
        care: if case == "nonkin" {
            vec![]
        } else {
            (1..count)
                .map(|dependent| Caregiver {
                    person: 0,
                    dependent,
                })
                .collect()
        },
    });
    let mut w = World::new(c)?;
    w.agents[0].nutrition = if case == "self_conflict" {
        4 * HOUR
    } else {
        18 * HOUR
    };
    for id in 1..count {
        w.agents[id].nutrition = 0;
    }
    if case == "caregiver_death" {
        w.agents[0].nutrition = 0;
        w.agents[0].starvation = 23 * HOUR;
        w.agents[1].nutrition = 12 * HOUR;
    } else {
        w.goods.add(
            Material::Food,
            6,
            Owner::Agent(0),
            Holder::Agent(0),
            0,
            48 * HOUR,
        );
    }
    if distant && case != "unknown" {
        // Explicit timestamped prior meeting, legitimate only as experimental history.
        w.household.memories[0].sightings.insert(
            1,
            Sighting {
                person: 1,
                node: 1,
                observed_at: 0,
                nutrition: 0,
                starvation: 0,
                present: true,
                alive: true,
            },
        );
        let view = w.node_view(0);
        w.agents[0].memory.nodes.insert(0, view);
    }
    w.audit()?;
    Ok(w)
}
/// Serialized experiment parameters include all non-Config seeded initial conditions.
pub fn parameters(w: &World) -> String {
    format!(
        "config={:?}\npeople={:?}\ngoods={:?}\ncare_memories={:?}\n",
        w.config, w.agents, w.goods, w.household.memories
    )
}
pub fn initial_food(w: &World) -> u64 {
    w.goods
        .batches
        .iter()
        .filter(|b| b.material == Material::Food)
        .map(|b| b.quantity)
        .sum()
}
pub fn production(w: &World) -> u64 {
    w.farm.produced
}
