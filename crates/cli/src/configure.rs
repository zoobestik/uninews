use anyhow::{Context, Result};
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_logger() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = FmtSubscriber::builder()
        .with_thread_names(true)
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global default subscriber")
}

pub fn configure() -> Result<()> {
    init_logger()?;
    dotenv().ok();
    Ok(())
}
