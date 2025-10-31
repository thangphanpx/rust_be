use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{config::AppState, handlers::user};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", post(user::create_user))
        .route("/", get(user::get_users))
        .route("/:id", get(user::get_user_by_id))
        .route("/:id", put(user::update_user))
        .route("/:id", delete(user::delete_user))
}