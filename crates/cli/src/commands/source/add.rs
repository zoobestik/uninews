use clap::Args;
use std::error::Error;
use uninews_core::models::SourceTypeValue;
use uninews_core::models::atom::AtomDraft;
use uninews_core::repo::source::sqlite::SqliteSourceRepository;
use uninews_core::repo::source::{SourceCreate, SourceRepository};
use uninews_core::url::parse_url;
use url::Url;

#[derive(Debug, Args)]
pub struct AddArgs {
    #[arg(value_parser = parse_url)]
    url: Url,
    source_type: Option<SourceTypeValue>,
}

pub async fn add_source(
    source_repo: SqliteSourceRepository,
    args: AddArgs,
) -> Result<(), Box<dyn Error>> {
    source_repo
        .insert(SourceCreate::Atom(AtomDraft::new(args.url)))
        .await?;

    Ok(())
}
