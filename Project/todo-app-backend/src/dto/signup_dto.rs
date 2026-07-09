use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpDto {
    pub email: String,
    pub password: String,
}
