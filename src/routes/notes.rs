use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use validator::Validate;
use tracing::{info, error};
use crate::{db, errors::AppError, models::{Note, NewNoteInput, UpdateNoteInput, PaginationParams, PaginatedResponse, PaginationInfo}, AppState};
use db::notes;

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Note>>, AppError> {
    info!(limit = params.limit, offset = params.offset, "listing notes");

    let (notes, total) = db::notes::list_paginated(&state.db, params.limit, params.offset).await?;

    info!(count = notes.len(), total = total, "notes retrieved successfully");

    Ok(Json(PaginatedResponse {
        data: notes,
        pagination: PaginationInfo {
            limit: params.limit,
            offset: params.offset,
            total,
        },
    }))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Note>, AppError> {
    info!(note_id = id, "fetching note");

    let note = notes::get_by_id(&state.db, id)
        .await?
        .ok_or_else(|| {
            error!(note_id = id, "note not found");
            AppError::NotFound
        })?;

    info!(note_id = id, "note retrieved successfully");
    Ok(Json(note))
}

pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<NewNoteInput>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    info!(content_length = input.content.len(), "creating note");

    input.validate()
        .map_err(|e| {
            error!("validation error: {}", e);
            AppError::ValidationError(e.to_string())
        })?;

    let note = notes::create(&state.db, &input.content).await?;
    info!(note_id = note.id, "note created successfully");
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateNoteInput>,
) -> Result<Json<Note>, AppError> {
    info!(note_id = id, content_length = input.content.len(), "updating note");

    input.validate()
        .map_err(|e| {
            error!("validation error: {}", e);
            AppError::ValidationError(e.to_string())
        })?;

    let note = notes::update(&state.db, id, &input.content)
        .await?
        .ok_or_else(|| {
            error!(note_id = id, "note not found for update");
            AppError::NotFound
        })?;

    info!(note_id = id, "note updated successfully");
    Ok(Json(note))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    info!(note_id = id, "deleting note");

    match notes::delete(&state.db, id).await? {
        true => {
            info!(note_id = id, "note deleted successfully");
            Ok(StatusCode::NO_CONTENT)
        }
        false => {
            error!(note_id = id, "note not found for deletion");
            Err(AppError::NotFound)
        }
    }
}