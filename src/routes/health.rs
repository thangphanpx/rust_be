use axum::{response::Json, routing::get, Router};

use crate::{config::AppState, models::responses::ApiResponse};

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthApiResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success(
        "OK".to_string(),
        "Service is healthy",
    ))
}

pub fn health_router() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}