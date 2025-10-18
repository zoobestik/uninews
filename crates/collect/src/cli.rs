mod runners;

use crate::services::AppServices;
use crate::state::AppState;
use runners::run_collectors;
use std::path::Path;
use std::sync::Arc;
use std::{env, process};
use tracing::error;
use uninews_core::cli::init_cli;

pub async fn run() {
    init_cli();

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
