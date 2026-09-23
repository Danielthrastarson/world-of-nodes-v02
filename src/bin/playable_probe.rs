//! Reproducible P0–P4 headless interface evidence. Observer metrics never enter policy.
use std::{
    collections::BTreeMap,
    env, fs,
    hint::black_box,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};
use world_of_nodes::provisioning::{
    playable::{
        MilitaryOrder,
        session::{PlayerCommand, ScenarioKind, Session},
        snapshot::{ViewMode, capture},
    },
    *,
};

type Row = BTreeMap<String, String>;
fn create(path: &Path) -> std::io::Result<BufWriter<fs::File>> {
    Ok(BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?,
    ))
}
fn csv(writer: &mut impl Write, row: &Row, header: bool) -> std::io::Result<()> {
    fn cell(s: &str) -> String {
        format!("\"{}\"", s.replace('"', "\"\""))
    }
    if header {
        writeln!(
            writer,
            "{}",
            row.keys().map(|s| cell(s)).collect::<Vec<_>>().join(",")
        )?;
    }
    writeln!(
        writer,
        "{}",
        row.values().map(|s| cell(s)).collect::<Vec<_>>().join(",")
    )
}
fn provenance(root: &Path, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = create(&root.join("parameters.txt"))?;
    writeln!(
        out,
        "{text}\nNo fake render entities; no outcome holds or parameter adjustments.\nMetrics use observer truth; Play snapshots remain bounded."
    )?;
    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()?;
    writeln!(
        out,
        "base_head={}",
        String::from_utf8_lossy(&head.stdout).trim()
    )?;
    let mut executable_hash = 0xcbf29ce484222325u64;
    let mut executable = BufReader::new(fs::File::open(env::current_exe()?)?);
    let mut block = [0u8; 65536];
    loop {
        let count = executable.read(&mut block)?;
        if count == 0 {
            break;
        }
        for byte in &block[..count] {
            executable_hash ^= *byte as u64;
            executable_hash = executable_hash.wrapping_mul(0x100000001b3);
        }
    }
    writeln!(out, "compiled_executable_fnv1a64={executable_hash:016x}")?;
    let diff = std::process::Command::new("git")
        .args(["status", "--short"])
        .output()?;
    writeln!(
        out,
        "working_tree_at_run:\n{}",
        String::from_utf8_lossy(&diff.stdout)
    )?;
    // Exact source fingerprint includes every .rs under src and this probe.
    fn visit(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit(&path, files)?;
            } else if path.extension().is_some_and(|e| e == "rs") {
                files.push(path);
            }
        }
        Ok(())
    }
    let mut files = vec![];
    visit(Path::new("src"), &mut files)?;
    files.sort();
    let mut hash = 0xcbf29ce484222325u64;
    for path in files {
        for byte in path.to_string_lossy().bytes().chain(fs::read(&path)?) {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    writeln!(
        out,
        "all_src_rs_fnv1a64={hash:016x}\nTiming is wall clock on this host, not a hardware-independent bound."
    )?;
    Ok(())
}
fn metrics(session: &Session, scenario: &str, arm: &str, seed: u64) -> Row {
    let w = session.world();
    let m = w.metrics();
    let mut row = Row::new();
    macro_rules! v {
        ($key:expr, $value:expr) => {
            row.insert($key.into(), format!("{}", $value));
        };
    }
    v!("scenario", scenario);
    v!("arm", arm);
    v!("seed", seed);
    v!("at_ms", w.now());
    v!("state_hash", format!("{:016x}", w.state_hash()));
    v!("event_hash", format!("{:016x}", w.event_hash()));
    v!("persons", w.agents().len());
    v!("deaths", m.deaths);
    v!("starvation_ms", m.deficit_ms);
    v!("consumed", m.consumed);
    v!("spoiled", m.spoiled);
    v!("delivered", m.delivered);
    v!("queue_ms", m.queue_ms);
    v!("jobs", m.jobs);
    let ruler = &w.agents()[session.ruler()];
    v!("ruler_alive", ruler.action != Action::Dead);
    v!(
        "ruler_death_ms",
        ruler
            .death_time
            .map_or_else(|| "none".into(), |t| t.to_string())
    );
    v!("ruler_location", format!("{:?}", ruler.location));
    v!("ruler_action", format!("{:?}", ruler.action));
    v!("ruler_nutrition_ms", ruler.nutrition);
    v!(
        "ruler_injuries",
        w.body(session.ruler()).map_or(0, |b| b.injuries)
    );
    v!("military_food", w.military_journal().food);
    v!(
        "soldier_starvation_ms",
        w.military_journal().soldier_starvation_ms
    );
    v!(
        "civilian_starvation_ms",
        w.military_journal().civilian_starvation_ms
    );
    v!("stolen", w.combat_journal().stolen);
    v!("combat_ms", w.combat_journal().combat_ms);
    v!(
        "first_contact_ms",
        w.combat_journal()
            .first_contact
            .map_or_else(|| "none".into(), |t| t.to_string())
    );
    v!(
        "military_service_roles",
        w.agents()
            .iter()
            .filter(|p| w.military_role(p.id).is_some())
            .count()
    );
    v!("input_count", session.command_log().len());
    v!(
        "input_errors",
        session
            .command_log()
            .iter()
            .filter(|r| r.outcome.is_err())
            .count()
    );
    v!("care_journal", format!("{:?}", w.care_journal()));
    v!(
        "care_food_transferred",
        w.care_journal().count("food_transferred")
    );
    v!("care_travel_ms", w.care_journal().care_travel_ms);
    v!(
        "household_separation_pair_ms",
        w.care_journal().separation_ms
    );
    v!(
        "observed_person_node_changes",
        w.care_journal().residence_changes
    );
    v!(
        "initial_dependents_starvation_ms",
        w.care_journal()
            .starvation_ms
            .iter()
            .filter(|(id, _)| **id >= 16)
            .map(|(_, v)| *v)
            .sum::<u128>()
    );
    v!(
        "body_injuries",
        w.agents()
            .iter()
            .map(|p| w.body(p.id).map_or(0, |b| b.injuries as u64))
            .sum::<u64>()
    );
    v!(
        "incapacitated_alive",
        w.agents()
            .iter()
            .filter(|p| p.action != Action::Dead && w.body(p.id).is_some_and(|b| b.injuries >= 2))
            .count()
    );

    for (k, value) in w.farming_counters() {
        v!(&format!("farm_{k}"), value);
    }
    for (k, value) in w.infrastructure_counters() {
        v!(&format!("crossing_{k}"), value);
    }
    for key in [
        "orders_issued",
        "orders_received",
        "orders_valid",
        "orders_invalid",
        "orders_transmitted",
        "obligations_formed",
        "obligations_completed",
        "obligations_failed",
        "obligations_refused",
        "obligations_revoked",
        "revocations_sent",
        "revocations_received",
        "revocations_transmitted",
    ] {
        v!(&format!("command_{key}"), w.command_journal().count(key));
    }
    for key in ["reports_created", "handoffs"] {
        v!(&format!("info_{key}"), w.information_journal().count(key));
    }
    v!(
        "crossing_complete_ms",
        w.crossing_projects()
            .first()
            .and_then(|p| p.completed_at)
            .map_or_else(|| "none".into(), |t| t.to_string())
    );
    let play = capture(w, session.ruler(), ViewMode::Play);
    v!("play_visible_person_records", play.persons.len());
    v!("play_received_reports", play.reports.len());
    v!(
        "issuer_known_order_status",
        play.orders
            .iter()
            .map(|o| format!("{}:{}", o.id, o.status))
            .collect::<Vec<_>>()
            .join(";")
    );
    row
}
fn scenarios(root: &Path, seeds: u64, hours: i64) -> Result<(), Box<dyn std::error::Error>> {
    provenance(
        root,
        &format!(
            "mode=scenarios\nseeds=0..{seeds}\nelapsed_hours={hours}\nArms use identical Session setup per Local/Remote seed. Local and Remote differ by the documented initial physical journey. No-command already contains a consenting four-Person militia, so it is not a no-military control. Seeded hostile raid is a controlled threat. Every row is one registered transcript, not an optimized policy.\nInputs at initial session time; commission_revoke adds revoke at elapsed 2h; hourly read checkpoints; exact transcript replay compared at end."
        ),
    )?;
    let arms = [
        "none",
        "commission",
        "hold",
        "move",
        "withdraw",
        "attack",
        "commission_hold",
        "commission_revoke",
    ];
    let mut results = create(&root.join("results.csv"))?;
    let mut first = true;
    for (label, scenario) in [
        ("local", ScenarioKind::Local),
        ("remote", ScenarioKind::Remote),
    ] {
        for seed in 0..seeds {
            for arm in arms {
                let dir = root.join(format!("{label}-{arm}-{seed}"));
                fs::create_dir(&dir)?;
                let mut session = Session::new(seed, scenario)?;
                let start = session.world().now();
                let end = start + hours * HOUR;
                let started = Instant::now();
                if arm.starts_with("commission") {
                    let _ = session.dispatch(PlayerCommand::Commission {
                        office: session.office(),
                        project: 0,
                    });
                }
                let at_node = match session.world().agents()[session.ruler()].location {
                    Location::Node(n) => n,
                    _ => return Err("initial ruler still travelling".into()),
                };
                let military = match arm {
                    "hold" | "commission_hold" => Some((MilitaryOrder::Hold, at_node)),
                    "move" => Some((MilitaryOrder::Move, 2)),
                    "withdraw" => Some((MilitaryOrder::Withdraw, if at_node == 0 { 1 } else { 0 })),
                    "attack" => Some((MilitaryOrder::Attack, 2)),
                    _ => None,
                };
                if let Some((verb, target)) = military {
                    let _ = session.dispatch(PlayerCommand::Unit { verb, target });
                }
                let mut timeline = create(&dir.join("timeline.csv"))?;
                csv(&mut timeline, &metrics(&session, label, arm, seed), true)?;
                for hour in 1..=hours {
                    session.advance_to(start + hour * HOUR)?;
                    if arm == "commission_revoke" && hour == 2 {
                        let _ = session.dispatch(PlayerCommand::Revoke { command: 0 });
                    }
                    session.world().audit()?;
                    csv(&mut timeline, &metrics(&session, label, arm, seed), false)?;
                }
                let run_ns = started.elapsed().as_nanos();
                let transcript = session.command_log().to_vec();
                let replay_started = Instant::now();
                let replay = Session::replay(seed, scenario, &transcript, end)?;
                let replay_ns = replay_started.elapsed().as_nanos();
                let equal = session.world().state_hash() == replay.world().state_hash()
                    && session.world().event_hash() == replay.world().event_hash();
                if !equal {
                    return Err(format!("replay mismatch {label}/{arm}/{seed}").into());
                }
                let mut row = metrics(&session, label, arm, seed);
                row.insert("replay_equal".into(), equal.to_string());
                row.insert("run_wall_ns".into(), run_ns.to_string());
                row.insert("replay_wall_ns".into(), replay_ns.to_string());
                csv(&mut results, &row, first)?;
                results.flush()?;
                first = false;
                writeln!(create(&dir.join("transcript.txt"))?, "{transcript:#?}")?;
                let mut events = create(&dir.join("events.tsv"))?;
                writeln!(events, "id\tat_ms\tperson\tcause\tkind\tdetail")?;
                for event in session.world().events() {
                    writeln!(
                        events,
                        "{}\t{}\t{:?}\t{:?}\t{}\t{}",
                        event.id,
                        event.at,
                        event.agent,
                        event.cause,
                        event.kind,
                        event.detail.replace('\t', " ")
                    )?;
                }
                writeln!(
                    create(&dir.join("final_state.txt"))?,
                    "commands={:#?}\nprojects={:#?}\ncombat={:#?}\nunit={:#?}\npeople={:#?}",
                    session.world().command_journal(),
                    session.world().crossing_projects(),
                    session.world().combat_journal(),
                    session.world().units(),
                    session.world().agents()
                )?;
                println!(
                    "{label} {arm} seed={seed} ruler_alive={} crossing={} stolen={} replay={equal}",
                    row["ruler_alive"], row["crossing_completed"], row["stolen"]
                );
            }
        }
    }
    Ok(())
}
fn rss() -> String {
    fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .filter(|l| l.starts_with("VmRSS:") || l.starts_with("VmHWM:"))
        .collect::<Vec<_>>()
        .join("; ")
}
fn performance(root: &Path, people: usize) -> Result<(), Box<dyn std::error::Error>> {
    if !(1..=5000).contains(&people) {
        return Err("bounded probe requires 1..=5000 Persons".into());
    }
    provenance(
        root,
        &format!(
            "mode=performance\nactual_persons={people}\nnodes=10 line; starts=id%10; 9 edges nominal15m capacity4; 2 food quanta per Person at node9; shelf48h; nutrition24h; fatigue0; autonomous=true; all other Config defaults; no institutions added.\nDuration=1 simulated hour. Capture50 repetitions per mode at each checkpoint 0/15m/60m. Report median/p95. No renderer measured; separate sparse information stress fixture, not the integrated game or a population-scale validation. RSS is this process including allocator retention; VmHWM is process peak, not live World footprint."
        ),
    )?;
    let before_rss = rss();
    let build_started = Instant::now();
    let mut world = World::new(Config {
        nodes: 10,
        starts: (0..people).map(|id| id % 10).collect(),
        edges: (0..9)
            .map(|a| Edge {
                a,
                b: a + 1,
                nominal_ms: HOUR / 4,
                capacity: 4,
            })
            .collect(),
        food: vec![(9, people as u64 * 2, 0, 48 * HOUR)],
        boards: vec![],
        communal: false,
        initial_nutrition: 24 * HOUR,
        initial_fatigue: 0,
        trace: false,
        ..Config::default()
    })?;
    let build_ns = build_started.elapsed().as_nanos();
    let mut writer = create(&root.join("performance.csv"))?;
    let mut first = true;
    let mut simulation_ns = 0u128;
    for target in [0, HOUR / 4, HOUR] {
        let started = Instant::now();
        world.advance_to(target)?;
        simulation_ns += started.elapsed().as_nanos();
        world.audit()?;
        for mode in [ViewMode::Play, ViewMode::Debug] {
            let state = world.state_hash();
            let events = world.event_hash();
            let sample = capture(&world, 0, mode);
            let mut durations = Vec::new();
            for _ in 0..50 {
                let start = Instant::now();
                black_box(capture(black_box(&world), 0, mode));
                durations.push(start.elapsed().as_nanos());
            }
            durations.sort_unstable();
            assert_eq!(world.state_hash(), state);
            assert_eq!(world.event_hash(), events);
            let mut row = Row::new();
            for (key, value) in [
                ("persons", people.to_string()),
                ("peak_persons", world.agents().len().to_string()),
                ("at_ms", world.now().to_string()),
                ("mode", format!("{mode:?}")),
                ("build_ns", build_ns.to_string()),
                ("cumulative_simulate_ns", simulation_ns.to_string()),
                (
                    "snapshot_median_ns",
                    ((durations[durations.len() / 2 - 1] + durations[durations.len() / 2]) / 2)
                        .to_string(),
                ),
                (
                    "snapshot_p95_ns",
                    durations[(durations.len() * 95 / 100).min(durations.len() - 1)].to_string(),
                ),
                ("visible_person_records", sample.persons.len().to_string()),
                (
                    "snapshot_debug_text_bytes",
                    format!("{sample:?}").len().to_string(),
                ),
                ("rss_before", before_rss.clone()),
                ("rss_now", rss()),
                ("state_hash", format!("{state:016x}")),
                ("event_hash", format!("{events:016x}")),
            ] {
                row.insert(key.into(), value);
            }
            csv(&mut writer, &row, first)?;
            first = false;
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() < 2 {
        return Err("playable_probe scenarios NEW_OUTDIR [SEEDS=1] [HOURS=24] | performance NEW_OUTDIR [PERSONS=1000]".into());
    }
    let root = Path::new(&args[1]);
    fs::create_dir(root)?;
    match args[0].as_str() {
        "scenarios" => {
            let seeds = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(1);
            let hours = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(24);
            if !(1..=30).contains(&seeds) || !(2..=72).contains(&hours) {
                return Err("bound is 1..=30 seeds and 2..=72 hours".into());
            }
            scenarios(root, seeds, hours)?;
        }
        "performance" => performance(
            root,
            args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(1000),
        )?,
        _ => return Err("unknown mode".into()),
    }
    Ok(())
}
