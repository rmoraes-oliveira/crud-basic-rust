use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Clone)]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Validate)]
pub struct NewNoteInput {
    #[validate(length(min = 1, max = 5000, message = "Content must be between 1 and 5000 characters"))]
    pub content: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateNoteInput {
    #[validate(length(min = 1, max = 5000, message = "Content must be between 1 and 5000 characters"))]
    pub content: String,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

fn default_offset() -> i64 {
    0
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}