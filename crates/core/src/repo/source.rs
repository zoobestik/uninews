pub mod sqlite;

use crate::models::SourceType;
use crate::models::atom::AtomDraft;
use crate::models::telegram::TelegramChannelDraft;
use async_trait::async_trait;
use uuid::Uuid;

pub enum CreateSource {
    Atom(AtomDraft),
    TelegramChannel(TelegramChannelDraft),
}

#[async_trait]
pub trait SourceRepository {
    async fn insert_or_update(&self, draft: CreateSource) -> Result<SourceType, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<SourceType, String>;
    // async fn find_by_ext_id(&self, id: Uuid) -> Result<SourceType, String>;
}
