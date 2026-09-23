//! F12 research setup. Initial courier purposes and recipient waiting are explicit controls.
use super::*;
pub const CASES: &[&str] = &[
    "short",
    "long",
    "congestion",
    "unknown_route",
    "fatigue",
    "hunger",
    "absent",
    "death",
    "depleted",
    "path_on",
    "path_off",
];
pub fn fixture(case: &str, assigned: bool, seed: u64) -> Result<World, String> {
    if !CASES.contains(&case) {
        return Err("unknown information case".into());
    }
    let n = if case == "congestion" {
        10
    } else if case == "depleted" {
        3
    } else {
        2
    };
    let duration = if case == "long" { 8 * HOUR } else { HOUR };
    let mut c = super::super::Config {
        information: true,
        seed,
        trace: true,
        nodes: 4,
        starts: vec![0; n],
        food: vec![],
        boards: vec![],
        initial_nutrition: 12 * HOUR,
        communal: false,
        household: Some(household::Config {
            enabled: true,
            ..Default::default()
        }),
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: duration,
                capacity: if case == "congestion" { 1 } else { 4 },
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: duration,
                capacity: 4,
            },
            Edge {
                a: 0,
                b: 3,
                nominal_ms: HOUR,
                capacity: 4,
            },
        ],
        ..Default::default()
    };
    c.starts[0] = 2;
    if case == "absent" {
        c.starts[1] = 3;
    }
    if case == "depleted" {
        c.starts[2] = 2;
    }
    if case.starts_with("path_") {
        c.paths = Some(paths::Config::ground(vec![0, 1], case == "path_on"));
    }
    let mut w = World::new(c)?;
    w.agents[0].nutrition = if case == "hunger" {
        5 * HOUR
    } else if case == "death" {
        0
    } else {
        24 * HOUR
    };
    if case == "fatigue" {
        w.agents[0].fatigue = 22 * HOUR;
    }
    if case == "death" {
        w.agents[0].starvation = 23 * HOUR;
    }
    if case != "death" {
        w.goods.add(
            Material::Food,
            if case == "depleted" { 8 } else { 60 },
            Owner::None,
            Holder::Loose(2),
            0,
            48 * HOUR,
        );
        w.goods.add(
            Material::Food,
            6,
            Owner::Agent(0),
            Holder::Agent(0),
            0,
            48 * HOUR,
        );
    }
    // Prior map-only observations: courier traversed the middle node in the past.
    // Recipient has only its own current node; it does not receive a hidden graph.
    if case != "unknown_route" {
        let mut middle = w.node_view(1);
        middle.observed_at = -HOUR;
        middle.people.clear();
        w.agents[0].memory.nodes.insert(1, middle);
    }
    let here = w.node_view(w.config.starts[1]);
    w.agents[1].memory.nodes.insert(here.node, here);
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
    let report = w.capture_report(0)?;
    if assigned {
        w.begin_report_delivery(0, report, 1, 0)?;
    }
    // Recipient is deliberately stationary until a common6h release. No physiology is frozen.
    // Background traffic has fixed departure commands; ordinary movement/queues still apply.
    w.transport_research.actors = Some(BTreeSet::from([0]));
    if case == "congestion" {
        w.apply_intents(
            (2..n)
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
        )?;
    }
    w.audit()?;
    Ok(w)
}
pub fn advance(w: &mut World, case: &str, to: Time) -> Result<(), String> {
    if case == "depleted" && w.now < HOUR && to >= HOUR {
        w.advance_to(HOUR)?;
        let b = w
            .goods
            .batches
            .iter()
            .find(|b| b.holder == Holder::Loose(2) && b.owner == Owner::None && b.quantity > 0)
            .map(|b| b.id);
        if let Some(batch) = b {
            w.apply_intents(vec![(2, Intent::Pickup { batch, quantity: 8 })])?;
        }
    }
    if w.now < 6 * HOUR && to >= 6 * HOUR {
        w.advance_to(6 * HOUR)?;
        w.transport_research.actors = None;
        // Release permits deliberation at the next normal policy opportunity;
        // it must not make arbitrary caller checkpoint times into decision times.
    }
    w.advance_to(to)
}
pub fn decision(w: &World, id: usize) -> Decision {
    let o = w.observe(id).unwrap();
    let mut rng = w.agents[id].rng;
    let mut beliefs = w.agents[id].memory.clone();
    if let Some(local) = &o.local {
        beliefs.nodes.insert(local.node, local.clone());
    }
    person::decide(
        person::Context {
            observation: &o,
            beliefs: &beliefs,
            capabilities: w.person_capabilities(id),
        },
        &mut rng,
    )
}
pub fn metrics(w: &World) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for key in [
        "reports_created",
        "handoffs",
        "belief_updates",
        "old_or_equal_reports_retained",
        "duplicate_handoffs",
        "deliveries_started",
        "deliveries_completed",
        "deliveries_cancelled",
    ] {
        out.insert(key.into(), w.information.journal.count(key).to_string());
    }
    out.insert(
        "received".into(),
        w.information.memories[1].reports.len().to_string(),
    );
    out.insert(
        "receipt_h".into(),
        w.information.memories[1]
            .reports
            .values()
            .next()
            .map(|r| (r.received_at as f64 / HOUR as f64).to_string())
            .unwrap_or_default(),
    );
    out.insert(
        "report_age_h".into(),
        w.information.memories[1]
            .reports
            .values()
            .next()
            .map(|r| {
                ((r.received_at - r.report.observation.observed_at) as f64 / HOUR as f64)
                    .to_string()
            })
            .unwrap_or_default(),
    );
    for id in 0..2 {
        out.insert(
            format!("person{id}_consumed"),
            w.agents[id].consumed.to_string(),
        );
        out.insert(
            format!("person{id}_dead"),
            u8::from(w.agents[id].action == Action::Dead).to_string(),
        );
        out.insert(
            format!("person{id}_nutrition_h"),
            (w.agents[id].nutrition as f64 / HOUR as f64).to_string(),
        );
        out.insert(
            format!("person{id}_starvation_h"),
            (w.household
                .journal
                .starvation_ms
                .get(&id)
                .copied()
                .unwrap_or(0) as f64
                / HOUR as f64)
                .to_string(),
        );
        for tag in ["travel", "queue", "sleep", "gather", "idle"] {
            out.insert(
                format!("person{id}_{tag}_h"),
                (w.household
                    .journal
                    .activity_ms
                    .get(&(id, tag.into()))
                    .copied()
                    .unwrap_or(0) as f64
                    / HOUR as f64)
                    .to_string(),
            );
        }
    }
    out.insert("consumed".into(), w.metrics.consumed.to_string());
    out.insert(
        "starvation_h".into(),
        (w.metrics.deficit_ms as f64 / HOUR as f64).to_string(),
    );
    out.insert("deaths".into(), w.metrics.deaths.to_string());
    out.insert("overtakes".into(), w.metrics.overtakes.to_string());
    out.insert("state_hash".into(), format!("{:016x}", w.state_hash()));
    out.insert("event_hash".into(), format!("{:016x}", w.event_hash()));
    out.insert(
        "report_copies".into(),
        w.information
            .memories
            .iter()
            .map(|m| m.reports.len())
            .sum::<usize>()
            .to_string(),
    );
    out.insert(
        "active_reported_nodes".into(),
        w.information
            .memories
            .iter()
            .map(|m| m.reported_nodes.len())
            .sum::<usize>()
            .to_string(),
    );
    for reason in [
        "DEATH",
        "RENDEZVOUS_RECIPIENT_ABSENT",
        "REPORT_NOT_HELD",
        "EXPLICIT_CANCEL",
    ] {
        out.insert(
            format!("cancel_{}", reason.to_lowercase()),
            w.events
                .iter()
                .filter(|e| {
                    e.kind == "ReportDeliveryCancelled"
                        && e.detail.contains(&format!("reason={reason}"))
                })
                .count()
                .to_string(),
        );
    }
    out
}
/// Research truth is written to a separate artifact and never returned to Person.
/// In live runs this is the first minute checkpoint at/after receipt, not a claim
/// of an exact sub-minute world snapshot. Matched receipt is sampled immediately.
pub fn truth_checkpoints(w: &World, recipient: usize) -> Vec<(ReportId, String)> {
    w.information.memories[recipient]
        .reports
        .iter()
        .map(|(&id, r)| {
            let v = &r.report.observation;
            let truth = w.node_view(v.node);
            let legal = |foods: &[FoodView]| {
                foods
                    .iter()
                    .filter(|f| {
                        f.expires_at > w.now
                            && (f.owner == Owner::None
                                || f.owner == Owner::Agent(recipient)
                                || (w.config.communal && f.owner == Owner::Commons(0)))
                    })
                    .map(|f| f.available)
                    .sum::<u64>()
            };
            (
                id,
                format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}",
                    v.observed_at,
                    r.received_at,
                    w.now,
                    v.node,
                    legal(&v.food),
                    legal(&truth.food),
                    v,
                    truth
                ),
            )
        })
        .collect()
}
/// Matched physical state after one real journey; only local report handoff differs.
pub fn matched(case: &str, share: bool, seed: u64) -> Result<(World, Decision, Decision), String> {
    let mut w = World::new(super::super::Config {
        information: true,
        seed,
        autonomous: false,
        trace: true,
        nodes: 3,
        starts: vec![1, 0, 1],
        food: vec![],
        boards: vec![],
        communal: false,
        household: Some(household::Config {
            enabled: true,
            ..Default::default()
        }),
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR,
                capacity: 4,
            },
            Edge {
                a: 0,
                b: 2,
                nominal_ms: HOUR,
                capacity: 4,
            },
        ],
        ..Default::default()
    })?;
    w.goods.add(
        Material::Food,
        8,
        Owner::None,
        Holder::Loose(1),
        0,
        if case == "matched_spoiled" {
            HOUR / 2
        } else {
            48 * HOUR
        },
    );
    let report = w.capture_report(0)?;
    if case == "matched_depleted" {
        w.apply_intents(vec![(
            2,
            Intent::Pickup {
                batch: 0,
                quantity: 8,
            },
        )])?;
    }
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 0,
            food: false,
        },
    )])?;
    w.advance_to(HOUR)?;
    let local = w.node_view(0);
    w.agents[1].memory.nodes.insert(0, local);
    let before = decision(&w, 1);
    if share {
        w.receive_report(0, 1, report)?;
    }
    let after = decision(&w, 1);
    w.config.autonomous = true;
    w.next_policy = w.now + MINUTE;
    Ok((w, before, after))
}

pub fn parameters(w: &World) -> String {
    format!("{:#?}", w.config)
}
