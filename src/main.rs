use std::net::SocketAddr;

use zero2prod::{configuration::get_configuration, startup::app};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
