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
    email_client::EmailClient,
    routes::{health_check, subscribe},
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
    pub email_client: Arc<EmailClient>,
}

#[derive(Clone)]
pub struct RequestSpan;

impl<B> tower_http::trace::MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        tracing::debug_span!(
            "rq",
            id = %Uuid::new_v4(),
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
        )
    }
}

pub fn app_router(db_pool: PgPool, email_client: EmailClient) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(AppState {
            db_pool: Arc::new(db_pool),
            email_client: Arc::new(email_client),
        })
        // use with RUST_LOG=tower_http=debug to see the logs
        .layer(TraceLayer::new_for_http().make_span_with(RequestSpan))
}
