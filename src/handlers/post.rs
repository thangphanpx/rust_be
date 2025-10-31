use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use validator::Validate;

use crate::{
    database::models::Post,
    models::{
        responses::{ApiResponse, PaginatedResponse, PostResponse},
        requests::{CreatePostRequest, PaginationParams, UpdatePostRequest},
    },
    AppState,
};

/// Create a new post
#[utoipa::path(
    post,
    path = "/posts",
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

    // For demo purposes, we'll use user_id = 1
    // In a real app, this would come from authentication middleware
    let user_id = 1;

    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (title, content, user_id, is_published, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING id, title, content, user_id, is_published, created_at, updated_at
        "#,
        payload.title,
        payload.content,
        user_id,
        payload.is_published.unwrap_or(false)
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create post: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = PostResponse {
        id: post.id,
        title: post.title,
        content: post.content,
        user_id: post.user_id,
        is_published: post.is_published,
        created_at: post.created_at,
        updated_at: post.updated_at,
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "Post created successfully")),
    ))
}

/// Get all posts with pagination
#[utoipa::path(
    get,
    path = "/posts",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of posts", body = PostsApiResponse)
    ),
    tag = "Posts"
)]
pub async fn get_posts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<PostResponse>>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    // Get total count
    let total_result = sqlx::query!("SELECT COUNT(*) as count FROM posts")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total = total_result.count.unwrap_or(0) as u64;

    // Get paginated posts
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, title, content, user_id, is_published, created_at, updated_at 
         FROM posts 
         ORDER BY created_at DESC 
         LIMIT $1 OFFSET $2",
        limit as i64,
        offset as i64
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post_responses: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse {
            id: post.id,
            title: post.title,
            content: post.content,
            user_id: post.user_id,
            is_published: post.is_published,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
        .collect();

    let total_pages = (total as f64 / limit as f64).ceil() as u64;

    let response = PaginatedResponse {
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
    path = "/posts/{id}",
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
    let post = sqlx::query_as!(
        Post,
        "SELECT id, title, content, user_id, is_published, created_at, updated_at 
         FROM posts WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match post {
        Some(post) => {
            let response = PostResponse {
                id: post.id,
                title: post.title,
                content: post.content,
                user_id: post.user_id,
                is_published: post.is_published,
                created_at: post.created_at,
                updated_at: post.updated_at,
            };

            Ok(Json(ApiResponse::success(
                response,
                "Post found successfully",
            )))
        }
        None => Ok(Json(ApiResponse::error("Post not found"))),
    }
}

/// Update post by ID
#[utoipa::path(
    put,
    path = "/posts/{id}",
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

    // Check if post exists
    let existing_post = sqlx::query!(
        "SELECT id FROM posts WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_post.is_none() {
        return Ok(Json(ApiResponse::error("Post not found")));
    }

    // Update fields individually if provided
    if let Some(title) = &payload.title {
        sqlx::query!("UPDATE posts SET title = $1, updated_at = NOW() WHERE id = $2", title, id)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let Some(content) = &payload.content {
        sqlx::query!("UPDATE posts SET content = $1, updated_at = NOW() WHERE id = $2", content, id)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let Some(is_published) = payload.is_published {
        sqlx::query!("UPDATE posts SET is_published = $1, updated_at = NOW() WHERE id = $2", is_published, id)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Get the updated post data
    let updated_post = sqlx::query_as!(
        Post,
        "SELECT id, title, content, user_id, is_published, created_at, updated_at 
         FROM posts WHERE id = $1",
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = PostResponse {
        id: updated_post.id,
        title: updated_post.title,
        content: updated_post.content,
        user_id: updated_post.user_id,
        is_published: updated_post.is_published,
        created_at: updated_post.created_at,
        updated_at: updated_post.updated_at,
    };

    Ok(Json(ApiResponse::success(
        response,
        "Post updated successfully",
    )))
}

/// Delete post by ID
#[utoipa::path(
    delete,
    path = "/posts/{id}",
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
    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete post: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Ok(Json(ApiResponse::error("Post not found")));
    }

    Ok(Json(ApiResponse::success(
        "Post deleted".to_string(),
        "Post deleted successfully",
    )))
}