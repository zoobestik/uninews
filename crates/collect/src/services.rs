use async_trait::async_trait;
use std::sync::Arc;
use uninews_core::{HttpService, NewsService};

#[async_trait]
pub trait AppServices: Send + Sync {
    async fn news_service(&self) -> &Arc<dyn NewsService>;
    async fn http_service(&self) -> Option<&Arc<dyn HttpService>>;
}
