use std::{
    env, fs,
    io::{self, BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{
    Action, HOUR,
    experiment::{self, SCENARIOS},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "--matrix") {
        let dir = Path::new(
            args.get(1)
                .ok_or("usage: --matrix OUTPUT_DIRECTORY [SEED_COUNT]")?,
        );
        fs::create_dir_all(dir)?;
        let count: u64 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(30);
        let mut rows = BufWriter::new(fs::File::create(dir.join("runs.csv"))?);
        let mut agents = BufWriter::new(fs::File::create(dir.join("agents.csv"))?);
        let mut params = BufWriter::new(fs::File::create(dir.join("parameters.txt"))?);
        writeln!(rows, "{}", experiment::csv_header())?;
        writeln!(
            agents,
            "scenario,seed,time_h,agent,alive,nutrition_h,fatigue_h,starvation_h,food_consumed,death_time_h,death_cause,location"
        )?;
        for name in SCENARIOS {
            eprintln!("running {name}: {count} paired seeds");
            for seed in 0..count {
                let r = experiment::run(name, seed, false)?;
                writeln!(rows, "{}", experiment::csv_row(name, seed, &r.world))?;
                writeln!(params, "{name} seed={seed} {:?}", r.parameters)?;
                for point in r.checkpoints {
                    for a in point.agents {
                        writeln!(
                            agents,
                            "{name},{seed},{},{},{},{:.6},{:.6},{:.6},{},{},{},\"{:?}\"",
                            point.at / HOUR,
                            a.id,
                            a.action != Action::Dead,
                            a.nutrition as f64 / HOUR as f64,
                            a.fatigue as f64 / HOUR as f64,
                            a.starvation as f64 / HOUR as f64,
                            a.consumed,
                            a.death_time.map_or(String::new(), |t| format!(
                                "{:.6}",
                                t as f64 / HOUR as f64
                            )),
                            if a.death_time.is_some() {
                                "starvation"
                            } else {
                                ""
                            },
                            a.location
                        )?;
                    }
                }
            }
            rows.flush()?;
            agents.flush()?;
        }
        return Ok(());
    }
    let trace = args.first().is_some_and(|s| s == "--trace");
    let offset = usize::from(trace);
    let name = args.get(offset).map(String::as_str).unwrap_or("reference");
    let seed = args
        .get(offset + 1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(0);
    let r = experiment::run(name, seed, trace)?;
    let mut out = BufWriter::new(io::stdout().lock());
    if trace {
        writeln!(out, "# configuration {:?}", r.parameters)?;
        for e in r.world.events() {
            writeln!(out, "{e:?}")?;
        }
    }
    writeln!(out, "{}", experiment::csv_header())?;
    writeln!(out, "{}", experiment::csv_row(name, seed, &r.world))?;
    Ok(())
}
