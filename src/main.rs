use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod database;
mod handlers;
mod models;
mod routes;
mod services;

use config::AppState;
use api::implement_apis::api_router;
use handlers::{health, post, user};
use routes::health::health_router;

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
            models::requests::CreateUserRequest,
            models::requests::UpdateUserRequest,
            models::requests::CreatePostRequest,
            models::requests::UpdatePostRequest,
            models::requests::PaginationParams,
            models::responses::UserResponse,
            models::responses::PostResponse,
            models::responses::PostWithUserResponse,
            models::responses::PaginatedUserResponse,
            models::responses::PaginatedPostResponse,
            models::responses::UserApiResponse,
            models::responses::UsersApiResponse,
            models::responses::PostApiResponse,
            models::responses::PostsApiResponse,
            models::responses::StringApiResponse,
            models::responses::HealthApiResponse,
        )
    ),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Posts", description = "Post management endpoints"),
        (name = "Productions", description = "Production management endpoints"),
        (name = "Health", description = "Health check endpoints")
    ),
    info(
        title = "Rust Backend API",
        description = "A REST API built with Axum, sqlx, and PostgreSQL",
        version = "0.1.0"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::load_config()?;
    
    // Initialize database connection
    let db_pool = database::connection::create_pool(&config.database_url).await?;

    // Create application state
    let app_state = AppState {
        db: db_pool,
        config,
    };

    // Build our application with centralized routes
    let app = Router::new()
        // API routes với prefix /api
        .nest("/api", api_router())
        // Health check
        .nest("/health", health_router())
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Add CORS layer
        .layer(CorsLayer::permissive())
        // Add shared state
        .with_state(app_state);

    // Parse the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}