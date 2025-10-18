use crate::configure::try_state_from_file;
use crate::services::AppServices;
use crate::source::atom::Atom;
use crate::source::telegram::TelegramChannel;
use std::path::Path;
use std::sync::Arc;
use uninews_core::models::Source;

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

    pub async fn try_from_file(
        config_path: &Path,
        app_services: Arc<AppServices>,
    ) -> Result<Self, String> {
        let config = try_state_from_file(config_path, app_services).await?;
        Ok(config)
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
