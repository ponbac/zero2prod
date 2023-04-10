use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

pub fn app_router(db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(AppState {
            db_pool: Arc::new(db_pool),
        })
}
