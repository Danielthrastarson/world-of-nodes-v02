//! Diagnostic only: run clean baseline, replace only final causal-event refs.
//! The temporary baseline adds diagnostic_set_event_refs after the run; neither
//! this helper nor its setter exists in the production kernel.
use world_of_nodes::provisioning::*;
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let seed = args[1].parse().unwrap();
    let mut w = combat::research::fixture(&args[0], true, true, seed).unwrap();
    combat::research::prepare(&mut w, true).unwrap();
    for quarter in 5..=96 { w.advance_to(quarter*15*MINUTE).unwrap(); }
    println!("baseline={:016x}",w.state_hash());
    let refs: Vec<_> = args[2..].iter().map(|pair| {
        let (id, value) = pair.split_once('=').unwrap();
        (id.parse().unwrap(), Some(value.parse().unwrap()))
    }).collect();
    w.diagnostic_set_event_refs(&refs);
    println!("only_event_refs_replaced={:016x}",w.state_hash());
}
