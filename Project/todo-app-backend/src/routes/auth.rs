use crate::handlers::auth::{login, signup};
use crate::state::state::AppState;
use axum::{Router, routing::get, routing::post};

pub fn auth_routes() -> Router<AppState> {
    return Router::new()
        .route("/login", get(login))
        .route("/signup", post(signup));
}
