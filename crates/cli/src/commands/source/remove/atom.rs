use clap::Args;
use std::error::Error;
use std::sync::Arc;
use uninews_core::models::SourceTypeValue;
use uninews_core::models::atom::AtomDraft;
use uninews_core::parse::parse_url;
use uninews_core::services::source::SourceService;
use url::Url;

#[derive(Debug, Args)]
pub struct RemoveAtom {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn remove_atom_source(
    sources: Arc<impl SourceService>,
    args: RemoveAtom,
) -> Result<(), Box<dyn Error>> {
    let id = AtomDraft::new(args.url).source_id;
    sources.delete_with_type(id, SourceTypeValue::Atom).await?;
    Ok(())
}
