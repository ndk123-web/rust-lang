use sqlx::{Pool, Sqlite};

#[allow(dead_code)]
pub async fn delete_tb_fn(pool: &Pool<Sqlite>, id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
            DELETE FROM todos
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    println!("Deleted!! {:#?}", result);

    return Ok(());
}
