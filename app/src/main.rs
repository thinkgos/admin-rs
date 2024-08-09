use axum::Router;

use configuration::Configuration;
use handler::{misc, swagger};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let c = Configuration::load()?;

    let app = Router::new()
        .merge(swagger::config_router())
        .nest("/api", Router::new().merge(misc::config_router_v1()));

    let listener = tokio::net::TcpListener::bind(c.app.addr()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
