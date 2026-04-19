use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    DbError(String),
    ValidationError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: message }),
            ).into_response(),

            AppError::DbError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: message }),
            ).into_response(),
        }
    }
}