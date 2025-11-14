use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::telegram::TelegramDraft;
use news_core::services::source::SourceDraft;
use news_core::services::source::SourceService;
use news_sqlite_core::utils::parse::parse_telegram_username;
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct RemoveTelegram {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn remove_telegram_source(
    sources: Arc<impl SourceService + 'static>,
    args: RemoveTelegram,
) -> Result<()> {
    Report::silent(move |task| {
        Box::pin(async move {
            let username = args.username;
            let draft = TelegramDraft::new(username.clone());

            sources
                .drop_by_draft(SourceDraft::Telegram(draft))
                .await
                .context(format!("Failed to remove Telegram channel: {username}"))?;

            task.finish_with_text(format!("Telegram channel removed successfully: {username}"));

            Ok(())
        })
    })
    .await
}
