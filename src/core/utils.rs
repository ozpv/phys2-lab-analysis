use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Path cannot contain parent traversal or backslash")]
    InvalidPath,
    #[error("The path was not found")]
    NotFound,
}

#[inline]
pub fn validate_path(path: impl AsRef<Path>) -> Result<(), PathError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(PathError::NotFound);
    }
    if !path.iter().all(|seg| seg != "\\" && seg != "..") {
        return Err(PathError::InvalidPath);
    }
    Ok(())
}
