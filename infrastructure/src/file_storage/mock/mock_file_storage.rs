use async_trait::async_trait;
use domain::file_storage::{FileStorage, FileStorageError};
use domain::Photo;

pub struct MockFileStorage {}

impl MockFileStorage {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl FileStorage for MockFileStorage {
    async fn store(&self, group: &str, photo: Photo) -> Result<Photo, FileStorageError> {
        Ok(Photo {
            spot_pub_id: None,
            storage_key: None,
            file_name: "".to_string(),
            content_type: "".to_string(),
            bytes: None,
        })
    }

    async fn load(&self) -> Result<(), FileStorageError> {
        Ok(())
    }
}