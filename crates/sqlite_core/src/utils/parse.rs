use thiserror::Error;
use url::Url;

pub fn parse_url(s: &str) -> Result<Url, String> {
    Url::parse(s).map_err(|e| format!("{e}"))
}

#[derive(Error, Debug)]
pub enum ParseTelegramNameError {
    #[error("{0} is too long for a nickname; it must be less than 32 characters.")]
    TooLong(String),

    #[error("{0} is too short for a nickname; it must be at least 5 characters.")]
    TooShort(String),

    #[error("{0} is an invalid nickname. It must contain only letters, numbers, and underscores.")]
    InvalidChars(String),
}

pub fn parse_telegram_username(name: &str) -> Result<String, ParseTelegramNameError> {
    let username = name.to_string();

    if name.len() > 32 {
        return Err(ParseTelegramNameError::TooLong(username));
    }

    if name.len() < 5 {
        return Err(ParseTelegramNameError::TooShort(username));
    }

    if name
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '_'))
    {
        return Err(ParseTelegramNameError::InvalidChars(username));
    }

    Ok(username)
}
