use std::error::Error;
use std::sync::Arc;
use repository::{Place, PlacesRepository};

pub struct ListPlacesQuery {
    place_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>,
}

impl ListPlacesQuery {
    pub fn new(place_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>) -> Self {
        ListPlacesQuery {
            place_repository
        }
    }

    pub async fn list_places(&self) -> Result<Vec<Place>, Box<dyn Error>> {
        self.place_repository
            .list_places()
            .await
    }
}
