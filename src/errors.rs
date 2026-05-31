use axum::{http::StatusCode, response::{IntoResponse, Response, Json}};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    ValidationError(String),
    Unauthorized,
    JwtCreationFailed,
    PasswordHashingFailed,
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
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized - invalid or missing token".to_string(),
            ),
            AppError::JwtCreationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to create token".to_string(),
            ),
            AppError::PasswordHashingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to hash password".to_string(),
            ),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}
