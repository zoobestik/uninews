use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::telegram::TelegramChannelDraft;
use uninews_core::parse::parse_telegram_username;
use uninews_core::services::source::{SourceCreate, SourceService};

#[derive(Debug, Args)]
pub struct AddTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn add_telegram_channel_source(
    sources: Arc<impl SourceService>,
    args: AddTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    sources
        .add(SourceCreate::TelegramChannel(TelegramChannelDraft::new(
            args.username,
        )))
        .await?;
    Ok(())
}
