use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::app_state::AppState;
use crate::error::AppError;
use crate::utils::jwt::verify_jwt;

#[allow(dead_code)]
pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.strip_prefix("Bearer ").unwrap()
        }
        _ => {
            return Err(AppError::Unauthorized("Missing or invalid authorization header".to_string()));
        }
    };

    match verify_jwt(token, &state.config.jwt_secret) {
        Ok(_claims) => {
            // You can add user info to request extensions here
            // req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(AppError::Unauthorized("Invalid token".to_string())),
    }
}