use crate::AppState;
use crate::queries::ListSpotsQuery;
use crate::use_cases::create_user_usecase::CreateUserUseCase;
use crate::use_cases::{CreateSpotUseCase, create_user_usecase};
use domain::repository::{SpotsRepository, UserRepository};
use domain::stats::StatsSender;
use repository::db::db_helper::DbHelper;
use repository::db::spots_db_repository::SpotsDbRepository;
use repository::db::user_db_repository::UserDbRepository;
use repository::mem::spots_memory_repository::MemSpotRepository;
use repository::mem::user_memory_repository::UserMemoryRepository;
use stats::stats_client::StatsClient;
use stats::stats_stub::StatsStub;
use std::env;
use std::error::Error;
use std::sync::Arc;

pub async fn init_state() -> Result<AppState, Box<dyn Error>> {
    let stats_mock = env::var("STATS_MOCK")?.parse::<bool>()?;
    let stats_client: Arc<dyn StatsSender + Send + Sync> = if stats_mock {
        Arc::new(StatsStub::new())
    } else {
        let host = env::var("STATS_HOST")?;
        let port = env::var("STATS_PORT")?;
        Arc::new(StatsClient::new(host, port.parse::<u16>()?))
    };

    let db_mock = env::var("DB_MOCK")?.parse::<bool>()?;
    let repositories: (
        Arc<dyn SpotsRepository + Send + Sync>,
        Arc<dyn UserRepository + Send + Sync>,
    ) = if !db_mock {
        let db_host = env::var("DB_HOST")?;
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_name = env::var("DB_NAME")?;

        let mut db_helper = DbHelper::new(db_user, db_password, db_host, db_name);
        db_helper.init().await?;
        let db_helper_arc = Arc::new(db_helper);

        (
            Arc::new(SpotsDbRepository::new(Arc::clone(&db_helper_arc))),
            Arc::new(UserDbRepository::new(Arc::clone(&db_helper_arc))),
        )
    } else {
        (
            Arc::new(MemSpotRepository::new()),
            Arc::new(UserMemoryRepository::new()),
        )
    };

    let create_spot_use_case = CreateSpotUseCase::new(repositories.0.clone());
    let list_spots_query = ListSpotsQuery::new(repositories.0.clone());
    let create_user_use_case = CreateUserUseCase::new(
        repositories.1.clone(),
        stats_client.clone()
    );

    Ok(AppState {
        stats_client,
        create_spot_use_case: Arc::new(create_spot_use_case),
        list_spots_query: Arc::new(list_spots_query),
        create_user_use_case: Arc::new(create_user_use_case),
    })
}
