use async_trait::async_trait;
use news_core::errors::ExternalServiceError;
use news_core::services::{HttpService, HttpUpdateHandle, WatchError};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

pub struct LiveHttpService {
    client: Client,
}

impl LiveHttpService {
    async fn get_and_update(
        &self,
        update_handler: &Arc<dyn HttpUpdateHandle>,
    ) -> Result<(), ExternalServiceError> {
        let url_str = update_handler.url().as_str();

        let response = self
            .client
            .get(url_str)
            .send()
            .await
            .map_err(|e| ExternalServiceError {
                service: "http".to_string(),
                message: format!("Failed to send HTTP request: {e}"),
            })?;

        if !response.status().is_success() {
            return Err(ExternalServiceError {
                service: "http".to_string(),
                message: format!(
                    "HTTP request failed with status[{}] for {}",
                    response.status(),
                    url_str
                ),
            });
        }

        update_handler
            .handle(response)
            .await
            .map_err(|e| ExternalServiceError {
                service: "http".to_string(),
                message: format!("Failed to handle response: {e}"),
            })?;

        Ok(())
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
        update_handler: Arc<dyn HttpUpdateHandle>,
    ) -> Result<(), WatchError> {
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
