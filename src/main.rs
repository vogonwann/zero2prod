use std::net::TcpListener;
use sqlx::{Connection, PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

//noinspection RsMainFunctionNotFound
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool).await?.await
}

