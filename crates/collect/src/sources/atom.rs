use crate::sources::Source;
use crate::utils::markdown::html_to_markdown;

use async_trait::async_trait;
use futures::future::try_join_all;
use rss::{Channel, Item};
use serde::Deserialize;
use std::io::Cursor;
use std::sync::Arc;
use tracing::{debug, error};
use uninews_core::{HttpService, News, NewsService};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub enum RefreshPeriod {
    Seconds(usize),
}

impl std::fmt::Display for RefreshPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds(seconds) => write!(f, "{seconds}s"),
        }
    }
}

const APP_UUID_NAMESPACE: Uuid = Uuid::from_u128(0x6ba7_b810_9dad_11d1_80b4_00c0_4fd4_30c8);

pub struct Atom {
    http_service: Arc<dyn HttpService>,
    news_service: Arc<dyn NewsService>,

    source_url: Url,
    #[allow(dead_code)] // @ToDo: implement
    refresh_period: RefreshPeriod,
}

impl Atom {
    fn validate_source_url(source_url: &str) -> Result<Url, String> {
        let url = Url::parse(source_url).map_err(|e| e.to_string())?;
        Ok(url)
    }

    /// Creates a new Atom sources instance from a URL.
    ///
    /// # Arguments
    /// * `source_url` - The URL of the Atom feed
    ///
    /// # Errors
    /// Never returns an error but uses Result for consistency with other sources types
    pub fn try_new(
        source_url: &str,
        refresh_period: RefreshPeriod,
        http_service: Arc<dyn HttpService>,
        news_service: Arc<dyn NewsService>,
    ) -> Result<Self, String> {
        let source_url = Self::validate_source_url(source_url)
            .map_err(|e| format!("[source_url=\"{source_url}\"] {e} "))?;

        Ok(Self {
            http_service,
            news_service,
            source_url,
            refresh_period,
        })
    }

    fn generate_uuid_id(&self, item: &Item) -> Result<Uuid, String> {
        let link = item
            .link
            .clone()
            .ok_or_else(|| "Missing link for one element".to_string())?;

        let guid = item
            .guid
            .clone()
            .ok_or_else(|| format!("Missing guid for {link}"))?
            .value;

        Ok(Uuid::new_v5(
            &APP_UUID_NAMESPACE,
            format!("{0}-{link}-{guid}", self.source_url).as_bytes(),
        ))
    }

    async fn extract_news_channel(&self, channel: Channel) -> Result<Vec<News>, String> {
        debug!(
            "[atom_feed=\"{0}\"] Processing {1} items",
            self.source_url,
            channel.items.len()
        );

        let news_futures = channel
            .items
            .into_iter()
            .map(async |item| -> Result<News, String> {
                let id = self.generate_uuid_id(&item)?;

                let content = item
                    .description
                    .ok_or_else(|| format!("Parsing error for {id}"))?;

                Ok(News {
                    id,
                    title: item.title.unwrap_or_default(),
                    content: html_to_markdown(content)
                        .await
                        .map_err(|e| format!("Parsing error for {id}: {e}"))?,
                    url: item.link.unwrap_or_default(),
                    image: None,
                    published_at: item.pub_date,
                })
            })
            .collect::<Vec<_>>();

        let list = try_join_all(news_futures).await?;
        debug!(
            "[atom_feed=\"{0}\"] Processed {1} news items",
            self.source_url,
            list.len()
        );

        Ok(list)
    }

    async fn read_news_periodically(&self) -> Result<Vec<News>, String> {
        debug!("[atom_feed=\"{0}\"] Fetching feed", self.source_url);

        let resp = self
            .http_service
            .request_by_schedule(self.source_url.clone())
            .await
            .map_err(|e| format!("Failed to fetch feed: {e}"))?;

        let content = resp
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {e}"))?;

        let news = self
            .extract_news_channel(
                Channel::read_from(Cursor::new(content))
                    .map_err(|e| format!("Failed to parse feed content: {e}"))?,
            )
            .await?;

        Ok(news)
    }
}

#[async_trait]
impl Source for Atom {
    async fn watch_updates(&self) {
        loop {
            match self.read_news_periodically().await {
                Ok(news) => {
                    debug!(
                        "[atom_feed=\"{0}\"] Updating {1} news items",
                        self.source_url,
                        news.len()
                    );
                    self.news_service.update_news(news).await;
                }
                Err(e) => error!("[atom_feed=\"{0}\"] {e}", self.source_url),
            }
        }
    }
}
