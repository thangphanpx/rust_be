use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{config::AppState, handlers::post};

pub fn post_router() -> Router<AppState> {
    Router::new()
        .route("/", post(post::create_post))
        .route("/", get(post::get_posts))
        .route("/:id", get(post::get_post_by_id))
        .route("/:id", put(post::update_post))
        .route("/:id", delete(post::delete_post))
}