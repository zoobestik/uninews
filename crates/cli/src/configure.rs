use crate::cli::output::OutputConfig;
use anyhow::{Context, Result};
use dotenvy::{dotenv, var};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_logger() -> Result<()> {
    let log_level = match OutputConfig::is_verbose() {
        true => "debug".to_string(),
        false => var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    };

    let subscriber = FmtSubscriber::builder()
        .with_thread_names(true)
        .with_target(false)
        .with_env_filter(EnvFilter::new(log_level))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global default subscriber")
}

pub fn configure() -> Result<()> {
    init_logger()?;
    dotenv().ok();
    Ok(())
}
