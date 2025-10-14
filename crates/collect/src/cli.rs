mod logger;
mod runners;

use logger::init_logger;
use runners::run_collectors;

use crate::state::AppState;

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

    let config = match AppState::try_from_file(Path::new(&config_path)).await {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("'{config_path}': {e}");
            process::exit(1);
        }
    };

    run_collectors(config).await;
}
