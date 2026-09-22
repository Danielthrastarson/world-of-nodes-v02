use std::{env, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::args()
        .nth(1)
        .ok_or("usage: congestion_probe NEW_OUTPUT_DIRECTORY")?;
    world_of_nodes::provisioning::congestion::run(Path::new(&dir))
}
