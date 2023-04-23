use std::net::SocketAddr;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::app_router,
    telemetry::{self},
};

#[tokio::main]
async fn main() {
    let subscriber = telemetry::get_subscriber(
        "zero2prod".into(),
        "info,tower_http=debug,axum=debug,sqlx=debug",
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    axum::Server::bind(&addr)
        .serve(app_router(connection_pool).into_make_service())
        .await
        .unwrap();
}
