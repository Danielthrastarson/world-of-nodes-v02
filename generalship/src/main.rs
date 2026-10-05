#[tokio::main]
async fn main() {
    if let Err(e) = generalship_g0::server::run().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
