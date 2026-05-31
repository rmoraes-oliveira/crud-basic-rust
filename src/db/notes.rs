use sqlx::PgPool;
use crate::models::Note;
use crate::errors::AppError;

pub async fn list(db: &PgPool) -> Result<Vec<Note>, AppError> {
    let notes = sqlx::query_as!(
        Note,
        "SELECT id, content, created_at FROM notes ORDER BY id"
    )
    .fetch_all(db)
    .await?;

    Ok(notes)
}

pub async fn get_by_id(db: &PgPool, id: i32) -> Result<Option<Note>, AppError> {
    let note = sqlx::query_as!(
        Note,
        "SELECT id, content, created_at FROM notes WHERE id = $1",
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(note)
}

pub async fn create(db: &PgPool, content: &str) -> Result<Note, AppError> {
    let note = sqlx::query_as!(
        Note,
        "INSERT INTO notes (content) VALUES ($1) RETURNING id, content, created_at",
        content
    )
    .fetch_one(db)
    .await?;

    Ok(note)
}

pub async fn delete(db: &PgPool, id: i32) -> Result<bool, AppError> {
    let result = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}
