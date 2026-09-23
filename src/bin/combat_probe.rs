//! F11C controlled raid, voluntary defense, immutable observational output.
use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::*;
fn file(p: &Path, n: &str) -> std::io::Result<BufWriter<fs::File>> {
    Ok(BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(p.join(n))?,
    ))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = env::args().skip(1).collect();
    let case = a
        .first()
        .ok_or("combat_probe CASE undefended|defended|peace|peace_defended NEW_DIR [SEEDS]")?;
    let arm = a.get(1).ok_or("arm")?;
    let (defense, threat) = match arm.as_str() {
        "undefended" => (false, true),
        "defended" => (true, true),
        "peace" => (false, false),
        "peace_defended" => (true, false),
        _ => return Err("arm".into()),
    };
    let root = Path::new(a.get(2).ok_or("directory")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = a.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(root, "runs.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut people = file(root, "people.csv")?;
    let mut timeline = file(root, "timeline.csv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut goods = file(root, "goods.tsv")?;
    let mut activity = file(root, "activity.csv")?;
    writeln!(events, "seed\tid\ttime_ms\tperson\tkind\tdetail")?;
    writeln!(goods, "seed\tbatch")?;
    writeln!(activity, "seed,person,activity,hours")?;
    writeln!(
        timeline,
        "seed,time_h,stock,stolen,hits,incapacitations,deaths,active_defenders,starvation_h,care_food"
    )?;
    writeln!(
        people,
        "seed,person,alive,injuries,nutrition_h,fatigue_h,food,starvation_h,mustered,raid_quantity,location"
    )?;
    for seed in 0..seeds {
        let mut w = combat::research::fixture(case, defense, threat, seed)?;
        let receipts = combat::research::prepare(&mut w, defense)?;
        if seed == 0 {
            writeln!(
                params,
                "{}\ncase={case}; arm={arm}; receipts={receipts}; outcome_window=24h absolute; threat begins2h; protect issued1h05m",
                combat::research::parameters(&w)
            )?;
        }
        for quarter in 5..=96 {
            if std::env::var_os("COMBAT_PROGRESS").is_some() {
                eprintln!("{case} {arm} seed={seed} advancing to quarter={quarter}");
            }
            w.advance_to(quarter * 15 * MINUTE)?;
            let j = w.combat_journal();
            writeln!(
                timeline,
                "{seed},{:.2},{},{},{},{},{},{},{:.9},{}",
                quarter as f64 / 4.,
                combat::research::stock(&w),
                j.stolen,
                j.count("hits"),
                j.count("incapacitations"),
                w.metrics().deaths,
                w.unit_snapshot(0).map_or(0, |s| s.active),
                w.metrics().deficit_ms as f64 / HOUR as f64,
                w.care_journal().count("food_transferred")
            )?;
        }
        w.audit()?;
        let mut r = combat::research::aggregate(&w);
        r.insert("seed".into(), seed.to_string());
        r.insert("case".into(), case.clone());
        r.insert("arm".into(), arm.clone());
        r.insert("receipts".into(), receipts.to_string());
        r.insert("state_hash".into(), format!("{:016x}", w.state_hash()));
        r.insert("event_hash".into(), format!("{:016x}", w.event_hash()));
        if seed == 0 {
            writeln!(runs, "{}", r.keys().cloned().collect::<Vec<_>>().join(","))?;
        }
        writeln!(
            runs,
            "{}",
            r.values().cloned().collect::<Vec<_>>().join(",")
        )?;
        for p in w.agents() {
            writeln!(
                people,
                "{seed},{},{},{},{:.9},{:.9},{},{:.9},{},{},\"{:?}\"",
                p.id,
                p.action != Action::Dead,
                w.body(p.id).unwrap().injuries,
                p.nutrition as f64 / HOUR as f64,
                p.fatigue as f64 / HOUR as f64,
                p.consumed,
                w.care_journal()
                    .starvation_ms
                    .get(&p.id)
                    .copied()
                    .unwrap_or(0) as f64
                    / HOUR as f64,
                w.units().values().any(|u| u
                    .members
                    .get(&p.id)
                    .is_some_and(|m| m.assembled_at.is_some())),
                w.events()
                    .iter()
                    .filter(|e| e.agent == Some(p.id) && e.kind == "FoodStolen")
                    .count(),
                p.location
            )?;
        }
        for ((id, tag), q) in &w.care_journal().activity_ms {
            writeln!(activity, "{seed},{id},{tag},{:.9}", *q as f64 / HOUR as f64)?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{}\t{:?}\t{}\t{}",
                    e.id, e.at, e.agent, e.kind, e.detail
                )?;
            }
            for b in w.batches() {
                writeln!(goods, "{seed}\t{b:?}")?;
            }
        }
    }
    Ok(())
}
