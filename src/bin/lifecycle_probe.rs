//! Bounded research runner. Population limits abort, never veto births.
use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
    time::Instant,
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
    let args: Vec<_> = env::args().skip(1).collect();
    let case = args.first().ok_or("lifecycle_probe CASE NEW_DIR [SEEDS]")?;
    let root = Path::new(args.get(1).ok_or("directory")?);
    let seeds: u64 = args.get(2).map(|x| x.parse()).transpose()?.unwrap_or(30);
    fs::create_dir_all(root)?;
    let mut runs = file(root, "runs.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut counts = file(root, "counts.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut activity = file(root, "activity.csv")?;
    let mut houses = file(root, "households.csv")?;
    let mut socials = file(root, "social.tsv")?;
    writeln!(
        runs,
        "seed,case,persons,alive,children_alive,child_deaths,reproductive_adults,conceptions,births,maturations,generation_depth,pregnancies,food_transferred,food_consumed,food_spoiled,starvation_h,adult_h,child_h,reproduction_person_h,unsupported_h,care_travel_h,relationship_edges,peak_persons,initial_state_text_bytes,peak_state_text_bytes,runtime_ms,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(
        people,
        "seed,person,sex,birth_time_ms,generation,parents,child,alive,nutrition_ms,consumed,starvation_h,death_ms"
    )?;
    writeln!(counts, "seed,key,count")?;
    writeln!(events, "seed\tid\ttime_ms\tperson\tkind\tdetail")?;
    writeln!(activity, "seed,person,activity,hours")?;
    writeln!(
        houses,
        "seed,household,alive_members,children,dependency_ratio,births"
    )?;
    writeln!(socials, "seed\tappraisal")?;
    for seed in 0..seeds {
        let start = Instant::now();
        let mut w = lifecycle::research::fixture(case, seed)?;
        if seed == 0 {
            writeln!(params, "{}", lifecycle::research::parameters(&w))?
        }
        let initial_bytes = lifecycle::research::peak_bytes(&w);
        let mut peak_bytes = initial_bytes;
        for t in 0..=60 {
            w.advance_to(t * HOUR)?;
            if case == "parent_death" {
                lifecycle::research::parent_death(&mut w)?
            }
            if case == "generational_control" {
                lifecycle::research::support_adults(&mut w)?;
                lifecycle::research::seed_descendant_partner(&mut w);
            }
            if case.starts_with("multigeneration") {
                lifecycle::research::support(&mut w)?;
                lifecycle::research::seed_descendant_partner(&mut w);
            }
            if w.agents().len() > 128 {
                writeln!(
                    params,
                    "ABORT safety population exceeded at {} count={}",
                    w.now(),
                    w.agents().len()
                )?;
                return Err("research safety bound exceeded; births were not suppressed".into());
            }
            peak_bytes = peak_bytes.max(lifecycle::research::peak_bytes(&w));
        }
        w.audit()?;
        let j = w.lifecycle_journal();
        let alive = w
            .agents()
            .iter()
            .filter(|a| a.action != Action::Dead)
            .count();
        let children_alive = w
            .agents()
            .iter()
            .filter(|a| a.action != Action::Dead && w.is_child(a.id))
            .count();
        let child_deaths = w
            .agents()
            .iter()
            .filter(|a| {
                a.death_time
                    .is_some_and(|t| t - w.biology(a.id).unwrap().birth_time < 24 * HOUR)
            })
            .count();
        let reproductive_adults = w
            .agents()
            .iter()
            .filter(|a| {
                a.action != Action::Dead && {
                    let b = w.biology(a.id).unwrap();
                    let age = b.age(w.now());
                    (24 * HOUR..96 * HOUR).contains(&age) && b.pregnancy.is_none()
                }
            })
            .count();
        let generation = w
            .agents()
            .iter()
            .map(|a| w.biology(a.id).unwrap().generation)
            .max()
            .unwrap_or(0);
        writeln!(
            runs,
            "{seed},{case},{},{alive},{children_alive},{child_deaths},{reproductive_adults},{},{},{},{generation},{},{},{},{},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{},{},{initial_bytes},{peak_bytes},{},0,{},{:016x},{:016x}",
            w.agents().len(),
            j.count("conceptions"),
            j.count("births"),
            j.count("maturations"),
            lifecycle::research::active_pregnancies(&w),
            w.care_journal().count("food_transferred"),
            w.metrics().consumed,
            w.metrics().spoiled,
            w.metrics().deficit_ms as f64 / HOUR as f64,
            j.adult_ms as f64 / HOUR as f64,
            j.child_ms as f64 / HOUR as f64,
            j.reproduction_ms as f64 / HOUR as f64,
            j.unsupported_ms as f64 / HOUR as f64,
            w.care_journal().care_travel_ms as f64 / HOUR as f64,
            lifecycle::research::social_edges(&w),
            j.peak,
            start.elapsed().as_millis(),
            w.metrics().overtakes,
            w.state_hash(),
            w.event_hash()
        )?;
        for (key, count) in &j.counts {
            writeln!(counts, "{seed},{key},{count}")?
        }
        for (key, count) in &w.social_journal().counts {
            writeln!(counts, "{seed},social:{key},{count}")?
        }
        for (key, count) in &w.care_journal().counts {
            writeln!(counts, "{seed},care:{key},{count}")?
        }
        for a in w.agents() {
            let b = w.biology(a.id).unwrap();
            let ps = b
                .parents
                .map(|p| format!("{}:{}", p[0], p[1]))
                .unwrap_or_default();
            writeln!(
                people,
                "{seed},{},{:?},{},{},{},{},{},{},{},{:.9},{}",
                a.id,
                b.sex,
                b.birth_time,
                b.generation,
                ps,
                w.is_child(a.id),
                a.action != Action::Dead,
                a.nutrition,
                a.consumed,
                w.care_journal()
                    .starvation_ms
                    .get(&a.id)
                    .copied()
                    .unwrap_or(0) as f64
                    / HOUR as f64,
                a.death_time.map(|x| x.to_string()).unwrap_or_default()
            )?;
        }
        for ((person, act), ms) in &w.care_journal().activity_ms {
            writeln!(
                activity,
                "{seed},{person},{act},{:.9}",
                *ms as f64 / HOUR as f64
            )?;
        }
        for (id, alive, children) in lifecycle::research::household_sizes(&w) {
            let births = w
                .events()
                .iter()
                .filter(|e| {
                    e.kind == "Born"
                        && e.agent.is_some_and(|p| {
                            w.household_view(p)
                                .unwrap()
                                .households
                                .iter()
                                .any(|h| h.id == id)
                        })
                })
                .count();
            let adults = alive - children;
            let ratio = if adults == 0 {
                if children == 0 {
                    "0".into()
                } else {
                    "inf".into()
                }
            } else {
                format!("{:.6}", children as f64 / adults as f64)
            };
            writeln!(houses, "{seed},{id},{alive},{children},{ratio},{births}")?;
        }
        for a in &w.social_journal().appraisals {
            writeln!(socials, "{seed}\t{a:?}")?
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{}\t{:?}\t{}\t{}",
                    e.id, e.at, e.agent, e.kind, e.detail
                )?
            }
            writeln!(
                params,
                "final biology / households: {}",
                lifecycle::research::parameters(&w)
            )?;
        }
    }
    Ok(())
}
