#[must_use]
pub fn truncate_with_dots(text: &str, n: usize) -> String {
    if text.chars().count() <= n {
        return text.to_string();
    }

    let result: String = text.chars().take(n).collect();
    let mut result = result.trim();

    let is_text_tail = text
        .chars()
        .nth(result.chars().count())
        .take_if(|c| c.is_alphanumeric())
        .is_some();

    if is_text_tail {
        result = result
            .trim_end_matches(|a: char| a.is_alphanumeric())
            .trim_end_matches(|a: char| !a.is_alphanumeric());
    }

    format!("{}…", result.trim_end_matches('.'))
}
