use anyhow::{Context, Result};
use clap::Parser;
use futures::future::try_join_all;
use news_watch::cli::{list_sources, source_watch};

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
    try_join_all(
        list_sources()
            .await
            .context("Failed to load content sources list")?
            .map(source_watch),
    )
    .await
    .context("Failed to watch content from sources")?;

    Ok(())
}
