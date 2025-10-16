use std::sync::Arc;
use tokio::sync::OnceCell;
use uninews_core::{HttpService, LiveHttpService, LiveNewsService, NewsService};

pub struct AppServices {
    news_service: OnceCell<Arc<dyn NewsService>>,
    http_service: OnceCell<Arc<dyn HttpService>>,
}

impl AppServices {
    pub fn new() -> Self {
        Self {
            news_service: OnceCell::new(),
            http_service: OnceCell::new(),
        }
    }

    pub async fn news_service(&self) -> Result<&Arc<dyn NewsService>, &'static str> {
        self.news_service
            .get_or_try_init(async || Ok(Arc::new(LiveNewsService::new()) as Arc<dyn NewsService>))
            .await
    }

    pub async fn http_service(&self) -> Result<&Arc<dyn HttpService>, &'static str> {
        self.http_service
            .get_or_try_init(|| async {
                Ok(Arc::new(LiveHttpService::new()) as Arc<dyn HttpService>)
            })
            .await
    }
}
