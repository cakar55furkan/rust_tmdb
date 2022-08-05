use sqlx::{Error, PgConnection, query};
use sqlx::postgres::PgQueryResult;

pub async fn execute_query(passed_query: &str, conn:&mut PgConnection) -> bool{
    let executable = sqlx::query(passed_query)
        .execute(conn).await;
    match executable {
        Ok(_) => println!("ok"),
        Err(_) => println!("error")
    }
    true
}