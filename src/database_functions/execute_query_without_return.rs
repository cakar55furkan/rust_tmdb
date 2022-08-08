use std::cmp;
use crate::movie_detail::get_movie_structs::MovieMovieId;
use crate::people::people_structs::cast;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, Error, PgConnection};
use termion::color;
use crate::genre::genre_structs::genre;
use crate::movie_image::image_structs::movie_image;

pub async fn execute_query(passed_query: &str, conn: &mut PgConnection) -> bool {
    let executable = sqlx::query(passed_query).execute(conn).await;
    match executable {
        Ok(_) => {
            print!("{}", color::Fg(color::LightGreen));
            println!("[SUCCESS]\t{}", String::from(&passed_query[..cmp::min(75, passed_query.len())]) + "...");
            print!("{}", color::Fg(color::Reset));
            return true;
        }
        Err(e) => {
            print!("{}", color::Fg(color::LightRed));
            println!("[ERROR]\t{}", String::from(&passed_query[..cmp::min(75, passed_query.len())]) + "...");
            print!("{}", color::Fg(color::Reset));
            return false;
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

    print!("{}", color::Fg(color::LightWhite));
    print!("{}","");
    println!("Movie Insert: {}====================================", movie_object.title);
    print!("{}", color::Bg(color::Reset));
    print!("{}", color::Fg(color::Reset));
    execute_query(&mut my_query, conn).await;
}

pub async fn insert_cast_person_to_movie_table(cast_object: &cast, conn: &mut PgConnection) {
    let mut my_query = format!(
        "insert into cast_person values ({},'{}','{}','{}',{},'{}',{},'{}','{}','{}','{}',{},'{}')",
        cast_object.adult.unwrap_or("false".parse().unwrap()),
        cast_object.biography.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\"")
            .replace("ː", ":")
            .replace("ˌ", ","),
        cast_object.birthday.as_deref().unwrap_or("1970-01-01")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.deathday.as_deref().unwrap_or("1970-01-01")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.gender.unwrap_or("0".parse().unwrap()),
        cast_object.homepage.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.id.unwrap_or("0".parse().unwrap()),
        cast_object.imdb_id.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.known_for_department.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.name.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.place_of_birth.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        cast_object.popularity.unwrap_or("0.0".parse().unwrap()),
        cast_object.profile_path.as_deref().unwrap_or("")
            .replace("\'", "\'\'")
            .replace("\"", "\"\"")
    );
    execute_query(&mut my_query, conn).await;

}

pub async fn insert_movie_cast_to_movie_cast_table(person_id:i32, movie_id:i32, character:&str, ordering: i32, conn: &mut PgConnection)  {
    let mut my_query = format!(
        "insert into movie_cast values ({},{},'{}',{})",
        person_id,
        movie_id,
        character
            .replace("\'", "\'\'")
            .replace("\"", "\"\""),
        ordering
    );
    execute_query(&mut my_query, conn).await;
}

pub async fn insert_genre_to_genre_table(id: i32, name:&str, conn: &mut PgConnection) -> bool {
    let mut my_query = format!(
        "insert into genre values ({},'{}')", id, String::from(name));
    execute_query(&mut my_query, conn).await
}

pub async fn insert_genre_to_genre_movie_table(movie_id: i32, genre_id: i32, conn: &mut PgConnection) -> bool {
    let mut my_query = format!(
        "insert into movie_genre values ({},'{}')", movie_id, genre_id);

    execute_query(&mut my_query, conn).await
}

pub async fn insert_image_to_image_table(movie_id: i32, image:movie_image, image_type: &str, conn: &mut PgConnection) -> bool {
    let mut my_query = format!(
        "insert into image values ({}, {}, {}, '{}', '{}', {}, {}, {}, '{}')",
        image.aspect_ratio,
        image.width,
        image.height,
        image.iso_639_1.as_deref().unwrap_or(""),
        image.file_path,
        image.vote_average,
        image.vote_count,
        movie_id,
        image_type
    );
    return execute_query(&mut my_query, conn).await
}