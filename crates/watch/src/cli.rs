use crate::source::atom::watch_atom_feed;
use crate::source::telegram::watch_telegram_channel;
use crate::state::AppState;
use news_core::models::source::SourceEnum;
use news_core::repos::source::GetAllError;
use std::sync::Arc;
use tokio::sync::OnceCell;

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub async fn app_state() -> Arc<AppState> {
    APP_STATE
        .get_or_init(async || Arc::new(AppState::new()))
        .await
        .clone()
}

pub async fn list_sources() -> Result<Vec<SourceEnum>, GetAllError> {
    let app_state = app_state().await;
    let sources = app_state
        .sources()
        .await
        .map_err(|e| GetAllError(Box::new(e)))?;

    Ok(sources.get_all().await?)
}

pub async fn source_watch(source_type: SourceEnum) -> Result<(), X> {
    let app_state = app_state().await;

    match source_type {
        SourceEnum::Atom(src) => watch_atom_feed(app_state, src).await?,
        SourceEnum::Telegram(src) => watch_telegram_channel(app_state, src).await?,
    }

    Ok(())
}
