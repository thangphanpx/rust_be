use crate::core::error::AppError;
use crate::models::user::Model as UserModel;
use crate::repositories::user_repo::UserRepository;
use crate::schemas::{CreateUserRequest, UpdateUserRequest, PaginationParams};
use sea_orm::DatabaseConnection;

#[derive(Debug)]
#[allow(dead_code)]
pub struct UserService {
    db: DatabaseConnection,
}

#[allow(dead_code)]
impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Tạo user mới với password hashing
    pub async fn create_user(&self, user_data: &CreateUserRequest) -> Result<UserModel, AppError> {
        // Hash password
        let hashed_password = hash_password(&user_data.password)?;

        // Create user with hashed password
        let user_data = CreateUserRequest {
            email: user_data.email.clone(),
            username: user_data.username.clone(),
            password: hashed_password,
            full_name: user_data.full_name.clone(),
        };

        UserRepository::create(&self.db, &user_data).await.map_err(Into::into)
    }

    /// Lấy danh sách users với pagination
    pub async fn get_users(&self, params: &PaginationParams) -> Result<Vec<UserModel>, AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);
        
        let users = UserRepository::find_all(&self.db, page, limit).await?;
        Ok(users)
    }

    /// Lấy user theo ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<UserModel>, AppError> {
        let user = UserRepository::find_by_id(&self.db, id).await?;
        Ok(user)
    }

    /// Update user
    pub async fn update_user(&self, id: i32, user_data: &UpdateUserRequest) -> Result<Option<UserModel>, AppError> {
        let updated_user = UserRepository::update(&self.db, id, user_data).await?;
        Ok(updated_user)
    }

    /// Delete user
    pub async fn delete_user(&self, id: i32) -> Result<bool, AppError> {
        let deleted = UserRepository::delete(&self.db, id).await?;
        Ok(deleted)
    }

    /// Lấy tổng số users
    pub async fn get_user_count(&self) -> Result<i64, AppError> {
        let count = UserRepository::count(&self.db).await?;
        Ok(count)
    }
}

/// Hash password using bcrypt
fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))
}