use tracing::debug;
use uninews_core::cli::init_cli;

pub async fn run() {
    init_cli();

    debug!("[cli_app]: done");
}
