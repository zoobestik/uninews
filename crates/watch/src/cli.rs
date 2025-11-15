use crate::source::atom::watch_atom_feed;
use crate::source::telegram::watch_telegram_channel;
use crate::state::LiveAppState;
use news_core::models::source::SourceEnum;
use news_core::services::WatchError;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;

static APP_STATE: OnceCell<Arc<LiveAppState>> = OnceCell::const_new();

pub async fn app_state() -> Arc<LiveAppState> {
    APP_STATE
        .get_or_init(async || Arc::new(LiveAppState::new()))
        .await
        .clone()
}

#[derive(Error, Debug)]
pub enum SourceWatchError {
    #[error(transparent)]
    Internal(#[from] WatchError),
}

/// Watches news updates from the provided source (Atom feed or Telegram channel).
///
/// # Arguments
/// * `source_type` - The source to watch, either an Atom feed or a Telegram channel.
///
/// # Errors
/// Returns [`SourceWatchError`] if watching the source fails, which wraps an underlying
/// [`WatchError`] that could occur when:
/// * There is a connection error while watching the source
/// * There is an error parsing updates from the source
/// * The source service is not accessible
pub async fn source_watch(source_type: SourceEnum) -> Result<(), SourceWatchError> {
    let app_state = app_state().await;

    match source_type {
        SourceEnum::Atom(src) => watch_atom_feed(app_state, src).await?,
        SourceEnum::Telegram(src) => watch_telegram_channel(app_state, src).await?,
    }

    Ok(())
}
