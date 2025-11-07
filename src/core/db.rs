use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::core::config::AppConfig;

/// Creates a database connection using the provided database URL
pub async fn create_database_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}

/// Runs database migrations
pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // For now, we'll just log that migrations should be run
    // In a real application, you would use sea-orm-cli to run migrations
    tracing::info!("Database connection established. Migrations should be run using sea-orm-cli.");
    Ok(())
}

/// Loads configuration, creates database connection, and runs migrations
pub async fn setup_database() -> Result<(AppConfig, DatabaseConnection), Box<dyn std::error::Error>> {
    let config = crate::core::config::load_config()?;
    let db = create_database_connection(&config.database_url).await?;
    
    // Run migrations
    tracing::info!("Connecting to database...");
    run_migrations(&db).await?;
    tracing::info!("Database setup completed successfully.");
    
    Ok((config, db))
}