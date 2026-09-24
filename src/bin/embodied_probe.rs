//! Read-only E0 setup provenance, not a second controller or visible game state.
use std::{fs, path::PathBuf};
use world_of_nodes::provisioning::{Action, playable::session::embodied::EmbodiedSession};
fn main() {
    let out = PathBuf::from(std::env::args().nth(1).expect("fresh output directory"));
    fs::create_dir(&out).expect("fresh directory");
    let s = EmbodiedSession::new(0).expect("E0 setup");
    fs::write(
        out.join("prehistory.txt"),
        format!("{:#?}", s.world().events()),
    )
    .unwrap();
    fs::write(
        out.join("ruler-snapshot.txt"),
        format!("{:#?}", s.snapshot()),
    )
    .unwrap();
    fs::write(
        out.join("physical-setup.txt"),
        format!(
            "now={}\npersons={}\nliving={}\ncarts={:#?}\ncrossings={:#?}\npeople={:#?}\n",
            s.now(),
            s.world().agents().len(),
            s.world()
                .agents()
                .iter()
                .filter(|p| p.action != Action::Dead)
                .count(),
            s.world().carts(),
            s.world().crossing_projects(),
            s.world().agents()
        ),
    )
    .unwrap();
    println!("E0 setup recorded without advancing or modifying it");
}
