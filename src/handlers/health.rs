use axum::Json;

use crate::models::responses::ApiResponse;

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