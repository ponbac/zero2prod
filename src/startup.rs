use std::sync::Arc;

use axum::{
    http,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use uuid::Uuid;

use crate::routes::{health_check, subscribe};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

#[derive(Clone)]
pub struct RequestSpan;

impl<B> tower_http::trace::MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        tracing::error_span!(
            "rq",
            id = %Uuid::new_v4(),
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
        )
    }
}

pub fn app_router(db_pool: PgPool) -> Router {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::from("info,tower_http=debug,axum=debug,sqlx=debug"));

    // initialize tracing if it hasn't been already
    if tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init()
        .is_err()
    {
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
        .layer(TraceLayer::new_for_http().make_span_with(RequestSpan))
}
