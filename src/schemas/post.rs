use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 1))]
    pub content: String,
    
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    
    #[validate(length(min = 1))]
    pub content: Option<String>,
    
    pub is_published: Option<bool>,
}