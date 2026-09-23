//! F6 registered research fixtures. No scenario name is visible to autonomous policy.
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let name = args
        .first()
        .ok_or("infrastructure_probe CASE available|unavailable NEW_DIR [SEEDS]")?;
    if ![
        "closed",
        "short",
        "bottleneck",
        "alternate",
        "scarce",
        "discovery",
        "local_food",
    ]
    .contains(&name.as_str())
    {
        return Err("unregistered case".into());
    }
    let enabled = match args.get(1).map(String::as_str) {
        Some("available") => true,
        Some("unavailable") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("new output path")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(root, "runs.csv")?;
    let mut metrics = file(root, "metrics.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut projects = file(root, "projects.tsv")?;
    let mut trips = file(root, "trips.csv")?;
    let mut traces = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut edges = file(root, "edges.csv")?;
    writeln!(
        runs,
        "seed,hour,alive,consumed,spoiled,delivered,starvation_h,queue_h,at_c,at_x,at_f,visited_f,completed,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(metrics, "seed,hour,key,value")?;
    writeln!(
        people,
        "seed,id,alive,nutrition_ms,starvation_ms,consumed,location"
    )?;
    writeln!(
        trips,
        "seed,agent,edge,from,to,admitted_ms,arrived_ms,nominal_ms,improved,food_departed,wood_departed"
    )?;
    writeln!(
        edges,
        "seed,edge,queue_h,occupancy_person_h,admissions,completed,food_departed,wood_departed"
    )?;
    for seed in 0..seeds {
        let mut graph = vec![
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
                capacity: if name == "short" { 4 } else { 0 },
            },
        ];
        if name == "alternate" {
            graph.extend([
                Edge {
                    a: 1,
                    b: 3,
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
                Edge {
                    a: 3,
                    b: 2,
                    nominal_ms: 2 * HOUR,
                    capacity: 4,
                },
            ]);
        }
        let c = Config {
            nodes: if name == "alternate" { 4 } else { 3 },
            edges: graph,
            seed,
            trace: seed == 0,
            prior_memory: name != "discovery",
            food: if name == "local_food" {
                vec![(1, 3000, 0, 48 * HOUR)]
            } else {
                vec![(1, 300, 0, 48 * HOUR), (2, 2700, 0, 48 * HOUR)]
            },
            initial_request: 600,
            infrastructure: Some(infrastructure::Config {
                enabled,
                wood: vec![(0, if name == "scarce" { 5 } else { 100 })],
                sites: vec![infrastructure::Site {
                    edge: 1,
                    node: 1,
                    wood_cost: 20,
                    work_ms: 8 * HOUR,
                    opened_nominal_ms: if name == "short" {
                        3 * HOUR / 16
                    } else {
                        HOUR / 4
                    },
                    opened_capacity: if name == "bottleneck" { 1 } else { 4 },
                }],
            }),
            ..Config::default()
        };
        writeln!(params, "{c:?}")?;
        let edge_count = c.edges.len();
        let mut w = World::new(c)?;
        for h in 0..=72 {
            w.advance_to(h * HOUR)?;
            w.audit()?;
            if [0, 12, 24, 36, 48, 72].contains(&h) {
                let m = w.metrics();
                let n = |node| {
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead && a.location == Location::Node(node))
                        .count()
                };
                let visited = w
                    .crossing_trips()
                    .iter()
                    .filter(|t| t.to == 2 && t.arrived.is_some())
                    .map(|t| t.agent)
                    .collect::<BTreeSet<_>>()
                    .len();
                writeln!(
                    runs,
                    "{seed},{h},{},{},{},{},{:.9},{:.9},{},{},{},{},{},0,{},{:016x},{:016x}",
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead)
                        .count(),
                    m.consumed,
                    m.spoiled,
                    m.delivered,
                    m.deficit_ms as f64 / HOUR as f64,
                    m.queue_ms as f64 / HOUR as f64,
                    n(0),
                    n(1),
                    n(2),
                    visited,
                    w.infrastructure_counters()["completed"],
                    m.overtakes,
                    w.state_hash(),
                    w.event_hash()
                )?;
                for (k, v) in w.infrastructure_counters() {
                    writeln!(metrics, "{seed},{h},{k},{v}")?;
                }
            }
        }
        for a in w.agents() {
            writeln!(
                people,
                "{seed},{},{},{},{},{},\"{:?}\"",
                a.id,
                a.action != Action::Dead,
                a.nutrition,
                a.starvation,
                a.consumed,
                a.location
            )?;
        }
        for p in w.crossing_projects() {
            writeln!(projects, "{seed}\t{p:?}")?;
        }
        for t in w.crossing_trips() {
            writeln!(
                trips,
                "{seed},{},{},{},{},{},{},{},{},{},{}",
                t.agent,
                t.edge,
                t.from,
                t.to,
                t.admitted,
                t.arrived.map_or(String::new(), |x| x.to_string()),
                t.nominal_ms,
                t.improved,
                t.food,
                t.wood
            )?;
        }
        for edge in 0..edge_count {
            let (q, o) = w.infrastructure_edge_time(edge);
            let ts: Vec<_> = w
                .crossing_trips()
                .iter()
                .filter(|t| t.edge == edge)
                .collect();
            writeln!(
                edges,
                "{seed},{edge},{:.9},{:.9},{},{},{},{}",
                q as f64 / HOUR as f64,
                o as f64 / HOUR as f64,
                ts.len(),
                ts.iter().filter(|t| t.arrived.is_some()).count(),
                ts.iter().map(|t| t.food).sum::<u64>(),
                ts.iter().map(|t| t.wood).sum::<u64>()
            )?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(traces, "{e:?}")?;
            }
        }
        runs.flush()?;
        metrics.flush()?;
        eprintln!(
            "{name} enabled={enabled} seed={seed} {:?} consumed={} starvation_h={:.3}",
            w.infrastructure_counters(),
            w.metrics().consumed,
            w.metrics().deficit_ms as f64 / HOUR as f64
        );
    }
    Ok(())
}
