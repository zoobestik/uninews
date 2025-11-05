use std::sync::Arc;
use tokio::sync::OnceCell;
use uninews_adapters::repos::news::SqliteNewsRepository;
use uninews_adapters::repos::source::SqliteSourceRepository;
use uninews_adapters::services::http::LiveHttpService;
use uninews_adapters::services::storage::LiveStorageService;
use uninews_adapters::utils::sqlite::tools::init_db_pool;
use uninews_core::repos::news::NewsRepository;
use uninews_core::repos::source::SourceRepository;
use uninews_core::services::{HttpService, StorageService};

pub struct AppState {
    sources: OnceCell<Arc<dyn SourceRepository>>,
    news: OnceCell<Arc<dyn NewsRepository>>,
    http: OnceCell<Arc<dyn HttpService>>,
    storage: OnceCell<Arc<dyn StorageService>>,
}

impl AppState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sources: OnceCell::new(),
            news: OnceCell::new(),
            http: OnceCell::new(),
            storage: OnceCell::new(),
        }
    }

    pub async fn sources(&self) -> Result<&Arc<dyn SourceRepository>, String> {
        self.sources
            .get_or_try_init(|| async {
                let pool = init_db_pool().await?;
                let service: Arc<dyn SourceRepository> =
                    Arc::new(SqliteSourceRepository::new(pool));
                Ok(service)
            })
            .await
    }

    pub async fn news(&self) -> Result<&Arc<dyn NewsRepository>, String> {
        self.news
            .get_or_try_init(|| async {
                let pool = init_db_pool().await?;
                let service: Arc<dyn NewsRepository> = Arc::new(SqliteNewsRepository::new(pool));
                Ok(service)
            })
            .await
    }

    pub async fn http(&self) -> Result<&Arc<dyn HttpService>, &'static str> {
        self.http
            .get_or_try_init(|| async {
                let service: Arc<dyn HttpService> = Arc::new(LiveHttpService::new());
                Ok(service)
            })
            .await
    }

    pub async fn storage(&self) -> Result<&Arc<dyn StorageService>, &'static str> {
        self.storage
            .get_or_try_init(|| async {
                let service: Arc<dyn StorageService> = Arc::new(LiveStorageService::new());
                Ok(service)
            })
            .await
    }
}
