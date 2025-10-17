use super::item::try_atom_news_from_rss_item;
use crate::source::SourceType;
use async_trait::async_trait;
use futures::future::try_join_all;
use rss::Channel;
use std::io::Cursor;
use std::sync::Arc;
use tracing::{debug, error, info};
use uninews_core::http::service::HttpService;
use uninews_core::news::News;
use uninews_core::news::service::NewsService;
use uninews_core::source::Source;
use uninews_core::storage::service::StorageService;
use url::Url;
use uuid::Uuid;

pub struct Atom {
    http_service: Arc<dyn HttpService>,
    news_service: Arc<dyn NewsService>,
    storage_service: Arc<dyn StorageService>,

    source_url: Url,
}

impl Atom {
    /// Creates a new Atom source instance from a URL.
    ///
    /// # Arguments
    /// * `source_url` - The URL of the Atom feed
    ///
    /// # Errors
    /// Never returns an error but uses Result for consistency with other source types
    pub fn try_new(
        source_url: &str,
        http_service: &Arc<dyn HttpService>,
        news_service: &Arc<dyn NewsService>,
        storage_service: &Arc<dyn StorageService>,
    ) -> Result<Self, String> {
        let source_url = Self::validate_source_url(source_url)
            .map_err(|e| format!("[source_url=\"{source_url}\"] {e} "))?;

        Ok(Self {
            http_service: http_service.clone(),
            news_service: news_service.clone(),
            storage_service: storage_service.clone(),

            source_url,
        })
    }

    fn validate_source_url(source_url: &str) -> Result<Url, String> {
        let url = Url::parse(source_url).map_err(|e| e.to_string())?;
        Ok(url)
    }

    async fn extract_news_channel(&self, channel: Channel) -> Result<Vec<Arc<dyn News>>, String> {
        debug!(
            "[atom_feed=\"{0}\"] Processing {1} items",
            self.source_url,
            channel.items.len()
        );

        let news_futures: Vec<_> = channel
            .items
            .into_iter()
            // @todo: In the `read_news_periodically` function, external HTTP requests are made,
            //  and there's comprehensive error handling. However, for better clarity and debugging,
            //  consider adding specific logging for response codes and context for external URL
            //  used (`self.source_url`) when the error occurs.
            .map(async |item| -> Result<Arc<dyn News>, String> {
                let news_item = try_atom_news_from_rss_item(self.source_id(), item).await?;
                Ok(Arc::new(news_item) as Arc<dyn News>)
            })
            .collect();

        let list = try_join_all(news_futures).await?;
        debug!(
            "[atom_feed=\"{0}\"] Processed {1} news items",
            self.source_url,
            list.len()
        );

        Ok(list)
    }

    async fn read_news_periodically(&self) -> Result<Vec<Arc<dyn News>>, String> {
        let resp = self
            .http_service
            .request_by_schedule(self.source_url.clone())
            .await
            .map_err(|e| format!("Failed to fetch feed: {e}"))?;

        let content = resp
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {e}"))?;

        // @todo: In the `watch_updates` function, there seems to be a potential issue with an infinite loop (`loop`) without any delay or condition. This can cause the service to block resources at high priority and lead to performance degradation. If the loop fails, it could also leave the application in an unstable state.
        let news = self
            .extract_news_channel(
                Channel::read_from(Cursor::new(&content))
                    .map_err(|e| format!("Failed to parse feed content: {e}"))?,
            )
            .await?;

        self.storage_service
            .save_raw(self.source_id(), content.as_ref())
            .await;

        Ok(news)
    }
}

#[async_trait]
impl Source for Atom {
    fn source_id(&self) -> Uuid {
        // &format!("{0}-{link}-{guid}", self.source_url)
        Uuid::nil()
    }

    fn source_type(&self) -> String {
        SourceType::Atom.to_string()
    }

    async fn watch_updates(&self) {
        info!("Watch [atom_feed=\"{0}\"] news", self.source_url);

        loop {
            match self.read_news_periodically().await {
                Ok(news) => {
                    debug!(
                        "[atom_feed=\"{0}\"] Updating {1} news items",
                        self.source_url,
                        news.len()
                    );

                    let news_refs: Vec<&dyn News> = news.iter().map(AsRef::as_ref).collect();

                    self.news_service.update_news(news_refs).await;
                }
                Err(e) => error!("[atom_feed=\"{0}\"] {e}", self.source_url),
            }
        }
    }
}
