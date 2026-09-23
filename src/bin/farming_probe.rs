//! Registered F3 paired fixtures. Configuration is research-only, never a policy input.
use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::*;
fn output(root: &Path, name: &str) -> std::io::Result<BufWriter<fs::File>> {
    Ok(BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))?,
    ))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let map = args
        .first()
        .ok_or("farming_probe line|branch none|base|slow|low_yield C|F NEW_DIR [SEEDS]")?;
    let model = args.get(1).ok_or("model")?;
    let site = args.get(2).ok_or("site")?;
    let root = Path::new(args.get(3).ok_or("output directory")?);
    let seeds: u64 = args.get(4).map(|x| x.parse()).transpose()?.unwrap_or(30);
    if !["line", "branch"].contains(&map.as_str())
        || !["none", "base", "slow", "low_yield"].contains(&model.as_str())
        || !["C", "F"].contains(&site.as_str())
    {
        return Err("unregistered fixture".into());
    }
    fs::create_dir_all(root)?;
    let mut runs = output(root, "runs.csv")?;
    writeln!(
        runs,
        "seed,hour,alive,at_c,at_f,deaths,consumed,spoiled,starvation_hours,queue_hours,sleep_hours,idle_hours,jobs,delivered,queue_overtakes,accounting_error,event_hash,state_hash"
    )?;
    let mut crops = output(root, "crops.csv")?;
    writeln!(crops, "seed,hour,key,value")?;
    let mut people = output(root, "people.csv")?;
    writeln!(
        people,
        "seed,hour,id,alive,nutrition_ms,fatigue_ms,starvation_ms,consumed,location"
    )?;
    let mut trace = output(root, "events.tsv")?;
    let mut params = output(root, "parameters.txt")?;
    let mut activities = output(root, "activity.csv")?;
    writeln!(activities, "seed,hour,activity,person_hours")?;
    let mut states = output(root, "plots.tsv")?;
    for seed in 0..seeds {
        let mut config = geography::configuration(map, seed, seed == 0)?;
        config.initial_request = 0;
        if model != "none" {
            let mut rules = farming::Rules::default();
            if model == "slow" {
                rules.grow_ms = 36 * HOUR;
            }
            if model == "low_yield" {
                rules.yield_quanta = 12;
            }
            config.farming = Some(farming::Config {
                units: vec![if site == "C" { 0 } else { 2 }; 50],
                rules,
            });
        }
        writeln!(params, "seed={seed} {config:?}")?;
        let mut w = World::new(config)?;
        for h in 0..=168 {
            w.advance_to(h * HOUR)?;
            w.audit()?;
            if [0, 36, 48, 72, 120, 168].contains(&h) {
                let m = w.metrics();
                writeln!(
                    runs,
                    "{seed},{h},{},{},{},{},{},{},{:.9},{:.9},{:.9},{:.9},{},{},{},0,{:016x},{:016x}",
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead)
                        .count(),
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead && a.location == Location::Node(0))
                        .count(),
                    w.agents()
                        .iter()
                        .filter(|a| a.action != Action::Dead && a.location == Location::Node(2))
                        .count(),
                    m.deaths,
                    m.consumed,
                    m.spoiled,
                    m.deficit_ms as f64 / HOUR as f64,
                    m.queue_ms as f64 / HOUR as f64,
                    m.sleep_ms as f64 / HOUR as f64,
                    m.idle_ms as f64 / HOUR as f64,
                    m.jobs,
                    m.delivered,
                    m.overtakes,
                    w.event_hash(),
                    w.state_hash()
                )?;
                for (k, v) in w.farming_counters() {
                    writeln!(crops, "{seed},{h},{k},{v}")?;
                }
                for (k, v) in w.farming_activity_ms() {
                    writeln!(activities, "{seed},{h},{k},{:.9}", *v as f64 / HOUR as f64)?;
                }
                for a in w.agents() {
                    writeln!(
                        people,
                        "{seed},{h},{},{},{},{},{},{},\"{:?}\"",
                        a.id,
                        a.action != Action::Dead,
                        a.nutrition,
                        a.fatigue,
                        a.starvation,
                        a.consumed,
                        a.location
                    )?;
                }
                if seed == 0 {
                    for p in w.plots() {
                        writeln!(states, "{h}\t{p:?}")?;
                    }
                }
            }
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(trace, "{e:?}")?;
            }
        }
        runs.flush()?;
        crops.flush()?;
        eprintln!(
            "seed {seed}: {:?}; alive={}",
            w.farming_counters(),
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count()
        );
    }
    Ok(())
}
