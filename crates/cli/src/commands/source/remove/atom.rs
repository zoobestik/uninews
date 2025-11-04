use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_adapters::utils::parse::parse_url;
use uninews_core::models::source::SourceTypeValue;
use uninews_core::models::source::atom::AtomDraft;
use uninews_core::repos::source::SourceRepository;
use url::Url;

#[derive(Debug, Args)]
pub struct RemoveAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn remove_atom_source(
    sources: Arc<impl SourceRepository>,
    args: RemoveAtom,
) -> Result<(), Box<dyn Error>> {
    let id = AtomDraft::new(args.url).source_id;
    sources.delete_with_type(id, SourceTypeValue::Atom).await?;
    Ok(())
}
