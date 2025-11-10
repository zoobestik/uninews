use feed_rs::model::Feed;
use feed_rs::parser::{ParseFeedError, parse};
use news_core::services::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AtomFeedParseError {
    #[error("Failed to get response text: {0}")]
    GetTextError(String),

    #[error("Failed to parse feed: {0}")]
    ParseError(ParseFeedError),
}

pub async fn atom_feed_parse(response: HttpResponse) -> Result<Feed, AtomFeedParseError> {
    let content = response
        .bytes()
        .await
        .map_err(|e| AtomFeedParseError::GetTextError(e.to_string()))?;

    let channel = parse(&content[..]).map_err(|e| AtomFeedParseError::ParseError(e))?;

    Ok(channel)
}
