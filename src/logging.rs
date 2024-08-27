use dotenv::dotenv;
use log::{info, LevelFilter};
use std::env;

#[feature("logging")]
pub fn init_logger() {
    // Load environment variables from a .env file, if present
    dotenv().ok();

    // Set the default logging level to INFO if not specified in RUST_LOG
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    // Initialize the logger
    env_logger::Builder::from_default_env()
        .filter(None, LevelFilter::Info) // Set default filter level
        .init();

    info!("Logger initialized");
}
