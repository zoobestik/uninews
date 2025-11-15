use crate::errors::Internal;
use crate::models::source::SourceEnum;
use crate::models::source::atom::AtomDraft;
use crate::models::source::telegram::TelegramDraft;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

pub enum SourceDraft {
    Atom(AtomDraft),
    Telegram(TelegramDraft),
}

#[derive(Error, Debug)]
pub enum AddError {
    #[error("Source with source_key={0} already exists")]
    AlreadyExists(String),

    #[error(transparent)]
    Internal(#[from] Internal),
}

#[derive(Error, Debug)]
pub enum GetError {
    #[error("Not found")]
    NotFound { id: String, entity: String },

    #[error(transparent)]
    Internal(#[from] Internal),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct GetAllError(#[from] pub Internal);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct DropError(#[from] pub Internal);

pub type DeleteCriteria = SourceDraft;

#[async_trait]
pub trait SourceService: Send + Sync {
    async fn add(&self, draft: SourceDraft) -> Result<(), AddError>;
    async fn get_by_id(&self, id: Uuid) -> Result<SourceEnum, GetError>;
    async fn get_all(&self) -> Result<impl IntoIterator<Item = SourceEnum>, GetAllError>;
    async fn drop_by(&self, criteria: DeleteCriteria) -> Result<(), DropError>;
}
