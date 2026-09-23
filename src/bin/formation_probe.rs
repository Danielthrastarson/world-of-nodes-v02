//! F11B: one local unit objective, never individual movement injection.
use std::{
    collections::BTreeMap,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
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
fn h(t: u128) -> f64 {
    t as f64 / HOUR as f64
}
fn time(t: Option<Time>, base: Time) -> String {
    t.map(|x| format!("{:.9}", (x - base) as f64 / HOUR as f64))
        .unwrap_or_default()
}
fn activity(j: &household::Journal, begin: usize, end: usize, tag: &str) -> u128 {
    j.activity_ms
        .iter()
        .filter(|((id, s), _)| *id >= begin && *id < end && s == tag)
        .map(|(_, q)| q)
        .sum()
}
fn starvation(j: &household::Journal, begin: usize, end: usize) -> u128 {
    j.starvation_ms
        .iter()
        .filter(|(id, _)| **id >= begin && **id < end)
        .map(|(_, q)| q)
        .sum()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let case = args
        .first()
        .ok_or("formation_probe CASE move|hold NEW_DIR [SEEDS]")?;
    let moving = match args.get(1).map(String::as_str) {
        Some("move") => true,
        Some("hold") => false,
        _ => return Err("arm".into()),
    };
    let root = Path::new(args.get(2).ok_or("directory")?);
    let seeds: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(30);
    fs::create_dir_all(root)?;
    let mut runs = file(root, "runs.csv")?;
    let mut events = file(root, "events.tsv")?;
    let mut people = file(root, "people.csv")?;
    let mut timeline = file(root, "timeline.csv")?;
    let mut positions = file(root, "positions.tsv")?;
    let mut activities = file(root, "activity.csv")?;
    let mut pauses = file(root, "pauses.csv")?;
    let mut params = file(root, "parameters.txt")?;
    let mut counts = file(root, "counts.csv")?;
    let mut trips = file(root, "trips.tsv")?;
    writeln!(events, "seed\tid\ttime_ms\tperson\tkind\tdetail")?;
    writeln!(
        positions,
        "seed\ttime_ms\tperson\tlocation\taction\tphase\tnutrition_ms\tfatigue_ms"
    )?;
    writeln!(
        people,
        "seed,person,alive,received,active,assembled_h,departed_h,arrived_h,later_endpoint_h,released_h,release_reason,phase,starvation_h,food,nutrition_h,fatigue_h,location"
    )?;
    writeln!(
        timeline,
        "seed,time_h,enlisted,ever_assembled,active,moving,queued,recovering,at_objective,separated,modal_fraction,objective_fraction,near_anchor_fraction,coincident_fraction,paused_away,active_without_order,max_endpoint_hops,departed,arrived,late_endpoint"
    )?;
    writeln!(activities, "seed,person,activity,hours")?;
    writeln!(pauses, "seed,reason,hours")?;
    writeln!(counts, "seed,key,count")?;
    writeln!(trips, "seed\ttrip")?;
    for seed in 0..seeds {
        let mut w = formation::research::fixture(case, seed)?;
        formation::research::prepare_command(&mut w, case)?;
        let start = w.now();
        let pre = w.state_hash();
        let j0 = w.formation_journal().clone();
        let c0 = w.care_journal().clone();
        let m0 = w.metrics().clone();
        let prod0 = formation::research::produced(&w);
        let food0: Vec<_> = w.agents().iter().map(|a| a.consumed).collect();
        let assembled_command = w.unit_snapshot(0).unwrap_or_default().active;
        let result = if moving {
            w.issue_unit_move(
                0,
                formation::research::issue_node(case),
                formation::research::target(case),
            )
        } else {
            Ok(0)
        };
        if seed == 0 {
            writeln!(
                params,
                "{}\ncase={case}; moving={moving}; command_time={start}; result={result:?}; horizon=24h after command; pre_command_hash={pre:016x}",
                formation::research::parameters(&w)
            )?;
        }
        let mut peak_moving = 0;
        let mut peak_separated = 0;
        let mut peak_queued = 0;
        let mut peak_active = 0;
        for step in 0..=96 {
            w.advance_to(start + step * 15 * MINUTE)?;
            let s = w.unit_snapshot(0).unwrap_or_default();
            peak_moving = peak_moving.max(s.moving);
            peak_separated = peak_separated.max(s.separated);
            peak_queued = peak_queued.max(s.queued);
            peak_active = peak_active.max(s.active);
            let frac = |v: f64| {
                if s.active > 0 {
                    format!("{v:.9}")
                } else {
                    String::new()
                }
            };
            writeln!(
                timeline,
                "{seed},{:.2},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                step as f64 / 4.,
                s.enlisted,
                s.ever_assembled,
                s.active,
                s.moving,
                s.queued,
                s.recovering,
                s.at_objective,
                s.separated,
                frac(s.modal_fraction),
                frac(s.objective_fraction),
                frac(s.near_anchor_fraction),
                frac(s.coincident_fraction),
                s.paused_away,
                s.active_without_order,
                s.max_endpoint_hops,
                s.departed,
                s.arrived,
                s.late_endpoint
            )?;
            if seed == 0 {
                for a in w.agents() {
                    writeln!(
                        positions,
                        "{seed}\t{}\t{}\t{:?}\t{:?}\t{}\t{}\t{}",
                        w.now(),
                        a.id,
                        a.location,
                        a.action,
                        w.unit_view(a.id).map_or("", |v| v.phase.as_str()),
                        a.nutrition,
                        a.fatigue
                    )?;
                }
            }
        }
        w.audit()?;
        let j = w.formation_journal();
        let c = w.care_journal();
        let m = w.metrics();
        let s = w.unit_snapshot(0).unwrap_or_default();
        let mut r: BTreeMap<String, String> = BTreeMap::new();
        macro_rules! set {
            ($k:expr,$v:expr) => {
                r.insert($k.into(), format!("{}", $v));
            };
        }
        set!("seed", seed);
        set!("case", case);
        set!("arm", if moving { "move" } else { "hold" });
        set!("pre_command_hash", format!("{pre:016x}"));
        set!("state_hash", format!("{:016x}", w.state_hash()));
        set!("event_hash", format!("{:016x}", w.event_hash()));
        set!("command_rejected", usize::from(result.is_err()));
        set!("receipts", result.unwrap_or(0));
        set!("enlisted", s.enlisted);
        set!("assembled_at_command", assembled_command);
        set!("ever_assembled", s.ever_assembled);
        set!("departed", s.departed);
        set!("arrived", s.arrived);
        set!("later_endpoint", s.late_endpoint);
        set!("released", s.released);
        set!("roster_dead", s.dead);
        set!("peak_moving_sampled", peak_moving);
        set!("peak_separated_sampled", peak_separated);
        set!("peak_queued_sampled", peak_queued);
        set!("peak_active_sampled", peak_active);
        let arrivals: Vec<_> = (0..w.agents().len())
            .filter_map(|id| w.unit_view(id)?.arrived_at)
            .collect();
        set!(
            "first_arrival_h",
            time(arrivals.iter().min().copied(), start)
        );
        set!(
            "last_arrival_h",
            time(arrivals.iter().max().copied(), start)
        );
        set!("arrival_spread_h", time(s.arrival_spread, 0));
        set!("role_queue_h", h(j.member_queue_ms - j0.member_queue_ms));
        set!("role_travel_h", h(j.member_travel_ms - j0.member_travel_ms));
        set!(
            "role_starvation_h",
            h(j.member_starvation_ms - j0.member_starvation_ms)
        );
        set!(
            "nonrole_starvation_h",
            h(j.civilian_starvation_ms - j0.civilian_starvation_ms)
        );
        set!("role_food", j.member_food - j0.member_food);
        set!("role_hours", h(j.member_time_ms - j0.member_time_ms));
        let elapsed = j.member_time_ms - j0.member_time_ms;
        set!(
            "role_mean_fatigue_h",
            if elapsed > 0 {
                (j.member_fatigue_ms - j0.member_fatigue_ms) as f64 / elapsed as f64 / HOUR as f64
            } else {
                0.
            }
        );
        set!("role_max_fatigue_h", j.max_fatigue as f64 / HOUR as f64);
        set!("food_consumed", m.consumed - m0.consumed);
        set!("food_produced", formation::research::produced(&w) - prod0);
        set!("food_spoiled", m.spoiled - m0.spoiled);
        set!("deaths", m.deaths - m0.deaths);
        set!("starvation_h", h(m.deficit_ms - m0.deficit_ms));
        set!("queue_h", h(m.queue_ms - m0.queue_ms));
        set!(
            "care_food",
            c.count("food_transferred") - c0.count("food_transferred")
        );
        set!("care_travel_h", h(c.care_travel_ms - c0.care_travel_ms));
        set!("care_queue_h", h(c.care_queue_ms - c0.care_queue_ms));
        set!(
            "household_separation_h",
            h(c.separation_ms - c0.separation_ms)
        );
        set!(
            "residence_changes",
            c.residence_changes - c0.residence_changes
        );
        let (crossings, carts, path_trips) = formation::research::infrastructure_counts(&w);
        set!("completed_crossings", crossings);
        set!("carts", carts);
        set!("path_trips_total", path_trips);
        set!("accounting_error", 0);
        set!("overtakes", m.overtakes);
        for (name, lo, hi) in [
            ("original_recruits", 0, 4),
            ("original_civilians", 4, w.agents().len()),
        ] {
            set!(
                format!("{name}_starvation_h"),
                h(starvation(c, lo, hi) - starvation(&c0, lo, hi))
            );
            for tag in [
                "queue",
                "travel",
                "sleep",
                "farming",
                "construction",
                "gather",
                "military_duty",
                "idle",
                "reproduction",
            ] {
                set!(
                    format!("{name}_{tag}_h"),
                    h(activity(c, lo, hi, tag) - activity(&c0, lo, hi, tag))
                );
            }
        }
        if seed == 0 {
            writeln!(runs, "{}", r.keys().cloned().collect::<Vec<_>>().join(","))?;
        }
        writeln!(
            runs,
            "{}",
            r.values().cloned().collect::<Vec<_>>().join(",")
        )?;
        for (key, q) in &j.counts {
            writeln!(counts, "{seed},{key},{}", q - j0.count(key))?;
        }
        for (key, q) in &j.pause_ms {
            writeln!(
                pauses,
                "{seed},{key},{:.9}",
                h(q - j0.pause_ms.get(key).copied().unwrap_or(0))
            )?;
        }
        for ((id, tag), q) in &c.activity_ms {
            writeln!(
                activities,
                "{seed},{id},{tag},{:.9}",
                h(q - c0
                    .activity_ms
                    .get(&(*id, tag.clone()))
                    .copied()
                    .unwrap_or(0))
            )?;
        }
        for a in w.agents() {
            let v = w.unit_view(a.id).unwrap();
            let member = v
                .unit
                .and_then(|id| w.units().get(&id))
                .and_then(|u| u.members.get(&a.id));
            writeln!(
                people,
                "{seed},{},{},{},{},{},{},{},{},{},{},{},{:.9},{},{:.9},{:.9},\"{:?}\"",
                a.id,
                a.action != Action::Dead,
                v.order.is_some(),
                v.active,
                time(member.and_then(|m| m.assembled_at), 0),
                time(v.departed_at, start),
                time(v.arrived_at, start),
                time(v.post_service_arrival, start),
                time(member.and_then(|m| m.ended.as_ref().map(|x| x.0)), start),
                member
                    .and_then(|m| m.ended.as_ref().map(|x| x.1.as_str()))
                    .unwrap_or(""),
                v.phase,
                h(c.starvation_ms.get(&a.id).copied().unwrap_or(0)
                    - c0.starvation_ms.get(&a.id).copied().unwrap_or(0)),
                a.consumed - food0.get(a.id).copied().unwrap_or(0),
                a.nutrition as f64 / HOUR as f64,
                a.fatigue as f64 / HOUR as f64,
                a.location
            )?;
        }
        if seed == 0 {
            for e in w.events() {
                writeln!(
                    events,
                    "{seed}\t{}\t{}\t{:?}\t{}\t{}",
                    e.id, e.at, e.agent, e.kind, e.detail
                )?;
            }
            for t in w.path_trips() {
                writeln!(trips, "{seed}\t{t:?}")?;
            }
        }
    }
    Ok(())
}
