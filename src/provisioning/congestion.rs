//! Research-only conditional transport fixtures and recorded-command replay.
//! Autonomous policy modules cannot access this instrument or its global snapshots.
use super::*;
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct Instrument {
    pub record: bool,
    pub audit_forecasts: bool,
    pub tape: Vec<(Time, usize, Intent)>,
    pub actors: Option<BTreeSet<usize>>,
    pub replay: BTreeMap<Time, Vec<(usize, Intent)>>,
}
#[derive(Clone, Debug)]
struct Outcome {
    stop: Time,
    admitted: Option<Time>,
    arrived: Option<Time>,
    nutrition: Time,
    fatigue: Time,
    reason: String,
    critical: bool,
    queue_ms: Time,
    sleep_ms: Time,
    first_sleep: Option<Time>,
}
fn h(t: Time) -> f64 {
    t as f64 / HOUR as f64
}
fn oh(t: Option<Time>) -> String {
    t.map(|t| format!("{:.9}", h(t))).unwrap_or_default()
}
fn queue(w: &World, focal: usize) -> Vec<(Time, usize, usize)> {
    let key = match w.agents[focal].action {
        Action::Queued { joined, .. } => (joined, focal),
        _ => (w.now, focal),
    };
    let mut q: Vec<_> = w
        .agents
        .iter()
        .filter_map(|a| {
            if let Action::Queued {
                edge: 0,
                to,
                joined,
            } = a.action
            {
                ((joined, a.id) < key).then_some((joined, a.id, to))
            } else {
                None
            }
        })
        .collect();
    q.sort();
    q
}
fn trim(w: &mut World) {
    w.events.clear();
    w.persistence.records.clear();
    w.acceptance.records.clear();
    w.transport_research.tape.clear();
}
fn run_attempt(mut w: World, id: usize, trace: Option<&Path>) -> Result<Outcome, String> {
    let start = w.now;
    let mut admitted = None;
    let mut critical = false;
    let mut reached = None;
    let mut cursor = 0;
    let mut queue_ms = 0;
    let mut sleep_ms = 0;
    let mut first_sleep = None;
    // Snapshot already contains the first local decision/queue entry.
    w.apply_intents(vec![])?;
    let (arrived, reason) = loop {
        let a = &w.agents[id];
        critical |= a.nutrition <= 6 * HOUR;
        for e in &w.events[cursor..] {
            if e.agent == Some(id) && e.kind == "Arrived" && e.detail == "node=0 edge=0" {
                reached = Some(e.at);
            }
            if e.agent == Some(id) && e.kind == "Admitted" && e.detail == "edge=0 from=1 to=0" {
                admitted.get_or_insert(e.at);
            }
        }
        cursor = w.events.len();
        if reached.is_some() || a.location == Location::Node(0) {
            break (reached.or(Some(w.now)), "C_ARRIVAL".to_string());
        }
        if a.action == Action::Dead {
            break (None, "DEATH".to_string());
        }
        if matches!(a.action, Action::Queued { edge: 1, .. })
            || matches!(a.location, Location::Edge { edge: 1, .. })
        {
            break (
                None,
                if a.nutrition <= 6 * HOUR {
                    "CRITICAL_HUNGER_REVERSAL"
                } else {
                    "OTHER_REVERSAL"
                }
                .to_string(),
            );
        }
        if w.now >= 48 * HOUR {
            break (None, "HORIZON".to_string());
        }
        let t = w.next_time(48 * HOUR);
        if matches!(a.action, Action::Queued { edge: 0, .. }) {
            queue_ms += t - w.now;
        }
        if matches!(a.action, Action::Sleep { .. }) {
            first_sleep.get_or_insert(w.now);
            sleep_ms += t - w.now;
        }
        w.advance_to(t)?;
    };
    if let Some(path) = trace {
        let mut f = BufWriter::new(fs::File::create(path).map_err(|e| e.to_string())?);
        writeln!(f, "fork_ms={start} focal={id}").map_err(|e| e.to_string())?;
        for e in &w.events {
            writeln!(f, "{e:?}").map_err(|e| e.to_string())?;
        }
        for r in w.take_persistence_records() {
            writeln!(f, "PLAN\t{}", r.tsv()).map_err(|e| e.to_string())?;
        }
    }
    w.audit()?;
    let a = &w.agents[id];
    Ok(Outcome {
        stop: w.now,
        admitted,
        arrived,
        nutrition: a.nutrition,
        fatigue: a.fatigue,
        reason,
        critical,
        queue_ms,
        sleep_ms,
        first_sleep,
    })
}
fn service_only(mut w: World, id: usize) -> Result<Outcome, String> {
    w.config.autonomous = false;
    w.transport_research = Instrument::default();
    // Keep every currently queued applicant eligible, no new applications.
    trim(&mut w);
    run_attempt(w, id, None)
}
fn unqueued_duration(w: &World, id: usize, delay: Time) -> Time {
    let a = &w.agents[id];
    let mut n = (a.nutrition - delay).max(0);
    let mut f = (a.fatigue + delay).min(24 * HOUR);
    let mut left = 2 * HOUR as u128 * SCALE;
    let mut dt = 0;
    while left > 0 {
        let r = performance(n, f);
        let step = awake_threshold(n, f).min(left.div_ceil(r) as Time).max(1);
        left = left.saturating_sub(r * step as u128);
        dt += step;
        n = (n - step).max(0);
        f = (f + step).min(24 * HOUR);
    }
    dt
}
fn micro(snapshot: &World, id: usize, ahead: usize, direction: &str) -> World {
    let mut w = snapshot.clone();
    trim(&mut w);
    w.transport_research = Instrument {
        actors: Some(BTreeSet::from([id])),
        ..Instrument::default()
    };
    // A conditional initial state: same 50 identities and all material, no history
    // is claimed for these alternative nonfocal node placements/queue submission times.
    let migration = snapshot
        .agents
        .iter()
        .find(|a| matches!(a.action, Action::Queued { edge: 0, to: 1, .. }))
        .unwrap();
    let background: Vec<_> = (0..50).filter(|&x| x != id).collect();
    for (rank, &j) in background.iter().enumerate() {
        let a = &mut w.agents[j];
        a.action = Action::Idle;
        a.location = Location::Node(0);
        a.job = None;
        a.goal = None;
        if rank < ahead {
            let to = match direction {
                "same" => 0,
                "mixed" => usize::from(rank % 2 == 0),
                "observed" => queue(snapshot, id).get(rank).map_or(1, |q| q.2),
                _ => 1,
            };
            a.nutrition = migration.nutrition;
            a.fatigue = migration.fatigue;
            a.starvation = migration.starvation;
            a.location = Location::Node(1 - to);
            a.action = Action::Queued {
                edge: 0,
                to,
                joined: w.now - 1,
            };
        }
    }
    w.gather_communal.clear();
    // No cargo assigned to background travelers. Existing goods of other agents
    // remain physically represented, but this model has no load speed penalty.
    w
}
fn write_out(
    f: &mut impl Write,
    label: &str,
    id: usize,
    count: usize,
    start: Time,
    o: &Outcome,
) -> std::io::Result<()> {
    writeln!(
        f,
        "{label},{id},{count},{:.9},{},{},{:.9},{:.9},{:.9},{},{},{:.9},{:.9},{:.9},{}",
        h(start),
        oh(o.admitted),
        oh(o.arrived),
        h(o.stop),
        h(o.nutrition),
        h(o.fatigue),
        o.reason,
        o.critical,
        h(o.admitted.unwrap_or(o.stop) - start),
        h(o.queue_ms),
        h(o.sleep_ms),
        oh(o.first_sleep)
    )
}
pub fn run(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir)?;
    if dir.join("bottleneck.csv").exists() {
        return Err("refuse to overwrite evidence".into());
    }
    let mut original = persistence::fixture("committed", 0, true)?;
    original.transport_research.record = true;
    original.advance_to(0)?;
    let mut snapshots = BTreeMap::new();
    while original.now < 72 * HOUR {
        let t = original.next_time(72 * HOUR);
        original.advance_to(t)?;
        for (&id, s) in &original.persistence.states {
            if s.active.is_some()
                && s.plans.iter().any(|p| p.reached_x)
                && original.agents[id].location == Location::Node(1)
                && original.agents[id].goal == Some((0, false))
                && !snapshots.contains_key(&id)
            {
                let mut snapshot = original.clone();
                trim(&mut snapshot);
                snapshots.insert(id, snapshot);
            }
        }
    }
    let mut provenance = BufWriter::new(fs::File::create(dir.join("rehydration.txt"))?);
    writeln!(
        provenance,
        "event_hash={:016x}\nstate_hash={:016x}\nsnapshots={}\ncommands={}\n{}",
        original.event_hash(),
        original.state_hash(),
        snapshots.len(),
        original.transport_research.tape.len(),
        experiment::csv_row("committed", 0, &original)
    )?;
    let tape = &original.transport_research.tape;
    let mut table = BufWriter::new(fs::File::create(dir.join("bottleneck.csv"))?);
    writeln!(
        table,
        "agent,arrival_x_h,nutrition_h,fatigue_h,ahead,opposite,same,occupants,until_critical_h,until_zero_h,until_death_h,expiry_h,unqueued_travel_h,min_c_arrival_h,conservative_queue_budget_h,fixed_queue_admission_h,fixed_queue_arrival_h"
    )?;
    let mut outcomes = BufWriter::new(fs::File::create(dir.join("replays.csv"))?);
    let header = "case,agent,ahead,start_h,admission_h,arrival_h,stop_h,nutrition_h,fatigue_h,outcome,critical_hunger,elapsed_until_admission_or_stop_h,actual_queue_h,sleep_h,first_sleep_h";
    writeln!(outcomes, "{header}")?;
    let mut micros = BufWriter::new(fs::File::create(dir.join("micro.csv"))?);
    writeln!(micros, "{header}")?;
    let mut details = BufWriter::new(fs::File::create(dir.join("snapshot-details.txt"))?);
    for (&id, snapshot) in &snapshots {
        let q = queue(snapshot, id);
        let occupied: Vec<_> = snapshot
            .agents
            .iter()
            .filter(|a| {
                matches!(a.location, Location::Edge { edge: 0, .. }) && a.action == Action::Travel
            })
            .collect();
        let a = &snapshot.agents[id];
        let duration = unqueued_duration(snapshot, id, 0);
        let service = service_only(snapshot.clone(), id)?;
        writeln!(
            table,
            "{id},{:.9},{:.9},{:.9},{},{},{},{},{:.9},{:.9},{:.9},48,{:.9},{:.9},{:.9},{},{}",
            h(snapshot.now),
            h(a.nutrition),
            h(a.fatigue),
            q.len(),
            q.iter().filter(|x| x.2 == 1).count(),
            q.iter().filter(|x| x.2 == 0).count(),
            occupied.len(),
            h((a.nutrition - 6 * HOUR).max(0)),
            h(a.nutrition),
            h(a.nutrition + 24 * HOUR - a.starvation),
            h(duration),
            h(snapshot.now + duration),
            h((a.nutrition - 6 * HOUR).max(0) - duration),
            oh(service.admitted),
            oh(service.arrived)
        )?;
        writeln!(
            details,
            "FOCAL {id} at={}\nagent={a:?}\nintention={:?}\nqueue_ahead={q:?}\noccupants={occupied:?}\ngoods={:?}",
            snapshot.now, snapshot.persistence.states[&id], snapshot.goods.batches
        )?;
        let migrations: Vec<_> = q.iter().filter(|x| x.2 == 1).map(|x| x.1).collect();
        for percent in [100, 75, 50, 25, 0] {
            let keep = migrations.len() * percent / 100;
            let retained: BTreeSet<_> = migrations.iter().take(keep).copied().collect();
            let excluded: BTreeSet<_> = migrations
                .iter()
                .filter(|x| !retained.contains(x))
                .copied()
                .collect();
            let mut replay = snapshot.clone();
            trim(&mut replay);
            replay.transport_research = Instrument {
                actors: Some(BTreeSet::from([id])),
                ..Instrument::default()
            };
            for &j in &excluded {
                replay.agents[j].action = Action::Idle;
                replay.agents[j].goal = None;
            }
            for (t, j, intent) in tape
                .iter()
                .filter(|(t, j, _)| *t > snapshot.now && *j != id && !excluded.contains(j))
            {
                replay
                    .transport_research
                    .replay
                    .entry(*t)
                    .or_default()
                    .push((*j, intent.clone()));
            }
            writeln!(
                details,
                "REPLAY focal={id} percent={percent} pending_denominator={} retained={retained:?} excluded={excluded:?} in_flight_unchanged={}",
                migrations.len(),
                occupied.len()
            )?;
            if percent == 100 {
                let mut oracle = replay.clone();
                oracle.transport_research.actors = Some(BTreeSet::new());
                let o = run_attempt(oracle, id, None)?;
                write_out(
                    &mut outcomes,
                    "recorded_background_hold_focal",
                    id,
                    keep,
                    snapshot.now,
                    &o,
                )?;
            }
            let trace = dir.join(format!("replay-{id}-{percent}.trace"));
            let o = run_attempt(replay, id, Some(&trace))?;
            write_out(
                &mut outcomes,
                &format!("load_{percent}"),
                id,
                keep,
                snapshot.now,
                &o,
            )?;
        }
        // Both actual physiological arrival cohorts; replicate IDs remain separately
        // represented by replay/bottleneck rows. No parameter scan beyond 40 ahead.
        if id == 0 || id == 4 {
            for direction in ["same", "opposite", "mixed", "observed"] {
                for count in 0..=40 {
                    let w = micro(snapshot, id, count, direction);
                    w.audit()?;
                    let o = run_attempt(w, id, None)?;
                    write_out(&mut micros, direction, id, count, snapshot.now, &o)?;
                }
            }
        }
    }
    Ok(())
}

impl World {
    pub fn enable_transport_forecast_audit(&mut self) {
        self.transport_research.audit_forecasts = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conditional_transport_has_atomic_arrival_and_direction_symmetric_service() {
        let mut snapshot = persistence::fixture("committed", 0, true).unwrap();
        snapshot.advance_to(8 * HOUR).unwrap();
        assert_eq!(snapshot.agents[0].nutrition, 15 * HOUR);
        let focal = snapshot.agents[0].clone();
        let mut results = vec![];
        for direction in ["same", "opposite", "mixed", "observed"] {
            let mut w = micro(&snapshot, 0, 8, direction);
            assert_eq!(w.agents[0], focal);
            assert_eq!(w.goods.batches, snapshot.goods.batches);
            w.config.autonomous = false;
            let o = run_attempt(w, 0, None).unwrap();
            assert!(o.arrived.is_some());
            results.push((o.admitted, o.arrived, o.nutrition));
        }
        assert!(results.iter().all(|r| r == &results[0]));
    }
    #[test]
    fn disabled_research_instrument_has_no_effect() {
        let mut w = persistence::fixture("committed", 0, false).unwrap();
        w.advance_to(10 * HOUR).unwrap();
        let mut v = persistence::fixture("committed", 0, false).unwrap();
        v.transport_research.record = true;
        v.advance_to(10 * HOUR).unwrap();
        assert_eq!(w.event_hash(), v.event_hash());
        assert_eq!(w.state_hash(), v.state_hash());
        assert!(!v.transport_research.tape.is_empty());
    }
}
