use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::SourceTypeValue;
use uninews_core::models::telegram::TelegramChannelDraft;
use uninews_core::parse::parse_telegram_username;
use uninews_core::repo::source::SourceRepository;

#[derive(Debug, Args)]
pub struct RemoveTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn rm_telegram_channel_source(
    repo: Arc<impl SourceRepository>,
    args: RemoveTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    let id = TelegramChannelDraft::new(args.username).source_id;
    repo.delete_with_type(id, SourceTypeValue::Telegram).await?;
    Ok(())
}
