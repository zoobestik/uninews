use clap::Args;
use std::error::Error;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::SourceTypeValue;
use uninews_core::repo::source::SourceRepository;
use uninews_core::url::parse_url;
use url::Url;

#[derive(Debug, Args)]
pub struct RmArgs {
    #[arg(value_parser = parse_url)]
    url: Url,
    source_type: Option<SourceTypeValue>,
}

pub async fn remove_source(
    repo: Arc<impl SourceRepository>,
    args: RmArgs,
) -> Result<(), Box<dyn Error>> {
    match args.source_type {
        Some(source_type) => remove_by_args(repo, source_type, args.url).await?,
        None => remove_by_url(repo, args.url).await?,
    }
    Ok(())
}

async fn remove_by_url(repo: Arc<impl SourceRepository>, url: Url) -> Result<(), Box<dyn Error>> {
    let mut ids = repo.find_by_url(url).await?.into_iter();

    match (ids.next(), ids.next()) {
        (Some(id), None) => {
            repo.delete_by_id(id).await?;
            Ok(())
        }
        (None, None) => {
            info!("No source found with the specified URL");
            Ok(())
        }
        _ => Err("More than one source found".into()),
    }
}

async fn remove_by_args(
    repo: Arc<impl SourceRepository>,
    source_type: SourceTypeValue,
    url: Url,
) -> Result<(), Box<dyn Error>> {
    repo.delete_by_type(url, source_type).await?;
    Ok(())
}
