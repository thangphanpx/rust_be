use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 6))]
    pub password: String,
    
    #[validate(length(min = 1, max = 100))]
    pub full_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub full_name: Option<String>,
    
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}