use crate::state::state::AppState;
use axum::extract::State;

pub async fn login(State(app_state): State<AppState>) -> &'static str {
    println!("App State: {:#?}", app_state);
    return "Login";
}

pub async fn signup() -> &'static str {
    return "Signup";
}
