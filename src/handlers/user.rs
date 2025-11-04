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
    repositories::user_repo::UserRepository,
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
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Invalid input data")),
        ));
    }

    // Create user in database
    match UserRepository::create(&state.db, &payload).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                full_name: user.full_name,
                is_active: user.is_active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::success(response, "User created successfully")),
            ))
        }
        Err(_) => {
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to create user")),
            ))
        }
    }
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
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedUserResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Get users from database
    match UserRepository::find_all(&state.db, page, limit).await {
        Ok(users) => {
            let total = match UserRepository::count(&state.db).await {
                Ok(count) => count as u64,
                Err(_) => return Ok(Json(ApiResponse::error("Failed to get user count"))),
            };

            let total_pages = (total as f64 / limit as f64).ceil() as u64;

            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: user.id,
                    email: user.email,
                    username: user.username,
                    full_name: user.full_name,
                    is_active: user.is_active,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
                .collect();

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
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to retrieve users")))
        }
    }
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
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // Get user from database
    match UserRepository::find_by_id(&state.db, id).await {
        Ok(Some(user)) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                full_name: user.full_name,
                is_active: user.is_active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok(Json(ApiResponse::success(
                response,
                "User found successfully",
            )))
        }
        Ok(None) => {
            Ok(Json(ApiResponse::error("User not found")))
        }
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to retrieve user")))
        }
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
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok(Json(ApiResponse::error("Invalid input data")));
    }

    // Update user in database
    match UserRepository::update(&state.db, id, &payload).await {
        Ok(Some(user)) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                full_name: user.full_name,
                is_active: user.is_active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok(Json(ApiResponse::success(
                response,
                "User updated successfully",
            )))
        }
        Ok(None) => {
            Ok(Json(ApiResponse::error("User not found")))
        }
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to update user")))
        }
    }
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
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Delete user from database
    match UserRepository::delete(&state.db, id).await {
        Ok(true) => {
            Ok(Json(ApiResponse::success(
                "User deleted".to_string(),
                "User deleted successfully",
            )))
        }
        Ok(false) => {
            Ok(Json(ApiResponse::<String>::error("User not found")))
        }
        Err(_) => {
            Ok(Json(ApiResponse::<String>::error("Failed to delete user")))
        }
    }
}