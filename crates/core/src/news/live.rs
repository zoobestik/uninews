use super::News;
use super::service::NewsService;
use async_trait::async_trait;
use futures::future::try_join_all;
use std::path::Path;
use tokio::fs;
use tracing::error;

pub struct LiveNewsService {}

impl LiveNewsService {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for LiveNewsService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NewsService for LiveNewsService {
    async fn update_news(&self, item: Vec<&dyn News>) {
        let fs_futures: Vec<_> = item
            .into_iter()
            .map(async move |news| -> Result<(), String> {
                let path_string =
                    format!("out/news/{1}/{0}.html", news.source_id(), news.parent_id());
                let path = Path::new(path_string.as_str());

                fs::create_dir_all(
                    path.parent()
                        .ok_or_else(|| format!("Failed to get parent {0}", path.display()))?,
                )
                .await
                .map_err(|e| format!("Failed to create directory {0}: {e}", path.display()))?;

                fs::write(path, news.description())
                    .await
                    .map_err(|e| format!("Failed to write file [{0}]: {e}", path.display()))?;

                Ok(())
            })
            .collect();

        if let Err(e) = try_join_all(fs_futures).await {
            error!("Failed to update news: {e}");
        }
    }
}
