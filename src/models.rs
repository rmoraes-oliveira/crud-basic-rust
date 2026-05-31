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