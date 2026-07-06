use sqlx::Sqlite;

/**
 * Use .Execute when u dont need rows
 */

pub async fn create_tb_fn(pool: &sqlx::Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    println!("Table Created!! {:#?}", result);

    Ok(())
}
