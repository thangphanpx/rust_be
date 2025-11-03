use sqlx::PgPool;
use crate::models::User;
use crate::schemas::{CreateUserRequest, UpdateUserRequest};

#[derive(Debug)]
#[allow(dead_code)]
pub struct UserRepository;

#[allow(dead_code)]
impl UserRepository {
    pub async fn create(pool: &PgPool, user_data: &CreateUserRequest) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash, full_name)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, password_hash, full_name, is_active, created_at, updated_at
            "#,
            user_data.email,
            user_data.username,
            user_data.password, // Note: In real app, this should be hashed
            user_data.full_name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(pool: &PgPool, page: u64, limit: u64) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * limit;
        
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn update(pool: &PgPool, id: i32, user_data: &UpdateUserRequest) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users 
            SET email = COALESCE($2, email),
                username = COALESCE($3, username),
                full_name = COALESCE($4, full_name),
                is_active = COALESCE($5, is_active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, email, username, password_hash, full_name, is_active, created_at, updated_at
            "#,
            id,
            user_data.email,
            user_data.username,
            user_data.full_name,
            user_data.is_active
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0))
    }
}