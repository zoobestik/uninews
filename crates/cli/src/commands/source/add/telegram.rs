use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use SourceDraft::Telegram;
use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::telegram::TelegramDraft;
use news_core::services::source::SourceDraft;
use news_core::services::source::SourceService;
use news_sqlite_core::utils::parse::parse_telegram_username;
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct AddTelegram {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn add_telegram_source(
    sources: Arc<impl SourceService + 'static>,
    args: AddTelegram,
) -> Result<()> {
    Report::silent(move |task| {
        Box::pin(async move {
            let draft = TelegramDraft::new(args.username);
            let username = &draft.username.to_string();

            sources
                .add(Telegram(draft))
                .await
                .context(format!("Failed to add Telegram channel: {username}"))?;

            task.finish_with_text(format!("Telegram channel added successfully: {username}"));
            Ok(())
        })
    })
    .await
}
