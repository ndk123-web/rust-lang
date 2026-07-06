mod crud;
mod db;
mod transaction;

use crud::{create_tb, delete_tb, insert_tb, read_tb, update_tb};
use db::sqlite_connection;
use transaction::transactions;

#[warn(dead_code, unused)]
#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // connect sqlite
    let conn_sqlite_pool = sqlite_connection::connect_sqlit().await?;

    // Create a table
    let _ = create_tb::create_tb_fn(&conn_sqlite_pool).await;

    // insert a row
    let _ = insert_tb::insert_tb_rs(&conn_sqlite_pool).await;

    // read from table
    let _ = read_tb::read_rb_rs(&conn_sqlite_pool).await;

    // update from table
    let _ = update_tb::update_tb_rs(&conn_sqlite_pool, 1 as i64, true).await;

    // delete a row
    let _ = delete_tb::delete_tb_fn(&conn_sqlite_pool, 2 as i64);

    // transaction
    let _ = transactions::transaction_rs(&conn_sqlite_pool).await;

    Ok(())
}
