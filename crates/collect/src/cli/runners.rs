use crate::sources::Source;
use crate::state::AppState;

use futures::future::join_all;

pub async fn run_collectors(config: AppState) {
    join_all(config.sources().map(Source::watch_updates)).await;
}
