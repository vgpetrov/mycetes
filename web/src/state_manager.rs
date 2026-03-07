use std::env;
use std::error::Error;
use std::sync::Arc;
use domain::repository::PlacesRepository;
use domain::stats::StatsSender;
use repository::db::db_helper::DbHelper;
use repository::db::places_db_repository::PlacesDbRepository;
use repository::mem::places_memory_repository::MemPlaceRepository;
use stats::stats_client::StatsClient;
use stats::stats_stub::StatsStub;
use crate::AppState;
use crate::queries::ListPlacesQuery;
use crate::use_cases::CreatePlaceUseCase;

pub async fn init_state() -> Result<AppState, Box<dyn Error>> {
    let stats_mock = env::var("STATS_MOCK")?.parse::<bool>()?;
    let stats_client: Arc<dyn StatsSender + Send + Sync> = if stats_mock {
        Arc::new(StatsStub::new())
    } else{
        let host = env::var("STATS_HOST")?;
        let port = env::var("STATS_PORT")?;
        Arc::new(StatsClient::new(host, port.parse::<u16>()?))
    };

    let db_mock = env::var("DB_MOCK")?.parse::<bool>()?;
    let places_repository: Arc<dyn PlacesRepository + Send + Sync> = if db_mock {
        Arc::new(MemPlaceRepository::new())
    } else {
        let db_host = env::var("DB_HOST")?;
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_name = env::var("DB_NAME")?;

        let mut db_helper = DbHelper::new(db_user, db_password, db_host, db_name);
        db_helper.init().await;
        let db_helper_arc = Arc::new(db_helper);
        Arc::new(PlacesDbRepository::new(Arc::clone(&db_helper_arc)))
    };

    let create_place_use_case = CreatePlaceUseCase::new(places_repository.clone());
    let list_places_query = ListPlacesQuery::new(places_repository.clone());

    Ok(AppState {
        stats_client,
        create_place_use_case: Arc::new(create_place_use_case),
        list_places_query: Arc::new(list_places_query),
    })
}