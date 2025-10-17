use crate::state::AppState;
use futures::future::join_all;
use uninews_core::source::Source;

pub async fn run_collectors(app_state: AppState) {
    join_all(app_state.sources().map(Source::watch_updates)).await;
}
