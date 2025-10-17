use std::sync::Arc;
use tokio::sync::OnceCell;
use uninews_core::http::live::LiveHttpService;
use uninews_core::http::service::HttpService;
use uninews_core::news::live::LiveNewsService;
use uninews_core::news::service::NewsService;
use uninews_core::storage::live::LiveStorageService;
use uninews_core::storage::service::StorageService;

pub struct AppServices {
    news: OnceCell<Arc<dyn NewsService>>,
    http: OnceCell<Arc<dyn HttpService>>,
    storage: OnceCell<Arc<dyn StorageService>>,
}

impl AppServices {
    pub fn new() -> Self {
        Self {
            news: OnceCell::new(),
            http: OnceCell::new(),
            storage: OnceCell::new(),
        }
    }

    pub async fn news_service(&self) -> Result<&Arc<dyn NewsService>, &'static str> {
        self.news
            .get_or_try_init(async || Ok(Arc::new(LiveNewsService::new()) as Arc<dyn NewsService>))
            .await
    }

    pub async fn http_service(&self) -> Result<&Arc<dyn HttpService>, &'static str> {
        self.http
            .get_or_try_init(|| async {
                Ok(Arc::new(LiveHttpService::new()) as Arc<dyn HttpService>)
            })
            .await
    }

    pub async fn storage_service(&self) -> Result<&Arc<dyn StorageService>, &'static str> {
        self.storage
            .get_or_try_init(|| async {
                Ok(Arc::new(LiveStorageService::new()) as Arc<dyn StorageService>)
            })
            .await
    }
}
