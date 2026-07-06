use sqlx::{Pool, Sqlite};

pub async fn transaction_rs(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {

    // create a transaction 
    let mut tx = pool.begin().await?;

    // here execute we gave transaction object which has connections 
    sqlx::query("INSERT INTO todos(title, completed) VALUES (?, ?)")
        .bind("Learn Go")
        .bind(false)
        .execute(&mut *tx)
        .await?;

    // here execute we gave transaction object which has connections
    sqlx::query("INSERT INTO todos(title, completed) VALUES (?, ?)")
        .bind("Learn CPP")
        .bind(false)
        .execute(&mut *tx)
        .await?;

    // after done commit it else if any fails before the ROlLBACK happens
    let _ = tx.commit().await;

    println!("Transaction Implemented!");

    return Ok(());
}
