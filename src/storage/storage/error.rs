use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("No current session found")]
    NoCurrentSession,
    #[error("Failed to save data: {0}")]
    SaveFailed(String),
    #[error("Failed to load data: {0}")]
    LoadFailed(String),
}

pub type StorageResult<T> = Result<T, StorageError>;
