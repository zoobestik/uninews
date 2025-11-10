use anyhow::Result;
use clap::Args;
use news_core::models::source::SourceEnum::{Atom, Telegram};
use news_core::repos::source::SourceRepository;
use std::error::Error;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Args)]
pub struct ArgsList {}

pub async fn list_sources(sources: Arc<impl SourceRepository>, _args: ArgsList) -> Result<()> {
    for source in sources.get_all().await? {
        match source {
            Atom(src) => info!("Atom/RSS\t= {0}", src.url),
            Telegram(src) => info!("Telegram\t= {0}", src.public_url),
        }
    }
    Ok(())
}
