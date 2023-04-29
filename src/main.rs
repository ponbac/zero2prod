use std::net::SocketAddr;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
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
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");
    let port = std::env::var("PORT").unwrap_or_else(|_| configuration.application.port.to_string());
    let socket_addr = format!("{}:{}", configuration.application.host, port)
        .parse::<SocketAddr>()
        .expect("Failed to parse address.");

    tracing::info!(
        "Starting application on {}:{}",
        configuration.application.host,
        port
    );
    axum::Server::bind(&socket_addr)
        .serve(app_router(connection_pool).into_make_service())
        .await
        .unwrap();
}
