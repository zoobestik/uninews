use crate::config::Config;
use crate::source::atom::{Atom, RefreshPeriod};
use crate::source::telegram::TelegramChannel;

use serde::Deserialize;
use std::path::Path;
use tokio::fs::read_to_string;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AtomRaw {
    pub source_url: String,

    #[serde(default = "default_refresh_period")]
    pub refresh_period: RefreshPeriod,
}

const fn default_refresh_period() -> RefreshPeriod {
    RefreshPeriod::Seconds(60)
}

impl TryFrom<AtomRaw> for Atom {
    type Error = String;

    fn try_from(raw: AtomRaw) -> Result<Self, Self::Error> {
        Self::try_new(&raw.source_url, raw.refresh_period)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelegramChannelRaw {
    pub nickname: String,
}

impl TryFrom<TelegramChannelRaw> for TelegramChannel {
    type Error = String;

    fn try_from(raw: TelegramChannelRaw) -> Result<Self, Self::Error> {
        Self::try_new(&raw.nickname)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    #[serde(default)]
    #[serde(rename = "atom")]
    atoms_channels: Vec<AtomRaw>,

    #[serde(default)]
    #[serde(rename = "telegram")]
    telegram_channels: Vec<TelegramChannelRaw>,
}

impl TryFrom<RawConfig> for Config {
    type Error = String;

    fn try_from(raw: RawConfig) -> Result<Self, Self::Error> {
        let telegram_channels = raw
            .telegram_channels
            .into_iter()
            .map(TelegramChannel::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let atoms_channels = raw
            .atoms_channels
            .into_iter()
            .map(Atom::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(atoms_channels, telegram_channels))
    }
}

/// Initialize configuration by reading and parsing a TOML file.
///
/// This function reads the content of a configuration file at the given path and parses it into a [`Config`] struct.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read from the given path
/// - The file contents cannot be parsed as valid TOML matching the [`Config`] structure
pub async fn init_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let file_content = read_to_string(path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

    let raw_cfg: RawConfig =
        toml::from_str(&file_content).map_err(|e| format!("Failed to parse TOML: {e}"))?;

    let cfg = Config::try_from(raw_cfg).map_err(|e| format!("Failed to validate config: {e}"))?;

    Ok(cfg)
}
