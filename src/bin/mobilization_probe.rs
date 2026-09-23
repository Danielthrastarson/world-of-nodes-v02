//! F11A observational runner. No service decisions are injected by this harness.
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
fn h(ms: u128) -> f64 {
    ms as f64 / HOUR as f64
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let case = args
        .first()
        .ok_or("mobilization_probe CASE on|off NEW_DIR [SEEDS]")?;
    let enabled = match args.get(1).map(String::as_str) {
        Some("on") => true,
        Some("off") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("directory")?);
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    fs::create_dir_all(root)?;
    let mut runs = file(root, "runs.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut counts = file(root, "counts.csv")?;
    let mut activity = file(root, "activity.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut social = file(root, "social.tsv")?;
    let mut timeline = file(root, "timeline.csv")?;
    writeln!(
        runs,
        "seed,case,enabled,observations,evaluations,volunteers,arrivals,completed,active_end,active_h,duty_h,muster_travel_h,muster_queue_h,soldier_starvation_h,civilian_starvation_h,military_food,cart_travel_h,food_consumed,food_produced,food_spoiled,deaths,starvation_h,total_queue_h,care_food,care_travel_h,care_queue_h,household_separation_h,residence_changes,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(events, "seed\tid\ttime_ms\tperson\tkind\tdetail")?;
    writeln!(social, "seed\tappraisal")?;
    writeln!(counts, "seed,key,count,unique_people")?;
    writeln!(activity, "seed,person,activity,hours")?;
    writeln!(
        people,
        "seed,person,alive,volunteered,arrived,starvation_h,consumed,location,nutrition_ms,death_ms"
    )?;
    writeln!(
        timeline,
        "seed,time_h,active,without_present_available_caregiver,care_food,produced,consumed,starvation_h,queue_h"
    )?;
    for seed in 0..seeds {
        let mut w = military::research::fixture(case, enabled, seed)?;
        if seed == 0 {
            writeln!(params, "{}", military::research::parameters(&w))?;
        }
        let start = w.now();
        for hour in 0..=24 {
            w.advance_to(start + hour * HOUR)?;
            writeln!(
                timeline,
                "{seed},{hour},{},{},{},{},{},{:.9},{:.9}",
                military::research::active(&w),
                military::research::unserved_dependents(&w),
                w.care_journal().count("food_transferred"),
                military::research::produced(&w),
                w.metrics().consumed,
                h(w.metrics().deficit_ms),
                h(w.metrics().queue_ms)
            )?;
        }
        w.audit()?;
        let j = w.military_journal();
        let c = w.care_journal();
        let m = w.metrics();
        writeln!(
            runs,
            "{seed},{case},{enabled},{},{},{},{},{},{},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{},{:.9},{},{},{},{},{:.9},{:.9},{},{:.9},{:.9},{:.9},{},0,{},{:016x},{:016x}",
            j.count("observations"),
            j.count("evaluations"),
            j.count("volunteers"),
            j.count("arrivals"),
            j.count("TERM_COMPLETED"),
            military::research::active(&w),
            h(j.active_ms),
            h(j.duty_ms),
            h(j.travel_ms),
            h(j.queue_ms),
            h(j.soldier_starvation_ms),
            h(j.civilian_starvation_ms),
            j.food,
            h(j.cart_travel_ms),
            m.consumed,
            military::research::produced(&w),
            m.spoiled,
            m.deaths,
            h(m.deficit_ms),
            h(m.queue_ms),
            c.count("food_transferred"),
            h(c.care_travel_ms),
            h(c.care_queue_ms),
            h(c.separation_ms),
            c.residence_changes,
            m.overtakes,
            w.state_hash(),
            w.event_hash()
        )?;
        for (key, count) in &j.counts {
            writeln!(
                counts,
                "{seed},{key},{count},{}",
                j.people.get(key).map_or(0, |p| p.len())
            )?;
        }
        for ((person, act), ms) in &c.activity_ms {
            writeln!(activity, "{seed},{person},{act},{:.9}", h(*ms))?;
        }
        for a in w.agents() {
            let v = j
                .people
                .get("volunteers")
                .is_some_and(|s| s.contains(&a.id));
            let r = j.people.get("arrivals").is_some_and(|s| s.contains(&a.id));
            writeln!(
                people,
                "{seed},{},{},{v},{r},{:.9},{},\"{:?}\",{},{}",
                a.id,
                a.action != Action::Dead,
                h(c.starvation_ms.get(&a.id).copied().unwrap_or(0)),
                a.consumed,
                a.location,
                a.nutrition,
                a.death_time.map(|x| x.to_string()).unwrap_or_default()
            )?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{}\t{:?}\t{}\t{}",
                    e.id, e.at, e.agent, e.kind, e.detail
                )?;
            }
            for a in &w.social_journal().appraisals {
                writeln!(social, "{seed}\t{a:?}")?;
            }
        }
    }
    Ok(())
}
