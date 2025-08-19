use crate::AppState;
use crate::places_handler::AppError::DbError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response, Json};
use repository::Place;

#[derive(Debug)]
pub enum AppError {
    DbError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DbError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn named_handler(Path(name): Path<String>) -> Html<String> {
    let s = format!("<h1>Hello stats, {name}!</h1>");
    Html(s)
}

pub async fn named_handler_stats1(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    state.stats_client.incr();
    let s = format!("<h1>This is stats1, {name}!</h1>");
    Html(s)
}

pub async fn named_handler_stats2(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    let s = format!("<h1>This is stats2, {name}!</h1>");
    Html(s)
}

pub async fn save_place(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let place = Place {
        id: None,
        name: String::from("Moscow"),
        user: 1,
        latitude: 50.0,
        longitude: 10.0,
        is_deleted: false,
    };

    state
        .places_repository
        .save(place)
        .await
        .map_err(|e| DbError(e.to_string()))?;

    Ok(String::from("Done saved!"))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PlaceResponse {
    pub id: Option<i64>,
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn list_places(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let places_list: Vec<PlaceResponse> = state
        .places_repository
        .list_places()
        .await
        .map_err(|e| DbError(e.to_string()))?
        .iter().map(|place| {
            PlaceResponse {
                id: place.id,
                name: place.name.clone(),
                user: place.user,
                latitude: place.latitude,
                longitude: place.longitude,
            }
        }).collect();
    
    Ok(Json(places_list))
}
