use sqlx::PgPool;
use crate::models::Note;
use crate::errors::AppError;

pub async fn list(db: &PgPool) -> Result<Vec<Note>, AppError> {
    let notes = sqlx::query_as!(
        Note,
        "SELECT id, content, created_at FROM notes ORDER BY created_at DESC"
    )
    .fetch_all(db)
    .await?;

    Ok(notes)
}

pub async fn list_paginated(
    db: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Note>, i64), AppError> {
    let notes = sqlx::query_as!(
        Note,
        "SELECT id, content, created_at FROM notes ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    let count_result = sqlx::query!("SELECT COUNT(*) as count FROM notes")
        .fetch_one(db)
        .await?;

    let total = count_result.count.unwrap_or(0);

    Ok((notes, total))
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

pub async fn update(db: &PgPool, id: i32, content: &str) -> Result<Option<Note>, AppError> {
    let note = sqlx::query_as!(
        Note,
        "UPDATE notes SET content = $1 WHERE id = $2 RETURNING id, content, created_at",
        content,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(note)
}

pub async fn delete(db: &PgPool, id: i32) -> Result<bool, AppError> {
    let result = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{setup_test_db, cleanup_test_db};

    #[tokio::test]
    async fn test_create_note() {
        let pool = setup_test_db().await;
        cleanup_test_db(&pool).await;

        let note = create(&pool, "Test note").await.unwrap();

        assert_eq!(note.content, "Test note");
        assert!(note.id > 0);
        assert!(note.created_at.is_some());

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_get_nonexistent_note() {
        let pool = setup_test_db().await;
        cleanup_test_db(&pool).await;

        let found = get_by_id(&pool, 99999).await.unwrap();
        assert!(found.is_none());

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_update_nonexistent_note() {
        let pool = setup_test_db().await;
        cleanup_test_db(&pool).await;

        let result = update(&pool, 99999, "New content").await.unwrap();
        assert!(result.is_none());

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_delete_nonexistent_note() {
        let pool = setup_test_db().await;
        cleanup_test_db(&pool).await;

        let deleted = delete(&pool, 99999).await.unwrap();
        assert!(!deleted);

        cleanup_test_db(&pool).await;
    }
}
