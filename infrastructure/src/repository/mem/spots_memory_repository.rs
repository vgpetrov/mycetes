use crate::SpotEntity;
use async_trait::async_trait;
use domain::repository::SpotsRepository;
use std::error::Error;
use std::sync::{Arc, Mutex};
use sqlx::types::chrono::{DateTime, Utc};

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
        let next_id = (self.datasource.lock().unwrap().len() as i64) + 1;

        let metadata = match spot.metadata {
            None => { String::from("{}") }
            Some(meta) => { meta }
        };

        let spot_entity = SpotEntity {
            id: next_id,
            pub_id: spot.pub_id,
            name: spot.name,
            user_id: spot.user_id,
            latitude: spot.latitude,
            longitude: spot.longitude,
            created_at: DateTime::<Utc>::from(spot.created_at),
            updated_at: DateTime::<Utc>::from(spot.updated_at),
            metadata,
            approved_by: None,
            deleted: false,
        };

        self.datasource.lock().unwrap().push(spot_entity);

        let spot = self.datasource.lock().unwrap().last().unwrap().clone();
        Ok(spot.into())
    }
}
