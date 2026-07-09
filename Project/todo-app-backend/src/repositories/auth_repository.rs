use crate::models::user::User;
use sqlx::SqlitePool;

pub async fn create_user(pool: &SqlitePool, data: User) -> Result<(), sqlx::Error> {
    println!("CREATE USER: {:#?}", data);
    println!("CREATE USER: {:#?}", pool);

    sqlx::query(
        "
    INSERT INTO users(email, password_hash)
    VALUES (?, ?)
    ",
    )
    .bind(data.email)
    .bind(data.password_hash)
    .execute(pool)
    .await?;

    Ok(())
}
