use axum::{extract::Path, http::StatusCode, routing::get, Router};

pub fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/health_check", get(health_check))
        .route("/hello/:name", get(hello))
}

async fn handler() -> &'static str {
    "Hello, world!"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
