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
    repositories::post_repo::PostRepository,
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
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PostResponse>>), StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Invalid input data")),
        ));
    }

    // For demo purposes, use user_id = 1
    // In a real app, this would come from authentication middleware
    let user_id = 1;

    // Create post in database
    match PostRepository::create(&state.db, user_id, &payload).await {
        Ok(post) => {
            let response = PostResponse {
                id: post.id,
                title: post.title,
                content: post.content,
                user_id: post.user_id,
                is_published: post.is_published,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::success(response, "Post created successfully")),
            ))
        }
        Err(_) => {
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to create post")),
            ))
        }
    }
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
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedPostResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Get posts from database
    match PostRepository::find_all(&state.db, page, limit).await {
        Ok(posts) => {
            let total = match PostRepository::count(&state.db).await {
                Ok(count) => count as u64,
                Err(_) => return Ok(Json(ApiResponse::error("Failed to get post count"))),
            };

            let total_pages = (total as f64 / limit as f64).ceil() as u64;

            let post_responses: Vec<PostResponse> = posts
                .into_iter()
                .map(|post| PostResponse {
                    id: post.id,
                    title: post.title,
                    content: post.content,
                    user_id: post.user_id,
                    is_published: post.is_published,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
                .collect();

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
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to retrieve posts")))
        }
    }
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
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PostResponse>>, StatusCode> {
    // Get post from database
    match PostRepository::find_by_id(&state.db, id).await {
        Ok(Some(post)) => {
            let response = PostResponse {
                id: post.id,
                title: post.title,
                content: post.content,
                user_id: post.user_id,
                is_published: post.is_published,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok(Json(ApiResponse::success(
                response,
                "Post found successfully",
            )))
        }
        Ok(None) => {
            Ok(Json(ApiResponse::error("Post not found")))
        }
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to retrieve post")))
        }
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
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, StatusCode> {
    // Validate input
    if let Err(_) = payload.validate() {
        return Ok(Json(ApiResponse::error("Invalid input data")));
    }

    // Update post in database
    match PostRepository::update(&state.db, id, &payload).await {
        Ok(Some(post)) => {
            let response = PostResponse {
                id: post.id,
                title: post.title,
                content: post.content,
                user_id: post.user_id,
                is_published: post.is_published,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            Ok(Json(ApiResponse::success(
                response,
                "Post updated successfully",
            )))
        }
        Ok(None) => {
            Ok(Json(ApiResponse::error("Post not found")))
        }
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to update post")))
        }
    }
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
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Delete post from database
    match PostRepository::delete(&state.db, id).await {
        Ok(true) => {
            Ok(Json(ApiResponse::success(
                "Post deleted".to_string(),
                "Post deleted successfully",
            )))
        }
        Ok(false) => {
            Ok(Json(ApiResponse::error("Post not found")))
        }
        Err(_) => {
            Ok(Json(ApiResponse::error("Failed to delete post")))
        }
    }
}