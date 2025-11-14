use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::atom::AtomDraft;
use news_core::services::source::SourceDraft;
use news_core::services::source::SourceService;
use news_sqlite_core::utils::parse::parse_url;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Args)]
pub struct RemoveAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn remove_atom_source(
    sources: Arc<impl SourceService + 'static>,
    args: RemoveAtom,
) -> Result<()> {
    Report::silent(move |task| {
        Box::pin(async move {
            let draft = AtomDraft::new(args.url);
            let url = draft.url.to_string();

            sources
                .drop_by_draft(SourceDraft::Atom(draft))
                .await
                .context(format!("Failed to remove Atom feed: {url}"))?;

            task.finish_with_text(format!("Atom source removed successfully: {url}"));
            Ok(())
        })
    })
    .await
}
