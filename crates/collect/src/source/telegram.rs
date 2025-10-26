mod item;
mod update_handler;

use crate::source::telegram::update_handler::TelegramWebUpdateHandler;
use crate::state::AppState;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::telegram::TelegramChannelSource;

pub async fn watch_telegram_channel(
    app_state: Arc<AppState>,
    source: TelegramChannelSource,
) -> Result<(), String> {
    let schedule = app_state.http().await?;
    let update_handler = Arc::new(TelegramWebUpdateHandler {
        app_state: app_state.clone(),
        url: source.url()?,
        source,
    });

    info!(
        "Watch [telegram_channel=\"{0}\"] news",
        update_handler.url.to_string()
    );
    schedule.watch_changes(update_handler).await?;

    Ok(())
}
