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
pub trait SourceRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<SourceType, String>;
    async fn find_all_sources(&self) -> Result<Vec<SourceType>, String>;

    async fn delete_by_id(&self, id: Uuid) -> Result<(), String>;
    async fn delete_with_type(&self, url: Uuid, source_type: SourceTypeValue)
    -> Result<(), String>;

    async fn insert(&self, draft: SourceCreate) -> Result<(), String>;
}
