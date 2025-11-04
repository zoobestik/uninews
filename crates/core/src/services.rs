use crate::models::News;
use crate::models::source::atom::AtomDraft;
use crate::models::source::telegram::TelegramChannelDraft;
use crate::models::source::{SourceType, SourceTypeValue};
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

#[async_trait]
pub trait NewsService: Send + Sync {
    async fn update_news(&self, news: &[Arc<dyn News>]) -> Result<(), String>;
}

pub enum SourceCreate {
    Atom(AtomDraft),
    TelegramChannel(TelegramChannelDraft),
}

#[async_trait]
pub trait SourceService: Send + Sync {
    async fn add(&self, draft: SourceCreate) -> Result<(), String>;

    async fn get_by_id(&self, id: Uuid) -> Result<SourceType, String>;
    async fn get_all(&self) -> Result<Vec<SourceType>, String>;

    async fn delete_by_id(&self, id: Uuid) -> Result<(), String>;
    async fn delete_with_type(&self, id: Uuid, source_type: SourceTypeValue) -> Result<(), String>;
}
