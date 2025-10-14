use crate::config::Config;
use crate::source::Source;

use futures::future::join_all;

pub async fn run_collectors(config: Config) {
    join_all(config.list().map(Source::watch_updates)).await;
}
