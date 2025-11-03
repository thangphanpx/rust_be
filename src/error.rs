use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    NotFound(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}