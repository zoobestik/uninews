use crate::state::AppState;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::telegram::TelegramChannelSource;

pub async fn watch_telegram_channel(
    _app_state: Arc<AppState>,
    source: &TelegramChannelSource,
) -> Result<(), String> {
    info!("Watch [telegram_channel=\"{0}\"] news", source.url()?);
    Ok(())
}
