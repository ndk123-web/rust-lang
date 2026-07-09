use crate::dto::signup_dto::SignUpDto;
use crate::services::auth_service;
use crate::state::state::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn login(State(app_state): State<AppState>) -> impl IntoResponse {
    println!("App State: {:#?}", app_state);

    (
        StatusCode::OK,
        Json(serde_json::json!({"message": "Login"})),
    )
}

pub async fn signup(
    State(app_state): State<AppState>,
    Json(data): Json<SignUpDto>,
) -> impl IntoResponse {
    match auth_service::create_user(&app_state.pool, data).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(err) => {
            eprintln!("Signup failed: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Signup failed"})),
            )
                .into_response()
        }
    }
}
