use crate::dto::signup_dto::SignUpDto;
use crate::services::auth_service;
use crate::state::state::AppState;
use axum::extract::{Json, State};

pub async fn login(State(app_state): State<AppState>) -> &'static str {
    println!("App State: {:#?}", app_state);
    return "Login";
}

pub async fn signup(
    State(app_state): State<AppState>,
    Json(data): Json<SignUpDto>,
) -> &'static str {
    auth_service::create_user(&app_state.pool, data).await;

    return "Signup Success";
}
