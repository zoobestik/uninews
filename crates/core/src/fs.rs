use std::path::Path;
use tokio::fs;

/// Asynchronously writes content to a file at the specified path, creating parent directories if needed.
///
/// # Arguments
/// * `path_string` - The path where the file should be written
/// * `content` - The content to write to the file
///
/// # Errors
/// This function will return an error if:
/// * Parent directory cannot be determined or created
/// * Writing the file fails
pub async fn write_to_file(path_string: &str, content: &str) -> Result<(), String> {
    let path = Path::new(path_string);

    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| format!("Failed to get parent {0}", path.display()))?,
    )
    .await
    .map_err(|e| format!("Failed to create directory {0}: {e}", path.display()))?;

    fs::write(path, content)
        .await
        .map_err(|e| format!("Failed to write file [{0}]: {e}", path.display()))?;

    Ok(())
}
