use crate::core::error::AppError;
use crate::models::post::Model as PostModel;
use crate::repositories::post_repo::PostRepository;
use crate::schemas::{CreatePostRequest, UpdatePostRequest, PaginationParams};
use sea_orm::DatabaseConnection;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PostService {
    db: DatabaseConnection,
}

#[allow(dead_code)]
impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Tạo post mới
    pub async fn create_post(&self, user_id: i32, post_data: &CreatePostRequest) -> Result<PostModel, AppError> {
        let post = PostRepository::create(&self.db, user_id, post_data).await?;
        Ok(post)
    }

    /// Lấy danh sách posts với pagination
    pub async fn get_posts(&self, params: &PaginationParams) -> Result<Vec<PostModel>, AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);
        
        let posts = PostRepository::find_all(&self.db, page, limit).await?;
        Ok(posts)
    }

    /// Lấy post theo ID
    pub async fn get_post_by_id(&self, id: i32) -> Result<Option<PostModel>, AppError> {
        let post = PostRepository::find_by_id(&self.db, id).await?;
        Ok(post)
    }

    /// Update post
    pub async fn update_post(&self, id: i32, post_data: &UpdatePostRequest) -> Result<Option<PostModel>, AppError> {
        let updated_post = PostRepository::update(&self.db, id, post_data).await?;
        Ok(updated_post)
    }

    /// Delete post
    pub async fn delete_post(&self, id: i32) -> Result<bool, AppError> {
        let deleted = PostRepository::delete(&self.db, id).await?;
        Ok(deleted)
    }

    /// Lấy tổng số posts
    pub async fn get_post_count(&self) -> Result<i64, AppError> {
        let count = PostRepository::count(&self.db).await?;
        Ok(count)
    }

    /// Lấy posts của user cụ thể
    pub async fn get_posts_by_user(&self, user_id: i32, params: &PaginationParams) -> Result<Vec<PostModel>, AppError> {
        // TODO: Implement trong PostRepository khi có proper relations
        // Tạm thời lấy tất cả và filter
        let all_posts = self.get_posts(params).await?;
        Ok(all_posts.into_iter().filter(|post| post.user_id == user_id).collect())
    }

    /// Lấy posts đã published
    pub async fn get_published_posts(&self, params: &PaginationParams) -> Result<Vec<PostModel>, AppError> {
        // TODO: Implement trong PostRepository
        // Tạm thời lấy tất cả và filter
        let all_posts = self.get_posts(params).await?;
        Ok(all_posts.into_iter().filter(|post| post.is_published).collect())
    }
}