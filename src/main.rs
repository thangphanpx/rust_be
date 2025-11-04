use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app_state;
mod core;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schemas;
mod services;
mod utils;

use app_state::AppState;
use core::{ApiDoc, setup_database};
use routes::{health::health_router, user_router, post_router};
// Create API routes
fn create_api_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_router())
        .nest("/posts", post_router())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration and initialize database connection
    let (config, db) = setup_database().await?;

    // Create application state
    let app_state = AppState {
        db,
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
