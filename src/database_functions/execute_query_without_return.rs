use sqlx::{Error, PgConnection, query};
use sqlx::postgres::PgQueryResult;
use crate::movie_movie_id::get_movie_structs::MovieMovieId;

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

pub async fn insert_movie_to_movie_table(movie_object: &MovieMovieId, conn: &mut PgConnection) {
    let mut my_query = format!("insert into movie values ('{}',{},{},{},'{}','{}','{}','{}',{},'{}','{}',{},'{}','{}',{},{})",
                               movie_object.backdrop_path,
                               movie_object.adult,
                               movie_object.budget,
                               movie_object.id,
                               movie_object.imdb_id,
                               movie_object.original_language,
                               movie_object.original_title,
                               movie_object.overview.replace("\'", "\'\'").replace("\"", "\"\""),
                               movie_object.popularity,
                               movie_object.poster_path,
                               movie_object.release_date,
                               movie_object.runtime,
                               movie_object.tagline.as_deref().unwrap_or("").replace("\'", "\'\'").replace("\"", "\"\""),
                               movie_object.title,
                               movie_object.vote_average.unwrap_or("0.0".parse().unwrap()),
                               movie_object.vote_count.unwrap_or(0)
    );

    execute_query(&mut my_query, conn).await;
}