use ammonia::Builder;
use htmd::HtmlToMarkdown;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use thiserror::Error;
use tokio::task::spawn_blocking;

static MD2HTML_CONVERTER: LazyLock<Arc<HtmlToMarkdown>> =
    LazyLock::new(|| Arc::new(HtmlToMarkdown::new()));

#[derive(Error, Debug)]
#[error("Failed to convert HTML to Markdown: {0}")]
pub struct ConvertError(String);

async fn sanitize_and_convert(
    html_dirty: &str,
    sanitizer: &'static Builder<'static>,
) -> Result<String, ConvertError> {
    let converter = MD2HTML_CONVERTER.clone();
    let html_dirty = html_dirty.to_string();

    spawn_blocking(move || {
        let html = sanitizer.clean(&html_dirty).to_string();
        let result = converter
            .convert(&html)
            .map_err(|e| ConvertError(e.to_string()))?;
        Ok(result)
    })
    .await
    .map_err(|e| ConvertError(e.to_string()))?
}

static CONTENT_SANITIZER: LazyLock<Builder<'static>> = LazyLock::new(|| {
    let mut sanitizer = Builder::new();
    sanitizer
        .tags(HashSet::from(CONTENT_ALLOWED_TAGS_ARR))
        .generic_attributes(HashSet::from(CONTENT_ALLOWED_ATTRS_ARR))
        .link_rel(Some("noopener noreferrer nofollow ugc"))
        .strip_comments(true);
    sanitizer
});

#[derive(Error, Debug)]
#[error("Failed to sanitize HTML: {0}")]
pub struct SanitizeError(String);

pub async fn sanitize_html(html_dirty: &str) -> Result<String, SanitizeError> {
    let html_owned = html_dirty.to_string();
    spawn_blocking(move || CONTENT_SANITIZER.clean(&html_owned).to_string())
        .await
        .map_err(|e| SanitizeError(e.to_string()))
}

pub async fn html_to_text(html_dirty: &str) -> Result<String, ConvertError> {
    let html_owned = html_dirty.to_string();

    spawn_blocking(move || {
        Builder::new()
            .tags(HashSet::new())
            .strip_comments(true)
            .clean(&html_owned)
            .to_string()
    })
    .await
    .map_err(|e| ConvertError(e.to_string()))
}

#[allow(dead_code)] // @todo: remove
pub async fn html_to_content(html_dirty: &str) -> Result<String, ConvertError> {
    sanitize_and_convert(html_dirty, &CONTENT_SANITIZER).await
}

static TITLE_SANITIZER: LazyLock<Builder<'static>> = LazyLock::new(|| {
    let mut sanitizer = Builder::new();
    sanitizer
        .tags(HashSet::from(TITLE_ALLOWED_TAGS_ARR))
        .link_rel(Some("noopener noreferrer nofollow ugc"))
        .strip_comments(true);
    sanitizer
});

#[allow(dead_code)] // @todo: remove
pub async fn html_to_title(html_dirty: &str) -> Result<String, ConvertError> {
    sanitize_and_convert(html_dirty, &TITLE_SANITIZER).await
}

const TITLE_ALLOWED_TAGS_ARR: [&str; 7] = ["em", "strong", "b", "i", "sup", "sub", "strike"];

const CONTENT_ALLOWED_ATTRS_ARR: [&str; 3] = ["href", "src", "alt"];
const CONTENT_ALLOWED_TAGS_ARR: [&str; 45] = [
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "p",
    "a",
    "img",
    "br",
    "em",
    "strong",
    "ul",
    "ol",
    "li",
    "blockquote",
    "hr",
    "code",
    "pre",
    "table",
    "thead",
    "tbody",
    "tr",
    "th",
    "td",
    "sup",
    "sub",
    "del",
    "ins",
    "mark",
    "abbr",
    "acronym",
    "cite",
    "q",
    "b",
    "i",
    "u",
    "s",
    "strike",
    "big",
    "small",
    "tt",
    "time",
    "audio",
    "video",
];
