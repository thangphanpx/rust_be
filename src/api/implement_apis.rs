use axum::Router;

use crate::config::AppState;
use crate::routes::{
    health,
    posts,
    productions,
    users,
};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/users", users::user_router())
        .nest("/posts", posts::post_router())
        .nest("/productions", productions::production_router())
        // Thêm các routes khác ở đây
        // .nest("/orders", orders::order_router())
        // .nest("/commands", commands::command_router())
        // .nest("/warehouse", warehouse::warehouse_router())
}