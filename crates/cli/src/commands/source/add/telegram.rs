use SourceDraft::Telegram;
use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::telegram::TelegramDraft;
use news_core::repos::SourceDraft;
use news_core::repos::source::SourceRepository;
use news_sqlite_core::utils::parse::parse_telegram_username;
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct AddTelegram {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn add_telegram_source(
    sources: Arc<impl SourceRepository>,
    args: AddTelegram,
) -> Result<()> {
    let draft = TelegramDraft::new(args.username);

    sources.add(Telegram(draft)).await.context(format!(
        "Failed to add Telegram channel: {}",
        draft.username
    ))?;

    println!("✓ Telegram channel added successfully: {}", draft.username);
    Ok(())
}
