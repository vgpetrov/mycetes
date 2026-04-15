use std::error::Error;
use std::sync::Arc;
use domain::repository::SpotsRepository;

pub struct ListSpotsQuery {
    spot_repository: Arc<dyn SpotsRepository + Send + Sync>,
}

impl ListSpotsQuery {
    pub fn new(spot_repository: Arc<dyn SpotsRepository + Send + Sync>) -> Self {
        ListSpotsQuery {
            spot_repository
        }
    }

    pub async fn list_spots(&self) -> Result<Vec<domain::Spot>, Box<dyn Error>> {
        self.spot_repository
            .list_spots()
            .await
    }
}
