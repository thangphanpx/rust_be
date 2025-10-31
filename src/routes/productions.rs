use axum::{routing::get, Router};

use crate::config::AppState;

// Example production endpoints - thêm handlers sau
async fn get_production_lines() -> &'static str {
    "Production lines data"
}

async fn get_production_status() -> &'static str {
    "Production status"
}

async fn get_production_logs() -> &'static str {
    "Production logs"
}

pub fn production_router() -> Router<AppState> {
    Router::new()
        .route("/lines", get(get_production_lines))
        .route("/status", get(get_production_status))
        .route("/logs", get(get_production_logs))
}