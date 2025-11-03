use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relations will be added back after fixing the relation syntax
}

impl ActiveModelBehavior for ActiveModel {}