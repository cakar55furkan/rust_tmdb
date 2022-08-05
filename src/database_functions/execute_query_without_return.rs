use sqlx::{Error, PgConnection, query};
use sqlx::postgres::PgQueryResult;

pub async fn execute_query(passed_query: &str, conn:&mut PgConnection) -> bool{
    let executable = sqlx::query(passed_query)
        .execute(conn).await;
    match executable {
        Ok(query_result) =>
            {
                println!("Inserted successfully!\nAffected rows:{}", query_result.rows_affected());
                return true
            },
        Err(e) => {
            println!("Couldn't insert to database!");
            println!("{:?}",e);
            return false
        }
    }
}