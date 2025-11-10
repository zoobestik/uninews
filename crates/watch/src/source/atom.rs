mod feed;
mod feed_item;
mod update_handle;

use crate::state::AppState;
use news_core::models::source::atom::AtomSource;
use news_core::services::WatchError;
use std::sync::Arc;
use tracing::info;
use update_handle::AtomUpdateHandle;

pub async fn watch_atom_feed(
    app_state: Arc<AppState>,
    source: AtomSource,
) -> Result<(), WatchError> {
    let http = app_state.http().await;

    let update_handler = Arc::new(AtomUpdateHandle {
        app_state: app_state.clone(),
        source,
    });

    info!("[atom_feed=\"{0}\"] watch news", update_handler.source.url);
    http.watch_changes(update_handler).await?;

    Ok(())
}
