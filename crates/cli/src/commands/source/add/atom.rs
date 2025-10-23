use super::SourceService;
use clap::Args;
use std::error::Error;
use uninews_core::models::atom::AtomDraft;
use uninews_core::repo::source::SourceCreate;
use uninews_core::url::parse_url;
use url::Url;

#[derive(Debug, Args)]
pub struct AddAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn add_atom_source(repo: SourceService, args: AddAtom) -> Result<(), Box<dyn Error>> {
    repo.insert(SourceCreate::Atom(AtomDraft::new(args.url)))
        .await?;
    Ok(())
}
