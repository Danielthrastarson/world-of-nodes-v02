//! F8 research only. Settlement labels, global accounts and summaries never enter policy.
use settlement_research::{CORRIDOR, TRANSIT};
use std::{
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
fn owner(o: Owner) -> String {
    match o {
        Owner::None => "unowned".into(),
        Owner::Agent(id) => format!("agent:{id}"),
        Owner::Commons(id) => format!("commons:{id}"),
    }
}
fn place(location: &Location) -> usize {
    match location {
        Location::Node(0) => 0,
        Location::Node(2) => 1,
        Location::Node(_) => CORRIDOR,
        _ => TRANSIT,
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let arm = args
        .first()
        .ok_or("settlement_probe ARM NEW_DIR [SEEDS] [FIRST_SEED]")?;
    let root = Path::new(args.get(1).ok_or("new directory")?);
    let seeds: u64 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(30);
    let first: u64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(0);
    let end = first.checked_add(seeds).ok_or("seed overflow")?;
    fs::create_dir_all(root)?;
    let mut runs = file(root, "runs.csv")?;
    let mut places = file(root, "places.csv")?;
    let mut accounts = file(root, "accounts.csv")?;
    let mut labor = file(root, "labor.csv")?;
    let mut counts = file(root, "counts.csv")?;
    let mut people = file(root, "people.csv")?;
    let mut inventory = file(root, "inventory.csv")?;
    let mut flows = file(root, "flows.csv")?;
    let mut transfers = file(root, "transfers.csv")?;
    let mut visits = file(root, "visits.csv")?;
    let mut carts = file(root, "carts.csv")?;
    let mut projects = file(root, "projects.csv")?;
    let mut paths = file(root, "paths.csv")?;
    let mut plots = file(root, "plots.tsv")?;
    let mut events = file(root, "events.tsv")?;
    let mut parameters = file(root, "parameters.txt")?;
    writeln!(
        runs,
        "seed,hour,alive,deaths,produced,consumed,spoiled,delivered,jobs,starvation_h,queue_h,food_flows_ab,food_flows_ba,wood_flows_ab,wood_flows_ba,visits,barter_attempts,barter_completed,barter_protocol_present,accounting_error,overtakes,state_hash,event_hash"
    )?;
    writeln!(
        places,
        "seed,hour,place,alive,critical,nutrition_h,food,personal_food,commons_food,natural_food,wood,waste,dead_owner_food,critical_near_foreign_loose_food,plots_free,plots_active,plots_dead_owner,board_demand"
    )?;
    writeln!(accounts, "seed,hour,place,key,value")?;
    writeln!(labor, "seed,hour,cohort,place,activity,person_hours")?;
    writeln!(counts, "seed,hour,place,event,count")?;
    writeln!(
        people,
        "seed,hour,id,cohort,place,alive,nutrition_ms,starvation_ms,consumed,death_ms,location"
    )?;
    writeln!(
        inventory,
        "seed,hour,batch,origin,material,quantity,owner,place,holder,created_ms,expires_ms"
    )?;
    writeln!(
        flows,
        "seed,at_ms,agent,from,to,origin,material,quantity,owner,communal_job,cart,expires_ms"
    )?;
    writeln!(
        transfers,
        "seed,at_ms,agent,place,kind,origin,material,quantity,owner_before,owner_after"
    )?;
    writeln!(visits, "seed,at_ms,agent,cohort,from,to")?;
    writeln!(
        carts,
        "seed,hour,id,owner,user,place,completed_ms,idle_h,trips,food_quanta_edges,load,location"
    )?;
    writeln!(
        projects,
        "seed,hour,id,investor,place,wood,remaining_scaled_ms,labor_h,cart,owner_alive"
    )?;
    writeln!(
        paths,
        "seed,hour,edge,compaction,passages,recoveries,nominal_ms,capacity,queue_h,occupancy_person_h"
    )?;
    for seed in first..end {
        let c = settlement_research::configuration(arm, seed, seed == first)?;
        writeln!(
            parameters,
            "{c:?}; research_place_mapping=[A,corridor,B]; same Commons(0); barter_protocol=absent"
        )?;
        let mut w = World::new(c)?;
        w.enable_settlement_research(vec![0, CORRIDOR, 1])?;
        for hour in 0..=168 {
            w.advance_to(hour * HOUR)?;
            w.audit()?;
            w.audit_settlement_research()?;
            if ![0, 12, 36, 48, 72, 120, 168].contains(&hour) {
                continue;
            }
            let j = w.settlement_journal();
            let m = w.metrics();
            let flow = |from, to, mat| {
                j.flows
                    .iter()
                    .filter(|f| f.from == from && f.to == to && f.material == mat)
                    .map(|f| f.quantity)
                    .sum::<u64>()
            };
            let produced = w.farming_counters()["produced"];
            writeln!(
                runs,
                "{seed},{hour},{},{},{produced},{},{},{},{},{:.9},{:.9},{},{},{},{},{},0,0,false,0,{},{:016x},{:016x}",
                w.agents()
                    .iter()
                    .filter(|a| a.action != Action::Dead)
                    .count(),
                m.deaths,
                m.consumed,
                m.spoiled,
                m.delivered,
                m.jobs,
                m.deficit_ms as f64 / HOUR as f64,
                m.queue_ms as f64 / HOUR as f64,
                flow(0, 1, Material::Food),
                flow(1, 0, Material::Food),
                flow(0, 1, Material::Wood),
                flow(1, 0, Material::Wood),
                j.visits.len(),
                m.overtakes,
                w.state_hash(),
                w.event_hash()
            )?;
            for p in 0..=TRANSIT {
                let residents: Vec<_> = w
                    .agents()
                    .iter()
                    .filter(|a| a.action != Action::Dead && place(&a.location) == p)
                    .collect();
                let stock: Vec<_> = w
                    .batches()
                    .iter()
                    .filter(|b| b.quantity > 0 && j.holder_place(&w, b.holder) == p)
                    .collect();
                let q = |mat| {
                    stock
                        .iter()
                        .filter(|b| b.material == mat)
                        .map(|b| b.quantity)
                        .sum::<u64>()
                };
                let food_owner = |kind| {
                    stock
                        .iter()
                        .filter(|b| {
                            b.material == Material::Food
                                && match kind {
                                    0 => matches!(b.owner, Owner::Agent(_)),
                                    1 => matches!(b.owner, Owner::Commons(_)),
                                    _ => b.owner == Owner::None,
                                }
                        })
                        .map(|b| b.quantity)
                        .sum::<u64>()
                };
                let critical = residents.iter().filter(|a| a.nutrition <= 6 * HOUR).count();
                let foreign=residents.iter().filter(|a|a.nutrition<=6*HOUR && stock.iter().any(|b|b.material==Material::Food&&matches!(b.holder,Holder::Loose(n) if a.location==Location::Node(n))&&matches!(b.owner,Owner::Agent(id) if id!=a.id))).count();
                let dead_food=stock.iter().filter(|b|b.material==Material::Food&&matches!(b.owner,Owner::Agent(id) if w.agents()[id].action==Action::Dead)).map(|b|b.quantity).sum::<u64>();
                let ps: Vec<_> = w
                    .plots()
                    .iter()
                    .filter(|x| j.node_places[x.node] == p)
                    .collect();
                let dead_plots=ps.iter().filter(|x|x.crop.as_ref().is_some_and(|c|matches!(c.owner,Owner::Agent(id) if w.agents()[id].action==Action::Dead))).count();
                let board = w
                    .boards()
                    .iter()
                    .filter(|b| j.node_places[b.node] == p)
                    .map(|b| b.quantity)
                    .sum::<u64>();
                writeln!(
                    places,
                    "{seed},{hour},{p},{},{critical},{:.9},{},{},{},{},{},{},{dead_food},{foreign},{},{},{dead_plots},{board}",
                    residents.len(),
                    residents
                        .iter()
                        .map(|a| a.nutrition as f64 / HOUR as f64)
                        .sum::<f64>(),
                    q(Material::Food),
                    food_owner(0),
                    food_owner(1),
                    food_owner(2),
                    q(Material::Wood),
                    q(Material::Waste),
                    ps.iter().filter(|x| x.crop.is_none()).count(),
                    ps.iter().filter(|x| x.crop.is_some()).count()
                )?;
            }
            for ((p, key), v) in &j.stats {
                writeln!(accounts, "{seed},{hour},{p},{key},{v}")?;
            }
            for ((cohort, p, activity), v) in &j.labor_ms {
                writeln!(
                    labor,
                    "{seed},{hour},{cohort},{p},{activity},{:.9}",
                    *v as f64 / HOUR as f64
                )?;
            }
            for ((p, key), v) in &j.events {
                writeln!(
                    counts,
                    "{seed},{hour},{p},\"{}\",{v}",
                    key.replace('"', "\"\"")
                )?;
            }
            for a in w.agents() {
                writeln!(
                    people,
                    "{seed},{hour},{},{},{},{},{},{},{},{},\"{:?}\"",
                    a.id,
                    j.cohorts[a.id],
                    place(&a.location),
                    a.action != Action::Dead,
                    a.nutrition,
                    a.starvation,
                    a.consumed,
                    a.death_time.map_or(String::new(), |t| t.to_string()),
                    a.location
                )?;
            }
            for b in w.batches().iter().filter(|b| b.quantity > 0) {
                writeln!(
                    inventory,
                    "{seed},{hour},{},{},{:?},{},{},{},\"{:?}\",{},{}",
                    b.id,
                    b.origin,
                    b.material,
                    b.quantity,
                    owner(b.owner),
                    j.holder_place(&w, b.holder),
                    b.holder,
                    b.created_at,
                    b.expires_at()
                )?;
            }
            for c in w.carts() {
                writeln!(
                    carts,
                    "{seed},{hour},{},{},{},{},{},{:.9},{},{},{},\"{:?}\"",
                    c.id,
                    owner(c.owner),
                    c.user.map_or(String::new(), |v| v.to_string()),
                    place(&c.location),
                    c.completed_at,
                    c.idle_ms as f64 / HOUR as f64,
                    c.trips.len(),
                    c.trips.iter().map(|t| t.food_arrived).sum::<u64>(),
                    w.cart_load(c.id),
                    c.location
                )?;
            }
            for p in w.cart_projects() {
                writeln!(
                    projects,
                    "{seed},{hour},{},{},{},{},{},{:.9},{},{}",
                    p.id,
                    p.investor,
                    j.node_places[p.node],
                    p.wood,
                    p.remaining,
                    p.labor_ms as f64 / HOUR as f64,
                    p.cart.map_or(String::new(), |v| v.to_string()),
                    w.agents()[p.investor].action != Action::Dead
                )?;
            }
            for (&edge, s) in w.path_surfaces() {
                let e = w.effective_edge(edge);
                let (q, o) = w.path_edge_time(edge);
                writeln!(
                    paths,
                    "{seed},{hour},{edge},{},{},{},{},{},{:.9},{:.9}",
                    s.compaction,
                    s.passages,
                    s.recovery_count,
                    e.nominal_ms,
                    e.capacity,
                    q as f64 / HOUR as f64,
                    o as f64 / HOUR as f64
                )?;
            }
            if seed == first {
                for p in w.plots() {
                    writeln!(plots, "{hour}\t{p:?}")?;
                }
            }
        }
        let j = w.settlement_journal();
        for f in &j.flows {
            writeln!(
                flows,
                "{seed},{},{},{},{},{},{:?},{},{},{},{},{}",
                f.at,
                f.agent,
                f.from,
                f.to,
                f.origin,
                f.material,
                f.quantity,
                owner(f.owner),
                f.communal_job,
                f.cart,
                f.expires_at
            )?;
        }
        for t in &j.transfers {
            writeln!(
                transfers,
                "{seed},{},{},{},{},{},{:?},{},{},{}",
                t.at,
                t.agent,
                t.place,
                t.kind,
                t.origin,
                t.material,
                t.quantity,
                owner(t.before),
                owner(t.after)
            )?;
        }
        for (at, id, from, to) in &j.visits {
            writeln!(visits, "{seed},{at},{id},{},{from},{to}", j.cohorts[*id])?;
        }
        if seed == first {
            for e in w.events() {
                writeln!(events, "{e:?}")?;
            }
        }
        runs.flush()?;
        eprintln!(
            "{arm} seed={seed} alive={} produced={} consumed={} delivered={} starvation_h={:.3}",
            w.agents()
                .iter()
                .filter(|a| a.action != Action::Dead)
                .count(),
            w.farming_counters()["produced"],
            w.metrics().consumed,
            w.metrics().delivered,
            w.metrics().deficit_ms as f64 / HOUR as f64
        );
    }
    Ok(())
}
