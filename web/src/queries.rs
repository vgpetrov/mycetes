use std::error::Error;
use std::sync::Arc;
use domain::repository::PlacesRepository;

pub struct ListPlacesQuery {
    place_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>,
}

impl ListPlacesQuery {
    pub fn new(place_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>) -> Self {
        ListPlacesQuery {
            place_repository
        }
    }

    pub async fn list_places(&self) -> Result<Vec<domain::Place>, Box<dyn Error>> {
        self.place_repository
            .list_places()
            .await
    }
}
