use sqlx::{Error, Pool, Sqlite};

pub async fn insert_tb_rs(pool: &Pool<Sqlite>) -> Result<(), Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO todos (title, completed)
        VALUES (?, ?)
        "#,
    )
    .bind("Learn Rust")
    .bind(false)
    .execute(pool)
    .await?;

    println!("Inserted!! {:#?}", result);

    return Ok(());
}
