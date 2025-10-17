use ammonia::Builder;
use htmd::HtmlToMarkdown;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use tokio::task::spawn_blocking;

static SANITIZER: LazyLock<Builder<'static>> = LazyLock::new(|| {
    let mut sanitizer = Builder::new();
    sanitizer
        .tags(HashSet::from(MARKDOWN_ALLOWED_TAGS_ARR))
        .generic_attributes(HashSet::from(MARKDOWN_ALLOWED_ATTRS_ARR))
        .link_rel(Some("noopener noreferrer nofollow ugc"))
        .strip_comments(true);
    sanitizer
});

static MD2HTML_CONVERTER: LazyLock<Arc<HtmlToMarkdown>> =
    LazyLock::new(|| Arc::new(HtmlToMarkdown::new()));

pub async fn html_sanitize(html_dirty: String) -> Result<String, String> {
    let clean_html = spawn_blocking(move || SANITIZER.clean(&html_dirty).to_string())
        .await
        .map_err(|e| format!("Failed in sanitize HTML in block: {e}"))?;

    Ok(clean_html)
}

#[allow(dead_code)]
pub async fn html_to_markdown(html: String) -> Result<String, String> {
    let converter = MD2HTML_CONVERTER.clone();

    spawn_blocking(move || -> Result<String, String> {
        let result = converter
            .convert(html.as_str())
            .map_err(|e| format!("Failed to convert HTML to Markdown: {e}"))?;
        Ok(result)
    })
    .await
    .map_err(|e| format!("Failed convert HTML to Markdown in block: {e}"))?
}

const MARKDOWN_ALLOWED_ATTRS_ARR: [&str; 3] = ["href", "src", "alt"];
const MARKDOWN_ALLOWED_TAGS_ARR: [&str; 46] = [
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
