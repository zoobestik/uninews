use async_trait::async_trait;
use reqwest::Response;
use std::sync::Arc;
use url::Url;

pub type HttpResponse = Response;

#[async_trait]
pub trait HttpUpdateHandler: Send + Sync {
    async fn handle(&self, message: Response) -> Result<(), String>;
    fn url(&self) -> &Url;
}

#[async_trait]
pub trait HttpService: Send + Sync {
    async fn watch_changes(&self, update_handler: Arc<dyn HttpUpdateHandler>)
    -> Result<(), String>;
}
