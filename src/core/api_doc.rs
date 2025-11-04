use utoipa::OpenApi;
use crate::handlers::{health, post, user};
use crate::schemas::{
    CreateUserRequest, UpdateUserRequest, CreatePostRequest, UpdatePostRequest,
    PaginationParams, UserResponse, PostResponse, PostWithUserResponse,
    PaginatedUserResponse, PaginatedPostResponse, UserApiResponse,
    UsersApiResponse, PostApiResponse, PostsApiResponse, StringApiResponse,
    HealthApiResponse
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        user::create_user,
        user::get_users,
        user::get_user_by_id,
        user::update_user,
        user::delete_user,
        post::create_post,
        post::get_posts,
        post::get_post_by_id,
        post::update_post,
        post::delete_post,
    ),
    components(
        schemas(
            CreateUserRequest,
            UpdateUserRequest,
            CreatePostRequest,
            UpdatePostRequest,
            PaginationParams,
            UserResponse,
            PostResponse,
            PostWithUserResponse,
            PaginatedUserResponse,
            PaginatedPostResponse,
            UserApiResponse,
            UsersApiResponse,
            PostApiResponse,
            PostsApiResponse,
            StringApiResponse,
            HealthApiResponse,
        )
    ),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Posts", description = "Post management endpoints"),
        (name = "Health", description = "Health check endpoints")
    ),
    info(
        title = "Rust Backend API",
        description = "A REST API built with Axum, sqlx, and PostgreSQL",
        version = "0.1.0"
    )
)]
pub struct ApiDoc;