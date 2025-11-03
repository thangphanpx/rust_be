use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use validator::Validate;

use crate::{
    schemas::{
        response::{ApiResponse, PaginatedUserResponse, UserResponse},
        user::{CreateUserRequest, PaginationParams, UpdateUserRequest},
    },
    app_state::AppState,
};
use chrono::Utc;

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserApiResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    ),
    tag = "Users"
)]
pub async fn create_user(
    State(_state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Invalid input data")),
        ));
    }

    // TODO: Replace with actual SeaORM database operations
    // Check for duplicate email would be implemented here

    // Hash password for demo (in real implementation)
    // let password_hash = bcrypt::hash(&payload.password, 10)
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Mock response for demo
    let response = UserResponse {
        id: 1,
        email: payload.email,
        username: payload.username,
        full_name: payload.full_name,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "User created successfully")),
    ))
}

/// Get all users with pagination
#[utoipa::path(
    get,
    path = "/api/users",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of users", body = UsersApiResponse)
    ),
    tag = "Users"
)]
pub async fn get_users(
    State(_state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedUserResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // TODO: Replace with actual SeaORM database operations
    // Mock response for now
    let user_responses = vec![
        UserResponse {
            id: 1,
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            full_name: Some("Test User".to_string()),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];

    let total = user_responses.len() as u64;
    let total_pages = (total as f64 / limit as f64).ceil() as u64;

    let response = PaginatedUserResponse {
        data: user_responses,
        page,
        limit,
        total,
        total_pages,
    };

    Ok(Json(ApiResponse::success(
        response,
        "Users retrieved successfully",
    )))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserApiResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
pub async fn get_user_by_id(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // TODO: Replace with actual SeaORM database operations
    if id == 1 {
        let response = UserResponse {
            id,
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            full_name: Some("Test User".to_string()),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(Json(ApiResponse::success(
            response,
            "User found successfully",
        )))
    } else {
        Ok(Json(ApiResponse::error("User not found")))
    }
}

/// Update user by ID
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserApiResponse),
        (status = 404, description = "User not found"),
        (status = 400, description = "Invalid input")
    ),
    tag = "Users"
)]
pub async fn update_user(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok(Json(ApiResponse::error("Invalid input data")));
    }

    // TODO: Replace with actual SeaORM database operations
    if id != 1 {
        return Ok(Json(ApiResponse::error("User not found")));
    }

    let response = UserResponse {
        id,
        email: payload.email.unwrap_or_else(|| "updated@example.com".to_string()),
        username: payload.username.unwrap_or_else(|| "updateduser".to_string()),
        full_name: payload.full_name,
        is_active: payload.is_active.unwrap_or(true),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Ok(Json(ApiResponse::success(
        response,
        "User updated successfully",
    )))
}

/// Delete user by ID
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = StringApiResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
pub async fn delete_user(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // TODO: Replace with actual SeaORM database operations
    if id != 1 {
        return Ok(Json(ApiResponse::error("User not found")));
    }

    Ok(Json(ApiResponse::success(
        "User deleted".to_string(),
        "User deleted successfully",
    )))
}