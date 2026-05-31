mod db;
mod errors;
mod models;
mod routes;

use axum::Router;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not defined");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("error connecting to database");

    sqlx::migrate!().run(&pool).await.expect("error running migrations");

    let state = AppState { db: pool };

    let app = Router::new()
        .merge(routes::notes_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}