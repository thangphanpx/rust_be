use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod app_state;
mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schemas;
mod services;
mod utils;

use app_state::AppState;
use config::load_config;
use handlers::{health, post, user};
use routes::{health::health_router, user_router, post_router};

// Database connection function
async fn create_database_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

// Create API routes
fn create_api_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_router())
        .nest("/posts", post_router())
}

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
            schemas::CreateUserRequest,
            schemas::UpdateUserRequest,
            schemas::CreatePostRequest,
            schemas::UpdatePostRequest,
            schemas::PaginationParams,
            schemas::UserResponse,
            schemas::PostResponse,
            schemas::PostWithUserResponse,
            schemas::PaginatedUserResponse,
            schemas::PaginatedPostResponse,
            schemas::UserApiResponse,
            schemas::UsersApiResponse,
            schemas::PostApiResponse,
            schemas::PostsApiResponse,
            schemas::StringApiResponse,
            schemas::HealthApiResponse,
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
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = load_config()?;
    
    // Initialize database connection
    let db_pool = create_database_pool(&config.database_url).await?;

    // Create application state
    let app_state = AppState {
        db: db_pool,
        config,
    };

    // Build our application with routes
    let app = Router::new()
        // API routes
        .nest("/api", create_api_routes())
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