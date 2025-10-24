use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::telegram::TelegramChannelDraft;
use uninews_core::parse::parse_telegram_username;
use uninews_core::repo::source::{SourceCreate, SourceRepository};

#[derive(Debug, Args)]
pub struct AddTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn add_telegram_channel_source(
    repo: Arc<impl SourceRepository>,
    args: AddTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    repo.insert(SourceCreate::TelegramChannel(TelegramChannelDraft::new(
        args.username,
    )))
    .await?;
    Ok(())
}
