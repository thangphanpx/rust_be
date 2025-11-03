use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, IntoActiveModel, QuerySelect, PaginatorTrait, ModelTrait};
use crate::models::user::{self, Model as UserModel};
use crate::schemas::{CreateUserRequest, UpdateUserRequest};

#[derive(Debug)]
#[allow(dead_code)]
pub struct UserRepository;

#[allow(dead_code)]
impl UserRepository {
    pub async fn create(db: &DatabaseConnection, user_data: &CreateUserRequest) -> Result<UserModel, DbErr> {
        // TODO: Implement actual password hashing
        // In a real implementation, you would hash the password here
        
        let user = user::ActiveModel {
            email: sea_orm::Set(user_data.email.clone()),
            username: sea_orm::Set(user_data.username.clone()),
            password_hash: sea_orm::Set(user_data.password.clone()), // Should be hashed
            full_name: sea_orm::Set(user_data.full_name.clone()),
            is_active: sea_orm::Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(user)
    }

    pub async fn find_all(db: &DatabaseConnection, page: u64, limit: u64) -> Result<Vec<UserModel>, DbErr> {
        let offset = (page - 1) * limit;
        
        let users = user::Entity::find()
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<UserModel>, DbErr> {
        let user = user::Entity::find_by_id(id)
            .one(db)
            .await?;

        Ok(user)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, user_data: &UpdateUserRequest) -> Result<Option<UserModel>, DbErr> {
        // First check if user exists
        let existing_user = user::Entity::find_by_id(id)
            .one(db)
            .await?;
        
        if let Some(user) = existing_user {
            let mut user_model = user.into_active_model();
            
            if let Some(email) = &user_data.email {
                user_model.email = sea_orm::Set(email.clone());
            }
            if let Some(username) = &user_data.username {
                user_model.username = sea_orm::Set(username.clone());
            }
            if let Some(full_name) = &user_data.full_name {
                user_model.full_name = sea_orm::Set(Some(full_name.clone()));
            }
            if let Some(is_active) = user_data.is_active {
                user_model.is_active = sea_orm::Set(is_active);
            }

            let updated_user = user_model.update(db).await?;
            Ok(Some(updated_user))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        let user = user::Entity::find_by_id(id)
            .one(db)
            .await?;
            
        if let Some(user_model) = user {
            user_model.delete(db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn count(db: &DatabaseConnection) -> Result<i64, DbErr> {
        let count = user::Entity::find()
            .count(db)
            .await? as i64;

        Ok(count)
    }
}