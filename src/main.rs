use std::borrow::Borrow;
use string_builder::Builder;
use reqwest;
use std::fs::{create_dir, File};
use std::path::Path;
use futures::TryFutureExt;
use std::env;
use std::fmt::format;
use std::io::Read;
use crate::search_movie::search_movie;
use crate::search_movie_structs::{SearchMovie, SearchMovieResultObject};
use serde::Deserialize;
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, Postgres, Row};
use crate::database_functions::execute_query_without_return::{insert_cast_person_to_movie_table,
                                                              insert_movie_to_movie_table,
                                                              insert_movie_cast_to_movie_cast_table,
                                                              insert_genre_to_genre_table,
                                                              insert_genre_to_genre_movie_table,
                                                              insert_image_to_image_table};

use crate::movie_credits::movie_credits::get_movie_credits;
use crate::movie_image::images::get_movie_images;
use log::{info, warn, error};// also we must use env_logger

mod search_movie;
mod search_movie_structs;
mod movie_detail;
mod database_functions;
mod movie_credits;
mod people;
mod genre;
mod movie_image;

#[tokio::main]
async fn main() {
    let mut file = File::open("movie_list.txt")
        .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let string_array = data.split("\n");
    for line in string_array{
        search_fetch(String::from(line)).await;
    }
}


async fn search_fetch (movie_id: String) {
    //
    // let mut user_input_query = movie_id.trim().parse().unwrap();
    // let mut search_results = search_movie(user_input_query).await;

    let mut conn = PgConnection::connect("postgres://furkancakar:123456@0.0.0.0:5432/furkancakar")
        .await
        .unwrap();

    //let check_unique_movie_query = format!("select if from movie where id = {}", search_results.results[0].id.to_string());


    let mut rte = movie_detail::get_movie::get_movie_details(movie_id).await;

    insert_movie_to_movie_table(&mut rte, &mut conn).await;

//    let credits = get_movie_credits(rte.id.to_string()).await;
    let credits = get_movie_credits(rte.id.to_string()).await;
    let mut successfully_inserted_cast = 0;
    let mut unsuccessfully_inserted_cast = 0;
    for n in 0..credits.cast.len()
    {
        // insert person
        let mut cast_people = people::get_cast::get_cast_details(credits.cast[n].id).await;
        insert_cast_person_to_movie_table(&mut cast_people, &mut conn).await;

        // insert character to movie_cast table
        let person_id = cast_people.id.unwrap_or(0);
        let movie_id = rte.id;
        let character = credits.cast[n].character.as_deref().unwrap_or("");
        let ordering = credits.cast[n].order;
        insert_movie_cast_to_movie_cast_table(person_id, movie_id, character, ordering, &mut conn).await;
    }

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
        insert_image_to_image_table(rte.id, backdrop_image, "backdrop", &mut conn).await;
    }
    for poster_image in image_json.posters{
        insert_image_to_image_table(rte.id, poster_image, "poster", &mut conn).await;
    }
    for logo in image_json.logos{
        insert_image_to_image_table(rte.id, logo, "logo", &mut conn).await;
    }
}