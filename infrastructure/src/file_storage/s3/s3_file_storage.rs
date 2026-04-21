use async_trait::async_trait;
use domain::file_storage::{FileStorage, FileStorageError};

pub struct S3FileStorage {

}

impl S3FileStorage {
    pub fn new() -> Self {
        Self {

        }
    }
}

#[async_trait]
impl FileStorage for S3FileStorage {

    async fn store(&self) -> Result<(), FileStorageError> {
        todo!()
    }

    async fn load(&self) -> Result<(), FileStorageError> {
        todo!()
    }
}