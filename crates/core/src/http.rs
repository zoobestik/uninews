use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

#[async_trait]
pub trait HttpService: Send + Sync {
    async fn refresh_by_schedule(&self, url: Url) -> Result<String, String>;
}

pub struct LiveHttpService {
    client: Client,
}

impl LiveHttpService {
    #[must_use]
    pub fn try_new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl HttpService for LiveHttpService {
    async fn refresh_by_schedule(&self, url: Url) -> Result<String, String> {
        sleep(Duration::from_secs(10)).await;

        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let resp = resp.text().await.map_err(|e| e.to_string())?;

        Ok(resp)
    }
}
