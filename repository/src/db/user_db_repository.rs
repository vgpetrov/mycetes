use crate::db::db_helper::DbHelper;
use async_trait::async_trait;
use domain::User;
use domain::repository::UserRepository;
use std::error::Error;
use std::sync::Arc;

pub struct UserDbRepository {
    db_helper: Arc<DbHelper>,
}

impl UserDbRepository {
    pub fn new(db_helper: Arc<DbHelper>) -> Self {
        Self { db_helper }
    }
}

#[async_trait]
impl UserRepository for UserDbRepository {
    async fn find_user(&self) -> Result<User, Box<dyn Error>> {
        todo!()
    }

    async fn save(&self, user: User) -> Result<User, Box<dyn Error>> {
        todo!()
    }
}
