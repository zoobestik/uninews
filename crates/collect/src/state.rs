use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uninews_core::fs::get_db_uri;
use uninews_core::services::http::{HttpService, LiveHttpService};
use uninews_core::services::news::{LiveNewsService, NewsService};
use uninews_core::services::source::SourceService;
use uninews_core::services::source::sqlite::SqliteSourceService;
use uninews_core::services::storage::{LiveStorageService, StorageService};

pub struct AppState {
    sources: OnceCell<Arc<dyn SourceService>>,
    news: OnceCell<Arc<dyn NewsService>>,
    http: OnceCell<Arc<dyn HttpService>>,
    storage: OnceCell<Arc<dyn StorageService>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sources: OnceCell::new(),
            news: OnceCell::new(),
            http: OnceCell::new(),
            storage: OnceCell::new(),
        }
    }

    pub async fn sources(&self) -> Result<&Arc<dyn SourceService>, String> {
        self.sources
            .get_or_try_init(async || {
                let db_uri = get_db_uri()?;

                let db_pool = SqlitePool::connect(&db_uri)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(Arc::new(SqliteSourceService::new(db_pool)) as Arc<dyn SourceService>)
            })
            .await
    }

    pub async fn news(&self) -> Result<&Arc<dyn NewsService>, &'static str> {
        self.news
            .get_or_try_init(async || Ok(Arc::new(LiveNewsService::new()) as Arc<dyn NewsService>))
            .await
    }

    pub async fn http(&self) -> Result<&Arc<dyn HttpService>, &'static str> {
        self.http
            .get_or_try_init(|| async {
                Ok(Arc::new(LiveHttpService::new()) as Arc<dyn HttpService>)
            })
            .await
    }

    pub async fn storage(&self) -> Result<&Arc<dyn StorageService>, &'static str> {
        self.storage
            .get_or_try_init(|| async {
                Ok(Arc::new(LiveStorageService::new()) as Arc<dyn StorageService>)
            })
            .await
    }
}
