use anyhow::{Context, Result};
use clap::Parser;
use futures::future::try_join_all;
use news_core::services::source::GetAllError;
use news_core::services::source::SourceService;
use news_watch::cli::{app_state, source_watch};

#[derive(Parser, Debug)]
#[command(
    about = "Collect and aggregate content from configured information sources",
    visible_aliases = ["col"],
)]
pub struct CollectCommand {
    #[clap(
        short,
        long,
        help = "Watch configured sources for content updates continuously"
    )]
    watch: bool, // @todo: implement continuous watching mode that runs source update checks periodically instead of one-time collection
}

pub async fn run_collect(_cmd: CollectCommand) -> Result<()> {
    let app_state = app_state().await;

    let sources = app_state
        .sources()
        .await
        .map_err(|e| GetAllError(Box::new(e)))?;

    let sources_list = sources
        .get_all()
        .await
        .context("Failed to load content sources list")?;

    let tasks = sources_list
        .into_iter()
        .map(source_watch)
        .collect::<Vec<_>>();

    try_join_all(tasks)
        .await
        .context("Failed to watch content from sources")?;

    Ok(())
}
