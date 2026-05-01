use crate::file_storage::s3::garage_client::GarageClient;
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use domain::Photo;
use domain::file_storage::{FileStorage, FileStorageError};
use random_string::charsets::ALPHANUMERIC;
use std::ops::Add;
use std::sync::Arc;

pub struct S3FileStorage {
    garage_client: Arc<GarageClient>,
}

impl S3FileStorage {
    pub fn new(client: Arc<GarageClient>) -> Self {
        Self {
            garage_client: client,
        }
    }
}

const MYCETES_FILES_BUCKET: &'static str = "mycetes-files";
const THUMBNAIL: &'static str = "/thumbnail/";
const ORIG: &'static str = "/orig/";

#[async_trait]
impl FileStorage for S3FileStorage {
    async fn store(&self, group: &str, photo: Photo) -> Result<Photo, FileStorageError> {
        // TODO: validate not null
        let photo_bytes = Bytes::from(photo.bytes.unwrap());
        let thumbnail_bytes = Bytes::from(photo.thumbnail_bytes.unwrap());

        let client = self
            .garage_client
            .get_client()
            .map_err(|e| FileStorageError::Store(e.to_string()))?;

        let key = random_string::generate(10, ALPHANUMERIC);

        let generated_key = group.to_string().add(THUMBNAIL).add(&key);
        client
            .put_object()
            .bucket(MYCETES_FILES_BUCKET)
            .key(&generated_key)
            .content_type(&photo.content_type)
            .body(ByteStream::from(thumbnail_bytes))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Store thumbnail error: {:?}", e);
                FileStorageError::Store(e.to_string())
            })?;

        let generated_key = group.to_string().add(ORIG).add(&key);
        let output = client
            .put_object()
            .bucket(MYCETES_FILES_BUCKET)
            .key(&generated_key)
            .content_type(&photo.content_type)
            .body(ByteStream::from(photo_bytes))
            .send()
            .await;

        match output {
            Ok(x) => Ok(Photo {
                spot_pub_id: Some(String::from(group)),
                storage_key: Some(generated_key),
                file_name: photo.file_name,
                content_type: photo.content_type,
                thumbnail_bytes: None,
                bytes: None,
            }),
            Err(e) => {
                tracing::error!("Store error: {:?}", e);
                Err(FileStorageError::Store(e.to_string()))
            }
        }
    }

    async fn load(&self) -> Result<(), FileStorageError> {
        todo!()
    }
}
