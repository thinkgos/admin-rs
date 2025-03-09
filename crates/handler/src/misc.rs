use axum::{
    Router,
    response::{IntoResponse, Json},
    routing,
};
use serde::{Deserialize, Serialize};

#[derive(utoipa::OpenApi)]
#[openapi(paths(healthy), components(schemas(HealthyResponse)))]
pub(crate) struct MiscApi;

pub fn config_router_v1() -> Router {
    Router::new().nest(
        "/v1",
        Router::new().route("/public/healthy", routing::get(healthy)),
    )
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
struct HealthyResponse {
    // running status
    status: String,
}

/// 健康检查
#[utoipa::path(
    tag = "开放接口",
    get,
    context_path = "v1",
    path = "/public/healthy",
    responses(
        (status = StatusCode::OK, body = inline(HealthyResponse))
    ),
)]
pub async fn healthy() -> impl IntoResponse {
    Json(HealthyResponse {
        status: "running".to_string(),
    })
}
