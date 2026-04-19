use crate::SpotEntity;
use async_trait::async_trait;
use domain::repository::SpotsRepository;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct MemSpotRepository {
    datasource: Arc<Mutex<Vec<SpotEntity>>>,
}

impl MemSpotRepository {
    pub fn new() -> Self {
        Self {
            datasource: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl SpotsRepository for MemSpotRepository {
    async fn list_spots(&self) -> Result<Vec<domain::Spot>, Box<dyn Error>> {
        let vec = self.datasource.lock().unwrap().to_vec();
        let result = vec.iter().map(|p| p.into()).collect();
        Ok(result)
    }

    async fn save(&self, spot: domain::Spot) -> Result<domain::Spot, Box<dyn Error>> {
        self.datasource.lock().unwrap().push(spot.into());

        let spot = self.datasource.lock().unwrap().last().unwrap().clone();
        Ok(spot.into())
    }
}
