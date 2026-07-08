use crate::handlers::auth::{login, signup};
use axum::{Router, routing::get};

pub fn auth_routes() -> Router {
    return Router::new()
        .route("/login", get(login))
        .route("/signup", get(signup));
}
