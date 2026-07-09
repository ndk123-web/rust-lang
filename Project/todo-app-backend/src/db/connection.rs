use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    return Ok(pool);
}
