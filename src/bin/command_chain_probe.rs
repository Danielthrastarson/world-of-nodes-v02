//! Registered F13B one-seed research run; creates new evidence only.
use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("command_chain_probe CASE control|order SEED NEW_OUTDIR".into());
    }
    let ordered = match args[1].as_str() {
        "order" => true,
        "control" => false,
        _ => return Err("unknown arm".into()),
    };
    let seed = args[2].parse()?;
    let root = Path::new(&args[3]);
    fs::create_dir_all(root)?;
    let mut run = command_research::fixture(&args[0], ordered, seed)?;
    writeln!(file(root, "parameters.txt")?, "{}", run.parameters())?;
    let mut timeline = file(root, "timeline.csv")?;
    for hour in 0..=run.end / HOUR {
        run.advance_to(hour * HOUR)?;
        let row = run.metrics()?;
        if hour == 0 {
            writeln!(
                timeline,
                "{}",
                row.keys().cloned().collect::<Vec<_>>().join(",")
            )?;
        }
        writeln!(
            timeline,
            "{}",
            row.values().cloned().collect::<Vec<_>>().join(",")
        )?;
    }
    let row = run.metrics()?;
    let mut results = file(root, "results.csv")?;
    writeln!(
        results,
        "{}",
        row.keys().cloned().collect::<Vec<_>>().join(",")
    )?;
    writeln!(
        results,
        "{}",
        row.values().cloned().collect::<Vec<_>>().join(",")
    )?;
    let mut events = file(root, "events.tsv")?;
    writeln!(events, "id\ttime_ms\tperson\tkind\tdetail")?;
    for e in run.world().events() {
        writeln!(
            events,
            "{}\t{}\t{:?}\t{}\t{}",
            e.id, e.at, e.agent, e.kind, e.detail
        )?;
    }
    let mut institutions = file(root, "institutions.txt")?;
    writeln!(
        institutions,
        "{:?}\n{:?}\n{:?}",
        run.world().offices(),
        run.world().commissions(),
        run.world().crossing_projects()
    )?;
    for id in 0..run.world().agents().len() {
        writeln!(
            institutions,
            "person={id}; command_memory={:?}",
            run.world().command_memory(id)
        )?;
    }
    writeln!(institutions, "journal={:?}", run.world().command_journal())?;
    Ok(())
}
