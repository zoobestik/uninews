use clap::Args;
use std::error::Error;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::SourceType;
use uninews_core::repo::source::SourceRepository;

#[derive(Debug, Args)]
pub struct ArgsList {}

pub async fn list_sources(
    repo: Arc<dyn SourceRepository>,
    _args: ArgsList,
) -> Result<(), Box<dyn Error>> {
    let sources = repo.find_all_sources().await?;

    for source in sources {
        match source {
            SourceType::Atom(src) => info!("Atom/RSS\t= {0}", src.url),
            SourceType::TelegramChannel(src) => info!("Telegram\t= {0}", src.url()?),
        }
    }

    Ok(())
}
