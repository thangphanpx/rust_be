use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

// Concrete response types for OpenAPI
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<UserResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<PaginatedUserResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<PostResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostsApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<PaginatedPostResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StringApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedUserResponse {
    pub data: Vec<UserResponse>,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedPostResponse {
    pub data: Vec<PostResponse>,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub is_published: bool,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostWithUserResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub is_published: bool,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
    pub user: UserResponse,
}