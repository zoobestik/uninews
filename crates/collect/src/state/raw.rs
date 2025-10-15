use crate::services::AppServices;
use crate::sources::atom::{Atom, RefreshPeriod};
use crate::sources::telegram::TelegramChannel;

use serde::Deserialize;
use std::sync::Arc;
use tokio::try_join;
use tracing::debug;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    #[serde(default)]
    #[serde(rename = "atom")]
    pub atoms_feeds: Vec<RawAtom>,

    #[serde(default)]
    #[serde(rename = "telegram")]
    pub telegram_channels: Vec<RawTelegramChannel>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawAtom {
    pub source_url: String,

    #[serde(default = "default_refresh_period")]
    pub refresh_period: RefreshPeriod,
}

const fn default_refresh_period() -> RefreshPeriod {
    RefreshPeriod::Seconds(60)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTelegramChannel {
    pub nickname: String,
}

pub fn from_atom_raw(
    list: Vec<RawAtom>,
    app_state: Arc<AppServices>,
) -> Vec<impl Future<Output = Result<Atom, String>>> {
    list.into_iter()
        .map(move |x| {
            let app_state = app_state.clone();
            async move {
                debug!("Initializing [atom_feed]: {0}", x.source_url);

                let (http_service, news_service) =
                    try_join!(app_state.http_service(), app_state.news_service())
                        .map_err(|e| format!("Failed to get services: {e}"))?;

                Atom::try_new(
                    &x.source_url,
                    x.refresh_period,
                    http_service.clone(),
                    news_service.clone(),
                )
            }
        })
        .collect()
}

pub fn from_telegram_channels_raw(
    list: Vec<RawTelegramChannel>,
) -> Vec<impl Future<Output = Result<TelegramChannel, String>>> {
    list.into_iter()
        .map(|x| async move {
            debug!("Initializing [telegram_channel]: {0}", x.nickname);
            TelegramChannel::try_new(&x.nickname)
        })
        .collect()
}
