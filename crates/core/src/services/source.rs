pub mod sqlite;

use crate::models::atom::AtomDraft;
use crate::models::telegram::TelegramChannelDraft;
use crate::models::{SourceType, SourceTypeValue};
use async_trait::async_trait;
use uuid::Uuid;

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
