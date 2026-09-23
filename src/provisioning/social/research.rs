//! Research-only physical history episodes and matched-state controls.
use super::*;
pub const HISTORIES: &[&str] = &[
    "no_history",
    "help_safe",
    "help_costly",
    "failed_attempt",
    "refusal",
    "remote_refusal",
    "asymmetric",
    "death_known",
    "death_unknown",
    "fear",
    "stranger",
];
fn config(seed: u64) -> super::super::Config {
    super::super::Config {
        seed,
        trace: true,
        autonomous: false,
        nodes: 2,
        starts: vec![0, 0, 1],
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: HOUR,
            capacity: 4,
        }],
        food: vec![],
        boards: vec![],
        communal: false,
        initial_nutrition: 18 * HOUR,
        social: Some(Config::default()),
        household: Some(household::Config {
            enabled: true,
            households: vec![household::Household {
                id: 0,
                members: vec![0, 1, 2],
                home: None,
            }],
            kin: vec![
                household::Relation {
                    from: 0,
                    to: 1,
                    kind: household::Kin::Parent,
                },
                household::Relation {
                    from: 1,
                    to: 2,
                    kind: household::Kin::Partner,
                },
            ],
            care: vec![
                household::Caregiver {
                    person: 0,
                    dependent: 1,
                },
                household::Caregiver {
                    person: 1,
                    dependent: 0,
                },
            ],
        }),
        ..super::super::Config::default()
    }
}
pub fn history(name: &str, seed: u64) -> Result<World, String> {
    if !HISTORIES.contains(&name) {
        return Err("unregistered social history".into());
    }
    let mut c = config(seed);
    if name == "remote_refusal" {
        c.starts[0] = 1;
    }
    if name == "death_known" {
        c.starts[2] = 0;
    }
    if name == "stranger" {
        c.household = None;
    }
    if name == "failed_attempt" {
        c.edges[0].capacity = 0;
    }
    let mut w = World::new(c)?;
    w.agents[1].nutrition = 0;
    match name {
        "help_safe" | "help_costly" | "asymmetric" => {
            w.agents[0].nutrition = if name == "help_safe" {
                18 * HOUR
            } else {
                7 * HOUR
            };
            let b = w.goods.add(
                Material::Food,
                6,
                Owner::Agent(0),
                Holder::Agent(0),
                0,
                48 * HOUR,
            );
            w.apply_intents(vec![(
                0,
                Intent::GiveCareFood {
                    dependent: 1,
                    batch: b,
                    quantity: 6,
                },
            )])?;
            let food = w
                .goods
                .batches
                .iter()
                .find(|b| b.quantity > 0 && b.owner == Owner::Agent(1))
                .unwrap()
                .id;
            w.apply_intents(vec![(
                1,
                Intent::Eat {
                    batch: food,
                    quantity: 6,
                },
            )])?;
        }
        "failed_attempt" => {
            let b = w.goods.add(
                Material::Food,
                6,
                Owner::Agent(0),
                Holder::Loose(1),
                0,
                48 * HOUR,
            );
            w.apply_intents(vec![(
                0,
                Intent::Travel {
                    edge: 0,
                    target: 1,
                    food: true,
                },
            )])?;
            w.apply_intents(vec![(
                0,
                Intent::GiveCareFood {
                    dependent: 1,
                    batch: b,
                    quantity: 6,
                },
            )])?;
        }
        "refusal" | "remote_refusal" => {
            let b = w.goods.add(
                Material::Food,
                6,
                Owner::Agent(0),
                Holder::Agent(0),
                0,
                48 * HOUR,
            );
            w.apply_intents(vec![(
                0,
                Intent::Eat {
                    batch: b,
                    quantity: 6,
                },
            )])?;
        }
        "death_known" | "death_unknown" => {
            w.agents[2].nutrition = 0;
            w.agents[2].starvation = 23 * HOUR;
        }
        "fear" => {
            w.agents[1].starvation = 17 * HOUR;
        }
        _ => {}
    }
    w.advance_to(HOUR)?;
    w.observe_social(1);
    w.audit()?;
    Ok(w)
}
/// Identical physical/spatial/care inputs; ONLY social memories differ.
/// This constructs a counterfactual experimental state, not a runtime power.
pub fn matched(history: &World, seed: u64) -> Result<World, String> {
    let mut c = config(seed);
    c.trace = true;
    let mut w = World::new(c)?;
    w.agents[0].nutrition = HOUR;
    w.agents[1].nutrition = 19 * HOUR;
    w.agents[2].nutrition = 0;
    w.agents[2].starvation = 23 * HOUR;
    w.goods.add(
        Material::Food,
        6,
        Owner::Agent(1),
        Holder::Agent(1),
        0,
        48 * HOUR,
    );
    w.advance_to(HOUR)?;
    w.social.memories = history.social.memories.clone();
    w.social.known_by.clear();
    for (id, m) in w.social.memories.iter().enumerate() {
        for &other in m.relationships.keys() {
            w.social.known_by.entry(other).or_default().insert(id);
        }
    }
    w.social.journal = Journal::default();
    // Shared personal observation is formed afresh at the matched decision point.
    for id in [0, 1] {
        w.observe_social(id);
        w.observe_household(id);
        let o = w.observe(id).unwrap();
        let v = o.local.unwrap();
        w.agents[id].memory.nodes.insert(v.node, v);
    }
    w.audit()?;
    Ok(w)
}
pub fn decision(w: &World, id: usize) -> Decision {
    let o = w.observe(id).unwrap();
    person::decide(
        person::Context {
            observation: &o,
            beliefs: &w.agents[id].memory,
            capabilities: w.person_capabilities(id),
        },
        &mut w.agents[id].rng.clone(),
    )
}
pub fn assessment(w: &World, id: usize) -> Option<Assessment> {
    let o = w.observe(id)?;
    let d = household::evaluate(&o, &w.agents[id].memory, &w.household.memories[id]).ok()?;
    assess(
        &o,
        &w.agents[id].memory,
        &w.household.memories[id],
        &w.social.memories[id],
        &d,
    )
}
pub fn physical_signature(w: &World) -> String {
    let mut agents = w.agents.clone();
    for a in &mut agents {
        a.last_event = None;
    }
    let mut config = w.config.clone();
    config.social = None;
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        w.now, agents, w.goods, w.boards, w.metrics, w.household.memories, config
    )
}
pub fn live(name: &str, enabled: bool, seed: u64) -> Result<World, String> {
    let mut w = household::research::fixture(name, true, seed)?;
    if enabled {
        w.config.social = Some(Config::default());
        w.social = State::new(&w.config);
    }
    Ok(w)
}
pub fn parameters(w: &World) -> String {
    format!(
        "{}\nsocial={:?}",
        household::research::parameters(w),
        w.social.memories
    )
}
/// Analytic active durations from pulse resets; no extra simulation updates.
pub fn affect_hours(w: &World, id: usize, kind: &str, end: Time) -> f64 {
    let end = w.agents[id].death_time.unwrap_or(end).min(end);
    let pulse = |a: &Affect| -> Pulse {
        match kind {
            "anger" => a.anger.clone(),
            "fear" => a.fear.clone(),
            _ => a.grief.clone(),
        }
    };
    let mut intervals = vec![];
    let mut collect = |p: Pulse| {
        if p.height > 0 {
            intervals.push((p.at, (p.at + p.height).min(end)));
        }
    };
    for a in &w.social.journal.appraisals {
        if a.evidence.observer == id {
            collect(pulse(&a.affect_before));
            collect(pulse(&a.affect_after));
        }
    }
    if let Some(m) = w.social.memories.get(id) {
        collect(pulse(&m.affect));
    }
    intervals.sort();
    let mut total = 0;
    let mut last = 0;
    for (a, b) in intervals {
        let start = a.max(last);
        if b > start {
            total += b - start;
            last = b;
        }
    }
    total as f64 / HOUR as f64
}

pub fn set_influence(w: &mut World, influence: bool) {
    w.config.social.as_mut().unwrap().influence = influence;
}
pub fn enact(w: &mut World, id: usize, d: Decision) -> Result<(), String> {
    let o = w.observe(id).unwrap();
    w.record_social_choice(id, &o, &d);
    w.record_care_decision(id, &o, &d);
    w.log(Some(id), "MatchedPersonChoice", format!("{d:?}"));
    w.apply_intents(vec![(id, d.intent)])?;
    w.config.autonomous = true;
    w.next_policy = w.now + w.config.policy_ms;
    Ok(())
}
/// Isolate the marginal physical cost of one prescribed gift, not a new policy.
pub fn cost_fixture(surplus: bool, give: bool, seed: u64) -> Result<World, String> {
    let mut c = config(seed);
    c.nodes = 1;
    c.edges.clear();
    c.starts = vec![0, 0];
    c.household.as_mut().unwrap().households[0].members = vec![0, 1];
    c.household.as_mut().unwrap().kin.truncate(1);
    c.social.as_mut().unwrap().influence = false;
    let mut w = World::new(c)?;
    w.agents[0].nutrition = if surplus { 18 * HOUR } else { 7 * HOUR };
    w.agents[1].nutrition = 0;
    let b = w.goods.add(
        Material::Food,
        6,
        Owner::Agent(0),
        Holder::Agent(0),
        0,
        48 * HOUR,
    );
    if surplus {
        w.goods.add(
            Material::Food,
            60,
            Owner::Agent(0),
            Holder::Loose(0),
            0,
            48 * HOUR,
        );
    }
    if give {
        w.apply_intents(vec![(
            0,
            Intent::GiveCareFood {
                dependent: 1,
                batch: b,
                quantity: 6,
            },
        )])?;
    }
    w.config.household.as_mut().unwrap().enabled = false;
    w.config.autonomous = true;
    w.next_policy = 0;
    w.audit()?;
    Ok(w)
}
