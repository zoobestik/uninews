use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::source::telegram::TelegramChannelDraft;
use uninews_core::repos::SourceCreate;
use uninews_core::repos::source::SourceRepository;
use uninews_services::utils::parse::parse_telegram_username;

#[derive(Debug, Args)]
pub struct AddTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn add_telegram_channel_source(
    sources: Arc<impl SourceRepository>,
    args: AddTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    sources
        .add(SourceCreate::TelegramChannel(TelegramChannelDraft::new(
            args.username,
        )))
        .await?;
    Ok(())
}
