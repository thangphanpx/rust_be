use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use validator::Validate;

use crate::{
    schemas::{
        response::{ApiResponse, PaginatedPostResponse, PostResponse},
        user::PaginationParams,
        post::{CreatePostRequest, UpdatePostRequest},
    },
    app_state::AppState,
};
use chrono::Utc;

/// Create a new post
#[utoipa::path(
    post,
    path = "/api/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = PostApiResponse),
        (status = 400, description = "Invalid input")
    ),
    tag = "Posts"
)]
pub async fn create_post(
    State(_state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PostResponse>>), StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Invalid input data")),
        ));
    }

    // TODO: Replace with actual SeaORM database operations
    // For demo purposes, return a mock response
    let response = PostResponse {
        id: 1,
        title: payload.title,
        content: payload.content,
        user_id: 1, // Demo user ID
        is_published: payload.is_published.unwrap_or(false),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "Post created successfully")),
    ))
}

/// Get all posts with pagination
#[utoipa::path(
    get,
    path = "/api/posts",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of posts", body = PostsApiResponse)
    ),
    tag = "Posts"
)]
pub async fn get_posts(
    State(_state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedPostResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // TODO: Replace with actual SeaORM database operations
    // Mock response for now
    let post_responses = vec![
        PostResponse {
            id: 1,
            title: "Sample Post".to_string(),
            content: "This is a sample post content".to_string(),
            user_id: 1,
            is_published: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];

    let total = post_responses.len() as u64;
    let total_pages = (total as f64 / limit as f64).ceil() as u64;

    let response = PaginatedPostResponse {
        data: post_responses,
        page,
        limit,
        total,
        total_pages,
    };

    Ok(Json(ApiResponse::success(
        response,
        "Posts retrieved successfully",
    )))
}

/// Get post by ID
#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post found", body = PostApiResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "Posts"
)]
pub async fn get_post_by_id(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PostResponse>>, StatusCode> {
    // TODO: Replace with actual SeaORM database operations
    if id == 1 {
        let response = PostResponse {
            id,
            title: "Sample Post".to_string(),
            content: "This is a sample post content".to_string(),
            user_id: 1,
            is_published: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(Json(ApiResponse::success(
            response,
            "Post found successfully",
        )))
    } else {
        Ok(Json(ApiResponse::error("Post not found")))
    }
}

/// Update post by ID
#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated successfully", body = PostApiResponse),
        (status = 404, description = "Post not found"),
        (status = 400, description = "Invalid input")
    ),
    tag = "Posts"
)]
pub async fn update_post(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok(Json(ApiResponse::error("Invalid input data")));
    }

    // TODO: Replace with actual SeaORM database operations
    if id != 1 {
        return Ok(Json(ApiResponse::error("Post not found")));
    }

    let response = PostResponse {
        id,
        title: payload.title.unwrap_or_else(|| "Updated Post Title".to_string()),
        content: payload.content.unwrap_or_else(|| "Updated content".to_string()),
        user_id: 1,
        is_published: payload.is_published.unwrap_or(true),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Ok(Json(ApiResponse::success(
        response,
        "Post updated successfully",
    )))
}

/// Delete post by ID
#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post deleted successfully", body = StringApiResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "Posts"
)]
pub async fn delete_post(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // TODO: Replace with actual SeaORM database operations
    if id != 1 {
        return Ok(Json(ApiResponse::error("Post not found")));
    }

    Ok(Json(ApiResponse::success(
        "Post deleted".to_string(),
        "Post deleted successfully",
    )))
}