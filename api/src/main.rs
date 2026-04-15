mod commands;
mod spot_handler;
mod queries;
mod requests;
mod responses;
mod state_manager;
mod use_cases;

use crate::spot_handler::{create_spot, list_spot};
use crate::queries::ListSpotsQuery;
use axum::Router;
use axum::routing::{get, post};
use domain::stats::StatsSender;
use dotenv::dotenv;
use repository::db;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tracing::info;
use crate::use_cases::create_user_usecase::CreateUserUseCase;
use crate::use_cases::CreateSpotUseCase;

#[derive(Clone)]
struct AppState {
    stats_client: Arc<dyn StatsSender + Send + Sync>,
    create_spot_use_case: Arc<CreateSpotUseCase>,
    list_spots_query: Arc<ListSpotsQuery>,
    create_user_use_case: Arc<CreateUserUseCase>
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
    let app_state = state_manager::init_state().await?;

    let app = Router::new()
        // .route("/", get(hello_handler))
        // .route("/{name}", get(move |name| named_handler(name)))
        // .route("/stats1/{name}", get(named_handler_stats1))
        // .route("/stats2/{name}", get(named_handler_stats2))
        .route("/spots/create", post(create_spot))
        .route("/spots/list", get(list_spot))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3002)).await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_logs() {
    tracing_subscriber::fmt()
        .with_env_filter("info") // or RUST_LOG env var
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    init_logs();
    
    let is_migration_enabled = env::var("ENABLE_MIGRATION")?.parse::<bool>()?;
    if is_migration_enabled {
        run_migration().await?;
    }

    run_server().await?;

    Ok(())
}
