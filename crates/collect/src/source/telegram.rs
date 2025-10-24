use crate::state::AppState;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::telegram::TelegramChannelSource;
use uninews_core::parse::parse_telegram_username;
use url::Url;

pub fn get_telegram_channel_url(name: &str) -> Result<Url, String> {
    let name = parse_telegram_username(name)?;

    let channel_url = Url::parse(&format!("https://t.me/s/{name}"))
        .map_err(|e| format!("[telegram_channel=\"{name}\"] invalid channel name: {e}"))?;

    Ok(channel_url)
}

pub async fn watch_telegram_channel(
    _app_state: Arc<AppState>,
    source: &TelegramChannelSource,
) -> Result<(), String> {
    let url = get_telegram_channel_url(&source.username)?;
    info!("Watch [telegram_channel=\"{url}\"] news");
    Ok(())
}
