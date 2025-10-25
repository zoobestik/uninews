use crate::source::telegram::item::TelegramItem;
use crate::state::AppState;
use crate::utils::html::html_to_content;
use async_trait::async_trait;
use futures::future::try_join_all;
use scraper::{Html, Selector};
use std::sync::Arc;
use uninews_core::models::telegram::TelegramChannelSource;
use uninews_core::services::http::{HttpResponse, HttpUpdateHandler};
use uninews_core::services::news::News;
use uninews_core::uuid::gen_consistent_uuid;
use url::Url;

#[allow(dead_code)]
pub struct TelegramWebUpdateHandler {
    pub app_state: Arc<AppState>,
    pub source: TelegramChannelSource,

    pub url: Url,
}

#[async_trait]
impl HttpUpdateHandler for TelegramWebUpdateHandler {
    async fn handle(&self, response: HttpResponse) -> Result<(), String> {
        let html_content = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {e}"))?;

        self.app_state
            .storage()
            .await?
            .save_raw(self.source.id, &html_content)
            .await;

        let html_strings: Vec<String> = {
            let document = Html::parse_document(&html_content);
            let title_selector =
                Selector::parse(".tgme_widget_message_wrap .tgme_widget_message_text").unwrap();

            document
                .select(&title_selector)
                .map(|element| element.html())
                .collect()
        };

        let html_futures = html_strings.into_iter().map(html_to_content);

        let result = try_join_all(html_futures).await?;

        let update: Vec<_> = result
            .into_iter()
            .map(|content| {
                Arc::new(TelegramItem {
                    parent_id: self.source.id,
                    source_id: gen_consistent_uuid(&self.source.id, &content),
                    description: content,
                }) as Arc<dyn News>
            })
            .collect();

        self.app_state.news().await?.update_news(&update).await?;

        Ok(())
    }

    fn url(&self) -> &Url {
        &self.url
    }
}
