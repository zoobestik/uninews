mod runners;

use crate::services::AppServices;
use crate::state::AppState;
use clap::Parser;
use runners::run_collectors;
use std::env;
use std::path::Path;
use std::sync::Arc;

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

/// Main function for content collection command execution
///
/// # Errors
///
/// Returns an error if:
/// - a Config file cannot be read or parsed
/// - Application state cannot be initialized with the given config
pub async fn run_collect(_cmd: CollectCommand) -> Result<(), String> {
    let config_path =
        env::var("UNINEWS_CONFIG_PATH").unwrap_or_else(|_| "./config.toml".to_string());

    let app_state =
        AppState::try_from_file(Path::new(&config_path), Arc::new(AppServices::new())).await?;

    run_collectors(app_state).await;

    Ok(())
}
