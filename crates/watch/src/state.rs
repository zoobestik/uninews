use news_core::services::HttpService;
use news_core::services::news::NewsService;
use news_core::services::source::SourceService;
use news_sqlite_core::db::init::DBInitError;
use news_sqlite_core::services::http::LiveHttpService;
use news_sqlite_core::services::news::SqliteNewsService;
use news_sqlite_core::services::source::SqliteSourceService;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;

pub struct LiveAppState {
    sources: OnceCell<Arc<SqliteSourceService>>,
    news: OnceCell<Arc<SqliteNewsService>>,
    http: OnceCell<Arc<LiveHttpService>>,
}

#[derive(Error, Debug)]
#[error("failed to initialize {0}")]
pub struct StateError(#[source] DBInitError);

impl Default for LiveAppState {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveAppState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sources: OnceCell::new(),
            news: OnceCell::new(),
            http: OnceCell::new(),
        }
    }

    pub async fn sources(&self) -> Result<Arc<impl SourceService + 'static>, StateError> {
        Ok(self
            .sources
            .get_or_try_init(async || {
                let sources = Arc::new(SqliteSourceService::try_new().await?);
                Ok(sources)
            })
            .await
            .map_err(StateError)?
            .clone())
    }

    pub async fn news(&self) -> Result<Arc<impl NewsService + 'static>, StateError> {
        Ok(self
            .news
            .get_or_try_init(async || {
                let news = Arc::new(SqliteNewsService::try_new().await?);
                Ok(news)
            })
            .await
            .map_err(StateError)?
            .clone())
    }

    pub async fn http(&self) -> Arc<impl HttpService + 'static> {
        self.http
            .get_or_init(async || Arc::new(LiveHttpService::new()))
            .await
            .clone()
    }
}
