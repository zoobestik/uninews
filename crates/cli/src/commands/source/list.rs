use clap::Args;
use std::error::Error;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::SourceType;
use uninews_core::services::source::SourceService;

#[derive(Debug, Args)]
pub struct ArgsList {}

pub async fn list_sources(
    sources: Arc<impl SourceService>,
    _args: ArgsList,
) -> Result<(), Box<dyn Error>> {
    let items = sources.get_all().await?;

    for source in items {
        match source {
            SourceType::Atom(src) => info!("Atom/RSS\t= {0}", src.url),
            SourceType::TelegramChannel(src) => info!("Telegram\t= {0}", src.url()?),
        }
    }

    Ok(())
}
