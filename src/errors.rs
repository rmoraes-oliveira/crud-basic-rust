use axum::{http::StatusCode, response::{IntoResponse, Response, Json}};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "resource not found".to_string()),
            AppError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {}", e),
            ),
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}
