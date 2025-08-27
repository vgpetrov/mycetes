use std::error::Error;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use domain::repository::PlacesRepository;
use crate::{Place};

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
    async fn list_places(&self) -> Result<Vec<domain::Place>, Box<dyn Error>> {
        let vec = self.datasource.lock().unwrap().to_vec();
        let result = vec.iter().map(|p| p.into()).collect();
        Ok(result)
    }

    async fn save(&self, place: domain::Place) -> Result<domain::Place, Box<dyn Error>> {
        self.datasource
            .lock()
            .unwrap()
            .push(place.into());

        let place = self.datasource
            .lock()
            .unwrap()
            .last()
            .unwrap()
            .clone();
        Ok(place.into())
    }
}