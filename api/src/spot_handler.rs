use crate::AppState;
use crate::app_errors::AppError;
use crate::requests::CreateSpotRequest;
use crate::responses::SpotResponse;
use crate::spot_handler::AppError::DbError;
use crate::use_cases::CreateSpotError;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json, Response};

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
    // Json(payload): Json<CreateSpotRequest>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {

    let mut name = None;
    let mut user = None;
    let mut latitude = None;
    let mut longitude = None;
    let mut metadata = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::MultipartError(e.to_string()))?
    {
        let field_name = field.name().unwrap().to_string();
        match field_name.as_str() {
            "name" => name = Some(field.text().await.unwrap()),
            "user" => user = Some(field.text().await.unwrap().parse::<i64>().unwrap()),
            "latitude" => latitude = Some(field.text().await.unwrap().parse::<f64>().unwrap()),
            "longitude" => longitude = Some(field.text().await.unwrap().parse::<f64>().unwrap()),
            "metadata" => metadata = Some(field.text().await.unwrap()),
            _ => {}
        }
    };

    let payload = CreateSpotRequest {
        name: name.unwrap(),
        user: user.unwrap(),
        latitude: latitude.unwrap(),
        longitude: longitude.unwrap(),
        metadata: metadata
    };

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
