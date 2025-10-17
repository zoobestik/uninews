mod logger;
mod runners;

use logger::init_logger;
use runners::run_collectors;

use crate::state::AppState;

use crate::services::AppServices;
use std::path::Path;
use std::sync::Arc;
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

    let app_state =
        AppState::try_from_file(Path::new(&config_path), Arc::new(AppServices::new())).await;

    run_collectors(match app_state {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("'{config_path}': {e}");
            process::exit(1);
        }
    })
    .await;
}
