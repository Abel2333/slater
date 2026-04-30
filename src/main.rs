#[tokio::main]
async fn main() {
    if let Err(error) = slater::cmd::run().await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
