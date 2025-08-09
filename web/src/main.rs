use std::env;
use std::error::Error;
use axum::{Router};
use axum::extract::{Path, State};
use axum::response::Html;
use axum::routing::get;
use std::sync::Arc;
use dotenv::dotenv;
use stats::stats_client::StatsClient;
use stats::stats_stub::StatsStub;
use stats::StatsSender;


#[derive(Clone)]
struct AppState {
    stats_client: Arc<Box<dyn StatsSender + Send + Sync>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let host = env::var("STATS_HOST")?;
    let port = env::var("STATS_PORT")?;

    let stats_client: Box<dyn StatsSender + Send + Sync> = Box::new(StatsClient::new(host, port.parse::<u16>()?));
    let arc_stats_client = Arc::new(stats_client);

    let app_state = AppState {
        stats_client: arc_stats_client,
    };

    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/{name}", get(move |name| named_handler(name)))
        .route("/stats1/{name}", get(named_handler_stats1))
        .route("/stats2/{name}", get(named_handler_stats1))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3002))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn named_handler(Path(name): Path<String>) -> Html<String> {
    let s = format!("<h1>Hello stats1, {name}!</h1>");
    Html(s)
}

async fn named_handler_stats1(Path(name): Path<String>, State(state): State<AppState>) -> Html<String> {
    state.stats_client.incr();
    let s = format!("<h1>This is stats2, {name}!</h1>");
    Html(s)
}