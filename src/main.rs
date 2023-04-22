use std::net::SocketAddr;

use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::app_router,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info,tower_http=debug,axum=debug,sqlx=debug",
    );
    init_subscriber(subscriber);

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
