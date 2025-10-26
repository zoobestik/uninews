use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uninews_core::fs::get_db_uri;
use uninews_core::services::http::{HttpService, LiveHttpService};
use uninews_core::services::news::{LiveNewsService, NewsService};
use uninews_core::services::source::SourceService;
use uninews_core::services::source::sqlite::SqliteSourceService;
use uninews_core::services::storage::{LiveStorageService, StorageService};

static DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

async fn init_db_pool() -> Result<SqlitePool, String> {
    DB_POOL
        .get_or_try_init(|| async {
            SqlitePool::connect(&get_db_uri()?)
                .await
                .map_err(|e| e.to_string())
        })
        .await
        .cloned()
}

pub struct AppState {
    sources: OnceCell<Arc<dyn SourceService>>,
    news: OnceCell<Arc<dyn NewsService>>,
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

    pub async fn sources(&self) -> Result<&Arc<dyn SourceService>, String> {
        self.sources
            .get_or_try_init(|| async {
                let pool = init_db_pool().await?;
                let service: Arc<dyn SourceService> = Arc::new(SqliteSourceService::new(pool));
                Ok(service)
            })
            .await
    }

    pub async fn news(&self) -> Result<&Arc<dyn NewsService>, String> {
        self.news
            .get_or_try_init(|| async {
                let pool = init_db_pool().await?;
                let service: Arc<dyn NewsService> = Arc::new(LiveNewsService::new(pool));
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
