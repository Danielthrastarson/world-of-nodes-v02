use std::{
    collections::BTreeMap,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::{decomposition::COUNTERS, *};
fn file(root: &Path, name: &str) -> std::io::Result<BufWriter<fs::File>> {
    Ok(BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))?,
    ))
}
fn fixture(seed: u64, meals: bool, trace: bool) -> Result<World, String> {
    World::new(Config {
        seed,
        trace,
        initial_request: 600,
        acceptance_audit: true,
        diagnostic_policy: if meals {
            DiagnosticPolicy::ConditionalMeals
        } else {
            DiagnosticPolicy::Current
        },
        ..Config::default()
    })
}
struct Output {
    runs: BufWriter<fs::File>,
    counts: BufWriter<fs::File>,
    rows: BufWriter<fs::File>,
    checkpoints: BufWriter<fs::File>,
    agents: BufWriter<fs::File>,
    events: BufWriter<fs::File>,
}
impl Output {
    fn new(root: &Path, arm: &str) -> std::io::Result<Self> {
        let mut runs = file(root, &format!("{arm}-runs.csv"))?;
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
        let mut counts = file(root, &format!("{arm}-counts.csv"))?;
        writeln!(counts, "seed,key,count")?;
        let mut checkpoints = file(root, &format!("{arm}-checkpoints.csv"))?;
        writeln!(checkpoints, "seed,hour,event_hash,state_hash")?;
        let mut agents = file(root, &format!("{arm}-agents.csv"))?;
        writeln!(
            agents,
            "seed,hour,id,alive,nutrition_ms,fatigue_ms,starvation_ms,zero_ms,consumed,location"
        )?;
        Ok(Self {
            runs,
            counts,
            checkpoints,
            agents,
            rows: file(root, &format!("{arm}-details.tsv"))?,
            events: file(root, &format!("{arm}-events.tsv"))?,
        })
    }
    fn drain(
        &mut self,
        w: &mut World,
        seed: u64,
        h: i64,
    ) -> std::io::Result<BTreeMap<(Time, usize), String>> {
        w.audit().unwrap();
        writeln!(
            self.checkpoints,
            "{seed},{h},{:016x},{:016x}",
            w.event_hash(),
            w.state_hash()
        )?;
        for row in w.take_meal_research_rows() {
            writeln!(self.rows, "{seed}\t{row}")?;
        }
        w.take_acceptance_records();
        w.take_persistence_records();
        if [36, 48, 72].contains(&h) {
            for a in w.agents() {
                writeln!(
                    self.agents,
                    "{seed},{h},{},{},{},{},{},{},{},\"{:?}\"",
                    a.id,
                    a.action != Action::Dead,
                    a.nutrition,
                    a.fatigue,
                    a.starvation,
                    w.zero_nutrition_ms(a.id),
                    a.consumed,
                    a.location
                )?;
            }
        }
        Ok(w.take_meal_research_decisions())
    }
    fn finish(&mut self, w: &World, arm: &str, seed: u64) -> std::io::Result<()> {
        writeln!(
            self.runs,
            "{},{}",
            experiment::csv_row(arm, seed, w),
            COUNTERS
                .iter()
                .map(|k| w.diagnostics().get(k).to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        for (k, n) in w.meal_research_counts() {
            writeln!(self.counts, "{seed},{k},{n}")?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(self.events, "{e:?}")?;
            }
        }
        self.runs.flush()?;
        self.counts.flush()?;
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let mode = args
        .first()
        .ok_or("meal_probe static|regressions|offline|live NEW_DIR [SEEDS]")?;
    let root = Path::new(args.get(1).ok_or("directory required")?);
    fs::create_dir_all(root)?;
    if mode == "static" {
        let mut out = file(root, "static.csv")?;
        writeln!(
            out,
            "arm,case,nutrition_h,fatigue_h,queue_length,forecast_ms,kernel_ms,forecast_nutrition,kernel_nutrition,forecast_fatigue,kernel_fatigue,forecast_cargo,kernel_cargo"
        )?;
        for (name, rows) in [
            ("no_meal", coherence::static_validation()?),
            (
                "conditional_meals",
                coherence::conditional_meal_validation()?,
            ),
        ] {
            eprintln!("{name}: {} exact", rows.len());
            for row in rows {
                writeln!(out, "{name},{row}")?;
            }
        }
        return Ok(());
    }
    if mode == "regressions" {
        let mut out = file(root, "historical.csv")?;
        writeln!(out, "{}", experiment::csv_header())?;
        for arm in [
            "reference",
            "current",
            "known_source",
            "reactive",
            "committed",
            "forecast_corrected",
            "contested_requests",
            "stale_depletion",
            "meal_aware",
        ] {
            let mut w = match arm {
                "reference" | "contested_requests" | "stale_depletion" => {
                    shadow::historical_fixture(arm, 0)?
                }
                "current" | "known_source" | "meal_aware" => acceptance::fixture(arm, 0, false)?,
                _ => persistence::fixture(arm, 0, false)?,
            };
            for h in 0..=72 {
                w.advance_to((h * HOUR).max(w.now()))?;
                w.take_acceptance_records();
                w.take_persistence_records();
                w.audit()?;
            }
            writeln!(out, "{}", experiment::csv_row(arm, 0, &w))?;
            out.flush()?;
            eprintln!(
                "historical {arm} {:016x} {:016x}",
                w.event_hash(),
                w.state_hash()
            );
        }
        let mut out = file(root, "adoption-equivalence.csv")?;
        writeln!(out, "scenario,checkpoints,state_equal,event_equal")?;
        for name in experiment::SCENARIOS {
            let mut c = experiment::configuration(name, 0, false)?;
            c.diagnostic_policy = DiagnosticPolicy::Current;
            let mut old = c.clone();
            old.diagnostic_policy = DiagnosticPolicy::CoherentForecast;
            let (mut a, mut b) = (World::new(c)?, World::new(old)?);
            for h in 0..=72 {
                a.advance_to(h * HOUR)?;
                b.advance_to(h * HOUR)?;
                a.audit()?;
                b.audit()?;
                assert_eq!(a.state_hash(), b.state_hash(), "{name} h={h}");
                assert_eq!(a.event_hash(), b.event_hash(), "{name} h={h}");
            }
            writeln!(out, "{name},73,true,true")?;
            out.flush()?;
        }
        return Ok(());
    }
    coherence::static_validation()?;
    coherence::conditional_meal_validation()?;
    if mode == "offline" {
        let mut out = Output::new(root, "old_meals")?;
        let mut w = acceptance::fixture("meal_aware", 0, true)?;
        let mut plain = acceptance::fixture("meal_aware", 0, false)?;
        w.enable_meal_research(true, true);
        for h in 0..=72 {
            w.advance_to(h * HOUR)?;
            plain.advance_to(h * HOUR)?;
            assert_eq!(w.state_hash(), plain.state_hash());
            assert_eq!(w.event_hash(), plain.event_hash());
            out.drain(&mut w, 0, h)?;
            plain.take_acceptance_records();
        }
        out.finish(&w, "meal_aware", 0)?;
        return Ok(());
    }
    if mode != "live" {
        return Err("unknown mode".into());
    }
    let seeds: u64 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let (mut ao, mut bo) = (
        Output::new(root, "no_meal")?,
        Output::new(root, "conditional_meals")?,
    );
    let mut diff = file(root, "paired-decisions.csv")?;
    writeln!(diff, "seed,matched,different,only_no_meal,only_meal")?;
    let mut params = file(root, "parameters.txt")?;
    for seed in 0..seeds {
        let (mut a, mut b) = (
            fixture(seed, false, seed == 0)?,
            fixture(seed, true, seed == 0)?,
        );
        writeln!(
            params,
            "no_meal {seed}: {:?}\nconditional_meals {seed}: {:?}",
            a.config_for_research(),
            b.config_for_research()
        )?;
        a.enable_meal_research(seed == 0, false);
        b.enable_meal_research(seed == 0, false);
        let (mut matched, mut changed, mut only_a, mut only_b) = (0, 0, 0, 0);
        for h in 0..=72 {
            a.advance_to(h * HOUR)?;
            b.advance_to(h * HOUR)?;
            let ad = ao.drain(&mut a, seed, h)?;
            let bd = bo.drain(&mut b, seed, h)?;
            for (k, v) in &ad {
                if let Some(other) = bd.get(k) {
                    matched += 1;
                    if other != v {
                        changed += 1;
                    }
                } else {
                    only_a += 1;
                }
            }
            only_b += bd.keys().filter(|k| !ad.contains_key(k)).count();
        }
        ao.finish(&a, "no_meal", seed)?;
        bo.finish(&b, "conditional_meals", seed)?;
        writeln!(diff, "{seed},{matched},{changed},{only_a},{only_b}")?;
        diff.flush()?;
        eprintln!(
            "seed {seed}: different={changed} jobs {}->{} quanta {}->{}",
            a.metrics().jobs,
            b.metrics().jobs,
            a.metrics().delivered,
            b.metrics().delivered
        );
    }
    Ok(())
}
