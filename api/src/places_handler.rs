use crate::AppState;
use crate::places_handler::AppError::DbError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json, Response};
use crate::requests::CreatePlaceRequest;
use crate::responses::PlaceResponse;

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

// pub async fn hello_handler() -> Html<&'static str> {
//     Html("<h1>Hello, World!</h1>")
// }
//
// pub async fn named_handler(Path(name): Path<String>) -> Html<String> {
//     let s = format!("<h1>Hello stats, {name}!</h1>");
//     Html(s)
// }
//
// pub async fn named_handler_stats1(
//     Path(name): Path<String>,
//     State(state): State<AppState>,
// ) -> Html<String> {
//     state.stats_client.incr();
//     let s = format!("<h1>This is stats1, {name}!</h1>");
//     Html(s)
// }
//
// pub async fn named_handler_stats2(
//     Path(name): Path<String>,
//     State(state): State<AppState>,
// ) -> Html<String> {
//     let s = format!("<h1>This is stats2, {name}!</h1>");
//     Html(s)
// }

pub async fn create_place(
    State(state): State<AppState>,
    Json(payload): Json<CreatePlaceRequest>,
) -> Result<impl IntoResponse, AppError> {
    let create_place_command = payload.into();

    state
        .create_place_use_case
        .create_place(create_place_command)
        .await
        .map_err(|e| DbError(e.to_string()))?;

    Ok((StatusCode::CREATED, String::from("Done saved!")))
}

pub async fn list_places(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let places_list: Vec<PlaceResponse> = state
        .list_places_query
        .list_places()
        .await
        .map_err(|e| DbError(e.to_string()))?
        .iter()
        .map(|place| place.into())
        .collect();

    Ok((StatusCode::OK, Json(places_list)))
}
