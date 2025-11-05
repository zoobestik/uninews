use async_trait::async_trait;
use reqwest::Response;
use std::sync::Arc;
use url::Url;
use uuid::Uuid;

pub type HttpResponse = Response;

#[async_trait]
pub trait HttpUpdateHandler: Send + Sync {
    fn url(&self) -> &Url;
    async fn handle(&self, response: Response) -> Result<(), String>;
}

#[async_trait]
pub trait HttpService: Send + Sync {
    async fn watch_changes(&self, update_handler: Arc<dyn HttpUpdateHandler>)
    -> Result<(), String>;
}

#[async_trait]
pub trait StorageService: Send + Sync {
    async fn save_raw(&self, key: Uuid, value: &str);
}
