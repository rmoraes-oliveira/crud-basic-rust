pub mod notes;
#[cfg(test)]
mod notes_tests;

use axum::{routing::get, Router};
use crate::AppState;

pub fn notes_router() -> Router<AppState> {
    Router::new()
        .route("/notes", get(notes::list).post(notes::create))
        .route("/notes/:id", get(notes::get).patch(notes::update).delete(notes::delete))
}