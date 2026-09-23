//! Read-only final-state diagnostic; compile against clean baseline/current rlibs.
use world_of_nodes::provisioning::*;
fn main() {
    let case = std::env::args().nth(1).unwrap();
    let seed = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(0);
    let mut w = combat::research::fixture(&case, true, true, seed).unwrap();
    combat::research::prepare(&mut w, true).unwrap();
    for quarter in 5..=96 { w.advance_to(quarter*15*MINUTE).unwrap(); }
    println!("HASH {:016x}",w.state_hash());
    for p in w.agents() {
        println!("PERSON {} {p:?}",p.id);
        let mut without_reference = p.clone();
        without_reference.last_event = None;
        println!("PERSON_WITHOUT_EVENT_REFERENCE {} {without_reference:?}",p.id);
        println!("BODY {} {:?}",p.id,w.body(p.id));
        println!("UNIT_VIEW {} {:?}",p.id,w.unit_view(p.id));
        println!("MILITARY_MEMORY {} {:?}",p.id,w.military_memory(p.id));
    }
    println!("METRICS {:?}",w.metrics());
    println!("GOODS {:?}",w.batches());
    println!("BOARDS {:?}",w.boards());
    println!("UNITS {:?}",w.units());
}
