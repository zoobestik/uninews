use anyhow::{Context, Result};
use clap::Args;
use news_core::models::source::atom::AtomDraft;
use news_core::repos::SourceDraft::Atom;
use news_core::repos::source::SourceRepository;
use news_sqlite_core::utils::parse::parse_url;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Args)]
pub struct AddAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn add_atom_source(sources: Arc<impl SourceRepository>, args: AddAtom) -> Result<()> {
    let draft = AtomDraft::new(args.url);
    let url = draft.url.to_string();

    sources
        .add(Atom(draft))
        .await
        .context(format!("Failed to add Atom feed: {}", url))?;

    println!("✓ Atom source added successfully: {}", url);
    Ok(())
}
