pub mod db;
pub mod errors;
pub mod models;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}
