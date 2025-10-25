mod item;
mod update_handler;

use crate::source::atom::update_handler::AtomUpdateHandler;
use crate::state::AppState;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::atom::AtomSource;

pub async fn watch_atom_feed(app_state: Arc<AppState>, source: AtomSource) -> Result<(), String> {
    let schedule = app_state.http().await?;
    let update_handler = Arc::new(AtomUpdateHandler {
        app_state: app_state.clone(),
        source,
    });

    info!("Watch [atom_feed=\"{0}\"] news", update_handler.source.url);
    schedule.watch_changes(update_handler).await?;

    Ok(())
}
