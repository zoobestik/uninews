use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::SourceTypeValue;
use uninews_core::models::telegram::TelegramChannelDraft;
use uninews_core::parse::parse_telegram_username;
use uninews_core::services::source::SourceService;

#[derive(Debug, Args)]
pub struct RemoveTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn remove_telegram_channel_source(
    sources: Arc<impl SourceService>,
    args: RemoveTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    let source_id = TelegramChannelDraft::new(args.username).source_id;
    sources
        .delete_with_type(source_id, SourceTypeValue::Telegram)
        .await?;
    Ok(())
}
