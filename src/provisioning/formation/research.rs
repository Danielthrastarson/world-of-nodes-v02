//! Experimental initial conditions and one local order; no injected per-soldier moves.
use super::*;
pub const CASES: &[&str] = &[
    "short",
    "sequence",
    "unknown_route",
    "congestion",
    "long",
    "food_stress",
    "fatigue_stress",
    "bottleneck",
    "path_on",
    "path_off",
    "partial",
    "late",
    "care",
    "shared_route",
    "crossing",
    "cart",
];
pub fn fixture(case: &str, seed: u64) -> Result<World, String> {
    if !CASES.contains(&case) {
        return Err("unknown formation case".into());
    }
    let civilians = if matches!(case, "congestion" | "shared_route") {
        8
    } else {
        0
    };
    let n = 4 + civilians + usize::from(matches!(case, "care" | "crossing"));
    let mut c = Config {
        seed,
        trace: true,
        formation: true,
        nodes: 3,
        starts: vec![0; n],
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: if case == "long" { 10 * HOUR } else { 2 * HOUR },
                capacity: if matches!(case, "bottleneck" | "congestion") {
                    1
                } else if case == "late" {
                    2
                } else {
                    4
                },
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: 2 * HOUR,
                capacity: 4,
            },
        ],
        food: vec![],
        boards: vec![],
        communal: false,
        initial_nutrition: if matches!(case, "late" | "partial" | "crossing" | "cart") {
            24 * HOUR
        } else {
            20 * HOUR
        },
        military: Some(military::Config {
            enabled: true,
            requests: vec![military::Request {
                id: 0,
                posting: 0,
                muster: if matches!(case, "late" | "partial") {
                    1
                } else {
                    0
                },
                attendance: if case == "partial" { 2 } else { 4 },
                opens: if matches!(case, "crossing" | "cart") {
                    5 * HOUR
                } else {
                    0
                },
                expires: 60 * HOUR,
                duration: 12 * HOUR,
                objective: military::Objective::ProtectCrossing,
            }],
        }),
        household: Some(household::Config {
            enabled: true,
            ..Default::default()
        }),
        lifecycle: Some(lifecycle::Config {
            people: (0..n)
                .map(|id| lifecycle::Seed {
                    birth_time: if case == "care" && id == 4 {
                        0
                    } else {
                        -24 * HOUR
                    },
                    sex: if id % 2 == 0 {
                        lifecycle::Sex::Female
                    } else {
                        lifecycle::Sex::Male
                    },
                    drive: false,
                })
                .collect(),
            rules: Default::default(),
        }),
        ..Config::default()
    };
    if case == "congestion" {
        for id in 4..n {
            c.starts[id] = 1;
        }
        c.food = vec![(0, 300, 0, 48 * HOUR)];
    }
    if case == "shared_route" {
        c.food = vec![(1, 300, 0, 48 * HOUR)];
        c.farming = Some(farming::Config {
            units: vec![1; 4],
            rules: Default::default(),
        });
    }
    if case == "care" {
        let h = c.household.as_mut().unwrap();
        h.households = vec![household::Household {
            id: 0,
            members: vec![0, 4],
            home: Some(0),
        }];
        h.kin = vec![household::Relation {
            from: 0,
            to: 4,
            kind: household::Kin::Parent,
        }];
        h.care = vec![household::Caregiver {
            person: 0,
            dependent: 4,
        }];
    }
    if case.starts_with("path_") {
        c.paths = Some(paths::Config::ground(vec![0], case == "path_on"));
    }
    if case == "cart" {
        c.capital = Some(capital::Config {
            rules: Default::default(),
            wood: vec![],
        });
    }
    if case == "crossing" {
        c.communal = true;
        c.edges[0].capacity = 0;
        c.infrastructure = Some(infrastructure::Config {
            enabled: true,
            wood: vec![],
            sites: vec![infrastructure::Site {
                edge: 0,
                node: 0,
                wood_cost: 1,
                work_ms: HOUR,
                opened_nominal_ms: 2 * HOUR,
                opened_capacity: 1,
            }],
        });
    }
    let mut w = World::new(c)?;
    for id in 0..n {
        if (civilians > 0 && id >= 4) || case == "care" && id == 4 {
            w.agents[id].nutrition = if case == "care" { 12 * HOUR } else { 5 * HOUR };
            continue;
        }
        if case != "food_stress" || id != 0 {
            w.goods.add(
                Material::Food,
                6,
                Owner::Agent(id),
                Holder::Agent(id),
                0,
                48 * HOUR,
            );
        }
    }
    if matches!(case, "sequence" | "long") {
        // Explicit documented earlier edge observation, not current destination stock.
        for id in 0..4 {
            w.agents[id].memory.nodes.insert(
                1,
                NodeView {
                    node: 1,
                    observed_at: -HOUR,
                    food: vec![],
                    edges: w
                        .config
                        .edges
                        .iter()
                        .enumerate()
                        .map(|(id, e)| EdgeView {
                            id,
                            edge: e.clone(),
                            queued: 0,
                            occupied: 0,
                        })
                        .collect(),
                    board: None,
                    people: vec![],
                },
            );
        }
    }
    if case.starts_with("path_") {
        let s = w.paths.surfaces.get_mut(&0).unwrap();
        s.passages = 50;
        s.last_passage = Some(0);
        if case == "path_on" {
            s.compaction = 50;
            s.next_recovery = Some(6 * HOUR);
        }
        w.log(
            None,
            "ResearchPathInitialCondition",
            "50 prior passages; physical effect enabled/disabled control; no military speed bonus"
                .into(),
        );
    }
    if matches!(case, "cart" | "crossing") {
        w.config.autonomous = false;
        let id = if case == "crossing" { 4 } else { 0 };
        let q = if case == "crossing" { 1 } else { 10 };
        // Existing construction consumes real initial material and authoritative time.
        // Cart builder needs free capacity; their initial food is stored personally on site.
        let carried: Vec<_> = w
            .goods
            .batches
            .iter()
            .filter(|b| b.holder == Holder::Agent(id))
            .map(|b| (b.id, b.quantity))
            .collect();
        for (batch, quantity) in carried {
            w.apply_intents(vec![(id, Intent::Drop { batch, quantity })])?;
        }
        let batch = w.goods.add(
            Material::Wood,
            q,
            Owner::Agent(id),
            Holder::Agent(id),
            0,
            Time::MAX / 4,
        );
        if case == "crossing" {
            w.apply_intents(vec![(
                id,
                Intent::ContributeCrossing {
                    project: 0,
                    batch,
                    quantity: q,
                },
            )])?;
            w.apply_intents(vec![(id, Intent::BuildCrossing { project: 0 })])?;
        } else {
            w.apply_intents(vec![(id, Intent::BuildCart)])?;
        }
        while matches!(
            w.agents[id].action,
            Action::BuildCart { .. } | Action::BuildCrossing { .. }
        ) {
            w.advance_to(w.next_time(w.now + HOUR))?;
        }
        if case == "cart" {
            w.apply_intents(vec![(id, Intent::HitchCart { cart: 0 })])?;
        }
        w.config.autonomous = true;
        w.next_policy = w.now;
    }
    Ok(w)
}
pub fn command_time(case: &str) -> Time {
    if matches!(case, "cart" | "crossing") {
        5 * HOUR + 10 * MINUTE
    } else if matches!(case, "late" | "partial") {
        2 * HOUR + 15 * MINUTE
    } else {
        HOUR
    }
}
pub fn target(case: &str) -> usize {
    if matches!(
        case,
        "sequence" | "unknown_route" | "long" | "late" | "partial"
    ) {
        2
    } else {
        1
    }
}
pub fn issue_node(case: &str) -> usize {
    usize::from(matches!(case, "late" | "partial"))
}
pub fn prepare_command(w: &mut World, case: &str) -> Result<(), String> {
    w.advance_to(command_time(case))?;
    match case {
        "food_stress" => {
            w.agents[0].nutrition = 5 * HOUR;
            w.log(
                Some(0),
                "ResearchCommandState",
                "counterfactual low nutrition at command; no food removed or created".into(),
            );
        }
        "fatigue_stress" => {
            w.agents[0].fatigue = 22 * HOUR;
            w.log(
                Some(0),
                "ResearchCommandState",
                "counterfactual critical fatigue at command; recovery mechanics unchanged".into(),
            );
        }
        "care" => {
            w.agents[4].nutrition = 0;
            w.log(
                Some(4),
                "ResearchCommandState",
                "counterfactual locally observable dependent need at command".into(),
            );
        }
        _ => {}
    }
    Ok(())
}
pub fn parameters(w: &World) -> String {
    format!(
        "{:#?}\ngoods={:#?}\npath={:#?}\ninitial memories={:#?}",
        w.config,
        w.goods.batches,
        w.paths.surfaces,
        w.agents.iter().map(|a| &a.memory).collect::<Vec<_>>()
    )
}
pub fn produced(w: &World) -> u64 {
    w.farm.produced
}
pub fn infrastructure_counts(w: &World) -> (usize, usize, u64) {
    (
        w.crossing_projects()
            .iter()
            .filter(|p| p.completed_at.is_some())
            .count(),
        w.carts().len(),
        w.path_trips().len() as u64,
    )
}
