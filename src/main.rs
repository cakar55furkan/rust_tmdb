use string_builder::Builder;
use reqwest;
use std::fs::{create_dir, File};
use std::path::Path;
use futures::TryFutureExt;
use std::env;
use std::fmt::format;
use crate::get_images::download_all_images_by_id;
use crate::search_movie::search_movie;
use crate::search_movie_structs::{SearchMovie, SearchMovieResultObject};
use serde::Deserialize;
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, Postgres};
use termion::color;

mod download_image;
mod get_images;
mod search_movie;
mod search_movie_structs;
mod movie_movie_id;
mod database_functions;

#[tokio::main]
async fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut user_input = String::new();
    println!("Enter Movie Name!");
    let b1 = std::io::stdin().read_line(&mut user_input).unwrap();
    let mut user_input_query = String::from(user_input);
    user_input_query = user_input_query.trim().parse().unwrap();
    let mut search_results = search_movie(user_input_query).await;

    print!("{}", color::Bg(color::Rgb(0, 51, 0)));
    println!(
        "{}------------------------------------------------",
        color::Fg(color::LightRed)
    );

    for search_result in search_results.results.iter() {
        print!("{}", color::Fg(color::LightWhite));
        println!("TMDB ID:\t{}", search_result.id);
        println!("Original Title:\t{}", search_result.original_title);
        println!("------------------------------------------------");
    }
    
    let mut conn = PgConnection::connect("postgres://furkancakar:123456@0.0.0.0:5432/furkancakar")
        .await
        .unwrap();
    /*
    let imdb_rating = sqlx::query!("select tconst, numvotes from title_ratings where tconst = 'tt0001963'")
        .fetch_all(&mut conn).await;

    for rec in imdb_rating{
        println!("{:?}", rec[0])
    }
    */


    let rte = movie_movie_id::get_movie::get_movie_details(search_results.results[0].id.to_string()).await;
    let mut my_query = format!("insert into movie values ('{}',{},{},{},'{}','{}','{}','{}',{},'{}','{}',{},'{}','{}',{},{})",
                               rte.backdrop_path,
                               rte.adult,
                               rte.budget,
                               rte.id,
                               rte.imdb_id,
                               rte.original_language,
                               rte.original_title,
                               rte.overview,
                               rte.popularity,
                               rte.poster_path,
                               rte.release_date,
                               rte.runtime,
                               rte.tagline.unwrap(),
                               rte.title,
                               rte.vote_average.unwrap(),
                               rte.vote_count.unwrap()
    );

    database_functions::execute_query_without_return::execute_query(&mut my_query, &mut conn).await;
    
/*    
    let all_images_of_movie = download_all_images_by_id(&search_results.results[0].id.to_string()).await;
    let mut movie_images_folder = String::from("/Users/furkancakar/Desktop/");
    movie_images_folder.push_str(&search_results.results[0].original_title);
    create_dir(movie_images_folder);

    let mut movie_images_backdrop_folder = String::from("/Users/furkancakar/Desktop/");
    movie_images_backdrop_folder.push_str(&search_results.results[0].original_title);
    movie_images_backdrop_folder.push_str("/backdrop");
    create_dir(&movie_images_backdrop_folder);

    for (index, backdrop) in all_images_of_movie.backdrops.iter().enumerate(){
        println!("{}/{}",index + 1, all_images_of_movie.backdrops.len() + 1);
        download_image::download_image(&backdrop.file_path, movie_images_backdrop_folder.as_str(), "500").await;
    }

    let mut movie_images_poster_folder = String::from("/Users/furkancakar/Desktop/");
    movie_images_poster_folder.push_str(&search_results.results[0].original_title);
    movie_images_poster_folder.push_str("/poster");
    create_dir(&movie_images_poster_folder);

    for (index, poster) in all_images_of_movie.posters.iter().enumerate(){
        println!("{}/{}",index + 1, all_images_of_movie.posters.len() + 1);
        download_image::download_image(&poster.file_path, movie_images_poster_folder.as_str(), "500").await;
    }

    */
     
}
