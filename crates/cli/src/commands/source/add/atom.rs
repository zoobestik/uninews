use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::atom::AtomDraft;
use uninews_core::parse::parse_url;
use uninews_core::repo::source::{SourceCreate, SourceRepository};
use url::Url;

#[derive(Debug, Args)]
pub struct AddAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn add_atom_source(
    repo: Arc<impl SourceRepository>,
    args: AddAtom,
) -> Result<(), Box<dyn Error>> {
    repo.insert(SourceCreate::Atom(AtomDraft::new(args.url)))
        .await?;
    Ok(())
}
