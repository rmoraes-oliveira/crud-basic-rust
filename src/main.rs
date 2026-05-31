use axum::Router;
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use notes_api::AppState;
use notes_api::routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not defined");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("error connecting to database");

    sqlx::migrate!().run(&pool).await.expect("error running migrations");

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
    .allow_origin(Any)           // qualquer origem — ajuste em produção
    .allow_methods(Any)          // GET, POST, DELETE, etc
    .allow_headers(Any);         // Content-Type, Authorization, etc

    let app = Router::new()
        .merge(routes::notes_router())
        .layer(TraceLayer::new_for_http())  // logging automático
        .layer(cors)                         // CORS
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}