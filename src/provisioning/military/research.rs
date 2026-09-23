//! Controlled requests/history are explicit experimental initial conditions, not NPC knowledge.
use super::*;
pub const CASES: &[&str] = &[
    "small",
    "large",
    "food_constrained",
    "caregiver",
    "farmer",
    "working_farmer",
    "distant",
    "bottleneck",
    "unknown",
    "history_help",
    "history_refusal",
    "civilian_traffic",
];
pub fn request(case: &str) -> Request {
    Request {
        id: 0,
        posting: if case == "unknown" { 2 } else { 0 },
        muster: 1,
        attendance: if case == "small" { 1 } else { 8 },
        opens: if case == "working_farmer" {
            3 * HOUR
        } else {
            0
        },
        expires: 12 * HOUR
            + if case == "working_farmer" {
                3 * HOUR
            } else {
                0
            },
        duration: 2 * HOUR,
        objective: Objective::ProtectCrossing,
    }
}
pub fn fixture(case: &str, enabled: bool, seed: u64) -> Result<World, String> {
    if !CASES.contains(&case) {
        return Err("unknown mobilization case".into());
    }
    if case.starts_with("history_") {
        let history = social::research::history(
            if case == "history_help" {
                "help_costly"
            } else {
                "refusal"
            },
            seed,
        )?;
        let mut w = social::research::matched(&history, seed)?;
        w.config.military = Some(Config {
            requests: vec![request(case)],
            enabled,
        });
        w.military = State::new(&w.config)?;
        w.config.autonomous = true;
        w.next_policy = w.now;
        w.log(
            None,
            "ResearchMilitaryInitialCondition",
            "matched physical decision state; B2 histories retained; local request seeded".into(),
        );
        return Ok(w);
    }
    let n = if case == "caregiver" {
        5
    } else if case == "civilian_traffic" {
        16
    } else {
        4
    };
    let mut c = super::super::Config {
        seed,
        trace: true,
        nodes: 3,
        starts: vec![0; n],
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: if case == "distant" { 6 * HOUR } else { HOUR },
                capacity: if matches!(case, "bottleneck" | "civilian_traffic") {
                    1
                } else {
                    4
                },
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: HOUR,
                capacity: 0,
            },
        ],
        food: vec![],
        boards: vec![],
        communal: false,
        initial_nutrition: 18 * HOUR,
        military: Some(Config {
            requests: vec![request(case)],
            enabled,
        }),
        household: Some(household::Config {
            enabled: true,
            ..Default::default()
        }),
        lifecycle: Some(lifecycle::Config {
            people: (0..n)
                .map(|id| lifecycle::Seed {
                    birth_time: if case == "caregiver" && id == 4 {
                        0
                    } else {
                        -24 * HOUR
                    },
                    sex: if id % 2 == 0 {
                        lifecycle::Sex::Male
                    } else {
                        lifecycle::Sex::Female
                    },
                    drive: false,
                })
                .collect(),
            rules: lifecycle::Rules::default(),
        }),
        ..super::super::Config::default()
    };
    if matches!(case, "farmer" | "working_farmer") {
        c.farming = Some(farming::Config {
            units: vec![0; 4],
            rules: Default::default(),
        });
    }
    if case == "caregiver" {
        let h = c.household.as_mut().unwrap();
        h.households.push(household::Household {
            id: 0,
            members: vec![0, 4],
            home: Some(0),
        });
        h.kin.push(household::Relation {
            from: 0,
            to: 4,
            kind: household::Kin::Parent,
        });
        h.care.push(household::Caregiver {
            person: 0,
            dependent: 4,
        });
    }
    if case == "civilian_traffic" {
        for id in 4..n {
            c.starts[id] = 1
        }
        c.food = vec![(0, 300, 0, 48 * HOUR)];
    }
    let mut w = World::new(c)?;
    for id in 0..n {
        if case == "caregiver" && id == 4 {
            w.agents[id].nutrition = 0;
            continue;
        }
        if case == "civilian_traffic" && id >= 4 {
            w.agents[id].nutrition = 5 * HOUR;
            continue;
        }
        if case == "food_constrained" {
            w.agents[id].nutrition = 7 * HOUR;
            continue;
        }
        w.goods.add(
            Material::Food,
            6,
            Owner::Agent(id),
            Holder::Agent(id),
            0,
            48 * HOUR,
        );
        if !matches!(case, "farmer" | "working_farmer") {
            w.goods.add(
                Material::Food,
                12,
                Owner::Agent(id),
                Holder::Loose(0),
                0,
                48 * HOUR,
            );
        }
    }
    Ok(w)
}
pub fn parameters(w: &World) -> String {
    format!(
        "config={:#?}\ninitial_goods={:#?}\npeople={:#?}",
        w.config, w.goods.batches, w.agents
    )
}
pub fn produced(w: &World) -> u64 {
    w.farm.produced
}
pub fn active(w: &World) -> usize {
    w.agents
        .iter()
        .filter(|a| w.military_role(a.id).is_some())
        .count()
}
pub fn unserved_dependents(w: &World) -> usize {
    w.config.household.as_ref().map_or(0, |h| {
        let deps: BTreeSet<_> = h.care.iter().map(|c| c.dependent).collect();
        deps.iter()
            .filter(|&&id| {
                w.agents[id].action != Action::Dead
                    && !h.care.iter().any(|c| {
                        c.dependent == id
                            && w.agents[c.person].action != Action::Dead
                            && w.agents[c.person].location == w.agents[id].location
                            && w.military_role(c.person).is_none()
                    })
            })
            .count()
    })
}
