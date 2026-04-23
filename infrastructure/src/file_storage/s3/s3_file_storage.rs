use std::sync::Arc;
use async_trait::async_trait;
use domain::file_storage::{FileStorage, FileStorageError};
use crate::file_storage::s3::garage_client::GarageClient;

pub struct S3FileStorage {
    garage_client: Arc<GarageClient>,
}

impl S3FileStorage {
    pub fn new(client: Arc<GarageClient>) -> Self {
        Self {
            garage_client: client
        }
    }
}

#[async_trait]
impl FileStorage for S3FileStorage {

    async fn store(&self) -> Result<(), FileStorageError> {
        self.garage_client.get_client()
            .map_err(|e|FileStorageError::Store(e.to_string()))?
            .put_object();

        todo!()
    }

    async fn load(&self) -> Result<(), FileStorageError> {
        todo!()
    }
}