use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignUpDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub message: String,
}
