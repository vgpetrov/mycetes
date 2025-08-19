use std::error::Error;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::{Place, PlacesRepository};

pub struct MemPlaceRepository {
    datasource: Arc<Mutex<Vec<Place>>>,
}

impl MemPlaceRepository {
    pub fn new() -> Self {
        Self { datasource: Arc::new(Mutex::new(Vec::new()))}
    }
}

#[async_trait]
impl PlacesRepository for MemPlaceRepository {
    async fn list_places(&self) -> Result<Vec<Place>, Box<dyn Error>> {
        Ok(self.datasource.lock().unwrap().to_vec())
    }

    async fn save(&self, place: Place) -> Result<Place, Box<dyn Error>> {
        self
            .datasource
            .lock()
            .unwrap()
            .push(place);

        Ok(self.datasource.lock().unwrap().last().unwrap().clone())
    }
}