use crate::models::user::User;
use sqlx::SqlitePool;

pub async fn create_user(pool: &SqlitePool, data: User) {
    println!("CREATE USER: {:#?}", data);
    println!("CREATE USER: {:#?}", pool);
}
