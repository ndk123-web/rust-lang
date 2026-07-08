use axum::{Router, routing::get};
use crate::state::state::AppState;

use crate::handlers::user::get_profile;

pub fn user_router() -> Router<AppState> {
    return Router::new().route("/profile", get(get_profile));
}
