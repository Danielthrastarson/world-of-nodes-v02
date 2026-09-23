//! B2 physical histories, matched states and live paired care worlds.
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
fn kind(d: &Decision) -> &'static str {
    match d.intent {
        Intent::GiveCareFood { .. } => "GIVE",
        Intent::TravelForCare { .. } => "CARE_TRAVEL",
        Intent::Eat { .. } => "EAT",
        Intent::Sleep => "SLEEP",
        Intent::Wait => "WAIT",
        _ => "OTHER",
    }
}
fn fnv(s: &str) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
fn traces(out: &mut impl Write, seed: u64, stage: &str, w: &World) -> std::io::Result<()> {
    for e in w.events() {
        writeln!(
            out,
            "{seed}\t{stage}\t{}\t{:?}\t{}\t{}",
            e.at, e.agent, e.kind, e.detail
        )?;
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = env::args().skip(1).collect();
    let mode = a
        .first()
        .ok_or("affect_probe history|live|cost CASE on|off NEW_DIR [SEEDS]")?;
    let case = a.get(1).ok_or("case")?;
    let enabled = match a.get(2).map(String::as_str) {
        Some("on") => true,
        Some("off") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(a.get(3).ok_or("output")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = a.get(4).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(root, "runs.csv")?;
    let mut rel = file(root, "relationships.csv")?;
    let mut appraisal = file(root, "appraisals.tsv")?;
    let mut events = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut metrics = file(root, "metrics.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut acts = file(root, "activity.csv")?;
    writeln!(
        runs,
        "seed,mode,case,arm,physical_signature,initial_choice,pressure,cost,alive,food_transferred,consumed,spoiled,starvation_h,care_travel_h,care_selected,care_refused,observed,anger_episodes,fear_episodes,grief_episodes,social_edges,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(
        rel,
        "seed,stage,observer,other,trust,attachment,grievance,obligation,anger,fear,grief"
    )?;
    writeln!(events, "seed\tstage\ttime_ms\tagent\tkind\tdetail")?;
    writeln!(appraisal, "seed\tstage\tappraisal")?;
    writeln!(metrics, "seed,key,count")?;
    writeln!(
        people,
        "seed,person,nutrition_ms,consumed,starvation_h,death_ms,anger_active_h,fear_active_h,grief_active_h"
    )?;
    writeln!(acts, "seed,person,activity,hours")?;
    for seed in 0..seeds {
        let mut history = None;
        let mut selected = String::new();
        let mut signature = String::new();
        let mut pressure = 0;
        let mut cost = 0;
        let mut w = if mode == "history" {
            let h = social::research::history(case, seed)?;
            let mut w = social::research::matched(&h, seed)?;
            social::research::set_influence(&mut w, enabled);
            let sig = social::research::physical_signature(&w);
            signature = format!("{:016x}", fnv(&sig));
            // Assert all histories share the same full physical/care/RNG/rights input, not just a hash.
            let neutral =
                social::research::matched(&social::research::history("no_history", seed)?, seed)?;
            if sig != social::research::physical_signature(&neutral) {
                return Err("matched physical inputs differ".into());
            }
            if let Some(score) = social::research::assessment(&w, 1) {
                pressure = score.pressure();
                cost = score.cost();
            }
            let d = social::research::decision(&w, 1);
            selected = kind(&d).into();
            writeln!(
                params,
                "seed={seed} matched BEFORE choice={}\n{}",
                d.reason,
                social::research::parameters(&w)
            )?;
            history = Some(h);
            social::research::enact(&mut w, 1, d)?;
            w
        } else if mode == "cost" && ["surplus", "scarce"].contains(&case.as_str()) {
            let w = social::research::cost_fixture(case == "surplus", enabled, seed)?;
            writeln!(
                params,
                "seed={seed} on=prescribed_gift off=no_gift; further care disabled\n{}",
                social::research::parameters(&w)
            )?;
            w
        } else if mode == "live"
            && ["colocated", "farming", "farming_distant", "construction"].contains(&case.as_str())
        {
            let w = social::research::live(case, enabled, seed)?;
            writeln!(params, "seed={seed}\n{}", social::research::parameters(&w))?;
            w
        } else {
            return Err("unregistered experiment".into());
        };
        if let Some(h) = &history {
            for id in 0..h.agents().len() {
                if let Some(m) = h.social_memory(id) {
                    for (&other, r) in &m.relationships {
                        writeln!(
                            rel,
                            "{seed},history,{id},{other},{},{},{},{},{},{},{}",
                            r.trust,
                            r.attachment,
                            r.grievance,
                            r.obligation,
                            m.affect.anger.value(h.now()),
                            m.affect.fear.value(h.now()),
                            m.affect.grief.value(h.now())
                        )?;
                    }
                }
            }
            for x in &h.social_journal().appraisals {
                writeln!(appraisal, "{seed}\thistory\t{x:?}")?;
            }
            if seed == 0 {
                traces(&mut events, seed, "history", h)?;
            }
        }
        w.advance_to(if mode == "history" {
            2 * HOUR
        } else {
            48 * HOUR
        })?;
        w.audit()?;
        let j = w.social_journal();
        let m = w.metrics();
        let care = w.care_journal();
        let edges: usize = (0..w.agents().len())
            .filter_map(|id| w.social_memory(id))
            .map(|m| m.relationships.len())
            .sum();
        writeln!(
            runs,
            "{seed},{mode},{case},{},{signature},{selected},{pressure},{cost},{},{},{},{},{:.9},{:.9},{},{},{},{},{},{},{edges},0,{},{:016x},{:016x}",
            if enabled { "on" } else { "off" },
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count(),
            care.count("food_transferred"),
            m.consumed,
            m.spoiled,
            m.deficit_ms as f64 / HOUR as f64,
            care.care_travel_ms as f64 / HOUR as f64,
            j.count("care_selected"),
            j.count("care_refused"),
            j.count("observed_evidence"),
            j.count("anger_episodes"),
            j.count("fear_episodes"),
            j.count("grief_episodes"),
            m.overtakes,
            w.state_hash(),
            w.event_hash()
        )?;
        for id in 0..w.agents().len() {
            let p = &w.agents()[id];
            let hunger = care.starvation_ms.get(&id).copied().unwrap_or(0) as f64 / HOUR as f64;
            writeln!(
                people,
                "{seed},{id},{},{},{hunger:.9},{},{:.9},{:.9},{:.9}",
                p.nutrition,
                p.consumed,
                p.death_time.map_or(String::new(), |t| t.to_string()),
                social::research::affect_hours(&w, id, "anger", w.now()),
                social::research::affect_hours(&w, id, "fear", w.now()),
                social::research::affect_hours(&w, id, "grief", w.now())
            )?;
            if let Some(m) = w.social_memory(id) {
                for (&other, r) in &m.relationships {
                    writeln!(
                        rel,
                        "{seed},final,{id},{other},{},{},{},{},{},{},{}",
                        r.trust,
                        r.attachment,
                        r.grievance,
                        r.obligation,
                        m.affect.anger.value(w.now()),
                        m.affect.fear.value(w.now()),
                        m.affect.grief.value(w.now())
                    )?;
                }
            }
        }
        for ((id, k), t) in &care.activity_ms {
            writeln!(acts, "{seed},{id},{k},{:.9}", *t as f64 / HOUR as f64)?;
        }
        for (k, v) in &j.counts {
            writeln!(metrics, "{seed},social:{k},{v}")?;
        }
        for (k, v) in &care.counts {
            writeln!(metrics, "{seed},care:{k},{v}")?;
        }
        for x in &j.appraisals {
            writeln!(appraisal, "{seed}\texecution\t{x:?}")?;
        }
        if seed == 0 {
            traces(&mut events, seed, "execution", &w)?;
        }
    }
    Ok(())
}
