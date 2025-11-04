use crate::utils::html::sanitize_html;
use async_trait::async_trait;
use feed_rs::model::Entry;
use futures::try_join;
use serde::{Deserialize, Serialize};
use uninews_core::models::News;
use uninews_core::models::source::atom::AtomSource;
use uninews_core::uuid::gen_consistent_uuid;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomItem {
    parent_id: Uuid,
    source_id: Uuid,

    link: Option<String>,
    guid: String,

    title: String,
    description: String,
    image: Option<Url>,
    published_at: Option<String>,

    content: Option<String>,
}

pub async fn try_atom_news_from_rss_item(
    source: &AtomSource,
    item: Entry,
) -> Result<AtomItem, String> {
    let mut links = item.links;

    links.sort_by(|a, b| b.href.cmp(&a.href));

    let link = links
        .iter()
        .find(|link| !link.href.is_empty())
        .map(|link| link.href.clone());

    let id = match (item.id.is_empty(), &link) {
        (false, _) => &item.id,
        (true, Some(url)) => url,
        (true, None) => return Err("Item id is empty".to_string()),
    };

    let parent_id = source.id;
    let source_id = gen_consistent_uuid(&parent_id, id);

    let title = item
        .title
        .map(|s| s.content)
        .ok_or_else(|| format!("Title is empty {source_id}"))?;

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

    let description = description.ok_or_else(|| format!("Description is empty {source_id}"))?;

    let (title, description) = try_join!(sanitize_html(&title), sanitize_html(&description))
        .map_err(|e| format!("Sanitize error for {source_id}: {e}"))?;

    let published_at = item.published.map(|s| s.to_string());

    Ok(AtomItem {
        parent_id,
        source_id,

        title,
        description,
        content: None,

        guid: item.id,
        link,
        image: None,
        published_at,
    })
}

#[async_trait]
impl News for AtomItem {
    fn source_id(&self) -> Uuid {
        self.source_id
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
