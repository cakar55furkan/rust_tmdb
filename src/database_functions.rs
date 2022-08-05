use sqlx::{PgConnection, query};

pub async fn execute_query(passed_query: &str, conn:&mut PgConnection) -> bool{
    let executable = sqlx::query!(passed_query)
        .execute(conn).await;
    match executable {
        Ok(t) => true,
        Err(e) => false
    }
}