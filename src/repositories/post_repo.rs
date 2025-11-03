use sqlx::PgPool;
use crate::models::Post;
use crate::schemas::{CreatePostRequest, UpdatePostRequest};

#[derive(Debug)]
#[allow(dead_code)]
pub struct PostRepository;

#[allow(dead_code)]
impl PostRepository {
    pub async fn create(pool: &PgPool, user_id: i32, post_data: &CreatePostRequest) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, user_id, is_published)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, user_id, is_published, created_at, updated_at
            "#,
            post_data.title,
            post_data.content,
            user_id,
            post_data.is_published.unwrap_or(false)
        )
        .fetch_one(pool)
        .await?;

        Ok(post)
    }

    pub async fn find_all(pool: &PgPool, page: u64, limit: u64) -> Result<Vec<Post>, sqlx::Error> {
        let offset = (page - 1) * limit;
        
        let posts = sqlx::query_as!(
            Post,
            "SELECT * FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(post)
    }

    pub async fn update(pool: &PgPool, id: i32, post_data: &UpdatePostRequest) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts 
            SET title = COALESCE($2, title),
                content = COALESCE($3, content),
                is_published = COALESCE($4, is_published),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, title, content, user_id, is_published, created_at, updated_at
            "#,
            id,
            post_data.title,
            post_data.content,
            post_data.is_published
        )
        .fetch_optional(pool)
        .await?;

        Ok(post)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM posts WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM posts"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0))
    }
}