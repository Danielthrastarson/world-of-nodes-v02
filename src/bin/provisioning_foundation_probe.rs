use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{
    decomposition::{self, COUNTERS, Schedule},
    *,
};
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
    let name = args.first().ok_or(
        "provisioning_foundation_probe regional|compact communal|self_only|idle|witness NEW_DIR",
    )?;
    let arm = args.get(1).ok_or("arm required")?;
    let dir = Path::new(args.get(2).ok_or("directory required")?);
    fs::create_dir_all(dir)?;
    if !["regional", "compact"].contains(&name.as_str())
        || !["communal", "self_only", "idle", "witness"].contains(&arm.as_str())
    {
        return Err("invalid arm".into());
    }
    let mut runs = file(dir, "runs.csv")?;
    writeln!(
        runs,
        "{},{}",
        experiment::csv_header(),
        COUNTERS
            .iter()
            .skip(8)
            .map(|k| format!("diag_{k}"))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    let mut counts = file(dir, "counts.csv")?;
    writeln!(counts, "seed,key,count")?;
    let mut parameters = file(dir, "parameters.txt")?;
    let mut health = file(dir, "health.csv")?;
    writeln!(
        health,
        "seed,hour,id,alive,nutrition_ms,starvation_ms,zero_ms,consumed,location"
    )?;
    let mut details = file(dir, "details.tsv")?;
    let mut jobs = file(dir, "jobs.tsv")?;
    for seed in 0..if arm == "witness" { 1 } else { 30 } {
        let mut c = geography::configuration("line", seed, seed == 0)?;
        if name == "compact" {
            for e in &mut c.edges {
                e.nominal_ms = 30 * MINUTE;
            }
        }
        c.communal = arm != "self_only";
        c.autonomous = arm != "idle" && arm != "witness";
        writeln!(parameters, "seed={seed} {c:?}")?;
        let mut w = if arm == "witness" {
            let (witness, w) = decomposition::schedule_witness_in_world(
                Schedule {
                    carriers: 4,
                    residents: 46,
                    ..Schedule::default()
                },
                c,
            )?;
            let mut out = file(dir, "witness.txt")?;
            writeln!(
                out,
                "{witness:?}\nThis is a feasible kernel schedule, not an optimality certificate.\nMaterial upper bound on any delivered food: 3000 quanta."
            )?;
            w
        } else {
            World::new(c)?
        };
        if arm != "witness" {
            w.enable_meal_research(seed == 0, false);
        }
        for h in 0..=72 {
            if arm == "witness" && h != 72 {
                continue;
            }
            w.advance_to(h * HOUR)?;
            w.audit()?;
            assert_eq!(w.metrics().overtakes, 0);
            for row in w.take_meal_research_rows() {
                writeln!(details, "{seed}\t{row}")?;
            }
            w.take_meal_research_decisions();
            w.take_acceptance_records();
            if [36, 48, 72].contains(&h) {
                for a in w.agents() {
                    writeln!(
                        health,
                        "{seed},{h},{},{},{},{},{},{},\"{:?}\"",
                        a.id,
                        a.action != Action::Dead,
                        a.nutrition,
                        a.starvation,
                        w.zero_nutrition_ms(a.id),
                        a.consumed,
                        a.location
                    )?;
                }
            }
        }
        for (k, n) in w.meal_research_counts() {
            writeln!(counts, "{seed},{k},{n}")?;
        }
        for j in &w.diagnostics().jobs {
            writeln!(jobs, "{seed}\t{j:?}")?;
        }
        writeln!(
            runs,
            "{},{}",
            experiment::csv_row(&format!("{name}_{arm}"), seed, &w),
            COUNTERS
                .iter()
                .skip(8)
                .map(|k| w.diagnostics().get(k).to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        runs.flush()?;
        if seed == 0 {
            let mut trace = file(dir, "events.tsv")?;
            for e in w.events() {
                writeln!(trace, "{e:?}")?;
            }
        }
        eprintln!(
            "{name}/{arm} seed{seed}: jobs={} deliveries={} consumed={} survivors={}",
            w.metrics().jobs,
            w.metrics().delivered,
            w.metrics().consumed,
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count()
        );
    }
    Ok(())
}
