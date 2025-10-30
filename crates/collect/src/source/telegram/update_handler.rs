use crate::source::telegram::item::TelegramItem;
use crate::state::AppState;
use crate::utils::html::{html_to_text, sanitize_html};
use async_trait::async_trait;
use futures::future::{try_join, try_join_all};
use scraper::{Html, Selector};
use std::sync::Arc;
use uninews_core::models::telegram::TelegramChannelSource;
use uninews_core::parse::truncate_with_dots;
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

const TITLE_MAX_LENGTH: usize = 100;

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

        // Extract all HTML data first to avoid lifetime issues
        let extract_data = {
            let document = Html::parse_document(&html_content);
            let message_selector = Selector::parse(".tgme_widget_message_wrap").unwrap();
            let title_selector = Selector::parse(".tgme_widget_message_text").unwrap();
            let body_selector = Selector::parse(".tgme_widget_message_text").unwrap();

            document
                .select(&message_selector)
                .filter_map(|element| {
                    let title = element.select(&title_selector).next()?.html();
                    let body = element.select(&body_selector).next()?.html();
                    Some((title, body))
                })
                .collect::<Vec<(String, String)>>()
        };

        let html_futures = extract_data
            .into_iter()
            .map(|(title_html, body_html)| async move {
                let (title_text, body_text) =
                    try_join(html_to_text(&title_html), sanitize_html(&body_html)).await?;
                let title_text = truncate_with_dots(&title_text, TITLE_MAX_LENGTH);
                Ok::<(String, String), String>((title_text, body_text))
            });

        let result = try_join_all(html_futures).await?;

        let update: Vec<_> = result
            .into_iter()
            .map(|(title, description)| {
                Arc::new(TelegramItem {
                    parent_id: self.source.id,
                    source_id: gen_consistent_uuid(&self.source.id, &description),
                    title,
                    description,
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
