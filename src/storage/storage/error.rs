use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("No active session found")]
    NoActiveSession,
    #[error("Failed to save data: {0}")]
    SaveFailed(String),
    #[error("failed to load data: {0}")]
    LoadFailed(String),
}

pub type StorageResult<T> = Result<T, StorageError>;
