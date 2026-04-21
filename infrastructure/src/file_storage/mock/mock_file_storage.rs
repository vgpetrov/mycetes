use async_trait::async_trait;
use domain::file_storage::{FileStorage, FileStorageError};

pub struct MockFileStorage {}

impl MockFileStorage {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl FileStorage for MockFileStorage {
    async fn store(&self) -> Result<(), FileStorageError> {
        Ok(())
    }

    async fn load(&self) -> Result<(), FileStorageError> {
        Ok(())
    }
}