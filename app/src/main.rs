use std::io;

use app::telemetry;
use axum::Router;

use configuration::Configuration;
use handler::{misc, swagger};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("admin", "debug", io::stdout));

    let c = Configuration::load()?;

    tracing::info!("starting ...");

    let app = Router::new()
        .merge(swagger::config_router())
        .nest("/api", Router::new().merge(misc::config_router_v1()));

    let listener = tokio::net::TcpListener::bind(c.app.addr()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
