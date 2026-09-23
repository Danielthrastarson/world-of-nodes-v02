//! Pre-registered F5 research worlds; no fixture labels reach NPC policy.
use std::{
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let name = args
        .first()
        .ok_or("capital_probe SCENARIO available|unavailable NEW_DIR [SEEDS]")?;
    let enabled = match args.get(1).map(String::as_str) {
        Some("available") => true,
        Some("unavailable") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("output directory")?);
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let (edge_time, request, wood, prior) = match name.as_str() {
        "short_low" => (HOUR / 4, 30, 500, true),
        "short_high" => (HOUR / 4, 600, 500, true),
        "medium_high" => (HOUR / 2, 600, 500, true),
        "long_high" => (2 * HOUR, 600, 500, true),
        "wood_scarce" => (HOUR / 4, 600, 5, true),
        "wood_contested" => (HOUR / 4, 600, 10, true),
        "discovery" => (HOUR / 2, 600, 500, false),
        _ => return Err("unregistered scenario".into()),
    };
    fs::create_dir_all(root)?;
    let mut runs = file(root, "runs.csv")?;
    writeln!(
        runs,
        "seed,hour,alive,deaths,consumed,spoiled,delivered,jobs,starvation_hours,queue_hours,accounting_error,overtakes,event_hash,state_hash"
    )?;
    let mut metrics = file(root, "metrics.csv")?;
    writeln!(metrics, "seed,hour,key,value")?;
    let mut people = file(root, "people.csv")?;
    writeln!(
        people,
        "seed,hour,id,alive,nutrition_ms,starvation_ms,consumed,capacity,load,location"
    )?;
    let mut carts = file(root, "carts.csv")?;
    writeln!(
        carts,
        "seed,id,owner,user,completed_h,first_use_delay_h,wood,construction_labor_h,idle_h,departures,arrivals,cargo_quanta_edges,edible_food_quanta_edges,matched_crossing_extra_quanta,minimum_extra_backpack_crossings,location"
    )?;
    let mut trips = file(root, "trips.tsv")?;
    let mut events = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    for seed in 0..seeds {
        let c = Config {
            seed,
            trace: seed == 0,
            prior_memory: prior,
            initial_request: request,
            capital: Some(capital::Config {
                rules: capital::Rules {
                    enabled,
                    ..capital::Rules::default()
                },
                wood: vec![(0, wood)],
            }),
            edges: vec![
                Edge {
                    a: 0,
                    b: 1,
                    nominal_ms: edge_time,
                    capacity: 4,
                },
                Edge {
                    a: 1,
                    b: 2,
                    nominal_ms: edge_time,
                    capacity: 4,
                },
            ],
            ..Config::default()
        };
        writeln!(params, "seed={seed} {c:?}")?;
        let mut w = World::new(c)?;
        for h in 0..=72 {
            w.advance_to(h * HOUR)?;
            w.audit()?;
            if [0, 12, 24, 36, 48, 72].contains(&h) {
                let m = w.metrics();
                writeln!(
                    runs,
                    "{seed},{h},{},{},{},{},{},{},{:.9},{:.9},0,{}, {:016x},{:016x}",
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead)
                        .count(),
                    m.deaths,
                    m.consumed,
                    m.spoiled,
                    m.delivered,
                    m.jobs,
                    m.deficit_ms as f64 / HOUR as f64,
                    m.queue_ms as f64 / HOUR as f64,
                    m.overtakes,
                    w.event_hash(),
                    w.state_hash()
                )?;
                for (k, v) in w.capital_counters() {
                    writeln!(metrics, "{seed},{h},{k},{v}")?;
                }
                for a in w.agents() {
                    writeln!(
                        people,
                        "{seed},{h},{},{},{},{},{},{},{},\"{:?}\"",
                        a.id,
                        a.action != Action::Dead,
                        a.nutrition,
                        a.starvation,
                        a.consumed,
                        w.carrier_capacity(a.id),
                        w.carrier_load(a.id),
                        a.location
                    )?;
                }
            }
        }
        for c in w.carts() {
            let p = w
                .cart_projects()
                .iter()
                .find(|p| p.cart == Some(c.id))
                .unwrap();
            let completed: Vec<_> = c.trips.iter().filter(|t| t.arrived.is_some()).collect();
            let extra: u64 = completed
                .iter()
                .map(|t| (t.food_arrived + t.backpack_food).saturating_sub(10))
                .sum();
            let extra_crossings: u64 = completed
                .iter()
                .map(|t| {
                    (t.food_arrived + t.backpack_food)
                        .div_ceil(10)
                        .saturating_sub(1)
                })
                .sum();
            writeln!(
                carts,
                "{seed},{},{:?},{:?},{:.9},{},{},{:.9},{:.9},{},{},{},{},{},{},\"{:?}\"",
                c.id,
                c.owner,
                c.user,
                c.completed_at as f64 / HOUR as f64,
                c.first_use.map_or(String::new(), |t| format!(
                    "{:.9}",
                    (t - c.completed_at) as f64 / HOUR as f64
                )),
                c.wood,
                p.labor_ms as f64 / HOUR as f64,
                c.idle_ms as f64 / HOUR as f64,
                c.trips.len(),
                completed.len(),
                completed.iter().map(|t| t.cargo).sum::<u64>(),
                completed.iter().map(|t| t.food_arrived).sum::<u64>(),
                extra,
                extra_crossings,
                c.location
            )?;
            for t in &c.trips {
                writeln!(trips, "{seed}\t{}\t{t:?}", c.id)?;
            }
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(events, "{e:?}")?;
            }
        }
        runs.flush()?;
        metrics.flush()?;
        eprintln!(
            "seed={seed} {:?} delivered={} alive={} starvation_h={}",
            w.capital_counters(),
            w.metrics().delivered,
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count(),
            w.metrics().deficit_ms as f64 / HOUR as f64
        );
    }
    Ok(())
}
