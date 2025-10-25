use crate::source::atom::item::try_atom_news_from_rss_item;
use crate::state::AppState;
use async_trait::async_trait;
use futures::future::try_join_all;
use rss::Channel;
use std::io::Cursor;
use std::sync::Arc;
use tracing::debug;
use uninews_core::models::atom::AtomSource;
use uninews_core::services::http::{HttpResponse, HttpUpdateHandler};
use uninews_core::services::news::News;
use url::Url;

pub struct AtomUpdateHandler {
    pub app_state: Arc<AppState>,
    pub source: AtomSource,
}

#[async_trait]
impl HttpUpdateHandler for AtomUpdateHandler {
    async fn handle(&self, resp: HttpResponse) -> Result<(), String> {
        let atom_channel = atom_feed_parse(resp).await?;

        self.app_state
            .storage()
            .await?
            .save_raw(self.source.id, &atom_channel.to_string())
            .await;

        let update = atom_items_parse(&self.source, atom_channel).await?;

        self.app_state.news().await?.update_news(&update).await?;

        debug!(
            "[atom_feed=\"{0}\"] Updated {1} news items",
            self.source.url,
            update.len()
        );

        Ok(())
    }

    fn url(&self) -> &Url {
        &self.source.url
    }
}

async fn atom_items_parse(src: &AtomSource, data: Channel) -> Result<Vec<Arc<dyn News>>, String> {
    let news_futures = data.items.into_iter().map(async |item| {
        let news_item = try_atom_news_from_rss_item(src, item).await?;
        Ok::<Arc<dyn News>, String>(Arc::new(news_item))
    });

    try_join_all(news_futures).await
}

async fn atom_feed_parse(resp: HttpResponse) -> Result<Channel, String> {
    let xml_text = resp
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    Channel::read_from(Cursor::new(&xml_text))
        .map_err(|e| format!("Failed to parse feed content: {e}"))
}
