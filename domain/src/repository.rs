use std::error::Error;
use async_trait::async_trait;
use crate::{Spot, User};

#[async_trait]
pub trait SpotsRepository: Send + Sync {
    async fn list_spots(&self) -> Result<Vec<Spot>, Box<dyn Error>>;
    async fn save(&self, spot: Spot) -> Result<Spot, Box<dyn Error>>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_user(&self) -> Result<User, Box<dyn Error>>;
    
    async fn save(&self, user: User) -> Result<User, Box<dyn Error>>;
}