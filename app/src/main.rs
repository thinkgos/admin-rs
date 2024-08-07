use axum::{routing, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use configuration::Configuration;
use handler::{misc, ApiDoc};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let c = Configuration::load()?;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/v1/public/healthy", routing::get(misc::healthy));

    let listener = tokio::net::TcpListener::bind(c.app.addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
