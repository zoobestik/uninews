use news_core::repos::news::NewsRepository;
use news_core::repos::source::SourceRepository;
use news_core::services::HttpService;
use news_sqlite_core::db::init::DBInitError;
use news_sqlite_core::repos::news::SqliteNewsRepository;
use news_sqlite_core::repos::source::SqliteSourceRepository;
use news_sqlite_core::services::http::LiveHttpService;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;

pub struct AppState {
    sources: OnceCell<Arc<dyn SourceRepository>>,
    news: OnceCell<Arc<dyn NewsRepository>>,
    http: OnceCell<Arc<dyn HttpService>>,
}

#[derive(Error, Debug)]
#[error("failed to initialize {0}")]
pub struct StateError(#[source] DBInitError);

impl AppState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sources: OnceCell::new(),
            news: OnceCell::new(),
            http: OnceCell::new(),
        }
    }

    pub async fn sources(&self) -> Result<Arc<dyn SourceRepository>, StateError> {
        Ok(self
            .sources
            .get_or_try_init(async || {
                let sources: Arc<dyn SourceRepository> =
                    Arc::new(SqliteSourceRepository::new().await?);
                Ok(sources)
            })
            .await
            .map_err(StateError)?
            .clone())
    }

    pub async fn news(&self) -> Result<Arc<dyn NewsRepository>, StateError> {
        Ok(self
            .news
            .get_or_try_init(async || {
                let news: Arc<dyn NewsRepository> = Arc::new(SqliteNewsRepository::new().await?);
                Ok(news)
            })
            .await
            .map_err(StateError)?
            .clone())
    }

    pub async fn http(&self) -> Arc<dyn HttpService> {
        self.http
            .get_or_init(async || {
                let http: Arc<dyn HttpService> = Arc::new(LiveHttpService::new());
                http
            })
            .await
            .clone()
    }
}
