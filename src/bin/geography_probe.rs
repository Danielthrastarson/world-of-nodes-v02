use std::{
    collections::BTreeSet,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};
use world_of_nodes::provisioning::*;
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
    let name = args.first().ok_or("geography_probe MAP NEW_DIR [SEEDS]")?;
    let dir = Path::new(args.get(1).ok_or("directory required")?);
    fs::create_dir_all(dir)?;
    let seeds: u64 = args.get(2).map(|v| v.parse()).transpose()?.unwrap_or(30);
    let mut runs = file(dir, "runs.csv")?;
    writeln!(
        runs,
        "{},source_discoverers,ever_visited_nodes",
        experiment::csv_header()
            .replace("alive_at_c", "alive_node_0")
            .replace("alive_at_f", "alive_node_2")
    )?;
    let mut pop = file(dir, "population.csv")?;
    writeln!(
        pop,
        "seed,hour,node,alive,nutrition_ms,starvation_ms,food_consumed"
    )?;
    let mut agents = file(dir, "agents.csv")?;
    writeln!(
        agents,
        "seed,hour,agent,alive,nutrition_ms,fatigue_ms,starvation_ms,zero_ms,location,known_nodes"
    )?;
    let mut parameters = file(dir, "parameters.txt")?;
    let mut checkpoints = file(dir, "checkpoints.csv")?;
    writeln!(checkpoints, "seed,hour,event_hash,state_hash")?;
    for seed in 0..seeds {
        let c = geography::configuration(name, seed, seed == 0)?;
        let sources: BTreeSet<_> = c.food.iter().map(|f| f.0).collect();
        let nodes = c.nodes;
        writeln!(parameters, "seed={seed} {c:?}")?;
        let mut w = World::new(c)?;
        for h in 0..=72 {
            w.advance_to(h * HOUR)?;
            w.audit()?;
            assert_eq!(w.metrics().overtakes, 0);
            writeln!(
                checkpoints,
                "{seed},{h},{:016x},{:016x}",
                w.event_hash(),
                w.state_hash()
            )?;
            if [36, 48, 72].contains(&h) {
                for node in 0..nodes {
                    let local: Vec<_> = w
                        .agents()
                        .iter()
                        .filter(|a| a.location == Location::Node(node) && a.action != Action::Dead)
                        .collect();
                    writeln!(
                        pop,
                        "{seed},{h},{node},{},{},{},{}",
                        local.len(),
                        local.iter().map(|a| a.nutrition).sum::<i64>(),
                        local.iter().map(|a| a.starvation).sum::<i64>(),
                        local.iter().map(|a| a.consumed).sum::<u64>()
                    )?;
                }
                for a in w.agents() {
                    writeln!(
                        agents,
                        "{seed},{h},{},{},{},{},{},{},\"{:?}\",{}",
                        a.id,
                        a.action != Action::Dead,
                        a.nutrition,
                        a.fatigue,
                        a.starvation,
                        w.zero_nutrition_ms(a.id),
                        a.location,
                        a.memory.nodes.len()
                    )?;
                }
            }
        }
        let discoverers = w
            .agents()
            .iter()
            .filter(|a| a.memory.nodes.keys().any(|n| sources.contains(n)))
            .count();
        let visited: BTreeSet<_> = w
            .agents()
            .iter()
            .flat_map(|a| a.memory.nodes.keys().copied())
            .collect();
        writeln!(
            runs,
            "{},{discoverers},{}",
            experiment::csv_row(name, seed, &w),
            visited.len()
        )?;
        runs.flush()?;
        if seed == 0 {
            let mut trace = file(dir, "events.tsv")?;
            for e in w.events() {
                writeln!(trace, "{e:?}")?;
            }
        }
        eprintln!(
            "{name} seed{seed}: survivors={} jobs={} delivered={} discoveries={discoverers}",
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count(),
            w.metrics().jobs,
            w.metrics().delivered
        );
    }
    Ok(())
}
