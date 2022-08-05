use reqwest;
use std::fs::{create_dir, File};
use std::path::Path;
use futures::TryFutureExt;
use std::env;
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
        println!("Original Title:\t{}", search_result.original_title);
        println!("Popularity:\t{}", search_result.popularity);
        match &search_result.release_date {
            Some(success) => println!("Release Date:\t{}", success),
            None => println!("Release Date:\tNOT FOUND!"),
        }
        println!("Vote Count:\t{}", search_result.vote_count);
        print!("{}", color::Fg(color::LightRed));
        println!("------------------------------------------------");
    }
    
    let mut conn = PgConnection::connect("postgres://furkancakar:123456@0.0.0.0:5432/furkancakar")
        .await
        .unwrap();


    let executable = sqlx::query!("insert into title_ratings values ('31', 31.31, 31)")
        .execute(&mut conn).await;
    match executable {
        Ok(t) => println!("Successfully Inserted Item"),
        Err(e) => println!("{:?}", e),
    };

    let imdb_rating = sqlx::query!("select tconst, numvotes from title_ratings where tconst = 'tt0001963'")
        .fetch_all(&mut conn).await;

    for rec in imdb_rating{
        println!("{:?}", rec[0])
    }


    print!("{}", color::Fg(color::LightWhite));
    let rte = movie_movie_id::get_movie::get_movie_details(search_results.results[0].id.to_string()).await;
    println!("{:?}", rte);


    
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
