use std::time::SystemTime;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use domain::Spot;

pub mod file_storage;
pub mod stats;
pub mod repository;

#[derive(Debug, FromRow, Clone)]
pub struct SpotEntity {
    pub id: i64,
    pub pub_id: String,
    pub name: String,
    pub user_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: String,
    pub approved_by: Option<i64>,
    pub deleted: bool,
}

#[derive(Debug, FromRow, Clone)]
pub struct SpotLightEntity {
    pub id: i64,
    pub pub_id: String,
}

impl From<&SpotEntity> for Spot {
    fn from(value: &SpotEntity) -> Self {
        Spot {
            pub_id: value.pub_id.clone(),
            name: value.name.clone(),
            user_id: value.user_id,
            latitude: value.latitude,
            longitude: value.longitude,
            created_at: SystemTime::from(value.created_at),
            updated_at: SystemTime::from(value.updated_at),
            metadata: Option::from(value.metadata.clone()),
            approved_by: value.approved_by,
            deleted: value.deleted
        }
    }
}

impl From<SpotEntity> for Spot {
    fn from(value: SpotEntity) -> Self {
        Spot {
            pub_id: value.pub_id.clone(),
            name: value.name.clone(),
            user_id: value.user_id,
            latitude: value.latitude,
            longitude: value.longitude,
            created_at: SystemTime::from(value.created_at),
            updated_at: SystemTime::from(value.updated_at),
            metadata: Some(value.metadata.clone()),
            approved_by: value.approved_by,
            deleted: value.deleted
        }
    }
}



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
