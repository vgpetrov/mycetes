use async_trait::async_trait;

pub enum FileStorageError {
    Store(String),
    Load(String)
}

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn store(&self) -> Result<(), FileStorageError>;

    async fn load(&self) -> Result<(), FileStorageError>;
}