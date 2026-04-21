use crate::AppState;
use crate::queries::ListSpotsQuery;
use crate::use_cases::create_user_usecase::CreateUserUseCase;
use crate::use_cases::{CreateSpotUseCase, create_user_usecase};
use domain::file_storage::FileStorage;
use domain::repository::{SpotsRepository, UserRepository};
use domain::stats::StatsSender;
use infrastructure::file_storage::mock::mock_file_storage::MockFileStorage;
use infrastructure::file_storage::s3::s3_file_storage::S3FileStorage;
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

    let stats_client = init_stats()?;
    let repositories = init_repositories().await?;
    let file_storage = init_file_storage()?;

    let create_spot_use_case = CreateSpotUseCase::new(repositories.0.clone(), Arc::clone(&file_storage));
    let list_spots_query = ListSpotsQuery::new(repositories.0.clone(), Arc::clone(&file_storage));
    let create_user_use_case = CreateUserUseCase::new(repositories.1.clone(), Arc::clone(&stats_client));

    Ok(AppState {
        stats_client,
        create_spot_use_case: Arc::new(create_spot_use_case),
        list_spots_query: Arc::new(list_spots_query),
        create_user_use_case: Arc::new(create_user_use_case),
    })
}


fn init_file_storage() -> Result<Arc<dyn FileStorage + Send + Sync>, Box<dyn Error>> {
    let file_storage_mock = env::var("FILE_STORAGE_MOCK")?.parse::<bool>()?;
    if file_storage_mock {
        Ok(Arc::new(MockFileStorage::new()))
    } else {
        Ok(Arc::new(S3FileStorage::new()))
    }
}

async fn init_repositories() -> Result<(Arc<dyn SpotsRepository + Send + Sync>, Arc<dyn UserRepository + Send + Sync>), Box<dyn Error>> {
    let db_mock = env::var("DB_MOCK")?.parse::<bool>()?;
    if !db_mock {
        let db_host = env::var("DB_HOST")?;
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_name = env::var("DB_NAME")?;

        let mut db_helper = DbHelper::new(db_user, db_password, db_host, db_name);
        db_helper.init().await?;
        let db_helper_arc = Arc::new(db_helper);

        Ok((
            Arc::new(SpotsDbRepository::new(Arc::clone(&db_helper_arc))),
            Arc::new(UserDbRepository::new(Arc::clone(&db_helper_arc))),
        ))
    } else {
        Ok((
            Arc::new(MemSpotRepository::new()),
            Arc::new(UserMemoryRepository::new()),
        ))
    }
}

fn init_stats() -> Result<Arc<dyn StatsSender + Send + Sync>, Box<dyn Error>> {
    let stats_mock = env::var("STATS_MOCK")?.parse::<bool>()?;
    if stats_mock {
        Ok(Arc::new(StatsStub::new()))
    } else {
        let host = env::var("STATS_HOST")?;
        let port = env::var("STATS_PORT")?.parse::<u16>()?;
        Ok(Arc::new(StatsClient::new(host, port)))
    }
}