use async_trait::async_trait;
use domain::User;
use domain::repository::UserRepository;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct UserMemoryRepository {
    datasource: Arc<Mutex<Vec<User>>>,
}

impl UserMemoryRepository {
    pub fn new() -> Self {
        UserMemoryRepository {
            datasource: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl UserRepository for UserMemoryRepository {
    async fn find_user(&self) -> Result<User, Box<dyn Error>> {
        todo!()
    }

    async fn save(&self, user: User) -> Result<User, Box<dyn Error>> {
        todo!()
    }
}
