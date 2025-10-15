use crate::sources::Source;
use crate::state::AppState;

use futures::future::join_all;

pub async fn run_collectors(app_state: AppState) {
    join_all(app_state.sources().map(Source::watch_updates)).await;
}
