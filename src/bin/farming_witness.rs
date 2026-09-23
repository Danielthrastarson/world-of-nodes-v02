//! Omniscient research control ONLY: legal cultivation schedule, not NPC policy.
use std::{
    env, fs,
    io::{BufWriter, Write},
};
use world_of_nodes::provisioning::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("farming_witness NEW_OUTPUT_FILE")?;
    let mut out = BufWriter::new(
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?,
    );
    let mut w = World::new(Config {
        autonomous: false,
        farming: Some(farming::Config {
            units: vec![0; 50],
            rules: farming::Rules::default(),
        }),
        ..Config::default()
    })?;
    let mut sleeping = [false; 50];
    for tick in 0..=168 * 60 {
        w.advance_to(tick * MINUTE)?;
        if [36 * 60, 48 * 60, 72 * 60, 120 * 60, 168 * 60].contains(&tick) {
            writeln!(
                out,
                "hour={} alive={} produced={:?} consumed={} starvation_hours={} initial_remote_stock={}",
                tick / 60,
                w.agents()
                    .iter()
                    .filter(|a| a.action != Action::Dead)
                    .count(),
                w.farming_counters(),
                w.metrics().consumed,
                w.metrics().deficit_ms as f64 / HOUR as f64,
                w.batches()
                    .iter()
                    .filter(|b| b.holder == Holder::Deposit(2))
                    .map(|b| b.quantity)
                    .sum::<u64>()
            )?;
        }
        if tick == 168 * 60 {
            break;
        }
        let mut commands = vec![];
        for a in w.agents() {
            if a.action == Action::Dead {
                continue;
            }
            let o = w.observe(a.id).unwrap();
            let food = o
                .local
                .as_ref()
                .unwrap()
                .food
                .iter()
                .find(|b| b.owner == Owner::Agent(a.id) && b.quantity > 0);
            let intent = if a.nutrition < 12 * HOUR
                && let Some(b) = food
            {
                Intent::Eat {
                    batch: b.batch,
                    quantity: (((24 * HOUR - a.nutrition) / HOUR) as u64).min(b.quantity),
                }
            } else if a.fatigue >= 22 * HOUR || (sleeping[a.id] && a.fatigue > 4 * HOUR) {
                sleeping[a.id] = true;
                Intent::Sleep
            } else {
                sleeping[a.id] = false;
                match &w.plots()[a.id].crop {
                    Some(farming::Crop {
                        stage: farming::Stage::Growing { .. },
                        ..
                    }) => Intent::Wait,
                    _ => Intent::Cultivate { plot: a.id },
                }
            };
            commands.push((a.id, intent));
        }
        w.apply_intents(commands)?;
        w.audit()?;
    }
    writeln!(
        out,
        "FIFO={} audit={:?} state={:016x} event={:016x}\nThis is a feasible research schedule, not optimal or autonomous. All agents stayed at C; initial F stock was untouched and spoiled normally.",
        w.metrics().overtakes,
        w.audit(),
        w.state_hash(),
        w.event_hash()
    )?;
    Ok(())
}
