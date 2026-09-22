use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{shadow, *};
fn fixture(name: &str, seed: u64) -> Result<World, String> {
    match name {
        "reference" => World::new(Config {
            diagnostic_policy: DiagnosticPolicy::LegacyForecast,
            seed,
            ..Config::default()
        }),
        "contested_requests" | "stale_depletion" => shadow::historical_fixture(name, seed),
        "current" | "known_source" => acceptance::fixture(name, seed, false),
        _ => persistence::fixture(name, seed, false),
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let name = args
        .first()
        .ok_or("shadow_probe ARM NEW_DIRECTORY [SEEDS]")?;
    let dir = Path::new(args.get(1).ok_or("missing directory")?);
    fs::create_dir_all(dir)?;
    let count: u64 = args.get(2).map(|v| v.parse()).transpose()?.unwrap_or(30);
    let create = |suffix: &str| -> std::io::Result<BufWriter<fs::File>> {
        Ok(BufWriter::new(
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dir.join(format!("{name}-{suffix}")))?,
        ))
    };
    let mut runs = create("runs.csv")?;
    writeln!(
        runs,
        "{},baseline_event_hash,baseline_state_hash,identical",
        experiment::csv_header()
    )?;
    let mut counts = create("counts.csv")?;
    writeln!(counts, "seed,key,count")?;
    let mut outputs = std::collections::BTreeMap::new();
    for &(key, header) in shadow::HEADERS {
        let mut f = create(&format!(
            "{key}.{}",
            if key == "captures" { "tsv" } else { "csv" }
        ))?;
        writeln!(f, "{header}")?;
        outputs.insert(key, f);
    }
    let mut static_output = create("static.csv")?;
    writeln!(
        static_output,
        "action,nutrition_ms,fatigue_ms,exact,time_error_ms,nutrition_error_ms,fatigue_error_ms,cargo_error,spoilage_error_ms"
    )?;
    for r in shadow::static_rows()
        .into_iter()
        .chain(shadow::institutional_rows())
    {
        writeln!(static_output, "{r}")?;
    }
    let mut counterexample = create("counterexample.csv")?;
    writeln!(
        counterexample,
        "case,policy_ms,shadow_ms,kernel_ms,time_error_ms,nutrition_error_ms"
    )?;
    writeln!(counterexample, "{}", shadow::queue_counterexample())?;
    for seed in 0..count {
        let mut baseline = fixture(name, seed)?;
        let mut audited = fixture(name, seed)?;
        audited.enable_shadow_audit(seed == 0);
        if args.get(3).is_some_and(|v| v == "kernel-validation") {
            audited.enable_shadow_kernel_validation();
        }
        for h in 0..=72 {
            baseline.advance_to((h * HOUR).max(baseline.now()))?;
            audited.advance_to((h * HOUR).max(audited.now()))?;
            assert_eq!(
                baseline.event_hash(),
                audited.event_hash(),
                "event {name} {seed} {h}"
            );
            assert_eq!(
                baseline.state_hash(),
                audited.state_hash(),
                "state {name} {seed} {h}"
            );
            for (key, rows) in audited.take_shadow_rows() {
                let f = outputs.get_mut(key).unwrap();
                for row in rows {
                    writeln!(f, "{row}")?;
                }
            }
            baseline.take_acceptance_records();
            audited.take_acceptance_records();
            baseline.take_persistence_records();
            audited.take_persistence_records();
        }
        writeln!(
            runs,
            "{},{:016x},{:016x},true",
            experiment::csv_row(name, seed, &audited),
            baseline.event_hash(),
            baseline.state_hash()
        )?;
        for (key, value) in audited.shadow_counts() {
            writeln!(counts, "{seed},{key},{value}")?;
        }
        runs.flush()?;
        counts.flush()?;
        eprintln!("{name} seed={seed} identical event/state at all 73 checkpoints");
    }
    Ok(())
}
