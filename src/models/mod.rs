pub mod user;
pub mod post;

// Database types (if needed for external use)
#[allow(dead_code)]
pub type UserModel = user::Model;
#[allow(dead_code)]
pub type PostModel = post::Model;