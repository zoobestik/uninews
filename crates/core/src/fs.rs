use std::env;
use std::path::{Path, PathBuf};
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

/// Returns the path to the `SQLite` database file.
///
/// Uses the `UNINEWS_DB_PATH` environment variable if set, otherwise defaults to "data/app.sqlite".
///
/// # Returns
/// The path to the `SQLite` database file.
///
/// # Errors
/// This function will return an error if:
/// * The path in `UNINEWS_DB_PATH` is invalid
pub fn get_db_path() -> Result<PathBuf, String> {
    let db_path = env::var("UNINEWS_DB_PATH").unwrap_or_else(|_| String::from("data/app.sqlite"));
    Ok(PathBuf::from(db_path))
}

/// Formats a database URI string from a path.
///
/// Adds `?mode=rwc` query parameter to allow read/write access with automatic database creation.
///
/// # Arguments
/// * `db_file` - Path to the `SQLite` database file
///
/// # Returns
/// A URI string in the format `sqlite:path?mode=rwc`
#[must_use]
pub fn to_db_uri(db_file: &Path) -> String {
    format!("sqlite:{0}?mode=rwc", db_file.display())
}

/// Returns a formatted database URI string by combining the database path with query parameters.
///
/// # Returns
/// A URI string in the format `sqlite:path?mode=rwc`
///
/// # Errors
/// This function will return an error if:
/// * Getting the database path fails (see [`get_db_path`])
pub fn get_db_uri() -> Result<String, String> {
    let db_file = get_db_path()?;
    Ok(to_db_uri(&db_file))
}
