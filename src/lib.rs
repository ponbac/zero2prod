use std::net::SocketAddr;

use axum::{extract::Path, http::StatusCode, routing::get, Router};

pub async fn run() {
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.
    let app = Router::new()
        .route("/", get(handler))
        .route("/health_check", get(health_check))
        .route("/hello/:name", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
