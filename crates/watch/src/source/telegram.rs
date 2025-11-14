use news_core::services::HttpService;
mod item;
mod parse;
mod update_handle;

use self::update_handle::TelegramWebUpdateHandle;
use crate::state::LiveAppState;
use news_core::models::source::telegram::TelegramSource;
use news_core::services::WatchError;
use std::sync::Arc;
use tracing::info;

pub async fn watch_telegram_channel(
    app_state: Arc<LiveAppState>,
    source: TelegramSource,
) -> Result<(), WatchError> {
    let schedule = app_state.http().await;
    let update_handler = Arc::new(TelegramWebUpdateHandle {
        app_state: app_state.clone(),
        url: source.public_url.clone(),
        source,
    });

    info!(
        "[telegram_channel=\"{0}\"] watch news",
        update_handler.url.to_string()
    );
    schedule.watch_changes(update_handler).await?;

    Ok(())
}
