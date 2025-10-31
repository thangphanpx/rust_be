use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use validator::Validate;

use crate::{
    config::AppState,
    database::models::User,
    models::{
        requests::{CreateUserRequest, PaginationParams, UpdateUserRequest},
        responses::{ApiResponse, PaginatedResponse, UserResponse},
    },
};

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
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

    // Check if user already exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        return Ok((
            StatusCode::CONFLICT,
            Json(ApiResponse::error("User with this email already exists")),
        ));
    }

    // Hash password
    let password_hash = bcrypt::hash(&payload.password, 10)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create user
    let user_result = sqlx::query!(
        r#"
        INSERT INTO users (email, username, password_hash, full_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, username, password_hash, full_name, is_active, created_at, updated_at
        "#,
        payload.email,
        payload.username,
        password_hash,
        payload.full_name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = UserResponse {
        id: user_result.id,
        email: user_result.email,
        username: user_result.username,
        full_name: user_result.full_name,
        is_active: user_result.is_active,
        created_at: user_result.created_at,
        updated_at: user_result.updated_at,
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "User created successfully")),
    ))
}

/// Get all users with pagination
#[utoipa::path(
    get,
    path = "/users",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of users", body = UsersApiResponse)
    ),
    tag = "Users"
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<UserResponse>>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    // Get total count
    let total_result = sqlx::query!(
        "SELECT COUNT(*) as count FROM users"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = total_result.count.unwrap_or(0) as u64;
    let total_pages = (total as f64 / limit as f64).ceil() as u64;

    // Get users with pagination
    let users = sqlx::query_as!(
        User,
        "SELECT id, email, username, password_hash, full_name, is_active, created_at, updated_at 
         FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit as i64,
        offset as i64
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            full_name: user.full_name,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
        .collect();

    let response = PaginatedResponse {
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
    path = "/users/{id}",
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
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, username, password_hash, full_name, is_active, created_at, updated_at 
         FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                full_name: user.full_name,
                is_active: user.is_active,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };

            Ok(Json(ApiResponse::success(
                response,
                "User found successfully",
            )))
        }
        None => Ok(Json(ApiResponse::error("User not found"))),
    }
}

/// Update user by ID
#[utoipa::path(
    put,
    path = "/users/{id}",
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

    // Check if user exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_none() {
        return Ok(Json(ApiResponse::error("User not found")));
    }

    // Simple update - update all fields
    let updated_user = sqlx::query!(
        r#"
        UPDATE users 
        SET email = COALESCE($2, email),
            username = COALESCE($3, username),
            full_name = COALESCE($4, full_name),
            is_active = COALESCE($5, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, username, password_hash, full_name, is_active, created_at, updated_at
        "#,
        id,
        payload.email,
        payload.username,
        payload.full_name,
        payload.is_active
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = UserResponse {
        id: updated_user.id,
        email: updated_user.email,
        username: updated_user.username,
        full_name: updated_user.full_name,
        is_active: updated_user.is_active,
        created_at: updated_user.created_at,
        updated_at: updated_user.updated_at,
    };

    Ok(Json(ApiResponse::success(
        response,
        "User updated successfully",
    )))
}

/// Delete user by ID
#[utoipa::path(
    delete,
    path = "/users/{id}",
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
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Ok(Json(ApiResponse::error("User not found")));
    }

    Ok(Json(ApiResponse::success(
        "User deleted".to_string(),
        "User deleted successfully",
    )))
}