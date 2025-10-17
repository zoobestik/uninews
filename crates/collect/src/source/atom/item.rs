use crate::utils::html::html_sanitize;
use async_trait::async_trait;
use rss::Item;
use serde::{Deserialize, Serialize};
use uninews_core::news::News;
use uninews_core::utils::uuid::gen_consistent_uuid;
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

impl AtomItem {
    pub async fn try_new(parent_id: Uuid, item: Item) -> Result<Self, String> {
        let link = item
            .link
            .clone()
            .ok_or_else(|| "Missing link for one element".to_string())?;

        let guid = item
            .guid
            .clone()
            .ok_or_else(|| format!("Missing guid for {link}"))?
            .value;

        let source_id = gen_consistent_uuid(&parent_id, format!("{link}-{guid}").as_str());

        let description = item
            .description
            .ok_or_else(|| format!("Parsing error for {source_id}"))?;

        let description = html_sanitize(description)
            .await
            .map_err(|e| format!("Sanitize error for {source_id}: {e}"))?;

        Ok(Self {
            parent_id,
            source_id,

            guid,
            link,

            title: item.title.unwrap_or_default(),
            image: None,
            description,
            published_at: item.pub_date,

            content: item.content,
        })
    }
}

#[async_trait]
impl News for AtomItem {
    fn source_id(&self) -> Uuid {
        self.source_id
    }

    fn parent_id(&self) -> Uuid {
        self.parent_id
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
