use anyhow::Result;
use clap::Args;
use news_core::models::source::SourceEnum::{Atom, Telegram};
use news_core::services::source::SourceService;
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct ArgsList {}

pub async fn list_sources(
    sources: Arc<impl SourceService + 'static>,
    _args: ArgsList,
) -> Result<()> {
    for source in sources.get_all().await? {
        let (name, url) = match source {
            Atom(src) => ("Atom/RSS", src.url),
            Telegram(src) => ("Telegram", src.public_url),
        };
        println!("{:>12} = {}", name, url);
    }
    Ok(())
}
