#[cfg(feature = "logging")]
use dotenv::dotenv;

#[cfg(feature = "logging")]
use log::{info, LevelFilter};

#[cfg(feature = "logging")]
use std::env;

#[cfg(feature = "logging")]
pub fn init_logger() {
    dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::Builder::from_default_env()
        .filter(None, LevelFilter::Info)
        .init();

    info!("Logger initialized");
}
