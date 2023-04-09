use std::net::SocketAddr;

use zero2prod::startup::app;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
