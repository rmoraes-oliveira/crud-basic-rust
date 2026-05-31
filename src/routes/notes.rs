use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use validator::Validate;
use crate::{db, errors::AppError, models::{Note, NewNoteInput, UpdateNoteInput, PaginationParams, PaginatedResponse, PaginationInfo}, AppState};
use db::notes;

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Note>>, AppError> {
    let (notes, total) = db::notes::list_paginated(&state.db, params.limit, params.offset).await?;

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
    notes::get_by_id(&state.db, id)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<NewNoteInput>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    input.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let note = notes::create(&state.db, &input.content).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateNoteInput>,
) -> Result<Json<Note>, AppError> {
    input.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    notes::update(&state.db, id, &input.content)
        .await?
        .ok_or(AppError::NotFound)
        .map(Json)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    match notes::delete(&state.db, id).await? {
        true  => Ok(StatusCode::NO_CONTENT),
        false => Err(AppError::NotFound),
    }
}