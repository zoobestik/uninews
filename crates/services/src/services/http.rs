use async_trait::async_trait;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error};
use uninews_core::services::{HttpService, HttpUpdateHandler};

pub struct LiveHttpService {
    client: Client,
}

impl LiveHttpService {
    async fn get_and_update(
        &self,
        update_handler: &Arc<dyn HttpUpdateHandler>,
    ) -> Result<(), String> {
        let url_str = update_handler.url().to_string();

        debug!("[http_service=\"{0}\"] fetching url", url_str);

        let response = self
            .client
            .get(url_str.as_str())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        debug!("[http_service=\"{0}\"] fetched url", url_str);

        if !response.status().is_success() {
            return Err(format!("Failed to fetch url: {url_str}"));
        }

        update_handler.handle(response).await
    }
}

impl LiveHttpService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for LiveHttpService {
    fn default() -> Self {
        Self::new()
    }
}

const POLL_INTERVAL_SECS: usize = 60;
const MAX_BACKOFF_SECS: usize = 10 * POLL_INTERVAL_SECS;

#[async_trait]
impl HttpService for LiveHttpService {
    async fn watch_changes(
        &self,
        update_handler: Arc<dyn HttpUpdateHandler>,
    ) -> Result<(), String> {
        let mut backoff_secs = POLL_INTERVAL_SECS;
        loop {
            backoff_secs = match self.get_and_update(&update_handler).await {
                Ok(()) => POLL_INTERVAL_SECS,
                Err(e) => {
                    error!("[http_service=\"{0}\"] {e}", update_handler.url());
                    (backoff_secs * 2).min(MAX_BACKOFF_SECS)
                }
            };

            sleep(Duration::from_secs(backoff_secs as u64)).await;
        }
    }
}
