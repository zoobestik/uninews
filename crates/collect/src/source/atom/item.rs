use crate::utils::html::{html_to_content, html_to_title};
use async_trait::async_trait;
use futures::try_join;
use rss::Item;
use serde::{Deserialize, Serialize};
use uninews_core::services::news::News;
use uninews_core::uuid::gen_consistent_uuid;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomItem {
    parent_id: Uuid,
    source_id: Uuid,

    link: String,
    guid: String,

    title: String,
    description: String,
    image: Option<Url>,
    published_at: Option<String>,

    content: Option<String>,
}

pub async fn try_atom_news_from_rss_item(parent_id: Uuid, item: Item) -> Result<AtomItem, String> {
    let link = item
        .link
        .clone()
        .ok_or_else(|| "Missing link for one element".to_string())?;

    let guid = item
        .guid
        .clone()
        .ok_or_else(|| format!("Missing guid for {link}"))?
        .value;

    let source_id = gen_consistent_uuid(&parent_id, &format!("{link}-{guid}"));

    let (title, description, content) = try_join!(
        html_to_title(item.title.unwrap_or_default()),
        html_to_content(
            item.description
                .ok_or_else(|| format!("Parsing error for {source_id}"))?,
        ),
        html_to_content(item.content.unwrap_or_default()),
    )
    .map_err(|e| format!("Sanitize error for {source_id}: {e}"))?;

    Ok(AtomItem {
        parent_id,
        source_id,

        guid,
        link,

        title,
        description,
        image: None,
        published_at: item.pub_date,

        content: if content.trim().is_empty() {
            None
        } else {
            Some(content)
        },
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
        // html_to_markdown(self.description.clone())
        //     .await
        //     .map_err(|e| format!("Parsing error for {0}: {e}", self.source_id))?
    }

    fn content(&self) -> &Option<String> {
        &self.content
    }
}
