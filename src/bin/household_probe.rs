//! B1 paired initial-condition experiments; no research truth enters choice.
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
    let case = args
        .first()
        .ok_or("household_probe CASE on|off NEW_DIR [SEEDS]")?;
    let enabled = match args.get(1).map(String::as_str) {
        Some("on") => true,
        Some("off") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("new output directory")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(root, "runs.csv")?;
    let mut metrics = file(root, "metrics.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut activities = file(root, "activity.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut states = file(root, "social-final.txt")?;
    writeln!(
        runs,
        "seed,hour,alive,deaths,consumed,spoiled,produced,starvation_h,care_goals,care_transfers,food_transferred,care_travel_h,care_queue_h,separation_pair_h,caregiver_losses,residence_changes,crossings_completed,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(
        people,
        "seed,hour,agent,alive,nutrition_ms,starvation_ms,starvation_total_h,consumed,retained_food,death_ms,location"
    )?;
    writeln!(metrics, "seed,key,count")?;
    writeln!(activities, "seed,agent,activity,hours")?;
    writeln!(events, "seed,time_ms,agent,kind,detail")?;
    for seed in 0..seeds {
        let mut w = household::research::fixture(case, enabled, seed)?;
        writeln!(
            params,
            "seed={seed}\n{}",
            household::research::parameters(&w)
        )?;
        for h in 0..=48 {
            w.advance_to(h * HOUR)?;
            w.audit()?;
            if ![0, 1, 6, 12, 24, 36, 48].contains(&h) {
                continue;
            }
            let j = w.care_journal();
            let m = w.metrics();
            writeln!(
                runs,
                "{seed},{h},{},{},{},{},{},{:.9},{},{},{},{:.9},{:.9},{:.9},{},{},{},0,{},{:016x},{:016x}",
                w.agents()
                    .iter()
                    .filter(|a| a.action != Action::Dead)
                    .count(),
                m.deaths,
                m.consumed,
                m.spoiled,
                household::research::production(&w),
                m.deficit_ms as f64 / HOUR as f64,
                j.count("goals"),
                j.count("transfers"),
                j.count("food_transferred"),
                j.care_travel_ms as f64 / HOUR as f64,
                j.care_queue_ms as f64 / HOUR as f64,
                j.separation_ms as f64 / HOUR as f64,
                j.losses.len(),
                j.residence_changes,
                w.crossing_projects()
                    .iter()
                    .filter(|p| p.completed_at.is_some())
                    .count(),
                m.overtakes,
                w.state_hash(),
                w.event_hash()
            )?;
            for a in w.agents() {
                let retained: u64 = w
                    .batches()
                    .iter()
                    .filter(|b| b.owner == Owner::Agent(a.id) && b.material == Material::Food)
                    .map(|b| b.quantity)
                    .sum();
                writeln!(
                    people,
                    "{seed},{h},{},{},{},{},{:.9},{},{},{},\"{:?}\"",
                    a.id,
                    a.action != Action::Dead,
                    a.nutrition,
                    a.starvation,
                    j.starvation_ms.get(&a.id).copied().unwrap_or(0) as f64 / HOUR as f64,
                    a.consumed,
                    retained,
                    a.death_time.map_or(String::new(), |t| t.to_string()),
                    a.location
                )?;
            }
        }
        for (k, v) in &w.care_journal().counts {
            writeln!(metrics, "{seed},{k},{v}")?;
        }
        for ((id, a), v) in &w.care_journal().activity_ms {
            writeln!(activities, "{seed},{id},{a},{:.9}", *v as f64 / HOUR as f64)?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{:?}\t{}\t{}",
                    e.at, e.agent, e.kind, e.detail
                )?;
            }
            for a in w.agents() {
                writeln!(states, "person={} view={:?}", a.id, w.household_view(a.id))?;
            }
        }
    }
    Ok(())
}
