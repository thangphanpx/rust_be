use sea_orm::{Database, DatabaseConnection};
use crate::core::config::AppConfig;

/// Creates a database connection using the provided database URL
pub async fn create_database_connection(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(database_url).await
}

/// Loads configuration and creates database connection
pub async fn setup_database() -> Result<(AppConfig, DatabaseConnection), Box<dyn std::error::Error>> {
    let config = crate::core::config::load_config()?;
    let db = create_database_connection(&config.database_url).await?;
    Ok((config, db))
}