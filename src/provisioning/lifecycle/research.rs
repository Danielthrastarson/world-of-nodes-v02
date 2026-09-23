//! Registered B3 fixtures. Controlled support/partner seeding are explicit research inputs.
use super::*;
pub const CASES: &[&str] = &[
    "basic",
    "stochastic",
    "infertile",
    "separated",
    "decline",
    "hungry",
    "dependency",
    "dependency_social",
    "parent_death",
    "multiple",
    "multigeneration",
    "multigeneration_social",
    "generational_control",
    "working",
    "working_no_birth",
    "seeded_one",
    "seeded_three",
];
pub fn fixture(name: &str, seed: u64) -> Result<World, String> {
    if !CASES.contains(&name) {
        return Err("unknown B3 fixture".into());
    }
    let n = if name == "seeded_one" {
        3
    } else if name == "seeded_three" {
        5
    } else if name.starts_with("multigeneration") || name == "generational_control" {
        8
    } else if name == "multiple" {
        4
    } else {
        2
    };
    let mut c = super::super::Config {
        seed,
        trace: true,
        nodes: 2,
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: HOUR,
            capacity: 4,
        }],
        starts: vec![0; n],
        food: vec![],
        boards: vec![],
        communal: false,
        initial_nutrition: 18 * HOUR,
        household: Some(household::Config {
            enabled: true,
            ..household::Config::default()
        }),
        ..super::super::Config::default()
    };
    if name != "separated" {
        c.nodes = 1;
        c.edges.clear();
    }
    if name == "separated" {
        c.starts[1] = 1;
        c.edges[0].capacity = 0;
    }
    if name.ends_with("social") {
        c.social = Some(social::Config::default())
    }
    let h = c.household.as_mut().unwrap();
    for i in (0..if name.starts_with("seeded_") { 2 } else { n }).step_by(2) {
        h.households.push(household::Household {
            id: i / 2,
            members: vec![i, i + 1],
            home: None,
        });
        h.kin.push(household::Relation {
            from: i,
            to: i + 1,
            kind: household::Kin::Partner,
        });
    }
    if name.starts_with("seeded_") {
        h.households[0].members = (0..n).collect();
        for child in 2..n {
            for parent in 0..2 {
                h.kin.push(household::Relation {
                    from: parent,
                    to: child,
                    kind: household::Kin::Parent,
                });
                h.care.push(household::Caregiver {
                    person: parent,
                    dependent: child,
                });
            }
        }
    }
    if name.starts_with("working") {
        c.farming = Some(farming::Config {
            units: vec![0, 0],
            rules: farming::Rules::default(),
        });
    }
    c.lifecycle = Some(Config {
        people: (0..n)
            .map(|i| Seed {
                birth_time: if name.starts_with("seeded_") && i >= 2 {
                    0
                } else {
                    -24 * HOUR
                },
                sex: if i % 2 == 0 { Sex::Female } else { Sex::Male },
                drive: !(name == "decline" && i == 1),
            })
            .collect(),
        rules: Rules {
            conception_per_million: match name {
                "stochastic" => 250_000,
                "infertile" | "working_no_birth" => 0,
                _ => 1_000_000,
            },
            ..Rules::default()
        },
    });
    let mut w = World::new(c)?;
    if name.starts_with("seeded_") {
        for child in 2..n {
            w.agents[child].nutrition = 0;
            w.lifecycle.people[child].parents = Some([0, 1]);
            w.lifecycle.people[child].generation = 2;
        }
    }
    if name == "hungry" {
        for a in &mut w.agents {
            a.nutrition = 0;
        }
    } else {
        for id in 0..if name.starts_with("seeded_") { 2 } else { n } {
            let node = w.config.starts[id];
            w.goods.add(
                Material::Food,
                6,
                Owner::Agent(id),
                Holder::Agent(id),
                0,
                48 * HOUR,
            );
            w.goods.add(
                Material::Food,
                if name.starts_with("working") {
                    0
                } else if name.starts_with("seeded_") {
                    24
                } else {
                    200
                },
                Owner::Agent(id),
                Holder::Loose(node),
                0,
                48 * HOUR,
            );
        }
    }
    Ok(w)
}
/// Controlled local Partner association for unrelated adult descendants; no mate search in policy.
pub fn seed_descendant_partner(w: &mut World) -> bool {
    if w.lifecycle.journal.count("research_partner_seed") > 0 {
        return false;
    }
    let adults: Vec<_> = w
        .agents
        .iter()
        .filter(|a| {
            a.action != Action::Dead
                && !w.is_child(a.id)
                && w.lifecycle.people[a.id].generation == 2
        })
        .map(|a| a.id)
        .collect();
    for &a in &adults {
        for &b in &adults {
            if a < b
                && w.lifecycle.people[a].sex != w.lifecycle.people[b].sex
                && w.lifecycle.people[a].parents != w.lifecycle.people[b].parents
                && w.agents[a].location == w.agents[b].location
            {
                w.config
                    .household
                    .as_mut()
                    .unwrap()
                    .kin
                    .push(household::Relation {
                        from: a,
                        to: b,
                        kind: household::Kin::Partner,
                    });
                w.lifecycle.journal.add("research_partner_seed");
                w.log(
                    None,
                    "ResearchPartnerSeed",
                    format!(
                        "adults={a},{b}; controlled relationship initial condition, not mate search"
                    ),
                );
                return true;
            }
        }
    }
    false
}
/// Controlled support uses only existing parent-owned carried goods and ordinary transfer.
/// No new food or nutrition is created; local parents must possess the gift.
pub fn support(w: &mut World) -> Result<(), String> {
    let children: Vec<_> = w
        .agents
        .iter()
        .filter(|a| a.action != Action::Dead && w.is_child(a.id) && a.nutrition <= 6 * HOUR)
        .map(|a| a.id)
        .collect();
    for child in children {
        if let Some(parents) = w.lifecycle.people[child].parents {
            for parent in parents {
                if w.agents[parent].action == Action::Dead
                    || w.agents[parent].location != w.agents[child].location
                {
                    continue;
                }
                let Some(batch) = w
                    .goods
                    .batches
                    .iter()
                    .find(|b| {
                        b.quantity >= 6
                            && b.material == Material::Food
                            && b.owner == Owner::Agent(parent)
                            && b.holder == Holder::Agent(parent)
                    })
                    .map(|b| b.id)
                else {
                    continue;
                };
                w.log(
                    Some(parent),
                    "ResearchCareCommand",
                    format!("dependent={child}; prescribed support, not autonomous motivation"),
                );
                w.apply_intents(vec![(
                    parent,
                    Intent::GiveCareFood {
                        dependent: child,
                        batch,
                        quantity: 6,
                    },
                )])?;
                break;
            }
        }
    }
    Ok(())
}
pub fn parent_death(w: &mut World) -> Result<(), String> {
    // A registered initial-condition intervention at10h; cannot silently kill/transfer property.
    if w.now == 10 * HOUR {
        for id in 0..2 {
            w.agents[id].nutrition = 0;
            w.agents[id].starvation = 24 * HOUR;
        }
        w.log(
            None,
            "ResearchFatalExposure",
            "both initial caregivers set to lethal starvation as controlled death fixture".into(),
        );
        w.advance_to(w.now + 1)?;
    }
    Ok(())
}
pub fn peak_bytes(w: &World) -> usize {
    format!(
        "{:?}|{:?}|{:?}|{:?}",
        w.agents, w.lifecycle, w.household, w.social
    )
    .len()
}
pub fn parameters(w: &World) -> String {
    format!(
        "config={:#?}\nbiology={:#?}\ngoods={:#?}",
        w.config, w.lifecycle.people, w.goods.batches
    )
}
pub fn social_edges(w: &World) -> usize {
    w.social
        .memories
        .iter()
        .map(|m| m.relationships.len())
        .sum()
}
pub fn active_pregnancies(w: &World) -> usize {
    w.lifecycle
        .people
        .iter()
        .filter(|b| b.pregnancy.is_some())
        .count()
}
pub fn household_sizes(w: &World) -> Vec<(usize, usize, usize)> {
    w.config
        .household
        .as_ref()
        .unwrap()
        .households
        .iter()
        .map(|h| {
            let alive: Vec<_> = h
                .members
                .iter()
                .copied()
                .filter(|&i| w.agents[i].action != Action::Dead)
                .collect();
            (
                h.id,
                alive.len(),
                alive.iter().filter(|&&i| w.is_child(i)).count(),
            )
        })
        .collect()
}

/// Additional controlled nourishment, explicitly separate from autonomous care.
pub fn support_adults(w: &mut World) -> Result<(), String> {
    let targets: Vec<_> = w
        .agents
        .iter()
        .filter(|a| {
            a.action != Action::Dead
                && a.nutrition < 18 * HOUR
                && w.lifecycle.people[a.id].parents.is_some()
        })
        .map(|a| a.id)
        .collect();
    for child in targets {
        for parent in w.lifecycle.people[child].parents.unwrap() {
            if w.agents[parent].action == Action::Dead
                || w.agents[parent].location != w.agents[child].location
            {
                continue;
            }
            let carried = w
                .goods
                .batches
                .iter()
                .find(|b| {
                    b.quantity >= 6
                        && b.material == Material::Food
                        && b.owner == Owner::Agent(parent)
                        && b.holder == Holder::Agent(parent)
                })
                .map(|b| b.id);
            if carried.is_none() && w.goods.load(parent) + 6 <= w.config.carrying_capacity {
                let node = match w.agents[parent].location {
                    Location::Node(n) => n,
                    _ => continue,
                };
                if let Some(batch) = w
                    .goods
                    .batches
                    .iter()
                    .find(|b| {
                        b.quantity >= 6
                            && b.material == Material::Food
                            && b.expires_at() > w.now
                            && b.owner == Owner::Agent(parent)
                            && b.holder == Holder::Loose(node)
                    })
                    .map(|b| b.id)
                {
                    w.log(
                        Some(parent),
                        "ResearchSupportPickup",
                        format!("own_batch={batch}; controlled preparation"),
                    );
                    w.apply_intents(vec![(parent, Intent::Pickup { batch, quantity: 6 })])?;
                }
            }
            if let Some(batch) = w
                .goods
                .batches
                .iter()
                .find(|b| {
                    b.quantity >= 6
                        && b.material == Material::Food
                        && b.owner == Owner::Agent(parent)
                        && b.holder == Holder::Agent(parent)
                })
                .map(|b| b.id)
            {
                w.log(
                    Some(parent),
                    "ResearchAdultSupport",
                    format!("dependent={child}; existing personal food; not autonomous care"),
                );
                w.apply_intents(vec![(
                    parent,
                    Intent::GiveCareFood {
                        dependent: child,
                        batch,
                        quantity: 6,
                    },
                )])?;
                break;
            }
        }
    }
    Ok(())
}
