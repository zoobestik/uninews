use crate::services::AppServices;
use crate::source::telegram::TelegramChannel;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTelegramChannel {
    pub nickname: String,
}

pub async fn try_telegram_channel_from_raw(
    nickname: &str,
    _: Arc<AppServices>,
) -> Result<TelegramChannel, String> {
    TelegramChannel::try_new(nickname)
}
