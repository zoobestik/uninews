use super::{RawAtom, RawTelegramChannel, try_atom_from_raw, try_telegram_channel_from_raw};
use crate::services::AppServices;
use crate::source::atom::Atom;
use crate::source::telegram::TelegramChannel;
use futures::future::try_join_all;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::read_to_string;
use tokio::try_join;
use tracing::debug;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    #[serde(default)]
    #[serde(rename = "atom")]
    atoms_feeds: Vec<RawAtom>,

    #[serde(default)]
    #[serde(rename = "telegram")]
    telegram_channels: Vec<RawTelegramChannel>,
}

pub type AppStateDraft = (Vec<Atom>, Vec<TelegramChannel>);

pub async fn try_new_draft(
    config: RawConfig,
    services: Arc<AppServices>,
) -> Result<AppStateDraft, String> {
    let atom_features: Vec<_> = config
        .atoms_feeds
        .iter()
        .map(|item| {
            let services = services.clone();
            async move {
                debug!("Initializing [atom_feed]: {0}", item.source_url());
                try_atom_from_raw(item, services).await
            }
        })
        .collect();

    let telegram_channels_features: Vec<_> = config
        .telegram_channels
        .iter()
        .map(|x| {
            let services = services.clone();
            async move {
                debug!("Initializing [telegram_channel]: {0}", x.nickname);
                try_telegram_channel_from_raw(&x.nickname, services).await
            }
        })
        .collect();

    let state = try_join!(
        try_join_all(atom_features),
        try_join_all(telegram_channels_features),
    )
    .map_err(|e| format!("Failed to init source: {e}"))?;

    Ok(state)
}

/// Creates a new `AppState` from a configuration file.
///
/// # Arguments
/// * `config_path` - Path to the TOML configuration file
///
/// # Errors
/// * When reading the configuration file fails
/// * When parsing TOML content fails
/// * When initializing source from the configuration fails
pub async fn try_state_from_file(
    config_path: &Path,
    services: Arc<AppServices>,
) -> Result<AppStateDraft, String> {
    let file_content = read_to_string(config_path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", config_path.display(), e))?;

    let config: RawConfig =
        toml::from_str(&file_content).map_err(|e| format!("Failed to parse TOML: {e}"))?;

    try_new_draft(config, services).await
}
