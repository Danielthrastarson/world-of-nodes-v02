use std::{env, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::args()
        .nth(1)
        .ok_or("usage: congestion_probe NEW_OUTPUT_DIRECTORY")?;
    if env::args().nth(2).as_deref() == Some("selected-traces") {
        world_of_nodes::provisioning::congestion::selected_traces(Path::new(&dir))
    } else {
        world_of_nodes::provisioning::congestion::run(Path::new(&dir))
    }
}
