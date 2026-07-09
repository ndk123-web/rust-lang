use crate::dto::signup_dto::{SignUpDto, SignUpResponse};
use crate::models::user::User;
use crate::repositories::auth_repository;
use crate::utils::password::hash_password;
use sqlx::SqlitePool;

pub async fn create_user(pool: &SqlitePool, data: SignUpDto) -> anyhow::Result<SignUpResponse> {
    let final_hashed_password = hash_password(&data.password)
        .map_err(|err| anyhow::anyhow!("failed to hash password: {err}"))?;

    let user = User {
        id: 0,
        email: data.email,
        password_hash: final_hashed_password,
    };

    auth_repository::create_user(pool, user).await?;

    Ok(SignUpResponse {
        message: "Success Signup".to_string(),
    })
}
