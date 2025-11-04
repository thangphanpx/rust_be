use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    #[allow(dead_code)]
    pub server_host: String,
    #[allow(dead_code)]
    pub server_port: u16,
    #[allow(dead_code)]
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            jwt_secret: std::env::var("JWT_SECRET")?,
        })
    }
    
    #[allow(dead_code)]
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    AppConfig::from_env()
}