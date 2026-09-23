//! Registered cross-system control using existing local institutional commands only.
use std::{
    collections::BTreeMap,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
    time::Instant,
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
fn csv(out: &mut impl Write, row: &BTreeMap<String, String>, header: bool) -> std::io::Result<()> {
    if header {
        writeln!(out, "{}", row.keys().cloned().collect::<Vec<_>>().join(","))?;
    }
    writeln!(
        out,
        "{}",
        row.values().cloned().collect::<Vec<_>>().join(",")
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let case = args
        .first()
        .ok_or("kingdom_readiness_probe CASE pending|commissioned NEW_DIR [SEEDS]")?;
    let arm = args.get(1).ok_or("arm")?;
    let commissioned = match arm.as_str() {
        "pending" => false,
        "commissioned" => true,
        _ => return Err("unregistered arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("new directory")?);
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    if seeds == 0 {
        return Err("positive seed count required".into());
    }
    fs::create_dir_all(root)?;
    let mut runs = file(root, "runs.csv")?;
    let mut timeline = file(root, "timeline.csv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut events = file(root, "events.tsv")?;
    let mut people = file(root, "people.csv")?;
    let mut goods = file(root, "goods.tsv")?;
    let mut beliefs = file(root, "initial_beliefs.tsv")?;
    let mut institutions = file(root, "institutions.tsv")?;
    let mut reports = file(root, "reports.tsv")?;
    let mut activities = file(root, "activities.csv")?;
    let mut trips = file(root, "trips.tsv")?;
    writeln!(events, "seed\tid\ttime_ms\tperson\tkind\tdetail")?;
    writeln!(goods, "seed\tcheckpoint\tbatch")?;
    writeln!(beliefs, "seed\tperson\tinitial_spatial_memory")?;
    writeln!(institutions, "seed\ttime_ms\tkind\tstate")?;
    writeln!(reports, "seed\tperson\treceipt")?;
    writeln!(activities, "seed,person,activity,hours")?;
    writeln!(trips, "seed\tkind\ttrip")?;
    writeln!(
        people,
        "seed,person,alive,injuries,consumed,nutrition_ms,fatigue_ms,starvation_ms,military_role,location"
    )?;
    writeln!(
        params,
        "case={case}; arm={arm}; seeds=0..{seeds}; horizon72h; traces=seed0; no new production mechanics; fixed before results"
    )?;
    for seed in 0..seeds {
        let began = Instant::now();
        let mut r = kingdom_research::fixture(case, commissioned, seed)?;
        if seed == 0 {
            writeln!(params, "{}", r.parameters())?;
            for a in r.world.agents() {
                writeln!(beliefs, "{seed}\t{}\t{:?}", a.id, a.memory)?;
            }
        }
        for b in r.world.batches() {
            writeln!(goods, "{seed}\tinitial\t{b:?}")?;
        }
        for quarter in 0..=288 {
            r.advance_to(quarter * 15 * MINUTE)
                .map_err(|e| format!("case={case} arm={arm} seed={seed} quarter={quarter}: {e}"))?;
            let mut row = r.metrics()?;
            row.insert("seed".into(), seed.to_string());
            csv(&mut timeline, &row, seed == 0 && quarter == 0)?;
        }
        let mut row = r.metrics()?;
        row.insert("seed".into(), seed.to_string());
        row.insert("case".into(), case.clone());
        row.insert("arm".into(), arm.clone());
        row.insert("elapsed_ms".into(), began.elapsed().as_millis().to_string());
        csv(&mut runs, &row, seed == 0)?;
        let w = &r.world;
        for a in w.agents() {
            writeln!(
                people,
                "{seed},{},{},{},{},{},{},{},{},\"{:?}\"",
                a.id,
                a.action != Action::Dead,
                w.body(a.id).unwrap().injuries,
                a.consumed,
                a.nutrition,
                a.fatigue,
                w.care_journal()
                    .starvation_ms
                    .get(&a.id)
                    .copied()
                    .unwrap_or(0),
                w.military_role(a.id)
                    .map(|x| x.to_string())
                    .unwrap_or_default(),
                a.location
            )?;
            if let Some(m) = w.information_memory(a.id) {
                for receipt in m.reports.values() {
                    writeln!(reports, "{seed}\t{}\t{receipt:?}", a.id)?;
                }
            }
        }
        for b in w.batches() {
            writeln!(goods, "{seed}\tfinal\t{b:?}")?;
        }
        for ((id, tag), ms) in &w.care_journal().activity_ms {
            writeln!(
                activities,
                "{seed},{id},{tag},{:.9}",
                *ms as f64 / HOUR as f64
            )?;
        }
        for office in w.offices() {
            writeln!(institutions, "{seed}\t{}\toffice\t{office:?}", w.now())?;
        }
        for commission in w.commissions() {
            writeln!(
                institutions,
                "{seed}\t{}\tcommission\t{commission:?}",
                w.now()
            )?;
        }
        for project in w.crossing_projects() {
            writeln!(institutions, "{seed}\t{}\tproject\t{project:?}", w.now())?;
        }
        for unit in w.units().values() {
            writeln!(institutions, "{seed}\t{}\tunit\t{unit:?}", w.now())?;
        }
        writeln!(
            institutions,
            "{seed}\t{}\tunit_snapshot\t{:?}",
            w.now(),
            w.unit_snapshot(0)
        )?;
        writeln!(
            institutions,
            "{seed}\t{}\tprotection_command_error\t{:?}",
            w.now(),
            r.protection_error
        )?;
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{}\t{:?}\t{}\t{}",
                    e.id, e.at, e.agent, e.kind, e.detail
                )?;
            }
            for t in w.crossing_trips() {
                writeln!(trips, "{seed}\tcrossing\t{t:?}")?;
            }
            for t in w.path_trips() {
                writeln!(trips, "{seed}\tpath\t{t:?}")?;
            }
        }
    }
    Ok(())
}
