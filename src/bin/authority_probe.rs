//! F10 controlled delegation and paired autonomous participation, not a government AI.
use std::{
    collections::BTreeSet,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::*;
fn file(root: &Path, name: &str) -> std::io::Result<BufWriter<fs::File>> {
    Ok(BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))?,
    ))
}
fn appoint(w: &mut World, holder: usize, node: usize) -> Result<(), String> {
    w.apply_intents(vec![(
        0,
        Intent::AppointSteward {
            candidate: holder,
            settlement: node,
        },
    )])
}
fn travel(w: &mut World, id: usize, edge: usize, to: usize) -> Result<(), String> {
    w.apply_intents(vec![(
        id,
        Intent::Travel {
            edge,
            target: to,
            food: false,
        },
    )])
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let case = args
        .first()
        .ok_or("authority_probe CASE command|pending|commissioned NEW_DIR [SEEDS]")?;
    let arm = args.get(1).ok_or("arm")?;
    let project = ["sufficient", "scarce", "discovery"].contains(&case.as_str());
    if (!project
        && ![
            "local",
            "distant",
            "bottleneck",
            "death_before",
            "death_after",
            "departure",
            "multiple",
        ]
        .contains(&case.as_str()))
        || (project && !["pending", "commissioned"].contains(&arm.as_str()))
        || (!project && arm != "command")
    {
        return Err("unregistered case/arm".into());
    }
    let root = Path::new(args.get(2).ok_or("new output path")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = args
        .get(3)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(if project { 30 } else { 1 });
    let mut runs = file(root, "runs.csv")?;
    let mut offices = file(root, "offices.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut transitions = file(root, "authority.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut projects = file(root, "projects.tsv")?;
    let mut people = file(root, "people.csv")?;
    let mut trips = file(root, "trips.tsv")?;
    writeln!(
        runs,
        "seed,hour,appointments,active_settlements,active_administration_h,unadministered_settlement_h,commissions,started,completed,wood,labor_h,donors,workers,improved_users,improved_trips,steward_deaths,steward_food,sovereign_food,steward_travel_h,steward_queue_h,alive,consumed,spoiled,delivered,starvation_h,queue_h,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(
        offices,
        "seed,office,polity,holder,settlement,appointment_ms,first_active_ms,active_h,unadministered_h,death_ms,departures,consumed,travel_h,queue_h,dead_without_replacement"
    )?;
    writeln!(events, "seed\tat_ms\tagent\tkind\tdetail")?;
    writeln!(transitions, "seed\tat_ms\toffice\tactive\treason")?;
    writeln!(
        people,
        "seed,id,alive,consumed,nutrition_ms,fatigue_ms,starvation_ms,location"
    )?;
    for seed in 0..seeds {
        let c = Config {
            seed,
            trace: seed == 0,
            autonomous: project,
            prior_memory: project && case != "discovery",
            authority: Some(authority::Config {
                polity_id: 17,
                sovereign: 0,
                settlements: vec![1, 2],
            }),
            starts: vec![if project { 1 } else { 0 }; 50],
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: if project {
                        HOUR / 4
                    } else {
                        match case.as_str() {
                            "distant" => 8 * HOUR,
                            "bottleneck" => 2 * HOUR,
                            "death_before" => 40 * HOUR,
                            _ => HOUR / 2,
                        }
                    },
                    capacity: if case == "bottleneck" { 1 } else { 4 },
                },
                Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: if project { HOUR / 4 } else { HOUR / 2 },
                    capacity: if project { 0 } else { 4 },
                },
            ],
            food: if project {
                vec![(1, 300, 0, 48 * HOUR), (2, 2700, 0, 48 * HOUR)]
            } else {
                vec![(2, 3000, 0, 48 * HOUR)]
            },
            initial_request: if project { 600 } else { 0 },
            infrastructure: project.then(|| infrastructure::Config {
                enabled: true,
                wood: vec![(1, if case == "scarce" { 5 } else { 100 })],
                sites: vec![infrastructure::Site {
                    edge: 1,
                    node: 1,
                    wood_cost: 20,
                    work_ms: 8 * HOUR,
                    opened_nominal_ms: HOUR / 4,
                    opened_capacity: 4,
                }],
            }),
            ..Config::default()
        };
        writeln!(params, "{c:?}")?;
        let mut w = World::new(c)?;
        let holder = if project { 1 } else { 9 };
        appoint(&mut w, holder, 1)?;
        if project && arm == "commissioned" {
            w.apply_intents(vec![(
                holder,
                Intent::CommissionCrossing {
                    office: 0,
                    project: 0,
                },
            )])?;
        }
        if !project {
            if case == "bottleneck" {
                w.apply_intents(
                    (1..=8)
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
            travel(&mut w, holder, 0, 1)?;
            if case == "multiple" {
                appoint(&mut w, 10, 2)?;
                travel(&mut w, 10, 0, 1)?;
            }
        }
        let mut departed = false;
        let mut second_leg = false;
        // Checkpoints do not alter kernel action boundaries; research commands only at registered times.
        for minute in 0..=72 * 60 {
            w.advance_to(minute * MINUTE)?;
            if case == "departure" && !departed && w.now() == 3 * HOUR / 2 {
                travel(&mut w, holder, 0, 0)?;
                departed = true;
            }
            if case == "multiple" && !second_leg && w.agents()[10].location == Location::Node(1) {
                travel(&mut w, 10, 1, 2)?;
                second_leg = true;
            }
            if ![0, 12 * 60, 24 * 60, 36 * 60, 48 * 60, 72 * 60].contains(&minute) {
                continue;
            }
            w.audit()?;
            let m = w.metrics();
            let j = w.authority_journal();
            let h = |x: u128| x as f64 / HOUR as f64;
            let active: u128 = j.active_ms.values().sum();
            let targets: BTreeSet<_> = w.offices().iter().map(|o| o.settlement).collect();
            let holders: BTreeSet<_> = w.offices().iter().map(|o| o.holder).collect();
            let counter = w.infrastructure_counters();
            let donors: BTreeSet<_> = w
                .crossing_projects()
                .iter()
                .flat_map(|p| p.donors.keys().copied())
                .collect();
            let workers: BTreeSet<_> = w
                .crossing_projects()
                .iter()
                .flat_map(|p| p.labor_ms.keys().copied())
                .collect();
            writeln!(
                runs,
                "{seed},{},{},{},{:.9},{:.9},{},{},{},{},{:.9},{},{},{},{},{},{},{},{:.9},{:.9},{},{},{},{},{:.9},{:.9},0,{},{:016x},{:016x}",
                minute / 60,
                w.offices().len(),
                w.administered_settlements().len(),
                h(active),
                h(w.now() as u128 * targets.len() as u128 - active),
                w.commissions().len(),
                counter["started"],
                counter["completed"],
                counter["wood"],
                h(counter["labor_ms"]),
                donors.len(),
                workers.len(),
                counter["improved_users"],
                counter["improved_trips"],
                holders
                    .iter()
                    .filter(|&&id| w.agents()[id].action == Action::Dead)
                    .count(),
                holders
                    .iter()
                    .map(|&id| w.agents()[id].consumed)
                    .sum::<u64>(),
                w.agents()[0].consumed,
                h(j.travel_ms.values().sum()),
                h(j.queue_ms.values().sum()),
                w.agents()
                    .iter()
                    .filter(|a| a.action != Action::Dead)
                    .count(),
                m.consumed,
                m.spoiled,
                m.delivered,
                h(m.deficit_ms),
                h(m.queue_ms),
                m.overtakes,
                w.state_hash(),
                w.event_hash()
            )?;
        }
        for o in w.offices() {
            let j = w.authority_journal();
            let a = &w.agents()[o.holder];
            let ts: Vec<_> = j.transitions.iter().filter(|t| t.office == o.id).collect();
            let first = ts
                .iter()
                .find(|t| t.active)
                .map_or(String::new(), |t| t.at.to_string());
            // Settlement union hours; no duplicate offices are used in these fixtures.
            let active = *j.active_ms.get(&o.settlement).unwrap_or(&0) as f64 / HOUR as f64;
            writeln!(
                offices,
                "{seed},{},{},{},{},{},{},{:.9},{:.9},{},{},{},{:.9},{:.9},{}",
                o.id,
                o.polity,
                o.holder,
                o.settlement,
                o.appointed_at,
                first,
                active,
                72.0 - active,
                a.death_time.map_or(String::new(), |t| t.to_string()),
                ts.iter().filter(|t| t.reason == "HOLDER_DEPARTED").count(),
                a.consumed,
                *j.travel_ms.get(&o.holder).unwrap_or(&0) as f64 / HOUR as f64,
                *j.queue_ms.get(&o.holder).unwrap_or(&0) as f64 / HOUR as f64,
                a.action == Action::Dead
            )?;
        }
        for t in &w.authority_journal().transitions {
            writeln!(
                transitions,
                "{seed}\t{}\t{}\t{}\t{}",
                t.at, t.office, t.active, t.reason
            )?;
        }
        for p in w.crossing_projects() {
            writeln!(projects, "{seed}\t{p:?}")?;
        }
        for t in w.crossing_trips() {
            writeln!(trips, "{seed}\t{t:?}")?;
        }
        for a in w.agents() {
            writeln!(
                people,
                "{seed},{},{},{},{},{},{},\"{:?}\"",
                a.id,
                a.action != Action::Dead,
                a.consumed,
                a.nutrition,
                a.fatigue,
                a.starvation,
                a.location
            )?;
        }
        for e in w.events() {
            writeln!(
                events,
                "{seed}\t{}\t{:?}\t{}\t{}",
                e.at, e.agent, e.kind, e.detail
            )?;
        }
    }
    Ok(())
}
