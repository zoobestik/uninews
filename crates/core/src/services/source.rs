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

impl SourceDraft {
    pub fn source_key(&self) -> &str {
        match self {
            Self::Atom(draft) => draft.url.as_str(),
            Self::Telegram(draft) => draft.username.as_str(),
        }
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct AddError(#[from] pub Internal);

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

#[async_trait]
pub trait SourceService: Send + Sync {
    async fn add(&self, draft: SourceDraft) -> Result<(), AddError>;
    async fn get_by_id(&self, id: Uuid) -> Result<SourceEnum, GetError>;
    async fn get_all(&self) -> Result<impl IntoIterator<Item = SourceEnum>, GetAllError>;
    async fn drop_by_draft(&self, draft: SourceDraft) -> Result<(), DropError>;
}
