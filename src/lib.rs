pub mod db;
pub mod errors;
pub mod models;
pub mod routes;

#[cfg(test)]
pub mod test_helpers;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}
