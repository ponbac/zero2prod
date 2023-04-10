use std::net::SocketAddr;

use sqlx::{Connection, PgPool};
use zero2prod::{configuration::get_configuration, startup::app_router};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    axum::Server::bind(&addr)
        .serve(app_router(connection_pool).into_make_service())
        .await
        .unwrap();
}
