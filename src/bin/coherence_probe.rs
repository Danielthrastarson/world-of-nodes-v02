use std::{
    collections::BTreeMap,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{coherence, decomposition::COUNTERS, *};
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
fn file(root: &Path, name: &str) -> std::io::Result<BufWriter<fs::File>> {
    Ok(BufWriter::new(
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(root.join(name))?,
    ))
}
fn static_files(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = file(root, "static.csv")?;
    writeln!(
        f,
        "case,nutrition_h,fatigue_h,queue_length,forecast_ms,kernel_ms,forecast_nutrition,kernel_nutrition,forecast_fatigue,kernel_fatigue,forecast_cargo,kernel_cargo"
    )?;
    let rows = coherence::static_validation()?;
    for r in &rows {
        writeln!(f, "{r}")?;
    }
    let mut p = file(root, "primitives.csv")?;
    writeln!(
        p,
        "action,nutrition_ms,fatigue_ms,exact,time_error_ms,nutrition_error_ms,fatigue_error_ms,cargo_error,spoilage_error_ms"
    )?;
    for r in shadow::static_rows()
        .into_iter()
        .chain(shadow::institutional_rows())
    {
        if r.split(',').nth(3) != Some("true") {
            return Err(format!("primitive mismatch {r}").into());
        }
        writeln!(p, "{r}")?;
    }
    eprintln!("static: {} composite kernel comparisons exact", rows.len());
    Ok(())
}
struct Outputs {
    rows: BTreeMap<&'static str, BufWriter<fs::File>>,
    shadow: BTreeMap<&'static str, BufWriter<fs::File>>,
    counts: BufWriter<fs::File>,
    runs: BufWriter<fs::File>,
    checkpoints: BufWriter<fs::File>,
    trace: BufWriter<fs::File>,
}
impl Outputs {
    fn new(root: &Path, name: &str, with_shadow: bool) -> std::io::Result<Self> {
        let mut rows = BTreeMap::new();
        for &(key, header) in coherence::HEADERS {
            let mut f = file(
                root,
                &format!(
                    "{name}-{key}.{}",
                    if ["trace", "decisions"].contains(&key) {
                        "tsv"
                    } else {
                        "csv"
                    }
                ),
            )?;
            writeln!(f, "{header}")?;
            rows.insert(key, f);
        }
        let mut shadow = BTreeMap::new();
        if with_shadow {
            for &(key, header) in shadow::HEADERS {
                let mut f = file(
                    root,
                    &format!(
                        "{name}-shadow-{key}.{}",
                        if key == "captures" { "tsv" } else { "csv" }
                    ),
                )?;
                writeln!(f, "{header}")?;
                shadow.insert(key, f);
            }
        }
        let mut counts = file(root, &format!("{name}-counts.csv"))?;
        writeln!(counts, "seed,key,count")?;
        let mut runs = file(root, &format!("{name}-runs.csv"))?;
        writeln!(
            runs,
            "{},{}",
            experiment::csv_header(),
            COUNTERS
                .iter()
                .map(|k| format!("diag_{k}"))
                .collect::<Vec<_>>()
                .join(",")
        )?;
        let mut checkpoints = file(root, &format!("{name}-checkpoints.csv"))?;
        writeln!(checkpoints, "seed,hour,event_hash,state_hash")?;
        let trace = file(root, &format!("{name}-events.tsv"))?;
        Ok(Self {
            rows,
            shadow,
            counts,
            runs,
            checkpoints,
            trace,
        })
    }
    fn drain(
        &mut self,
        w: &mut World,
        seed: u64,
        h: i64,
    ) -> std::io::Result<BTreeMap<(Time, usize), String>> {
        for (k, rows) in w.take_coherence_rows() {
            for r in rows {
                writeln!(self.rows.get_mut(k).unwrap(), "{r}")?;
            }
        }
        for (k, rows) in w.take_shadow_rows() {
            for r in rows {
                writeln!(self.shadow.get_mut(k).unwrap(), "{r}")?;
            }
        }
        w.take_acceptance_records();
        w.take_persistence_records();
        writeln!(
            self.checkpoints,
            "{seed},{h},{:016x},{:016x}",
            w.event_hash(),
            w.state_hash()
        )?;
        Ok(w.take_coherence_decisions())
    }
    fn finish(&mut self, w: &World, name: &str, seed: u64) -> std::io::Result<()> {
        for (k, n) in w.coherence_counts() {
            writeln!(self.counts, "{seed},{k},{n}")?;
        }
        writeln!(
            self.runs,
            "{},{}",
            experiment::csv_row(name, seed, w),
            COUNTERS
                .iter()
                .map(|k| w.diagnostics().get(k).to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        if seed == 0 {
            for e in w.events() {
                writeln!(self.trace, "{e:?}")?;
            }
        }
        self.counts.flush()?;
        self.runs.flush()?;
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let mode = args
        .first()
        .ok_or("coherence_probe static|replay|live NEW_DIR [ARM|SEEDS]")?;
    let root = Path::new(args.get(1).ok_or("directory required")?);
    fs::create_dir_all(root)?;
    if mode == "static" {
        return static_files(root);
    }
    // Always block this binary's research runs if covered kernel contracts fail.
    coherence::static_validation()?;
    if mode == "replay" {
        let arm = args.get(2).ok_or("replay arm required")?;
        let mut out = Outputs::new(root, arm, false)?;
        let mut w = fixture(arm, 0)?;
        let mut plain = fixture(arm, 0)?;
        w.enable_coherence_audit(true);
        for h in 0..=72 {
            w.advance_to((h * HOUR).max(w.now()))?;
            plain.advance_to((h * HOUR).max(plain.now()))?;
            assert_eq!(w.event_hash(), plain.event_hash());
            assert_eq!(w.state_hash(), plain.state_hash());
            out.drain(&mut w, 0, h)?;
            plain.take_acceptance_records();
            plain.take_persistence_records();
        }
        out.finish(&w, arm, 0)?;
        eprintln!(
            "replay {arm}: hashes unchanged; {} direct intent differences; {} kernel mismatches",
            w.coherence_counts()
                .get("intent_changed_on_same_inputs")
                .unwrap_or(&0),
            w.coherence_counts().get("kernel_mismatch").unwrap_or(&0)
        );
    } else if mode == "live" {
        let count: u64 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(30);
        let mut oldout = Outputs::new(root, "control", true)?;
        let mut newout = Outputs::new(root, "coherent", true)?;
        let mut diffs = file(root, "live-differences.csv")?;
        writeln!(
            diffs,
            "seed,matched_decision_points,different_intents,only_control,only_corrected"
        )?;
        for seed in 0..count {
            let mut a = acceptance::fixture("current", seed, seed == 0)?;
            let mut config = a.config_for_research().clone();
            config.diagnostic_policy = DiagnosticPolicy::CoherentForecast;
            let mut b = World::new(config)?;
            a.enable_coherence_audit(seed == 0);
            b.enable_coherence_audit(seed == 0);
            // Decision maps are collected for all seeds; retained detailed files only seed0.
            if seed == 0 {
                a.enable_shadow_audit(true);
                b.enable_shadow_audit(true);
            }
            let (mut matched, mut changed, mut only_old, mut only_new) = (0, 0, 0, 0);
            for h in 0..=72 {
                a.advance_to(h * HOUR)?;
                b.advance_to(h * HOUR)?;
                let ad = oldout.drain(&mut a, seed, h)?;
                let bd = newout.drain(&mut b, seed, h)?;
                for (k, v) in &ad {
                    if let Some(other) = bd.get(k) {
                        matched += 1;
                        if v != other {
                            changed += 1;
                        }
                    } else {
                        only_old += 1;
                    }
                }
                only_new += bd.keys().filter(|k| !ad.contains_key(k)).count();
            }
            oldout.finish(&a, "control", seed)?;
            newout.finish(&b, "coherent", seed)?;
            writeln!(diffs, "{seed},{matched},{changed},{only_old},{only_new}")?;
            diffs.flush()?;
            eprintln!(
                "live seed={seed}: different_intents={changed}; jobs {} -> {}; quanta {} -> {}",
                a.metrics().jobs,
                b.metrics().jobs,
                a.metrics().delivered,
                b.metrics().delivered
            );
        }
    } else {
        return Err("unknown mode".into());
    }
    Ok(())
}
