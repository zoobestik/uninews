use ammonia::Builder;
use htmd::HtmlToMarkdown;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use tokio::task::spawn_blocking;

static MD2HTML_CONVERTER: LazyLock<Arc<HtmlToMarkdown>> =
    LazyLock::new(|| Arc::new(HtmlToMarkdown::new()));

async fn sanitize_and_convert(
    html_dirty: String,
    sanitizer: &'static Builder<'static>,
) -> Result<String, String> {
    let converter = MD2HTML_CONVERTER.clone();

    spawn_blocking(move || {
        let html = sanitizer.clean(&html_dirty).to_string();
        let result = converter
            .convert(html.as_str())
            .map_err(|e| format!("Failed to convert HTML to Markdown: {e}"))?;
        Ok(result)
    })
    .await
    .map_err(|e| format!("Failed convert HTML to Markdown in block: {e}"))?
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

pub async fn html_to_content(html_dirty: String) -> Result<String, String> {
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

pub async fn html_to_title(html_dirty: String) -> Result<String, String> {
    sanitize_and_convert(html_dirty, &TITLE_SANITIZER).await
}

const TITLE_ALLOWED_TAGS_ARR: [&str; 7] = ["em", "strong", "b", "i", "sup", "sub", "strike"];

const CONTENT_ALLOWED_ATTRS_ARR: [&str; 3] = ["href", "src", "alt"];
const CONTENT_ALLOWED_TAGS_ARR: [&str; 46] = [
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
    "abbr",
    "time",
    "audio",
    "video",
];
