use axum::Router;
use axum::extract::Path;
use axum::response::Html;
use axum::routing::get;
use std::sync::Arc;
use statsd::Client;

#[tokio::main]
async fn main() {
    let statsd_client = Client::new("telegraf:8125", "mycetes.request").unwrap();
    let arc_statsd = Arc::new(statsd_client);

    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/:name", get(move |name| named_handler(name)))
        .route(
            "/stats1/:name",
            get({
                let arc = Arc::clone(&arc_statsd);
                move |name| named_handler_stats(name, arc)
            }),
        )
        .route(
            "/stats2/:name",
            get({
                let arc = Arc::clone(&arc_statsd);
                move |name| named_handler_stats(name, arc)
            }),
        );

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3002))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn named_handler(Path(name): Path<String>) -> Html<String> {
    let s = format!("<h1>Hello, {name}!</h1>");
    Html(s)
}

async fn named_handler_stats(Path(name): Path<String>, statsd: Arc<Client>) -> Html<String> {
    statsd.incr("count");
    let s = format!("<h1>This is stats, {name}!</h1>");
    Html(s)
}
