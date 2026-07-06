use sqlx::{Pool, Sqlite};

#[allow(dead_code)]
pub async fn update_tb_rs(
    pool: &Pool<Sqlite>,
    id: i64,
    completed: bool,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE todos
        SET completed = ?
        WHERE id = ?
        "#,
    )
    .bind(completed)
    .bind(id)
    .execute(pool)
    .await?;

    println!("Update Done!! {:#?}", result);

    return Ok(());
}
