use env::var;
use fs::{create_dir_all, write};
use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;

#[derive(Error, Debug)]
pub enum WriteToFileError {
    #[error("Failed to write to file [{0}]: {1}")]
    Write(String, #[source] std::io::Error),

    #[error(transparent)]
    CreateParentDirs(CreateParentDirsError),
}

pub async fn write_to_file(path_string: &str, content: &str) -> Result<(), WriteToFileError> {
    let path = Path::new(path_string);

    create_parent_dirs(path)
        .await
        .map_err(WriteToFileError::CreateParentDirs)?;

    write(path, content)
        .await
        .map_err(|e| WriteToFileError::Write(path.display().to_string(), e))?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum CreateParentDirsError {
    #[error("No parent directory for path: {0}")]
    NoParent(String),

    #[error("Failed to create directory {0}")]
    CreateDir(String, #[source] std::io::Error),
}

pub async fn create_parent_dirs(path: &Path) -> Result<&Path, CreateParentDirsError> {
    let parent = path
        .parent()
        .ok_or_else(|| CreateParentDirsError::NoParent(path.display().to_string()))?;

    create_dir_all(parent)
        .await
        .map_err(|e| CreateParentDirsError::CreateDir(path.display().to_string(), e))?;

    Ok(parent)
}

pub fn get_db_path() -> PathBuf {
    PathBuf::from(var("UNINEWS_DB_PATH").unwrap_or_else(|_| String::from("data/app.sqlite")))
}

#[must_use]
pub fn to_db_uri(db_file: &Path) -> String {
    format!("sqlite:{0}?mode=rwc", db_file.display())
}

pub fn get_db_uri() -> String {
    let db_file = get_db_path();
    to_db_uri(&db_file)
}
