use url::Url;

/// Parses a URL string into a URL object.
///
/// # Arguments
/// * `s` - The URL string to parse
///
/// # Returns
/// The parsed URL if successful
///
/// # Errors
/// This function will return an error if:
/// * The URL string is malformed
/// * The URL scheme is not supported
/// * The URL contains invalid characters
pub fn parse_url(s: &str) -> Result<Url, String> {
    Url::parse(s).map_err(|e| format!("{e}"))
}

/// Validates and parses a Telegram username.
///
/// A valid Telegram username must:
/// - Be between 5-32 characters long
/// - Contain only letters, numbers, and underscores
///
/// # Arguments
/// * `name` - The username to validate
///
/// # Returns
/// The validated username as a String if valid
///
/// # Errors
/// This function will return an error if:
/// * Username is less than 5 characters
/// * Username is more than 32 characters
/// * Username contains invalid characters (anything other than letters, numbers, underscores)
pub fn parse_telegram_username(name: &str) -> Result<String, String> {
    if name.len() > 32 {
        return Err(format!(
            "{name} is too long for a nickname; it must be less than 32 characters."
        ));
    }

    if name.len() < 5 {
        return Err(format!(
            "{name} is too short for a nickname; it must be at least 5 characters."
        ));
    }

    let has_invalid_chars = name
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '_'));

    if has_invalid_chars {
        return Err(format!(
            "{name} is an invalid nickname. It must contain only letters, numbers, and underscores."
        ));
    }

    Ok(name.to_string())
}
