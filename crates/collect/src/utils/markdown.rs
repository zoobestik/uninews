use ammonia::Builder;
use std::collections::HashSet;
use std::sync::LazyLock;
use tokio::task::spawn_blocking;

static MARKDOWN_ALLOWED_TAGS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(MARKDOWN_ALLOWED_TAGS_ARR));

static MARKDOWN_ALLOWED_ATTRS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(MARKDOWN_ALLOWED_ATTRS_ARR));

pub async fn html_to_markdown(html_dirty: String) -> Result<String, String> {
    spawn_blocking(move || -> Result<String, String> {
        Ok(Builder::new()
            .tags(MARKDOWN_ALLOWED_TAGS.clone())
            .generic_attributes(MARKDOWN_ALLOWED_ATTRS.clone())
            .link_rel(Some("noopener noreferrer nofollow ugc"))
            .strip_comments(true)
            .clean(&html_dirty)
            .to_string())
    })
    .await
    .unwrap()
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
