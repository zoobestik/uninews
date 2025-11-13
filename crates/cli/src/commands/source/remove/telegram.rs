use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::SourceType::Telegram;
use news_core::models::source::telegram::TelegramDraft;
use news_core::repos::source::SourceRepository;
use news_sqlite_core::utils::parse::parse_telegram_username;
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct RemoveTelegram {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

pub async fn remove_telegram_source(
    sources: Arc<impl SourceRepository + 'static>,
    args: RemoveTelegram,
) -> Result<()> {
    Report::silent(move |task| {
        Box::pin(async move {
            let draft = TelegramDraft::new(args.username);

            sources
                .drop_by_id_and_type(draft.source_id, Telegram)
                .await
                .context(format!(
                    "Failed to remove Telegram channel: {}",
                    draft.username
                ))?;

            task.finish_with_text(format!(
                "✓ Telegram channel removed successfully: {}",
                draft.username
            ));

            Ok(())
        })
    })
    .await
}
