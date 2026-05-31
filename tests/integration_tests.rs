use sqlx::PgPool;

async fn setup_test_db() -> PgPool {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("DATABASE_URL or TEST_DATABASE_URL not set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

async fn cleanup_test_notes(pool: &PgPool) {
    sqlx::query!("TRUNCATE TABLE notes RESTART IDENTITY")
        .execute(pool)
        .await
        .ok();
}

// Single test to verify database connectivity and basic operations
#[tokio::test]
async fn test_database_integration() {
    let pool = setup_test_db().await;
    cleanup_test_notes(&pool).await;

    // Test CREATE
    let note = sqlx::query!(
        "INSERT INTO notes (content) VALUES ($1) RETURNING id, content, created_at",
        "Integration test note"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to create note");

    assert_eq!(note.content, "Integration test note");
    assert!(note.id > 0);
    assert!(note.created_at.is_some());

    let note_id = note.id;

    // Test READ
    let found = sqlx::query!(
        "SELECT id, content FROM notes WHERE id = $1",
        note_id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch note");

    assert_eq!(found.content, "Integration test note");

    // Test UPDATE
    sqlx::query!(
        "UPDATE notes SET content = $1 WHERE id = $2",
        "Updated integration test",
        note_id
    )
    .execute(&pool)
    .await
    .expect("Failed to update note");

    let updated = sqlx::query!(
        "SELECT content FROM notes WHERE id = $1",
        note_id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch updated note");

    assert_eq!(updated.content, "Updated integration test");

    // Test DELETE
    let deleted = sqlx::query!("DELETE FROM notes WHERE id = $1", note_id)
        .execute(&pool)
        .await
        .expect("Failed to delete note");

    assert!(deleted.rows_affected() > 0);

    let found = sqlx::query!(
        "SELECT id FROM notes WHERE id = $1",
        note_id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to check deleted note");

    assert!(found.is_none());

    cleanup_test_notes(&pool).await;
}

#[tokio::test]
async fn test_pagination_queries() {
    let pool = setup_test_db().await;
    cleanup_test_notes(&pool).await;

    // Small delay to ensure clean state
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Create 5 notes
    for i in 1..=5 {
        let content = format!("Test note {}", i);
        sqlx::query!(
            "INSERT INTO notes (content) VALUES ($1)",
            content
        )
        .execute(&pool)
        .await
        .expect("Failed to create note");
    }

    // Test total count
    let result = sqlx::query!("SELECT COUNT(*) as count FROM notes")
        .fetch_one(&pool)
        .await
        .expect("Failed to count notes");

    assert!(result.count.unwrap_or(0) >= 5, "Should have at least 5 notes");

    // Test limit and offset
    let notes = sqlx::query!(
        "SELECT id, content, created_at FROM notes ORDER BY created_at DESC LIMIT 2 OFFSET 0"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch with limit");

    assert_eq!(notes.len(), 2, "Should return exactly 2 notes");

    // Test offset > 0
    let notes = sqlx::query!(
        "SELECT id, content, created_at FROM notes ORDER BY created_at DESC LIMIT 2 OFFSET 2"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch with offset");

    assert_eq!(notes.len(), 2, "Should return 2 more notes with offset");

    cleanup_test_notes(&pool).await;
}
