mod places_handler;

use std::env;
use std::error::Error;
use axum::{Router};
use axum::routing::{get, post};
use std::sync::Arc;
use dotenv::dotenv;
use repository::{db, PlacesRepository};
use repository::db::db_helper::DbHelper;
use repository::db::places_db_repository::PlacesDbRepository;
use repository::mem::places_memory_repository::MemPlaceRepository;
use stats::stats_client::StatsClient;
use stats::stats_stub::StatsStub;
use stats::StatsSender;
use crate::places_handler::{hello_handler, list_places, named_handler, named_handler_stats1, named_handler_stats2, save_place};

#[derive(Clone)]
struct AppState {
    stats_client: Arc<Box<dyn StatsSender + Send + Sync>>,
    places_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>,
}

async fn init_state() -> Result<AppState, Box<dyn Error>> {
    let stats_mock = env::var("STATS_MOCK")?.parse::<bool>()?;
    let stats_client: Box<dyn StatsSender + Send + Sync> = if stats_mock {
        Box::new(StatsStub::new())
    } else{
        let host = env::var("STATS_HOST")?;
        let port = env::var("STATS_PORT")?;
        Box::new(StatsClient::new(host, port.parse::<u16>()?))
    };

    let db_mock = env::var("DB_MOCK")?.parse::<bool>()?;
    let places_repository: Box<dyn PlacesRepository + Send + Sync> = if db_mock {
        Box::new(MemPlaceRepository::new())
    } else {
        let db_host = env::var("DB_HOST")?;
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_name = env::var("DB_NAME")?;

        let mut db_helper = DbHelper::new(db_user, db_password, db_host, db_name);
        db_helper.init().await;
        let db_helper_arc = Arc::new(db_helper);
        Box::new(PlacesDbRepository::new(Arc::clone(&db_helper_arc)))
    };

    Ok(AppState {
        stats_client: Arc::new(stats_client),
        places_repository: Arc::new(places_repository),
    })
}

async fn run_migration() -> Result<(), Box<dyn Error>> {
    let db_host = env::var("DB_HOST")?;
    let db_user = env::var("DB_USER")?;
    let db_password = env::var("DB_PASSWORD")?;
    let db_name = env::var("DB_NAME")?;

    db::migration::migrate_files(db_user, db_password, db_host, db_name).await?;
    Ok(())
}

async fn run_server() -> Result<(), Box<dyn Error>> {
    let app_state = init_state().await?;

    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/{name}", get(move |name| named_handler(name)))
        .route("/stats1/{name}", get(named_handler_stats1))
        .route("/stats2/{name}", get(named_handler_stats2))
        .route("/places/save", post(save_place))
        .route("/places/list", get(list_places))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3002))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let is_migration_enabled = env::var("ENABLE_MIGRATION")?.parse::<bool>()?;
    if is_migration_enabled {
        run_migration().await?;
    }

    run_server().await?;

    Ok(())
}