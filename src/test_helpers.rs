#[cfg(test)]
use sqlx::{PgPool, postgres::PgPoolOptions};

#[cfg(test)]
pub async fn setup_test_db() -> PgPool {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("DATABASE_URL or TEST_DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

#[cfg(test)]
pub async fn cleanup_test_db(pool: &PgPool) {
    sqlx::query!("TRUNCATE TABLE notes RESTART IDENTITY")
        .execute(pool)
        .await
        .ok();
}
