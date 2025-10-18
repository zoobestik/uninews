mod runners;

use crate::services::AppServices;
use crate::state::AppState;
use clap::Parser;
use runners::run_collectors;
use std::path::Path;
use std::sync::Arc;
use std::{env, process};
use tracing::error;

#[derive(Parser, Debug)]
#[command(
    about = "Collect and aggregate content from configured information sources",
    visible_aliases = ["col"],
)]
pub struct CollectCommand {
    #[clap(
        short,
        long,
        help = "Watch configured sources for content updates continuously"
    )]
    watch: bool, // @todo: implement continuous watching mode that runs source update checks periodically instead of one-time collection
}

pub async fn run_collect(_cmd: CollectCommand) {
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
