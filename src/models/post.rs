use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 1))]
    pub content: String,
    
    pub is_published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    
    #[validate(length(min = 1))]
    pub content: Option<String>,
    
    pub is_published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostWithUserResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: super::user::UserResponse,
}