use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::source::SourceTypeValue;
use uninews_core::models::source::telegram::TelegramChannelDraft;
use uninews_core::repos::source::SourceRepository;
use uninews_services::utils::parse::parse_telegram_username;

#[derive(Debug, Args)]
pub struct RemoveTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn remove_telegram_channel_source(
    sources: Arc<impl SourceRepository>,
    args: RemoveTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    let source_id = TelegramChannelDraft::new(args.username).source_id;
    sources
        .delete_with_type(source_id, SourceTypeValue::Telegram)
        .await?;
    Ok(())
}
