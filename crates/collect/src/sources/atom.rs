use super::Source;

use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;
use uninews_core::{HttpService, NewsService};
use url::Url;

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

pub struct Atom {
    http_service: Arc<dyn HttpService>,
    news_service: Arc<dyn NewsService>,

    source_url: Url,
    #[allow(dead_code)] // @ToDo: implement
    refresh_period: RefreshPeriod,
}

impl Atom {
    fn validate_source_url(source_url: &str) -> Result<Url, String> {
        let url = Url::parse(source_url).map_err(|e| format!("{e}"))?;
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
}

#[async_trait]
impl Source for Atom {
    async fn watch_updates(&self) {
        loop {
            let content = self
                .http_service
                .refresh_by_schedule(self.source_url.clone())
                .await;

            let content = match content {
                Ok(content) => content,
                Err(e) => {
                    error!("Fetch {e}");
                    return;
                }
            };

            self.news_service.update_news(content).await;
        }
    }
}
