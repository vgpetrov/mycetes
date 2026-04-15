use domain::repository::UserRepository;
use domain::stats::StatsSender;
use std::sync::Arc;

pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    stats_client: Arc<dyn StatsSender + Send + Sync>,
}

impl CreateUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        stats_client: Arc<dyn StatsSender + Send + Sync>,
    ) -> Self {
        Self {
            user_repository,
            stats_client,
        }
    }
}
