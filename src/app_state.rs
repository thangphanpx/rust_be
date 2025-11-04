use sea_orm::DatabaseConnection;
use crate::core::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    #[allow(dead_code)]
    pub config: AppConfig,
}