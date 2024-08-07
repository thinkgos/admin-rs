use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
struct HealthyResponse {
    // running status
    status: String,
}

/// 健康检查
#[utoipa::path(
    tag = "开放接口",
    get,
    context_path = "/v1",
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
