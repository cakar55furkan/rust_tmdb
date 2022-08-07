use string_builder::Builder;
use reqwest;
use std::fs::{create_dir, File};
use std::path::Path;
use futures::TryFutureExt;
use std::env;
use std::fmt::format;
use crate::search_movie::search_movie;
use crate::search_movie_structs::{SearchMovie, SearchMovieResultObject};
use serde::Deserialize;
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, Postgres};
use termion::color;
use crate::database_functions::execute_query_without_return::{insert_cast_person_to_movie_table, insert_movie_to_movie_table, insert_movie_cast_to_movie_cast_table, insert_genre_to_genre_table, insert_genre_to_genre_movie_table};
use crate::movie_credits::movie_credits::get_movie_credits;
use crate::movie_image::images::get_movie_images;

mod search_movie;
mod search_movie_structs;
mod movie_movie_id;
mod database_functions;
mod movie_credits;
mod people;
mod genre;
mod movie_image;

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


    let mut rte = movie_movie_id::get_movie::get_movie_details(search_results.results[0].id.to_string()).await;

    insert_movie_to_movie_table(&mut rte, &mut conn).await;

//    let credits = get_movie_credits(rte.id.to_string()).await;
    let credits = get_movie_credits(rte.id.to_string()).await;
    let mut successfully_inserted_cast = 0;
    let mut unsuccessfully_inserted_cast = 0;
    for n in 0..credits.cast.len()
    {
        let mut cast_people = people::get_cast::get_cast_details(credits.cast[n].id).await;
        //println!("{}, {}", cast_people.name.unwrap_or(String::from("NONE")), cast_people.place_of_birth.unwrap_or(String::from("NONE")));
        insert_cast_person_to_movie_table(&mut cast_people, &mut conn).await;
        // insert to movie_cast table
        let person_id = cast_people.id.unwrap_or(0);
        let movie_id = rte.id;
        let character = credits.cast[n].character.as_deref().unwrap_or("");
        let ordering = credits.cast[n].order;

        if insert_movie_cast_to_movie_cast_table(person_id, movie_id, character, ordering, &mut conn).await {
            successfully_inserted_cast += 1;
        }
        else {
            unsuccessfully_inserted_cast += 1;
        }
        //println!("{}, {}, {}, {}", person_id, movie_id, character, ordering);
    }
    println!("Number of Successful insert: {}", successfully_inserted_cast);
    println!("Number of Unsuccessful insert: {}", unsuccessfully_inserted_cast);

    if rte.genres.as_ref().unwrap().len() != 0 {
        for genre in rte.genres.unwrap(){
            //println!("{:?}", genre);
            insert_genre_to_genre_table(genre.id, &*genre.name, &mut conn).await;
            insert_genre_to_genre_movie_table(rte.id, genre.id, &mut conn).await;
        }
    }
    // call images and parse the json
    let mut image_json = get_movie_images(rte.id).await;

    for backdrop_image in image_json.backdrops{
        println!()
    }

    // then, insert it into database

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
