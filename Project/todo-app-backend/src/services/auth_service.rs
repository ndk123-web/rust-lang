use crate::dto::signup_dto::SignUpDto;
use crate::models::user::User;
use crate::repositories::auth_repository;
use sqlx::SqlitePool;

pub async fn create_user(pool: &SqlitePool, data: SignUpDto) {
    let user = User {
        id: 0,
        email: data.email,
        password_hash: data.password,
    };

    auth_repository::create_user(pool, user).await;
}
