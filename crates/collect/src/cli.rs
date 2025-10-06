use crate::config::Config;
use crate::sources::{Source, SourceOrigin};

use std::path::Path;
use std::{env, process};
use tokio::fs::read_to_string;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

async fn init_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let path_ref = path.as_ref();

    let s = read_to_string(path_ref)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path_ref.display(), e))?;

    let cfg: Config = toml::from_str(&s).map_err(|e| format!("Failed to parse TOML: {e}"))?;

    Ok(cfg)
}

fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = FmtSubscriber::builder()
        .with_thread_names(true)
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
}

pub async fn run() {
    init_logger();
    dotenvy::dotenv().ok();

    let config_path =
        env::var("UNINEWS_CONFIG_PATH").unwrap_or_else(|_| "./config.toml".to_string());

    let config = match init_config(config_path.clone()).await {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to read config file '{config_path}': {e}");
            process::exit(1);
        }
    };

    config.list().for_each(|source| {
        info!(
            "{}",
            match source {
                Source::Atom(atom) => format!("[atom] {}", SourceOrigin::original_url(atom)),
                Source::TelegramChannel(tl) => format!("[tlgr] {}", SourceOrigin::original_url(tl)),
            }
        );
    });
}
