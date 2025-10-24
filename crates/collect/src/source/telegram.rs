//     /// Creates a new Telegram channel instance from a nickname.
//     ///
//     /// # Arguments
//     /// * `name` - The nickname of the Telegram channel (5-32 characters, alphanumeric and underscores only)
//     ///
//     /// # Errors
//     /// Returns error if:
//     /// * Nickname validation fails - too short/long or contains invalid characters
//     /// * Cannot construct valid URL from the nickname
//     pub fn try_new(name: &str) -> Result<Self, String> {
//         Self::validate_nickname(name)?;
//
//         let channel_url = Url::parse(&format!("https://t.me/{name}")).map_err(|e| {
//             debug!(error = %e);
//             "Invalid channel name."
//         })?;
//
//         Ok(Self {
//             channel_url,
//             group_uuid: Uuid::from_u128(0x0000_0000_0000_0000_0000_0000_0000_0000_0001),
//         })
//     }
// }

use crate::state::AppState;
use std::sync::Arc;
use tracing::{debug, info};
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
    info!("Watch [telegram_channel=\"{0}\"] news", source.id);
    let url = get_telegram_channel_url(&source.username)?;
    debug!("fetch {url}");
    Ok(())
}
