use crate::source::atom::watch_atom_feed;
use crate::source::telegram::watch_telegram_channel;
use crate::state::AppState;
use clap::Parser;
use futures::future::try_join_all;
use std::sync::Arc;
use uninews_core::models::source::SourceType;

#[derive(Parser, Debug)]
#[command(
    about = "Collect and aggregate content from configured information sources",
    visible_aliases = ["cl"],
)]
pub struct CollectCommand {
    #[clap(
        short,
        long,
        help = "Watch configured sources for content updates continuously"
    )]
    watch: bool, // @todo: implement continuous watching mode that runs source update checks periodically instead of one-time collection
}

/// Collects and aggregates content from configured information sources.
///
/// # Arguments
/// * `_cmd` - Command with options for content collection
///
/// # Errors
/// Returns error string if:
/// - Failed to initialize application state
/// - Failed to fetch sources
/// - Failed to start content watchers
/// - Content watching tasks failed
pub async fn run_collect(_cmd: CollectCommand) -> Result<(), String> {
    let app_state = Arc::new(AppState::new());
    let sources = app_state.sources().await?.get_all().await?;

    let watchers = sources.into_iter().map(|source| {
        let app_state = app_state.clone();
        async move {
            match source {
                SourceType::Atom(src) => watch_atom_feed(app_state, src).await?,
                SourceType::TelegramChannel(src) => watch_telegram_channel(app_state, src).await?,
            }
            Ok::<(), String>(())
        }
    });

    try_join_all(watchers).await?;

    Ok(())
}
