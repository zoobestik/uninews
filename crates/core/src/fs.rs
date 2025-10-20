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

    create_parent_dirs(path).await?;

    fs::write(path, content)
        .await
        .map_err(|e| format!("Failed to write file [{0}]: {e}", path.display()))?;

    Ok(())
}

/// Asynchronously creates parent directories for a given path.
///
/// # Arguments
/// * `path` - The path for which to create parent directories
///
/// # Returns
/// The parent path if successful
///
/// # Errors
/// This function will return an error if:
/// * The parent directory cannot be determined
/// * Directory creation fails
pub async fn create_parent_dirs(path: &Path) -> Result<&Path, String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("Failed to get parent {0}", path.display()))?;

    fs::create_dir_all(parent)
        .await
        .map_err(|e| format!("Failed to create directory {0}: {e}", path.display()))?;

    Ok(parent)
}
