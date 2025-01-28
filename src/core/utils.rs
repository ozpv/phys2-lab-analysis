use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Path cannot contain parent traversal or backslash")]
    InvalidPath,
    #[error("The path was not found")]
    NotFound,
}

#[inline(always)]
pub fn validate_path(path: &PathBuf) -> Result<(), PathError> {
    if !path.exists() {
        return Err(PathError::NotFound);
    }
    if !path.iter().all(|seg| seg != "\\" && seg != "..") {
        return Err(PathError::InvalidPath);
    }
    Ok(())
}
