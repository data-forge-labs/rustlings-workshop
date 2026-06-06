use std::sync::Arc;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_cookies::CookieManagerLayer;
use axum_shop::{db, AppState};

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    db::run_migrations(&pool).await;
    db::seed_database(&pool).await;

    let state: AppState = Arc::new(pool);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(tower_sessions::cookie::time::Duration::hours(24)));

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.1:5173".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true);

    let app = Router::new()
        .merge(axum_shop::routes::auth_routes::router())
        .merge(axum_shop::routes::dashboard::router())
        .merge(axum_shop::routes::products::router())
        .merge(axum_shop::routes::categories::router())
        .merge(axum_shop::routes::customers::router())
        .merge(axum_shop::routes::orders::router())
        .nest_service("/static", ServeDir::new("app/static"))
        .layer(CookieManagerLayer::new())
        .layer(session_layer)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
