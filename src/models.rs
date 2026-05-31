use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct NewNoteInput {
    pub content: String,
}