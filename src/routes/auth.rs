use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;
use tracing::{info, error};

use crate::{
    auth::create_jwt,
    errors::AppError,
    models::{AuthResponse, LoginRequest},
    AppState,
};

pub async fn login(
    State(_state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    info!(username = input.username, "login attempt");

    input.validate()
        .map_err(|e| {
            error!("validation error: {}", e);
            AppError::ValidationError(e.to_string())
        })?;

    // For now, accept any login (in production, validate against database)
    // Create token for user_id = 1 (demo purposes)
    let token = create_jwt(1)?;

    info!(username = input.username, "login successful");

    Ok((
        StatusCode::OK,
        Json(AuthResponse { token }),
    ))
}
