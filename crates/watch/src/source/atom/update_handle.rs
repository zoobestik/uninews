use super::feed::{AtomFeedParseError, atom_feed_parse};
use super::feed_item::{AtomItemFromEntryError, atom_items_parse};
use crate::state::{AppState, StateError};
use async_trait::async_trait;
use news_core::models::source::atom::AtomSource;
use news_core::repos::news::NewsUpdateError;
use news_core::services::{HandleError, HttpResponse, HttpUpdateHandle};
use std::sync::Arc;
use tracing::debug;
use url::Url;

pub struct AtomUpdateHandle {
    pub app_state: Arc<AppState>,
    pub source: AtomSource,
}

// ========== Error conversions ==========

impl From<AtomFeedParseError> for HandleError {
    fn from(e: AtomFeedParseError) -> Self {
        match e {
            AtomFeedParseError::GetTextError(e) => todo!(),
            AtomFeedParseError::ParseError(e) => todo!(),
        }
    }
}

impl From<AtomItemFromEntryError> for HandleError {
    fn from(e: AtomItemFromEntryError) -> Self {
        match e {
            AtomItemFromEntryError::IdEmpty => todo!(),
            AtomItemFromEntryError::TitleEmpty(e) => todo!(),
            AtomItemFromEntryError::TitleSanitize(e) => todo!(),
            AtomItemFromEntryError::DescriptionEmpty(e) => todo!(),
            AtomItemFromEntryError::DescriptionSanitize(e) => todo!(),
        }
    }
}

impl From<StateError> for HandleError {
    fn from(_e: StateError) -> Self {
        todo!()
    }
}

fn map_news_update_error(e: NewsUpdateError) -> HandleError {
    match e {
        NewsUpdateError::UpdateItem { id, title, error } => todo!(),
        NewsUpdateError::Internal(e) => todo!(),
    }
}

// ========== HttpUpdateHandle ==========

#[async_trait]
impl HttpUpdateHandle for AtomUpdateHandle {
    fn url(&self) -> &Url {
        &self.source.url
    }

    async fn handle(&self, response: HttpResponse) -> Result<(), HandleError> {
        let atom_channel = atom_feed_parse(response).await?;
        let update = atom_items_parse(&self.source, atom_channel).await?;

        self.app_state
            .news()
            .await?
            .update(&update)
            .await
            .map_err(map_news_update_error)?;

        debug!(
            "[atom_feed=\"{0}\"] Updated {1} news items",
            self.source.url,
            update.len()
        );

        Ok(())
    }
}
