use std::sync::Arc;

use axum::{
    http,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::{
    routes::{health_check, subscribe},
    telemetry::{get_subscriber, init_subscriber},
};

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
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info,tower_http=debug,axum=debug,sqlx=debug",
    );
    init_subscriber(subscriber);

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
