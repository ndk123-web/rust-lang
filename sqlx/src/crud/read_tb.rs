use sqlx::{FromRow, Pool, Sqlite};

/**
 * `FromRow` is Like Json Deserialization but for Sqlite Rows
 */
#[allow(dead_code)]
#[derive(FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[allow(dead_code)]
pub async fn read_rb_rs(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // it has 2 generic values (DBTYPE, Output Struct)
    // here DBTYPE = Sqlite
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT *
        FROM todos
        "#,
    )
    .fetch_all(pool)
    .await?;

    println!("Todos: \n{:#?}", todos);

    return Ok(());
}
