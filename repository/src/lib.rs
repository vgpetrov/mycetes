use std::error::Error;
use async_trait::async_trait;
use sqlx::FromRow;

pub mod db;
pub mod mem;

#[derive(Debug, FromRow, Clone)]
pub struct Place {
    pub id: Option<i64>,
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub is_deleted: bool,
}

#[async_trait]
pub trait PlacesRepository: Send + Sync {
    async fn list_places(&self) -> Result<Vec<Place>, Box<dyn Error>>;
    async fn save(&self, place: Place) -> Result<Place, Box<dyn Error>>;
}