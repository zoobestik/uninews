use super::item::TelegramItem;
use super::parse::{ParseHtmlError, parse_html};
use crate::state::AppState;
use async_trait::async_trait;
use news_core::models::news::News;
use news_core::models::source::telegram::TelegramSource;
use news_core::services::{HandleError, HttpResponse, HttpUpdateHandle};
use news_core::uuid::gen_consistent_uuid;
use std::sync::Arc;
use url::Url;

#[allow(dead_code)]
pub struct TelegramWebUpdateHandle {
    pub app_state: Arc<AppState>,
    pub source: TelegramSource,
    pub url: Url,
}
// ========== Error conversions ==========

impl From<ParseHtmlError> for HandleError {
    fn from(err: ParseHtmlError) -> Self {
        match err {
            ParseHtmlError::TitleSelector(e) => todo!(),
            ParseHtmlError::TitleConvert(e) => todo!(),
            ParseHtmlError::BodySelector(e) => todo!(),
            ParseHtmlError::BodyConvert(e) => todo!(),
            ParseHtmlError::MessageSelector(e) => todo!(),
        }
    }
}

// ========== HttpUpdateHandle ==========

#[async_trait]
impl HttpUpdateHandle for TelegramWebUpdateHandle {
    fn url(&self) -> &Url {
        &self.url
    }

    async fn handle(&self, response: HttpResponse) -> Result<(), HandleError> {
        let html_content = response
            .text()
            .await
            .map_err(|e| HandleError::from(e.into()))?;
        let result = parse_html(&html_content).await?;

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

        let news = self.app_state.news().await?;

        news.update(&update)
            .await
            .map_err(|e| HandleError::from(e.into()))?;

        Ok(())
    }
}
