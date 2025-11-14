use crate::utils::html::sanitize_html;
use async_trait::async_trait;
use feed_rs::model::{Entry, Feed};
use futures::future::try_join_all;
use futures::{TryFutureExt, try_join};
use news_core::models::news::News;
use news_core::models::source::atom::AtomSource;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct AtomItem {
    parent_id: Uuid,
    source_id: String,

    // link: Option<String>,
    // guid: String,
    title: String,
    description: String,
    // image: Option<Url>,
    // published_at: Option<String>,
    content: Option<String>,
}

#[derive(Error, Debug)]
pub enum AtomItemFromEntryError {
    #[error("Atom ID is empty")]
    IdEmpty,

    #[error("Atom title is empty {0}")]
    TitleEmpty(String),
    #[error("Atom title is not sanitized {0}")]
    TitleSanitize(String),

    #[error("Atom description is empty {0}")]
    DescriptionEmpty(String),
    #[error("Atom description is not sanitized {0}")]
    DescriptionSanitize(String),
}

pub async fn atom_items_parse(
    src: &AtomSource,
    data: Feed,
) -> Result<Vec<Arc<AtomItem>>, AtomItemFromEntryError> {
    let news_futures = data.entries.into_iter().map(async |item| {
        let news_item = try_atom_item_from_entry(src, item).await?;
        Ok(Arc::new(news_item))
    });

    try_join_all(news_futures).await
}

pub async fn try_atom_item_from_entry(
    source: &AtomSource,
    item: Entry,
) -> Result<AtomItem, AtomItemFromEntryError> {
    let mut links = item.links;

    links.sort_by(|a, b| b.href.cmp(&a.href));

    let link = links
        .iter()
        .find(|link| !link.href.is_empty())
        .map(|link| link.href.clone());

    let id = match (item.id.is_empty(), &link) {
        (false, _) => &item.id,
        (true, Some(url)) => url,
        (true, None) => return Err(AtomItemFromEntryError::IdEmpty),
    };

    let parent_id = source.id;

    let title = item
        .title
        .map(|s| s.content)
        .ok_or_else(|| AtomItemFromEntryError::TitleEmpty(id.clone()))?;

    let mut description = item
        .content
        .and_then(|s| s.body)
        .and_then(|body| if body.is_empty() { None } else { Some(body) });

    if description.is_none() {
        description = item
            .summary
            .map(|s| s.content)
            .and_then(|body| if body.is_empty() { None } else { Some(body) });
    }

    let description =
        description.ok_or_else(|| AtomItemFromEntryError::DescriptionEmpty(id.clone()))?;

    let future_title =
        sanitize_html(&title).map_err(|e| AtomItemFromEntryError::TitleSanitize(e.to_string()));

    let future_description = sanitize_html(&description)
        .map_err(|e| AtomItemFromEntryError::DescriptionSanitize(e.to_string()));

    let (title, description) = try_join!(future_title, future_description)?;

    // let published_at = item.published.map(|dt| dt.to_string());

    Ok(AtomItem {
        parent_id,
        source_id: id.clone(),

        title,
        description,
        content: None,
        // guid: item.id,
        // link,
        // image: None,
        // published_at,
    })
}

#[async_trait]
impl News for AtomItem {
    fn source_id(&self) -> &str {
        self.source_id.as_str()
    }
    fn parent_id(&self) -> Uuid {
        self.parent_id
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn content(&self) -> &Option<String> {
        &self.content
    }
}
