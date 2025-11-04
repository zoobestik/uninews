use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_adapters::utils::parse::parse_url;
use uninews_core::models::source::atom::AtomDraft;
use uninews_core::repos::SourceCreate;
use uninews_core::repos::source::SourceRepository;
use url::Url;

#[derive(Debug, Args)]
pub struct AddAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn add_atom_source(
    sources: Arc<impl SourceRepository>,
    args: AddAtom,
) -> Result<(), Box<dyn Error>> {
    sources
        .add(SourceCreate::Atom(AtomDraft::new(args.url)))
        .await?;
    Ok(())
}
