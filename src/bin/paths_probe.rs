//! Registered F7 paired experiments. The autonomous policy never sees fixture names.
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
            .create_new(true)
            .write(true)
            .open(root.join(name))?,
    ))
}
fn configuration(name: &str, seed: u64, enabled: bool) -> Result<Config, String> {
    let edge = |a, b, time, capacity| Edge {
        a,
        b,
        nominal_ms: time,
        capacity,
    };
    let mut c = Config {
        seed,
        trace: seed == 0,
        initial_request: 600,
        edges: vec![edge(0, 1, HOUR / 2, 4), edge(1, 2, HOUR / 2, 4)],
        ..Config::default()
    };
    match name {
        "line" => {}
        "local_food" => {
            c.food = vec![(0, 3000, 0, 48 * HOUR)];
            c.prior_memory = true;
        }
        "diamond" | "detour" => {
            c.nodes = 4;
            c.food = vec![(3, 3000, 0, 48 * HOUR)];
            c.edges = vec![
                edge(0, 1, HOUR / 2, if name == "detour" { 1 } else { 4 }),
                edge(1, 3, HOUR / 2, if name == "detour" { 1 } else { 4 }),
                edge(
                    0,
                    2,
                    if name == "detour" {
                        5 * HOUR / 8
                    } else {
                        HOUR / 2
                    },
                    4,
                ),
                edge(
                    2,
                    3,
                    if name == "detour" {
                        5 * HOUR / 8
                    } else {
                        HOUR / 2
                    },
                    4,
                ),
            ];
        }
        "dead_end" => {
            c.nodes = 4;
            c.edges.push(edge(0, 3, HOUR / 2, 4));
        }
        "crossing" => {
            c.edges = vec![edge(0, 1, HOUR / 4, 4), edge(1, 2, HOUR / 4, 0)];
            c.food = vec![(1, 300, 0, 48 * HOUR), (2, 2700, 0, 48 * HOUR)];
            c.prior_memory = true;
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
        "carts" => {
            c.prior_memory = true;
            c.capital = Some(capital::Config {
                rules: capital::Rules::default(),
                wood: vec![(0, 500)],
            });
        }
        _ => return Err("unregistered path fixture".into()),
    }
    c.paths = Some(paths::Config::ground(
        if name == "crossing" {
            vec![0]
        } else {
            (0..c.edges.len()).collect()
        },
        enabled,
    ));
    Ok(c)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let name = args
        .first()
        .ok_or("paths_probe CASE enabled|frozen NEW_DIR [SEEDS]")?;
    let enabled = match args.get(1).map(String::as_str) {
        Some("enabled") => true,
        Some("frozen") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("new output path")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(root, "runs.csv")?;
    let mut surfaces = file(root, "surfaces.csv")?;
    let mut trips = file(root, "trips.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut assets = file(root, "assets.tsv")?;
    writeln!(
        runs,
        "seed,hour,alive,consumed,spoiled,delivered,starvation_h,queue_h,sleep_h,visited_food,at_c,at_food,completed_trips,matched_work_reduction_h,overtakes,accounting_error,state_hash,event_hash"
    )?;
    writeln!(
        surfaces,
        "seed,hour,edge,a,b,capacity,base_ms,effective_ms,stage,compaction,passages,recovery_count,last_passage_ms,next_recovery_ms,queue_h,occupancy_person_h"
    )?;
    writeln!(
        trips,
        "seed,agent,edge,from,to,admitted_ms,arrived_ms,nominal_ms,untreated_ms,compaction,food_departed,load_departed"
    )?;
    writeln!(people, "seed,id,alive,nutrition_ms,consumed,location")?;
    for seed in 0..seeds {
        let c = configuration(name, seed, enabled)?;
        let graph = c.edges.clone();
        let food_node = if name == "diamond" || name == "detour" {
            3
        } else if name == "local_food" {
            0
        } else {
            2
        };
        writeln!(params, "{c:?}")?;
        let mut w = World::new(c)?;
        for hour in 0..=72 {
            w.advance_to(hour * HOUR)?;
            w.audit()?;
            if [0, 12, 24, 36, 48, 72].contains(&hour) {
                let m = w.metrics();
                let count = |n| {
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead && a.location == Location::Node(n))
                        .count()
                };
                let completed: Vec<_> = w
                    .path_trips()
                    .iter()
                    .filter(|t| t.arrived.is_some())
                    .collect();
                let visited = completed
                    .iter()
                    .filter(|t| t.to == food_node)
                    .map(|t| t.agent)
                    .collect::<BTreeSet<_>>()
                    .len();
                let reduction: i64 = completed
                    .iter()
                    .map(|t| t.untreated_ms - t.nominal_ms)
                    .sum();
                writeln!(
                    runs,
                    "{seed},{hour},{},{},{},{},{:.9},{:.9},{:.9},{},{},{},{},{:.9},{},0,{:016x},{:016x}",
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead)
                        .count(),
                    m.consumed,
                    m.spoiled,
                    m.delivered,
                    m.deficit_ms as f64 / HOUR as f64,
                    m.queue_ms as f64 / HOUR as f64,
                    m.sleep_ms as f64 / HOUR as f64,
                    visited,
                    count(0),
                    count(food_node),
                    completed.len(),
                    reduction as f64 / HOUR as f64,
                    m.overtakes,
                    w.state_hash(),
                    w.event_hash()
                )?;
                for (&edge, s) in w.path_surfaces() {
                    let e = w.effective_edge(edge);
                    let (q, o) = w.path_edge_time(edge);
                    writeln!(
                        surfaces,
                        "{seed},{hour},{edge},{},{},{},{},{},{},{},{},{},{},{},{:.9},{:.9}",
                        e.a,
                        e.b,
                        e.capacity,
                        graph[edge].nominal_ms,
                        e.nominal_ms,
                        s.stage(),
                        s.compaction,
                        s.passages,
                        s.recovery_count,
                        s.last_passage.map_or(String::new(), |t| t.to_string()),
                        s.next_recovery.map_or(String::new(), |t| t.to_string()),
                        q as f64 / HOUR as f64,
                        o as f64 / HOUR as f64
                    )?;
                }
            }
        }
        for t in w.path_trips() {
            writeln!(
                trips,
                "{seed},{},{},{},{},{},{},{},{},{},{},{}",
                t.agent,
                t.edge,
                t.from,
                t.to,
                t.admitted,
                t.arrived.map_or(String::new(), |v| v.to_string()),
                t.nominal_ms,
                t.untreated_ms,
                t.compaction_at_admission,
                t.food,
                t.load
            )?;
        }
        for a in w.agents() {
            writeln!(
                people,
                "{seed},{},{},{},{},\"{:?}\"",
                a.id,
                a.action != Action::Dead,
                a.nutrition,
                a.consumed,
                a.location
            )?;
        }
        writeln!(
            assets,
            "seed={seed} crossings={:?} carts={:?}",
            w.crossing_projects(),
            w.carts()
        )?;
        if seed == 0 {
            for e in w.events() {
                writeln!(events, "{e:?}")?;
            }
        }
        runs.flush()?;
        surfaces.flush()?;
        eprintln!(
            "{name} enabled={enabled} seed={seed} alive={} consumed={} starvation_h={:.3} passages={}",
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count(),
            w.metrics().consumed,
            w.metrics().deficit_ms as f64 / HOUR as f64,
            w.path_surfaces().values().map(|s| s.passages).sum::<u64>()
        );
    }
    Ok(())
}
