use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::SourceType::Atom;
use news_core::models::source::atom::AtomDraft;
use news_core::repos::source::SourceRepository;
use news_sqlite_core::utils::parse::parse_url;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Args)]
pub struct RemoveAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn remove_atom_source(
    sources: Arc<impl SourceRepository + 'static>,
    args: RemoveAtom,
) -> Result<()> {
    Report::silent(move |task| {
        Box::pin(async move {
            let draft = AtomDraft::new(args.url);
            let url = draft.url.to_string();

            sources
                .drop_by_id_and_type(draft.source_id, Atom)
                .await
                .context(format!("Failed to remove Atom feed: {url}"))?;

            task.finish_with_text(format!("Atom source removed successfully: {url}"));
            Ok(())
        })
    })
    .await
}
