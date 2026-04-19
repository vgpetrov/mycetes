use crate::AppState;
use crate::requests::CreateSpotRequest;
use crate::responses::SpotResponse;
use crate::spot_handler::AppError::DbError;
use crate::use_cases::CreateSpotError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json, Response};
use tracing::log;
use crate::app_errors::AppError;

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

pub async fn create_spot(
    State(state): State<AppState>,
    Json(payload): Json<CreateSpotRequest>,
) -> Result<impl IntoResponse, AppError> {
    let create_spot_command = payload.into();

    state
        .create_spot_use_case
        .create_spot(create_spot_command)
        .await
        .map_err(|e| match e {
            CreateSpotError::Db(msg) => AppError::DbError(msg),
            CreateSpotError::Validation(msg) => AppError::ValidationError(msg),
        })?;

    state.stats_client.incr("create_spot", vec![]);

    Ok((StatusCode::CREATED, String::from("Ok")))
}

pub async fn list_spot(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let spots_list: Vec<SpotResponse> = state
        .list_spots_query
        .list_spots()
        .await
        .map_err(|e| DbError(e.to_string()))?
        .iter()
        .map(|spot| spot.into())
        .collect();

    Ok((StatusCode::OK, Json(spots_list)))
}
