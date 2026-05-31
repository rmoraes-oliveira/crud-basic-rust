pub mod notes;

use axum::{routing::get, Router};
use crate::AppState;

pub fn notes_router() -> Router<AppState> {
    Router::new()
        .route("/notes", get(notes::list).post(notes::create))
        .route("/notes/:id", get(notes::get).delete(notes::delete))
}