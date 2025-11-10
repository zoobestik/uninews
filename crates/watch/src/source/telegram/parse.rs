use crate::utils::html::{ConvertError, SanitizeError, html_to_text, sanitize_html};
use futures::TryFutureExt;
use futures::future::{try_join, try_join_all};
use news_sqlite_core::utils::text::truncate_with_dots;
use scraper::{Html, Selector};
use thiserror::Error;

const TITLE_MAX_LENGTH: usize = 100;

#[derive(Error, Debug)]
pub enum ParseHtmlError {
    #[error("Failed to construct title selector: {0}")]
    TitleSelector(String),
    #[error("Failed to convert title to text: {0}")]
    TitleConvert(#[source] ConvertError),

    #[error("Failed to construct body selector: {0}")]
    BodySelector(String),
    #[error("Failed to find body selector: {0}")]
    BodyConvert(#[source] SanitizeError),

    #[error("Failed to construct message selector: {0}")]
    MessageSelector(String),
}

pub async fn parse_html(html_content: &str) -> Result<Vec<(String, String)>, ParseHtmlError> {
    let message_selector = Selector::parse(".tgme_widget_message_wrap")
        .map_err(|e| ParseHtmlError::MessageSelector(e.to_string()))?;
    let title_selector = Selector::parse(".tgme_widget_message_text")
        .map_err(|e| ParseHtmlError::TitleSelector(e.to_string()))?;
    let body_selector = Selector::parse(".tgme_widget_message_text")
        .map_err(|e| ParseHtmlError::BodySelector(e.to_string()))?;

    let document = Html::parse_document(html_content);

    let html_futures = document
        .select(&message_selector)
        .filter_map(|element| {
            let title = element.select(&title_selector).next()?.html();
            let body = element.select(&body_selector).next()?.html();
            Some((title, body))
        })
        .map(|(title_html, body_html)| async move {
            let title_text = html_to_text(&title_html)
                .map_ok(|text| truncate_with_dots(&text, TITLE_MAX_LENGTH))
                .map_err(ParseHtmlError::TitleConvert);

            let body_text = sanitize_html(&body_html).map_err(ParseHtmlError::BodyConvert);

            Ok::<(String, String), ParseHtmlError>(try_join(title_text, body_text).await?)
        });

    Ok(try_join_all(html_futures).await?)
}
