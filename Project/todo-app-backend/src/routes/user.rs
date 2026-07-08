use axum::{Router, routing::get};

use crate::handlers::user::get_profile;

pub fn user_router() -> Router {
    return Router::new().route("/profile", get(get_profile));
}
