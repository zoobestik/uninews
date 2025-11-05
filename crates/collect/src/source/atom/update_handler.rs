use super::item::try_atom_news_from_rss_item;
use crate::state::AppState;
use async_trait::async_trait;
use feed_rs::model::Feed;
use feed_rs::parser;
use futures::future::try_join_all;
use std::sync::Arc;
use tracing::debug;
use uninews_core::models::News;
use uninews_core::models::source::atom::AtomSource;
use uninews_core::services::{HttpResponse, HttpUpdateHandler};
use url::Url;

pub struct AtomUpdateHandler {
    pub app_state: Arc<AppState>,
    pub source: AtomSource,
}

#[async_trait]
impl HttpUpdateHandler for AtomUpdateHandler {
    fn url(&self) -> &Url {
        &self.source.url
    }

    async fn handle(&self, resp: HttpResponse) -> Result<(), String> {
        let atom_channel = atom_feed_parse(resp).await?;

        // @todo: save atom channel or delete it
        // self.app_state
        //     .storage()
        //     .await?
        //     .save_raw(self.source.id, atom_channel.)
        //     .await;

        let update = atom_items_parse(&self.source, atom_channel).await?;

        self.app_state.news().await?.update_news(&update).await?;

        debug!(
            "[atom_feed=\"{0}\"] Updated {1} news items",
            self.source.url,
            update.len()
        );

        Ok(())
    }
}

async fn atom_items_parse(src: &AtomSource, data: Feed) -> Result<Vec<Arc<dyn News>>, String> {
    let news_futures = data.entries.into_iter().map(async |item| {
        let news_item = try_atom_news_from_rss_item(src, item).await?;
        Ok::<Arc<dyn News>, String>(Arc::new(news_item))
    });

    try_join_all(news_futures).await
}

async fn atom_feed_parse(response: HttpResponse) -> Result<Feed, String> {
    let content = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    let channel =
        parser::parse(&content[..]).map_err(|e| format!("Failed to parse feed content: {e}"))?;

    Ok(channel)
}
