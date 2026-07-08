use crate::handlers::auth::{login, signup};
use axum::{Router, routing::get};
use crate::state::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    return Router::new()
        .route("/login", get(login))
        .route("/signup", get(signup));
}
