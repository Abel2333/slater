fn main() {
    if let Err(error) = slater::cmd::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
