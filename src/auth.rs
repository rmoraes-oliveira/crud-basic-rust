use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::errors::AppError;

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub exp: i64,
    pub iat: i64,
}

pub struct AuthenticatedUser {
    pub user_id: i32,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                warn!("Missing Authorization header");
                AppError::Unauthorized
            })?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                warn!("Invalid Authorization header format");
                AppError::Unauthorized
            })?;

        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )
        .map_err(|e| {
            warn!("Failed to decode JWT: {}", e);
            AppError::Unauthorized
        })?
        .claims;

        Ok(AuthenticatedUser {
            user_id: claims.user_id,
        })
    }
}

pub fn create_jwt(user_id: i32) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        user_id,
        exp: now + 86400, // 24 hours
        iat: now,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AppError::JwtCreationFailed)
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, 10).map_err(|_| AppError::PasswordHashingFailed)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_jwt() {
        let token = create_jwt(1).expect("Failed to create JWT");
        assert!(!token.is_empty());
    }

    #[test]
    fn test_hash_password() {
        let password = "test_password_123";
        let hash = hash_password(password).expect("Failed to hash password");
        assert!(verify_password(password, &hash));
    }

    #[test]
    fn test_verify_password_invalid() {
        let password = "test_password_123";
        let hash = hash_password(password).expect("Failed to hash password");
        assert!(!verify_password("wrong_password", &hash));
    }
}
