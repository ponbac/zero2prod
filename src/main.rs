use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    startup::app_router,
    telemetry::{self},
};

#[tokio::main]
async fn main() {
    // Init telemetry
    let subscriber = telemetry::get_subscriber(
        "zero2prod".into(),
        "info,tower_http=debug,axum=debug,sqlx=debug,hyper=info",
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

    // Read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Setup database
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database.with_db());

    // Create host address
    let socket_addr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    )
    .parse::<SocketAddr>()
    .expect("Failed to parse address.");

    // Start server
    tracing::info!(
        "Starting application on {}:{}",
        configuration.application.host,
        configuration.application.port
    );
    axum::Server::bind(&socket_addr)
        .serve(app_router(connection_pool).into_make_service())
        .await
        .unwrap();
}
