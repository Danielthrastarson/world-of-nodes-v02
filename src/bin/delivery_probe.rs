use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{
    decomposition::{self, COUNTERS, Schedule},
    experiment, *,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let dir = Path::new(
        args.get(1)
            .ok_or("usage: delivery_probe --matrix|--envelope OUTPUT_DIRECTORY [SEEDS]")?,
    );
    fs::create_dir_all(dir)?;
    if args[0] == "--envelope" {
        let mut out = BufWriter::new(fs::File::create(dir.join("witnesses.csv"))?);
        writeln!(
            out,
            "carriers,residents,meal_target_h,rest_at_h,rest_to_h,cargo,rescue_only,delivered,first_delivery_ms,residents_alive_72,residents_nourished_48,resident_starvation_h,event_hash"
        )?;
        let mut schedules = vec![];
        // Registered finite sweep; optimization applies only to the research controller.
        for carriers in [4, 8, 12, 16, 20] {
            for meal in [12, 18, 23] {
                for rest in [12, 16, 20] {
                    schedules.push(Schedule {
                        carriers,
                        meal_target_h: meal,
                        rest_at_h: rest,
                        ..Schedule::default()
                    });
                }
            }
        }
        for meal in [8, 9, 10, 11, 12] {
            schedules.push(Schedule {
                carriers: 1,
                cargo: 1,
                meal_target_h: meal,
                rest_at_h: 24,
                ..Schedule::default()
            });
        }
        for carriers in [4, 8, 12] {
            for residents in [4, 8, 12, 24, 36] {
                for rescue_only in [false, true] {
                    if carriers + residents <= 50 {
                        schedules.push(Schedule {
                            carriers,
                            residents,
                            rescue_only,
                            ..Schedule::default()
                        });
                    }
                }
            }
        }
        for carriers in [8, 12, 16] {
            for residents in 1..=12 {
                schedules.push(Schedule {
                    carriers,
                    residents,
                    meal_target_h: 23,
                    rest_at_h: 12,
                    ..Schedule::default()
                });
            }
        }
        let mut earliest = i64::MAX;
        let mut best = 0;
        for p in schedules {
            let (r, w) = decomposition::schedule_witness(p.clone(), true)?;
            writeln!(
                out,
                "{},{},{},{},{},{},{},{},{},{},{},{:.6},{:016x}",
                p.carriers,
                p.residents,
                p.meal_target_h,
                p.rest_at_h,
                p.rest_to_h,
                p.cargo,
                p.rescue_only,
                r.delivered,
                r.first_delivery.map_or(String::new(), |t| t.to_string()),
                r.residents_alive_72,
                r.residents_without_starvation_48,
                r.resident_starvation_ms as f64 / HOUR as f64,
                r.event_hash
            )?;
            if let Some(t) = r.first_delivery
                && t < earliest
            {
                earliest = t;
                let mut trace =
                    BufWriter::new(fs::File::create(dir.join("earliest-witness.trace"))?);
                writeln!(trace, "# {r:?}")?;
                for e in w.events() {
                    writeln!(trace, "{e:?}")?;
                }
            }
            if r.delivered > best {
                best = r.delivered;
                let mut trace = BufWriter::new(fs::File::create(dir.join("best-witness.trace"))?);
                writeln!(trace, "# {r:?}")?;
                for e in w.events() {
                    writeln!(trace, "{e:?}")?;
                }
            }
            eprintln!(
                "witness {p:?}: delivered={} first={:?}",
                r.delivered, r.first_delivery
            );
            out.flush()?;
        }
        return Ok(());
    }
    if args[0] != "--matrix" {
        return Err("unknown mode".into());
    }
    let seeds: u64 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut out = BufWriter::new(fs::File::create(dir.join("runs.csv"))?);
    let mut jobs = BufWriter::new(fs::File::create(dir.join("jobs.tsv"))?);
    let mut params = BufWriter::new(fs::File::create(dir.join("parameters.txt"))?);
    writeln!(
        out,
        "{},{}",
        experiment::csv_header(),
        COUNTERS
            .iter()
            .map(|s| format!("diag_{s}"))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(jobs, "scenario\tseed\tjob_record")?;
    for (name, variant) in [
        ("current", DiagnosticPolicy::Current),
        ("queue", DiagnosticPolicy::QueueCommitment),
        ("sleep", DiagnosticPolicy::SleepCommitment),
        ("job", DiagnosticPolicy::JobCommitment),
        ("reference", DiagnosticPolicy::Current),
        ("contested", DiagnosticPolicy::Current),
    ] {
        for seed in 0..seeds {
            let mut w = if name == "reference" || name == "contested" {
                World::new(experiment::configuration(
                    if name == "contested" {
                        "contested_requests"
                    } else {
                        "reference"
                    },
                    seed,
                    seed == 0,
                )?)?
            } else {
                decomposition::delivery_fixture(seed, variant, seed == 0)?
            };
            writeln!(params, "{name} {seed} {:?}", w.config_for_research())?;
            for h in [36, 48, 72] {
                w.advance_to(h * HOUR)?;
            }
            writeln!(
                out,
                "{},{}",
                experiment::csv_row(name, seed, &w),
                COUNTERS
                    .iter()
                    .map(|key| w.diagnostics().get(key).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
            for j in &w.diagnostics().jobs {
                writeln!(jobs, "{name}\t{seed}\t{j:?}")?;
            }
            if seed == 0 {
                let mut trace =
                    BufWriter::new(fs::File::create(dir.join(format!("{name}.trace")))?);
                for e in w.events() {
                    writeln!(trace, "{e:?}")?;
                }
                fs::write(
                    dir.join(format!("{name}-decisions.txt")),
                    w.diagnostics().samples.join("\n"),
                )?;
                fs::write(
                    dir.join(format!("{name}-reasons.txt")),
                    format!("{:?}", w.diagnostics().reasons),
                )?;
            }
        }
        out.flush()?;
        jobs.flush()?;
        eprintln!("completed {name}: {seeds} seeds");
    }
    Ok(())
}
