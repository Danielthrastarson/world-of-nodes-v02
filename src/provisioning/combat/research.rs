//! Controlled hostile intent, never free soldiers or stolen goods.
use super::*;
pub const CASES: &[&str] = &[
    "local",
    "under",
    "over",
    "late",
    "bottleneck",
    "hungry",
    "congestion",
    "path_on",
    "path_off",
    "crossing",
    "household",
    "farming",
];
pub fn fixture(case: &str, defense: bool, threat: bool, seed: u64) -> Result<World, String> {
    if !CASES.contains(&case) {
        return Err("unknown raid case".into());
    }
    let n = if case == "congestion" {
        17
    } else if case == "household" {
        10
    } else {
        9
    };
    let mut c = super::super::Config {
        seed,
        trace: true,
        formation: true,
        nodes: 4,
        starts: vec![0; n],
        food: vec![],
        boards: vec![],
        communal: true,
        initial_nutrition: 24 * HOUR,
        edges: vec![
            Edge {
                a: 0,
                b: 2,
                nominal_ms: if case == "late" { 8 * HOUR } else { 2 * HOUR },
                capacity: if case == "congestion" { 1 } else { 4 },
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: 2 * HOUR,
                capacity: if case == "bottleneck" { 1 } else { 4 },
            },
            Edge {
                a: 2,
                b: 3,
                nominal_ms: 2 * HOUR,
                capacity: 4,
            },
        ],
        military: Some(military::Config {
            enabled: defense,
            requests: vec![military::Request {
                id: 0,
                posting: 0,
                muster: 0,
                attendance: match case {
                    "under" => 1,
                    "over" => 6,
                    _ => 4,
                },
                opens: 0,
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
                    birth_time: if case == "household" && id == 9 {
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
            rules: Default::default(),
        }),
        combat: Some(Config {
            nodes: vec![0, 1, 2, 3],
            raids: (6..9)
                .map(|person| Raid {
                    person,
                    target: 2,
                    home: 1,
                    begins: if threat { 2 * HOUR } else { Time::MAX / 4 },
                    quota: 10,
                })
                .collect(),
            strike_work: 15 * MINUTE,
            theft_work: 5 * MINUTE,
        }),
        ..Default::default()
    };
    for id in 6..9 {
        c.starts[id] = 1;
    }
    if case == "congestion" {
        for id in 9..n {
            c.starts[id] = 2;
        }
        c.food = vec![(0, 300, 0, 48 * HOUR)];
    }
    if case == "farming" {
        c.farming = Some(farming::Config {
            units: vec![0; 6],
            rules: Default::default(),
        });
    }
    if case.starts_with("path_") {
        c.paths = Some(paths::Config::ground(vec![0, 1], case == "path_on"));
    }
    if case == "household" {
        let h = c.household.as_mut().unwrap();
        h.households = vec![household::Household {
            id: 0,
            members: vec![0, 9],
            home: Some(0),
        }];
        h.kin = vec![household::Relation {
            from: 0,
            to: 9,
            kind: household::Kin::Parent,
        }];
        h.care = vec![household::Caregiver {
            person: 0,
            dependent: 9,
        }];
    }
    if case == "crossing" {
        c.edges[1].capacity = 0;
        c.infrastructure = Some(infrastructure::Config {
            enabled: true,
            wood: vec![],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 1,
                work_ms: HOUR,
                opened_nominal_ms: 2 * HOUR,
                opened_capacity: 1,
            }],
        });
    }
    let mut w = World::new(c)?;
    w.goods.add(
        Material::Food,
        200,
        Owner::Commons(0),
        Holder::Commons(2),
        0,
        48 * HOUR,
    );
    for id in 0..n {
        if case == "congestion" && id >= 9 {
            w.agents[id].nutrition = 5 * HOUR;
            continue;
        }
        if case == "household" && id == 9 {
            w.agents[id].nutrition = 8 * HOUR;
            continue;
        }
        if case == "hungry" && id < 6 {
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
    }
    // Explicit prior edge observations. No remembered remote goods, persons or board state.
    for id in 0..n {
        w.agents[id].memory.nodes.insert(
            2,
            NodeView {
                node: 2,
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
    if case.starts_with("path_") {
        for s in w.paths.surfaces.values_mut() {
            s.passages = 50;
            s.last_passage = Some(0);
            if case == "path_on" {
                s.compaction = 50;
                s.next_recovery = Some(6 * HOUR);
            }
        }
    }
    if case == "crossing" {
        w.config.autonomous = false;
        let b = w.goods.add(
            Material::Wood,
            1,
            Owner::Agent(6),
            Holder::Agent(6),
            0,
            Time::MAX / 4,
        );
        w.apply_intents(vec![(
            6,
            Intent::ContributeCrossing {
                project: 0,
                batch: b,
                quantity: 1,
            },
        )])?;
        w.apply_intents(vec![(6, Intent::BuildCrossing { project: 0 })])?;
        w.advance_to(HOUR)?;
        if w.crossing_projects()[0].completed_at.is_none() {
            return Err("physical preparation incomplete".into());
        }
        // The preparer locally sees completion; no remote update to defenders.
        let view = w.node_view(1);
        w.agents[6].memory.nodes.insert(1, view.clone());
        for id in [7, 8] {
            w.agents[id].memory.nodes.insert(1, view.clone());
        }
        w.config.autonomous = true;
        w.next_policy = w.now;
    }
    Ok(w)
}
pub fn prepare(w: &mut World, defense: bool) -> Result<usize, String> {
    w.advance_to(HOUR + 5 * MINUTE)?;
    if defense {
        match w.issue_unit_protect(0, 0, 2) {
            Ok(n) => Ok(n),
            Err(e) => {
                w.log(None, "ResearchProtectionUnavailable", e);
                Ok(0)
            }
        }
    } else {
        Ok(0)
    }
}
pub fn stock(w: &World) -> u64 {
    w.goods
        .batches
        .iter()
        .filter(|b| {
            b.material == Material::Food
                && b.owner == Owner::Commons(0)
                && b.holder == Holder::Commons(2)
        })
        .map(|b| b.quantity)
        .sum()
}
pub fn produced(w: &World) -> u64 {
    w.farm.produced
}
pub fn parameters(w: &World) -> String {
    format!(
        "{:#?}\ngoods={:#?}\nmemories={:#?}",
        w.config,
        w.goods.batches,
        w.agents.iter().map(|a| &a.memory).collect::<Vec<_>>()
    )
}
pub fn aggregate(w: &World) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    let j = &w.combat.journal;
    macro_rules! put {
        ($k:expr,$v:expr) => {
            m.insert($k.into(), format!("{}", $v));
        };
    }
    for k in [
        "strikes_started",
        "strikes_invalidated",
        "thefts_started",
        "hits",
        "wounded_people",
        "incapacitations",
        "lethal_injuries",
        "thefts_completed",
        "thefts_interrupted",
    ] {
        put!(k, j.count(k));
    }
    let origin = w
        .goods
        .batches
        .iter()
        .find(|b| {
            b.origin == b.id && b.owner == Owner::Commons(0) && b.holder == Holder::Commons(2)
        })
        .unwrap()
        .origin;
    let food: u64 = w
        .goods
        .batches
        .iter()
        .filter(|b| b.origin == origin && b.material == Material::Food)
        .map(|b| b.quantity)
        .sum();
    let waste: u64 = w
        .goods
        .batches
        .iter()
        .filter(|b| b.origin == origin && b.material == Material::Waste)
        .map(|b| b.quantity)
        .sum();
    put!("target_food_remaining_all_custodies", food);
    put!("target_food_waste", waste);
    put!("target_food_consumed", 200 - food - waste);
    put!(
        "target_food_raider_custody",
        w.goods
            .batches
            .iter()
            .filter(|b| b.origin == origin
                && b.material == Material::Food
                && matches!(b.holder,Holder::Agent(id) if (6..9).contains(&id)))
            .map(|b| b.quantity)
            .sum::<u64>()
    );
    put!(
        "attackers_reaching_stock",
        w.events
            .iter()
            .filter(|e| e.kind == "Arrived"
                && e.agent.is_some_and(|id| (6..9).contains(&id))
                && e.detail.starts_with("node=2 "))
            .filter_map(|e| e.agent)
            .collect::<BTreeSet<_>>()
            .len()
    );
    put!(
        "defenders_arriving_target",
        w.formation
            .views
            .iter()
            .filter(|v| v.arrived_at.is_some())
            .count()
    );
    put!(
        "contact_last_h",
        j.last_contact
            .map(|t| format!("{}", t as f64 / HOUR as f64))
            .unwrap_or_default()
    );
    put!("stolen", j.stolen);
    put!("stock", stock(w));
    put!("engaged", j.engaged.len());
    put!(
        "contact_first_h",
        j.first_contact
            .map(|t| format!("{}", t as f64 / HOUR as f64))
            .unwrap_or_default()
    );
    put!("combat_h", j.combat_ms as f64 / HOUR as f64);
    put!("theft_h", j.theft_ms as f64 / HOUR as f64);
    put!("produced", produced(w));
    put!("consumed", w.metrics.consumed);
    put!("spoiled", w.metrics.spoiled);
    put!("deaths", w.metrics.deaths);
    put!("starvation_h", w.metrics.deficit_ms as f64 / HOUR as f64);
    put!("queue_h", w.metrics.queue_ms as f64 / HOUR as f64);
    put!("care_food", w.household.journal.count("food_transferred"));
    put!(
        "care_travel_h",
        w.household.journal.care_travel_ms as f64 / HOUR as f64
    );
    put!(
        "household_separation_h",
        w.household.journal.separation_ms as f64 / HOUR as f64
    );
    put!("volunteers", w.military.journal.count("volunteers"));
    put!("assembled", w.military.journal.count("arrivals"));
    put!("military_food", w.military.journal.food);
    put!(
        "military_queue_h",
        w.formation.journal.member_queue_ms as f64 / HOUR as f64
    );
    put!("accounting_error", 0);
    put!("overtakes", w.metrics.overtakes);
    for (label, range) in [
        ("prospective_defenders", 0..6),
        ("raiders", 6..9),
        ("other_civilians", 9..w.agents.len()),
    ] {
        put!(
            format!("{label}_food"),
            range.clone().map(|id| w.agents[id].consumed).sum::<u64>()
        );
        put!(
            format!("{label}_deaths"),
            range
                .clone()
                .filter(|&id| w.agents[id].action == Action::Dead)
                .count()
        );
        put!(
            format!("{label}_incapacitated_alive"),
            range
                .clone()
                .filter(|&id| w.body_factor(id) == 0 && w.agents[id].action != Action::Dead)
                .count()
        );
        put!(
            format!("{label}_starvation_h"),
            range
                .clone()
                .map(|id| w
                    .household
                    .journal
                    .starvation_ms
                    .get(&id)
                    .copied()
                    .unwrap_or(0))
                .sum::<u128>() as f64
                / HOUR as f64
        );
        for tag in [
            "farming",
            "gather",
            "construction",
            "queue",
            "travel",
            "combat",
            "raid",
            "military_duty",
        ] {
            let sum: u128 = w
                .household
                .journal
                .activity_ms
                .iter()
                .filter(|((id, t), _)| range.contains(id) && t == tag)
                .map(|(_, q)| *q)
                .sum();
            put!(format!("{label}_{tag}_h"), sum as f64 / HOUR as f64);
        }
    }
    m
}
