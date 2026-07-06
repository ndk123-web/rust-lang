use sqlx::{Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

#[allow(dead_code)]
pub async fn connect_sqlit() -> Result<sqlx::Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite://todo.db")?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    return Ok(pool);
}
