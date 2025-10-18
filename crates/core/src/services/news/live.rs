use super::News;
use super::service::NewsService;
use crate::fs::write_to_file;
use async_trait::async_trait;
use futures::future::try_join_all;
use tokio::try_join;
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
                let short_path = format!(
                    "out/news/{1}/{0}.short.md",
                    news.source_id(),
                    news.parent_id()
                );
                let short_text = format!("# {1}\n\n{0}", news.description(), news.title());

                let long_path = format!(
                    "out/news/{1}/{0}.long.md",
                    news.source_id(),
                    news.parent_id()
                );
                let long_text = format!(
                    "# {1}\n\n{0}",
                    news.content().as_ref().unwrap_or(&String::new()),
                    news.title()
                );

                try_join!(
                    write_to_file(short_path.as_str(), short_text.as_str()),
                    write_to_file(long_path.as_str(), long_text.as_str()),
                )
                .map_err(|e| format!("Failed to update news: {e}"))?;

                Ok(())
            })
            .collect();

        if let Err(e) = try_join_all(fs_futures).await {
            error!("Failed to update news: {e}");
        }
    }
}
