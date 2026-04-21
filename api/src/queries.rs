use domain::file_storage::FileStorage;
use domain::repository::SpotsRepository;
use std::error::Error;
use std::sync::Arc;

pub struct ListSpotsQuery {
    spot_repository: Arc<dyn SpotsRepository + Send + Sync>,
    file_storage: Arc<dyn FileStorage + Send + Sync>,
}

impl ListSpotsQuery {
    pub fn new(
        spot_repository: Arc<dyn SpotsRepository + Send + Sync>,
        file_storage: Arc<dyn FileStorage + Send + Sync>,
    ) -> Self {
        ListSpotsQuery {
            spot_repository,
            file_storage,
        }
    }

    pub async fn list_spots(&self) -> Result<Vec<domain::Spot>, Box<dyn Error>> {
        self.spot_repository.list_spots().await
    }
}
