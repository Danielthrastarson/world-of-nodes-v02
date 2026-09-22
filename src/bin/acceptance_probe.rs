use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{
    acceptance::{self, GATES, JOURNAL_HEADER},
    decomposition::COUNTERS,
    experiment, *,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let name = args
        .first()
        .ok_or("usage: acceptance_probe CASE OUTPUT_DIRECTORY [SEEDS]; full journal on stdout")?;
    let dir = Path::new(args.get(1).ok_or("missing output directory")?);
    fs::create_dir_all(dir)?;
    let count: u64 = args.get(2).map(|v| v.parse()).transpose()?.unwrap_or(30);
    let mut runs = BufWriter::new(fs::File::create(dir.join(format!("{name}-runs.csv")))?);
    let mut gates = BufWriter::new(fs::File::create(dir.join(format!("{name}-gates.csv")))?);
    let mut agents = BufWriter::new(fs::File::create(dir.join(format!("{name}-agents.csv")))?);
    let mut params = BufWriter::new(fs::File::create(
        dir.join(format!("{name}-parameters.txt")),
    )?);
    let mut jobs = BufWriter::new(fs::File::create(dir.join(format!("{name}-jobs.tsv")))?);
    let mut journal = BufWriter::new(std::io::stdout().lock());
    writeln!(journal, "scenario\tseed\t{JOURNAL_HEADER}")?;
    writeln!(
        runs,
        "{},shadow_meal_passes,{}",
        experiment::csv_header(),
        COUNTERS
            .iter()
            .map(|c| format!("diag_{c}"))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(gates, "scenario,seed,scope,stage,gate,count")?;
    writeln!(
        agents,
        "scenario,seed,time_h,agent,alive,nutrition_h,fatigue_h,current_starvation_h,zero_nutrition_h,continuously_nourished,consumed,location"
    )?;
    writeln!(jobs, "scenario\tseed\tjob")?;
    for seed in 0..count {
        let mut w = acceptance::fixture(name, seed, seed == 0)?;
        writeln!(
            params,
            "seed={seed} {:?}\ninitial_memories={:?}",
            w.config_for_research(),
            w.agents().iter().map(|a| &a.memory).collect::<Vec<_>>()
        )?;
        for h in 0..=72 {
            w.advance_to(h * HOUR)?;
            for record in w.take_acceptance_records() {
                writeln!(journal, "{name}\t{seed}\t{record}")?;
            }
            if [36, 48, 72].contains(&h) {
                for a in w.agents() {
                    let z = w.zero_nutrition_ms(a.id);
                    writeln!(
                        agents,
                        "{name},{seed},{h},{},{},{:.9},{:.9},{:.9},{:.9},{},{},\"{:?}\"",
                        a.id,
                        a.action != Action::Dead,
                        a.nutrition as f64 / HOUR as f64,
                        a.fatigue as f64 / HOUR as f64,
                        a.starvation as f64 / HOUR as f64,
                        z as f64 / HOUR as f64,
                        a.action != Action::Dead && z == 0,
                        a.consumed,
                        a.location
                    )?;
                }
            }
        }
        for scope in ["LOCAL", "ALL"] {
            for stage in ["DEFERRED", "EVALUATION"] {
                for gate in GATES {
                    let n = w
                        .acceptance_counts()
                        .get(&(scope.into(), stage.into(), (*gate).into()))
                        .copied()
                        .unwrap_or(0);
                    writeln!(gates, "{name},{seed},{scope},{stage},{gate},{n}")?;
                }
            }
        }
        writeln!(
            runs,
            "{},{},{}",
            experiment::csv_row(name, seed, &w),
            w.meal_shadow_passes(),
            COUNTERS
                .iter()
                .map(|k| w.diagnostics().get(k).to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        for j in &w.diagnostics().jobs {
            writeln!(jobs, "{name}\t{seed}\t{j:?}")?;
        }
        if seed == 0 {
            let mut trace = BufWriter::new(fs::File::create(dir.join(format!("{name}.trace")))?);
            for e in w.events() {
                writeln!(trace, "{e:?}")?;
            }
        }
        runs.flush()?;
        gates.flush()?;
        agents.flush()?;
        journal.flush()?;
        eprintln!(
            "{name} seed={seed} jobs={} deliveries={} starvation_h={:.3} shadows={}",
            w.metrics().jobs,
            w.metrics().delivered,
            w.metrics().deficit_ms as f64 / HOUR as f64,
            w.meal_shadow_passes()
        );
    }
    Ok(())
}
