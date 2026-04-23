pub mod app_errors;
mod commands;
mod queries;
mod requests;
mod responses;
mod spot_handler;
mod state_manager;
mod use_cases;

use crate::queries::ListSpotsQuery;
use crate::spot_handler::{create_spot, list_spot};
use crate::use_cases::CreateSpotUseCase;
use crate::use_cases::create_user_usecase::CreateUserUseCase;
use axum::Router;
use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use domain::stats::StatsSender;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use infrastructure::repository::db;

#[derive(Clone)]
struct AppState {
    stats_client: Arc<dyn StatsSender + Send + Sync>,
    create_spot_use_case: Arc<CreateSpotUseCase>,
    list_spots_query: Arc<ListSpotsQuery>,
    create_user_use_case: Arc<CreateUserUseCase>,
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
    let run_server_time = Instant::now();
    let app_state = state_manager::init_state().await?;

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &http::Request<Body>| {
            let request_id = request
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");

            let matched_path = request
                .extensions()
                .get::<axum::extract::MatchedPath>()
                .map(|p| p.as_str().to_string())
                .unwrap_or_else(|| request.uri().path().to_string());

            tracing::info_span!(
                "http_request",
                method = %request.method(),
                path = %matched_path,
                request_id = %request_id,
            )
        })
        .on_response(
            |response: &http::Response<_>, latency: Duration, _span: &tracing::Span| {
                tracing::info!(
                    status = %response.status(),
                    latency_ms = latency.as_millis(),
                    "request finished"
                );
            },
        )
        .on_failure(());

    let app = Router::new()
        // .route("/", get(hello_handler))
        // .route("/{name}", get(move |name| named_handler(name)))
        // .route("/stats1/{name}", get(named_handler_stats1))
        // .route("/stats2/{name}", get(named_handler_stats2))
        .route(
            "/spots/create",
            post(create_spot).layer(DefaultBodyLimit::max(10 * 1024 * 1024)), // 10Mb
        )
        .route("/spots/list", get(list_spot))
        .with_state(app_state)
        .layer(trace_layer);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3002)).await?;
    info!("Run server time: {} ms, listening on {}", run_server_time.elapsed().as_millis(), listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing(log_level: &str) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    let json_output = env::var("LOG_JSON")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let fmt_layer = if json_output {
        fmt::layer().json().boxed()
    } else {
        fmt::layer().pretty().boxed()
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let migration_time = Instant::now();

    match dotenv() {
        Ok(_) => {}
        Err(e) => {
            return Err("Failed to load .env file".into());
        }
    }

    init_tracing("info");

    let is_migration_enabled = env::var("ENABLE_MIGRATION")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    if is_migration_enabled {
        run_migration().await?;
    }

    info!("Db migration complete, time: {} ms", migration_time.elapsed().as_millis());
    run_server().await?;

    Ok(())
}
