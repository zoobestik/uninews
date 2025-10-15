mod raw;

use crate::services::AppServices;
use crate::sources::Source;
use crate::sources::atom::Atom;
use crate::sources::telegram::TelegramChannel;
use crate::state::raw::{RawConfig, from_atom_raw, from_telegram_channels_raw};

use futures::future::try_join_all;
use futures::try_join;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::read_to_string;

pub struct AppState {
    atoms_channels: Vec<Atom>,
    telegram_channels: Vec<TelegramChannel>,
}

impl AppState {
    pub const fn new(atoms_channels: Vec<Atom>, telegram_channels: Vec<TelegramChannel>) -> Self {
        Self {
            atoms_channels,
            telegram_channels,
        }
    }

    /// Creates new `AppState` from raw config
    ///
    /// # Arguments
    /// * `cfg_raw` - Raw config that will be parsed and validated
    ///
    /// # Errors
    /// * When sources initialization fails with a detailed error message
    pub async fn try_from_raw(cfg_raw: RawConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let app_state = Arc::new(AppServices::new());

        let sources = try_join!(
            try_join_all(from_atom_raw(cfg_raw.atoms_feeds, app_state)),
            try_join_all(from_telegram_channels_raw(cfg_raw.telegram_channels)),
        );

        let (atoms_channels, telegram_channels) =
            sources.map_err(|e| format!("Failed to init sources: {e}"))?;

        Ok(Self::new(atoms_channels, telegram_channels))
    }

    /// Creates a new `AppState` from a configuration file.
    ///
    /// # Arguments
    /// * `config_path` - Path to the TOML configuration file
    ///
    /// # Errors
    /// * When reading the configuration file fails
    /// * When parsing TOML content fails
    /// * When initializing sources from the configuration fails
    pub async fn try_from_file(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = read_to_string(config_path)
            .await
            .map_err(|e| format!("Failed to read file '{}': {}", config_path.display(), e))?;

        let cfg_raw: RawConfig =
            toml::from_str(&file_content).map_err(|e| format!("Failed to parse TOML: {e}"))?;

        let app_state = Self::try_from_raw(cfg_raw).await?;

        Ok(app_state)
    }

    pub fn sources(&self) -> impl Iterator<Item = &dyn Source> {
        let atom_iter = self
            .atoms_channels
            .iter()
            .map(|atom_source| atom_source as &dyn Source);

        let rss_iter = self
            .telegram_channels
            .iter()
            .map(|rss_source| rss_source as &dyn Source);

        atom_iter.chain(rss_iter)
    }
}
