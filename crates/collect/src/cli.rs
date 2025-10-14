mod config;
mod logger;
mod runners;

pub use config::init_config;
pub use logger::init_logger;
pub use runners::run_collectors;

use std::path::Path;
use std::{env, process};
use tracing::error;

pub async fn run() {
    if let Err(e) = init_logger() {
        error!("Failed to initialize logger: {e}");
        process::exit(1);
    }

    dotenvy::dotenv().ok();

    let config_path =
        env::var("UNINEWS_CONFIG_PATH").unwrap_or_else(|_| "./config.toml".to_string());

    let config = match init_config(Path::new(&config_path)).await {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("'{config_path}': {e}");
            process::exit(1);
        }
    };

    run_collectors(config).await;
}
