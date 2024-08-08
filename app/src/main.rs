use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use configuration::Configuration;
use handler::{misc, ApiDoc};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let c = Configuration::load()?;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger/index.html").url("/swagger/doc.json", ApiDoc::openapi()))
        .nest("/api/v1", Router::new().merge(misc::config_router()));

    let listener = tokio::net::TcpListener::bind(c.app.addr()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
