//! F12 controlled-purpose courier and matched-receipt experiments.
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
    let a: Vec<_> = env::args().skip(1).collect();
    let case = a
        .first()
        .ok_or("information_probe CASE on|off NEW_DIR [SEEDS]")?;
    let enabled = match a.get(1).map(String::as_str) {
        Some("on") => true,
        Some("off") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(a.get(2).ok_or("directory")?);
    fs::create_dir_all(root)?;
    let seeds: u64 = a.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(root, "runs.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut reports = file(root, "reports.tsv")?;
    let mut decisions = file(root, "decisions.tsv")?;
    let mut truth = file(root, "truth.tsv")?;
    writeln!(events, "seed\tid\ttime_ms\tperson\tkind\tdetail")?;
    writeln!(reports, "seed\tperson\treport")?;
    writeln!(decisions, "seed\tbefore\tafter")?;
    writeln!(
        truth,
        "seed\trecipient\treport\tobserved_ms\treceived_ms\tcheckpoint_ms\tsource_node\tbelieved_legal_available\ttrue_legal_available\tbelief\ttruth"
    )?;
    for seed in 0..seeds {
        let clock = Instant::now();
        let matched = case.starts_with("matched_");
        let (mut w, before, after) = if matched {
            let (w, b, a) = information::research::matched(case, enabled, seed)?;
            (w, Some(b), Some(a))
        } else {
            (
                information::research::fixture(case, enabled, seed)?,
                None,
                None,
            )
        };
        if seed == 0 {
            writeln!(
                params,
                "case={case};assigned_or_received={enabled};\n{:#?}\ninitial_bodies={:?}\ninitial_goods={:?}\ninitial_information={:?}",
                information::research::parameters(&w),
                w.agents(),
                w.batches(),
                w.information_memory(0)
            )?;
        }
        let mut decision_at_release = None;
        let mut seen = std::collections::BTreeSet::new();
        for (id, row) in information::research::truth_checkpoints(&w, 1) {
            writeln!(truth, "{seed}\t1\t{id}\t{row}")?;
            seen.insert(id);
        }
        for minute in (w.now() / MINUTE + 1)..=24 * 60 {
            if matched {
                w.advance_to(minute * MINUTE)?;
            } else {
                information::research::advance(&mut w, case, minute * MINUTE)?;
            }
            if minute == 6 * 60 {
                decision_at_release = Some(information::research::decision(&w, 1));
            }
            for (id, row) in information::research::truth_checkpoints(&w, 1) {
                if seen.insert(id) {
                    writeln!(truth, "{seed}\t1\t{id}\t{row}")?;
                }
            }
        }
        w.audit()?;
        let mut r = information::research::metrics(&w);
        r.insert("seed".into(), seed.to_string());
        r.insert("case".into(), case.clone());
        r.insert("enabled".into(), enabled.to_string());
        r.insert(
            "matched_choice_changed".into(),
            u8::from(
                before
                    .as_ref()
                    .zip(after.as_ref())
                    .is_some_and(|(b, a)| b.intent != a.intent),
            )
            .to_string(),
        );
        r.insert("runtime_ms".into(), clock.elapsed().as_millis().to_string());
        if seed == 0 {
            writeln!(runs, "{}", r.keys().cloned().collect::<Vec<_>>().join(","))?;
        }
        writeln!(
            runs,
            "{}",
            r.values().cloned().collect::<Vec<_>>().join(",")
        )?;
        writeln!(
            decisions,
            "{seed}\t{before:?}\t{:?}",
            after.or(decision_at_release)
        )?;
        for id in 0..w.agents().len() {
            writeln!(reports, "{seed}\t{id}\t{:?}", w.information_memory(id))?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{}\t{:?}\t{}\t{}",
                    e.id, e.at, e.agent, e.kind, e.detail
                )?;
            }
        }
    }
    Ok(())
}
