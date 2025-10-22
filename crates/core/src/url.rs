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
