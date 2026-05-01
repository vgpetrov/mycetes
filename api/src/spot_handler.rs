use crate::AppState;
use crate::app_errors::AppError;
use crate::requests::{CreateSpotRequest, PhotoRequest};
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
    let mut name: Option<String> = None;
    let mut user: Option<i64> = None;
    let mut latitude: Option<f64> = None;
    let mut longitude: Option<f64> = None;
    let mut metadata: Option<String> = None;

    let mut photos: Vec<PhotoRequest> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::MultipartError(e.to_string()))?
    {
        let field_name = field.name().unwrap().to_string();
        // TODO: write proper validation
        match field_name.as_str() {
            "name" => name = Some(field.text().await.unwrap()),
            "user" => user = Some(field.text().await.unwrap().parse::<i64>().unwrap()),
            "latitude" => latitude = Some(field.text().await.unwrap().parse::<f64>().unwrap()),
            "longitude" => longitude = Some(field.text().await.unwrap().parse::<f64>().unwrap()),
            "metadata" => metadata = Some(field.text().await.unwrap()),
            "photos" => {
                let file_name = field
                    .file_name()
                    .ok_or_else(|| {
                        AppError::ValidationError("Photo filename is missing".to_string())
                    })?
                    .to_string();

                let content_type = field
                    .content_type()
                    .ok_or_else(|| {
                        AppError::ValidationError("Photo content type is missing".to_string())
                    })?
                    .to_string();

                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::MultipartError(e.to_string()))?;

                let handle = tokio::task::spawn_blocking(move || {
                    let thumbnail_result = PhotoRequest::create_thumbnail(&bytes);

                    let photo_result =
                        PhotoRequest::validate_photo(file_name, content_type, bytes.to_vec());

                    (thumbnail_result, photo_result)
                })
                .await;

                let (thumbnail, photo) = handle.unwrap();
                let thumbnail = thumbnail.map_err(|e| AppError::MultipartError(e.to_string()))?;
                let (photo_file_name, photo_content_type, photo_bytes) = photo.map_err(|e| e)?;

                photos.push(PhotoRequest {
                    file_name: photo_file_name,
                    content_type: photo_content_type,
                    thumbnail_bytes: thumbnail,
                    bytes: photo_bytes,
                });
            }
            _ => {}
        }
    }

    let payload = CreateSpotRequest {
        name: name.unwrap(),
        user: user.unwrap(),
        latitude: latitude.unwrap(),
        longitude: longitude.unwrap(),
        metadata,
        photos,
    };

    let create_spot_command = payload.into();
    state
        .create_spot_use_case
        .create_spot(create_spot_command)
        .await
        .map_err(|e| match e {
            CreateSpotError::Db(msg) => AppError::DbError(msg),
            CreateSpotError::Validation(msg) => AppError::ValidationError(msg),
            CreateSpotError::FileStorage(msg) => AppError::MultipartError(msg),
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
