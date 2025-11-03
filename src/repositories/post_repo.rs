use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, IntoActiveModel, QuerySelect, PaginatorTrait, ModelTrait};
use crate::models::post::{self, Model as PostModel};
use crate::schemas::{CreatePostRequest, UpdatePostRequest};

#[derive(Debug)]
#[allow(dead_code)]
pub struct PostRepository;

#[allow(dead_code)]
impl PostRepository {
    pub async fn create(db: &DatabaseConnection, user_id: i32, post_data: &CreatePostRequest) -> Result<PostModel, DbErr> {
        let post = post::ActiveModel {
            title: sea_orm::Set(post_data.title.clone()),
            content: sea_orm::Set(post_data.content.clone()),
            user_id: sea_orm::Set(user_id),
            is_published: sea_orm::Set(post_data.is_published.unwrap_or(false)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(post)
    }

    pub async fn find_all(db: &DatabaseConnection, page: u64, limit: u64) -> Result<Vec<PostModel>, DbErr> {
        let offset = (page - 1) * limit;
        
        let posts = post::Entity::find()
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(posts)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PostModel>, DbErr> {
        let post = post::Entity::find_by_id(id)
            .one(db)
            .await?;

        Ok(post)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, post_data: &UpdatePostRequest) -> Result<Option<PostModel>, DbErr> {
        // First check if post exists
        let existing_post = post::Entity::find_by_id(id)
            .one(db)
            .await?;
        
        if let Some(post) = existing_post {
            let mut post_model = post.into_active_model();
            
            if let Some(title) = &post_data.title {
                post_model.title = sea_orm::Set(title.clone());
            }
            if let Some(content) = &post_data.content {
                post_model.content = sea_orm::Set(content.clone());
            }
            if let Some(is_published) = post_data.is_published {
                post_model.is_published = sea_orm::Set(is_published);
            }

            let updated_post = post_model.update(db).await?;
            Ok(Some(updated_post))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        let post = post::Entity::find_by_id(id)
            .one(db)
            .await?;
            
        if let Some(post_model) = post {
            post_model.delete(db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn count(db: &DatabaseConnection) -> Result<i64, DbErr> {
        let count = post::Entity::find()
            .count(db)
            .await? as i64;

        Ok(count)
    }
}