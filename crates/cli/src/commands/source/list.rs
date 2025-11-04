use clap::Args;
use std::error::Error;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::source::SourceType;
use uninews_core::repos::source::SourceRepository;

#[derive(Debug, Args)]
pub struct ArgsList {}

pub async fn list_sources(
    sources: Arc<impl SourceRepository>,
    _args: ArgsList,
) -> Result<(), Box<dyn Error>> {
    for source in sources.get_all().await? {
        match source {
            SourceType::Atom(src) => info!("Atom/RSS\t= {0}", src.url),
            SourceType::TelegramChannel(src) => info!("Telegram\t= {0}", src.url()?),
        }
    }
    Ok(())
}
