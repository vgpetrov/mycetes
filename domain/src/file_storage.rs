use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use crate::Photo;

#[derive(Debug)]
pub enum FileStorageError {
    Store(String),
    Load(String)
}

impl Display for FileStorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStorageError::Store(msg) => write!(f, "Failed to store file: {}", msg),
            FileStorageError::Load(msg) => write!(f, "Failed to load file: {}", msg),
        }
    }
}

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn store(&self, group: &str, photo: Photo) -> Result<Photo, FileStorageError>;

    async fn load(&self) -> Result<(), FileStorageError>;
}