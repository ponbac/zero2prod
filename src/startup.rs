use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::routes::{health_check, subscribe};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

pub fn app_router(db_pool: PgPool) -> Router {
    // initialize tracing if it hasn't been already
    if tracing_subscriber::fmt::try_init().is_err() {
        tracing::warn!("tracing is already initialized");
    }

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(AppState {
            db_pool: Arc::new(db_pool),
        })
        // use with RUST_LOG=tower_http=debug to see the logs
        .layer(TraceLayer::new_for_http())
}
