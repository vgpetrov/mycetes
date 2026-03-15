use std::error::Error;
use async_trait::async_trait;
use crate::{Place, User};

#[async_trait]
pub trait PlacesRepository: Send + Sync {
    async fn list_places(&self) -> Result<Vec<Place>, Box<dyn Error>>;
    async fn save(&self, place: Place) -> Result<Place, Box<dyn Error>>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_user(&self) -> Result<User, Box<dyn Error>>;
    
    async fn save(&self, user: User) -> Result<User, Box<dyn Error>>;
}